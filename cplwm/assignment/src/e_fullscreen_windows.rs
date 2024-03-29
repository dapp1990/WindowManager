//! Optional: Fullscreen Windows
//!
//! Extend your window manager with support for fullscreen windows, i.e. the
//! ability to temporarily make a window take up the whole screen, thereby
//! obscuring all other windows. See the documentation of the
//! [`FullscreenSupport`] trait for the precise requirements. Don't confuse
//! this with the first assignment, in which you built a window manager that
//! displayed all windows fullscreen.
//!
//! Like in the previous assignments, either make a copy of, or define a
//! wrapper around your previous window manager to implement the
//! [`FullscreenSupport`] trait as well. Note that this window manager must
//! still implement all the traits from previous assignments.
//!
//! [`FullscreenSupport`]: ../../cplwm_api/wm/trait.FullscreenSupport.html
//!
//! # Status
//!
//! **TODO**: Replace the question mark below with YES, NO, or PARTIAL to
//! indicate the status of this assignment. If you want to tell something
//! about this assignment to the grader, e.g., you have a bug you can't fix,
//! or you want to explain your approach, write it down after the comments
//! section.
//!
//! COMPLETED: YES
//!
//! COMMENTS:
//!
//! ## General approach
//! I take advantage of the boolean attribute already stored in the
//! MinisedWindow structure used in the last excersice, the major change now
//! are in *get_window_layout* where now it should hide the windows and show
//! just the fullscreen one if there is one. I updated the name of the
//! structure to make it consistent with the name of the window manager
//!
//! Fucntions *add_window*, *focus_window*, *swap_with_master*,
//! *swap_windows*, *toggle_minimised* and *toggle_floating*, *remove_window*,
//! *resize_screen* and *toggle_fullscreen* affect the state
//! of the fullscreen.
//!
//! The *add_window*, *focus_window*, *swap_with_master*, *swap_windows*
//! funtions involve to change the focused window, the focused window is
//! always the fullscreen if there is one. Hence one has to "remove"
//! the fullscreen window in order to navigate to the new focused element.
//! The fullscreen is setting back to the previous window type (tiled or
//! floating).
//!
//! Funtions *toggle_minimised*, *toggle_floating*, *remove_window* affect
//! the fullscreen window only if it is applied to the fullscreen window
//! itself. If other non-fullscreen window is used, then it is done in a
//! second plane and it will only visible (unhide) by *get_window_layout*
//! when the fullscreen is disable or when you call *get_window_info* funtion.
//!
//! Funtion *resize_screen* only affects the fullscreen in terms of geometry,
//! since the screen is updated the fullscreen screen should be updated as
//! well as the other tiled windows.
//!
//! All get funtions do not affect fullscreen since they are only retriving
//! information of the current state of the window manager.
//!
//! I see *set_window_geometry* funtion as a speacial case, it can be applied
//! to a floating windows and they will be updated in a second plane,
//! but if it is applied to the fullscren window a NoFloatingWindow error is
//! thrown, instead.

// Add imports here
use std::error;
use std::fmt;

use cplwm_api::types::{FloatOrTile, Geometry, PrevOrNext, Screen, Window, WindowLayout,
                       WindowWithInfo};
use cplwm_api::wm::WindowManager;
use cplwm_api::wm::TilingSupport;
use cplwm_api::wm::FloatSupport;
use cplwm_api::wm::MinimiseSupport;
use cplwm_api::wm::FullscreenSupport;

/// **TODO**: Documentation
pub type WMName = FullscreenWM;

/// The FullscreenWindow struct
#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct FullscreenWindow {
    /// The window.
    pub window: Window,
    /// The geometry of the window.
    pub geometry: Geometry,
    /// The saved floating geometry of the window.
    pub saved_geometry: Geometry,
    /// Indicate whether the window should float or tile.
    pub float_or_tile: FloatOrTile,
    /// Indicate whether the window should be displayed fullscreen or not.
    pub fullscreen: bool,
    /// Indicate whether the window is minimised or not.
    pub minimised: bool,
}

#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
/// TODO: *documentation*
pub struct FullscreenWM {
    /// Vector where the elementes are minimised windows
    pub windows: Vec<FullscreenWindow>,
    /// Vector that stores the order in which the windows were minimised
    pub minimised_windows: Vec<Window>,
    /// The size of the screen.
    pub screen: Screen,
    /// The index of the focused window in the collection, if there is no
    /// focused window a None is placed
    pub index_foused_window: Option<usize>,
}

/// Supported functions
impl FullscreenWM {
    /// The method was upgraded to skip not only floating but also minimised
    /// windows
    pub fn get_next_tile_index(&self, index: usize, saved: usize) -> Option<usize> {
        let mut next_index = 0;
        if self.windows.len() - 1 > index {
            next_index = index + 1;
        }
        if next_index == saved {
            None
        } else {
            match self.windows.get(next_index) {
                None => None,
                Some(window) => {
                    if window.float_or_tile == FloatOrTile::Tile {
                        Some(next_index)
                    } else {
                        self.get_next_tile_index(next_index, saved)
                    }
                }
            }
        }
    }

    /// The method was upgraded to skip not only floating but also minimised
    /// windows
    pub fn get_prev_tile_index(&self, index: usize, saved: usize) -> Option<usize> {
        let mut prev_index = self.windows.len() - 1;
        if index != 0 {
            prev_index = index - 1;
        };
        if prev_index == saved {
            None
        } else {
            match self.windows.get(prev_index) {
                None => None,
                Some(window) => {
                    if window.float_or_tile == FloatOrTile::Tile {
                        Some(prev_index)
                    } else {
                        self.get_prev_tile_index(prev_index, saved)
                    }
                }
            }
        }
    }

    /// get the master window index
    ///
    /// The method was upgraded to skip not only floating but also minimised
    /// windows
    fn get_master_index(&self) -> Option<usize> {
        if !self.windows.is_empty() {
            self.windows
                .iter()
                .position(|w| (*w).float_or_tile == FloatOrTile::Tile && !(*w).minimised)
        } else {
            None
        }
    }

    /// 	returns the index where the first floating window starts
    ///
    /// if there is no  floating window, a None element is returns
    fn get_partion_index(&self) -> Option<usize> {
        if !self.windows.is_empty() {
            self.windows.iter().position(|w| (*w).float_or_tile == FloatOrTile::Float)
        } else {
            None
        }
    }

    /// This method calculated the geometries of windows.
    ///
    /// if there is a fullscreen it is updated wiht the screen geometry
    fn update_geometries(&mut self) {
        if !self.windows.is_empty() {


            let mut fullscreen = 0;

            for f_w in self.windows.iter_mut().filter(|x| (*x).fullscreen) {
                f_w.geometry = self.screen.to_geometry();
                fullscreen = 1;
            }

            // Divisor now should be update to just the tiled windows
            let non_tiled_windows =
                self.get_floating_windows().len() + self.get_minimised_tiled_windows().len() +
                self.get_minimised_floating_windows().len() + 1 + fullscreen;
            let total_windows = self.windows.len();

            // if the divisor is greater than 0 we need to calculate slave windows
            if total_windows > non_tiled_windows {
                let divisor = total_windows - non_tiled_windows;
                let divisor_2 = divisor as u32;
                let height_side = self.screen.height / divisor_2;
                let width_side = self.screen.width / 2;
                let x_point = (self.screen.width / 2) as i32;
                // It is already tested that there is more than 1 window,
                // hence one can use unwrap method being sure that a
                // Some intance of option will be returned
                let master_window = self.get_master_window().unwrap();

                let mut y_point = 0 as i32;

                for fullscreen_window in
                    self.windows.iter_mut().filter(|x| {
                        (*x).float_or_tile == FloatOrTile::Tile && !(*x).minimised &&
                        !(*x).fullscreen
                    }) {
                    if master_window != fullscreen_window.window {
                        // I calculate the values of the secondary windows
                        // (right windows)
                        let rigth_geometry = Geometry {
                            x: x_point,
                            y: y_point,
                            width: width_side,
                            height: height_side,
                        };
                        fullscreen_window.geometry = rigth_geometry;
                        y_point += (height_side) as i32;

                    } else {
                        // I calculate the values for master window
                        let master_geometry = Geometry {
                            x: 0,
                            y: 0,
                            width: width_side,
                            height: self.screen.height,
                        };

                        fullscreen_window.geometry = master_geometry;
                    }
                }
            } else {
                // It could be the posibility that there is just one window
                // (the master window)
                match self.get_master_index() {
                    None => (),
                    Some(i) => {
                        // unwrap() is used because I ensure there is at
                        // least one element
                        let window = self.windows.get_mut(i).unwrap();
                        window.geometry = self.screen.to_geometry();
                    }
                }
            }
        };
    }

    /// gets a vector with windows that are both minised and tiled windows
    fn get_minimised_tiled_windows(&self) -> Vec<Window> {
        let mut temp_windows = Vec::new();
        for fullscreen_window in
            self.windows
                .iter()
                .filter(|x| (*x).float_or_tile == FloatOrTile::Tile && (*x).minimised) {
            temp_windows.push(fullscreen_window.window.clone())
        }
        temp_windows
    }

    /// gets a vector with windows that are both minised and floating windows
    fn get_minimised_floating_windows(&self) -> Vec<Window> {
        let mut temp_windows = Vec::new();
        for minimised_window in self.windows
            .iter()
            .filter(|x| (*x).float_or_tile == FloatOrTile::Float && (*x).minimised) {
            temp_windows.push(minimised_window.window.clone())
        }
        temp_windows
    }

    /// Removes the given window from the *minimised_windows* vector and set
    /// its attribute to false in the corresponding
    /// window of *windows* vector
    fn remove_minimised_window(&mut self, window: Window) {
        match self.minimised_windows.iter().position(|w| *w == window) {
            None => (),
            Some(i) => {
                self.minimised_windows.remove(i);
                match self.windows.iter().position(|w| (*w).window == window) {
                    None => (),
                    Some(i_2) => {
                        // unwrap() is used because Some(i_2) ensures there
                        // is at least one element
                        self.windows.get_mut(i_2).unwrap().minimised = false;
                    }
                }
            }
        }
    }

    /// set the given window to the *minimised_windows* vector and set its
    /// attribute to true in the corresponding
    /// window of *windows* vector
    fn set_minimised_window(&mut self, window: Window) {
        match self.windows.iter().position(|w| (*w).window == window) {
            None => (),
            Some(i) => {
                // unwrap() is used because Some(i) ensures there is at least
                // one element
                let window_min = self.windows.get_mut(i).unwrap();
                window_min.minimised = true;
                self.minimised_windows.push(window_min.window.clone());
            }
        }
    }
    /// Set the fullscreen attribute of the given window to true
    fn set_fullscreen_window(&mut self, window: Window) {
        for f_w in self.windows.iter_mut().filter(|x| (*x).window == window) {
            f_w.fullscreen = true
        }
    }

    /// Set the fullscreen attribute of the given window to false
    fn remove_fullscreen_window(&mut self, window: Window) {
        for f_w in self.windows.iter_mut().filter(|x| (*x).window == window) {
            f_w.fullscreen = false
        }
    }
}

/// The errors that this window manager can return.
#[derive(Debug)]
pub enum FullscreenWMError {
    /// This window is not known by the window manager.
    UnknownWindow(Window),
    /// This window is not known by the window manager.
    ManagedWindow(Window),
    /// This window is not a floating window.
    NoFloatingWindow(Window),
    /// This window is not a tiled window
    NoTiledWindow(Window),
}

impl fmt::Display for FullscreenWMError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FullscreenWMError::UnknownWindow(ref window) => write!(f, "Unknown window: {}", window),
            FullscreenWMError::ManagedWindow(ref window) => {
                write!(f, "Window {} is already managed", window)
            }
            FullscreenWMError::NoFloatingWindow(ref window) => {
                write!(f, "Window {} is not floating", window)
            }
            FullscreenWMError::NoTiledWindow(ref window) => {
                write!(f, "Window {} is not tiled", window)
            }
        }
    }
}

impl error::Error for FullscreenWMError {
    fn description(&self) -> &'static str {
        match *self {
            FullscreenWMError::UnknownWindow(_) => "Unknown window",
            FullscreenWMError::ManagedWindow(_) => "Window is already managed",
            FullscreenWMError::NoFloatingWindow(_) => "Window is not floating",
            FullscreenWMError::NoTiledWindow(_) => "Window is not tiled",
        }
    }
}


impl WindowManager for FullscreenWM {
    type Error = FullscreenWMError;

    /// The FullscreenWM constructor.
    ///
    /// windows and minimised_windows are initialised as empty vec, screen
    /// as the given screen and focused index as None.
    fn new(screen: Screen) -> FullscreenWM {
        FullscreenWM {
            windows: Vec::new(),
            minimised_windows: Vec::new(),
            screen: screen,
            index_foused_window: None,
        }
    }

    /// Returns all the managed windows in the window manager.
    fn get_windows(&self) -> Vec<Window> {
        let mut temp_windows = Vec::new();
        for window_with_info in self.windows.iter() {
            temp_windows.push(window_with_info.window.clone());
        }
        temp_windows
    }

    /// gets the current focused window.
    ///
    /// The list can be no empty and no focused window simultaneously.
    fn get_focused_window(&self) -> Option<Window> {
        if !self.windows.is_empty() {
            match self.index_foused_window {
                None => None,
                Some(index) => Some(self.windows.get(index).unwrap().window),
            }
        } else {
            None
        }
    }

    /// adds new window_with_info to the vec windows and set the geometry to
    /// fullscreen.
    ///
    /// the fullscreen windows are accepted and added, but they are treated
    /// as tiled or floating window. The given geometry by the
    /// window_with_info is set as the saved_geometry and geometry
    /// attributes of the FloatingWindow structure.
    ///
    /// if the given window has an attribute FloatOrTile::Tile then the
    /// geometries for tiled windows should be updated.
    ///
    /// add_window adds the tiled and floating windows in order, so the
    /// first part of the vec contains
    /// the tiled windows while the second part are the floating windows
    ///
    /// add_window converts every window_with_info to fullscreen_window,
    /// nsetting the minimised
    /// attribute as false
    ///
    /// returns an ManagedWindow error if the given window_with_info is
    /// already managed by the window manager.
    fn add_window(&mut self, window_with_info: WindowWithInfo) -> Result<(), Self::Error> {
        if !self.is_managed(window_with_info.window) {
            for f_w in self.windows.iter_mut().filter(|x| (*x).fullscreen) {
                f_w.fullscreen = false;
            }
            // All new added windows are set minimised = false by default
            let fullscreen_window = FullscreenWindow {
                window: window_with_info.window,
                geometry: window_with_info.geometry,
                saved_geometry: window_with_info.geometry,
                float_or_tile: window_with_info.float_or_tile,
                fullscreen: window_with_info.fullscreen,
                minimised: false,
            };
            if window_with_info.float_or_tile == FloatOrTile::Float {
                // if there is a floating window, it is insert at the end
                // of the vec
                self.windows.push(fullscreen_window);
                let temp = self.windows.len() - 1;
                self.index_foused_window = Some(temp);
            } else {
                // if not, I get the index where the first floating window
                //  start and insert the tiled window there, also the
                // geometries should be updated.
                match self.get_partion_index() {
                    None => {
                        self.windows.push(fullscreen_window);
                        let temp = self.windows.len() - 1;
                        self.index_foused_window = Some(temp);
                    }
                    Some(partion_index) => {
                        self.windows.insert(partion_index, fullscreen_window);
                        self.index_foused_window = Some(partion_index);
                    }
                };
                self.update_geometries();
            };
            Ok(())
        } else {
            Err(FullscreenWMError::ManagedWindow(window_with_info.window))
        }
    }

    /// removes the given window from the window manager.
    ///
    /// Every time that a element is removed the index_foused_window should
    /// be updated if it is necessary.
    /// Important to noticy here is that when the focused element is the
    /// same as the removed element, no focused window is set.
    ///
    /// There is no effect in the fullscreen window if a window is removed
    /// unless the fullscreen itself is removed.
    fn remove_window(&mut self, window: Window) -> Result<(), Self::Error> {
        match self.windows.iter().position(|w| (*w).window == window) {
            None => Err(FullscreenWMError::UnknownWindow(window)),
            Some(i) => {
                // unwrap() is used because Some(i) ensures there is at
                // least one element
                let temp_window = self.windows.get(i).unwrap().clone();
                self.windows.remove(i);

                if temp_window.minimised {
                    self.remove_fullscreen_window(temp_window.window);
                };

                if temp_window.float_or_tile == FloatOrTile::Tile {
                    self.update_geometries();
                };
                match self.index_foused_window {
                    None => Ok(()),

                    Some(index) => {
                        // If the focused_element is the one that is erased,
                        // None focused elemente
                        if index == i {
                            self.index_foused_window = None;
                            Ok(())
                        } else {
                            // If the focused_element is in the right side
                            // of the vector
                            // the focused elemente is keep it, otherwise
                            // the focused index is decreased by one.
                            let mut temp = index;
                            if index > i {
                                temp = index - 1;
                            }
                            self.index_foused_window = Some(temp);
                            Ok(())
                        }
                    }
                }
            }
        }
    }

    /// returns the layout of all managed windows.
    ///
    /// The array is already order thus no need to rearrange the vector.
    /// The minised windows are filtered out.
    ///
    /// Now get_window_layout hides the windows when there is a fullscreen
    /// window
    fn get_window_layout(&self) -> WindowLayout {

        if !self.windows.is_empty() {

            let mut fullscreen_window = None;

            for fs_w in self.windows.iter().filter(|x| (*x).fullscreen) {
                fullscreen_window = Some(fs_w.clone())
            }

            match fullscreen_window {
                // if there is a fullscreen, I hide the others
                Some(window) => {
                    WindowLayout {
                        focused_window: Some(window.window),
                        windows: vec![(window.window, window.geometry)],
                    }
                }
                None => {
                    let mut temp_windows = Vec::new();

                    for window_info in self.windows.iter().filter(|x| !(*x).minimised) {
                        temp_windows.push((window_info.window.clone(),
                        window_info.geometry.clone()))
                    }

                    let temp_focused_window = match self.index_foused_window {
                        None => None,
                        Some(index) => {
                            // unwrap() is used because Some(index) ensures
                            // there is at least one element
                            let window_type = self.windows.get(index).unwrap();
                            match self.get_partion_index() {
                                // unwrap() is used because
                                // self.get_partion_index ensures there is
                                // at least one element
                                None => Some(self.windows.get(index).unwrap().window),
                                // if a focused window is a floating one,
                                // then the temp_windows is reorder to
                                // put the focused window at the very end
                                // of the vec
                                Some(partion_index) => {
                                    if partion_index >= index &&
                                       window_type.float_or_tile == FloatOrTile::Float {
                                        match temp_windows.iter().position(|w| {
                                            (*w) == (window_type.window, window_type.geometry)
                                        }) {
                                            None => (),

                                            Some(real_index) => {
                                                // unwrap() is used because
                                                // Some(real_index) ensures
                                                // there is at least one element
                                                let (temp_window, temp_geometry) =
                                                    temp_windows.get(real_index)
                                                        .unwrap()
                                                        .clone();
                                                temp_windows.remove(real_index);
                                                temp_windows.push((temp_window, temp_geometry));
                                            }
                                        };
                                    };
                                    // unwrap() is used because
                                    // Some(partion_index) ensures there is
                                    // at least one element
                                    Some(self.windows
                                        .get(index)
                                        .unwrap()
                                        .window)
                                }
                            }
                        }
                    };

                    WindowLayout {
                        focused_window: temp_focused_window,
                        windows: temp_windows,
                    }
                }
            }
        } else {
            WindowLayout::new()
        }
    }

    /// set the focused window in the window manager with the given window.
    ///
    /// if the given window is minised, after focus_window succeeds, such
    /// windows is unminimised.
    ///
    /// One can focused the fullscreen, if that is the case, there is no
    /// effect.
    fn focus_window(&mut self, window: Option<Window>) -> Result<(), Self::Error> {
        match window {
            None => {
                self.index_foused_window = None;
                Ok(())
            }

            Some(gw) => {
                match self.windows.iter().position(|w| (*w).window == gw) {
                    None => Err(FullscreenWMError::UnknownWindow(gw)),

                    Some(i) => {
                        // unwrap() is used because Some(i) ensures there
                        // is at least one element
                        let fullscreen_window = self.windows.get(i).unwrap().clone();

                        if !fullscreen_window.fullscreen {
                            self.index_foused_window = Some(i);
                            for fullscreen_window in
                                self.windows.iter_mut().filter(|x| (*x).fullscreen) {
                                fullscreen_window.fullscreen = false
                            }
                            if fullscreen_window.minimised {
                                // unwrap() is used because Some(i) ensures
                                // there is at least one element
                                Ok(self.remove_minimised_window(window.unwrap()))
                            } else {
                                Ok(())
                            }
                        } else {
                            Ok(())
                        }
                    }
                }
            }
        }
    }

    /// back/forth to the next window from the current focused window.
    ///
    /// the iteration in this function is over the current order of the
    /// *windows* vec, that means that the client can jump over floating
    /// and tiled windows if the order of *windows* vec is in such way.
    /// If the PrevOrNext is a minised window, it is unminimised.
    /// Whenever it is used, the fullscreen is disable
    fn cycle_focus(&mut self, dir: PrevOrNext) {
        if self.windows.len() > 1 {
            match self.index_foused_window {

                None => self.index_foused_window = Some(0),

                Some(index) => {
                    match dir {
                        PrevOrNext::Prev => {
                            if index != 0 {
                                let temp = index - 1;
                                self.index_foused_window = Some(temp);
                            } else {
                                let temp = self.windows.len() - 1;
                                self.index_foused_window = Some(temp);
                            }
                        }

                        PrevOrNext::Next => {
                            let last_index = self.windows.len() - 1;
                            if index != last_index {
                                let temp = index + 1;
                                self.index_foused_window = Some(temp);
                            } else {
                                self.index_foused_window = Some(0);
                            }
                        }
                    }
                }
            }
        } else {
            if self.windows.len() == 1 {
                self.index_foused_window = Some(0);
            }
        };

        match self.index_foused_window {
            Some(index) => {
                // unwrap() is used because Some(index) ensures there is
                // at least one element
                let minimised_window = self.windows.get(index).unwrap().clone();
                // I know that the given minised_window already exists
                // in the manager no need to catch the output
                self.remove_minimised_window(minimised_window.window);
            }
            None => (),
        }

        let mut fullscreen_window = false;
        for f_w in self.windows.iter_mut().filter(|x| (*x).fullscreen) {
            f_w.fullscreen = false;
            fullscreen_window = true
        }

        if fullscreen_window {
            self.update_geometries()
        }
    }

    /// gets the complete current information of the given window.
    ///
    /// If the given window is not managed by the window manager,
    /// UnknownWindow error is shown.
    fn get_window_info(&self, window: Window) -> Result<WindowWithInfo, Self::Error> {
        match self.windows.iter().position(|w| (*w).window == window) {
            None => Err(FullscreenWMError::UnknownWindow(window)),
            Some(i) => {
                // unwrap() is used because Some(i) ensures there is at
                // least one element
                let fullscreen_window = self.windows.get(i).unwrap().clone();
                let window_with_info = WindowWithInfo {
                    window: fullscreen_window.window,
                    geometry: fullscreen_window.geometry,
                    float_or_tile: fullscreen_window.float_or_tile,
                    fullscreen: fullscreen_window.fullscreen,
                };
                Ok(window_with_info)
            }
        }
    }

    /// gets the current window screen size.
    fn get_screen(&self) -> Screen {
        self.screen
    }

    // When the scren is resized, the tiled windows should be updated
    // accordingly
    fn resize_screen(&mut self, screen: Screen) {
        self.screen = screen;
        self.update_geometries()
    }
}

impl TilingSupport for FullscreenWM {
    /// if *windows* is not empty it returns the master window, otherwise
    /// return None.
    ///
    /// In this window manager the first tiled element of the *windows* vec
    /// is always the master window.
    fn get_master_window(&self) -> Option<Window> {
        if !self.windows.is_empty() {
            match self.get_master_index()
            {None => None,
			Some(index) =>
				// unwrap() is used because Some(index) ensures there
                // is at least one element
				Some(self.windows.get(index).unwrap().window),
			}
        } else {
            None
        }
    }

    /// swap the position of the given window with the master window.
    ///
    /// This functions actually affects the order in which the windows were
    /// added, also the geomtries should be accordingly.
    ///
    /// In case the given window is the master window  and the window master
    /// is not focused, the only effect of this funtions is
    /// changing the focused windows to the master window.
    ///
    /// If the given window is minised, this funtion should unminimised it
    ///
    /// If the the fullscreen is removed, if any.
    fn swap_with_master(&mut self, window: Window) -> Result<(), Self::Error> {
        for f_w in self.windows.iter_mut().filter(|x| (*x).fullscreen) {
            f_w.fullscreen = false;
        }
        match self.windows.iter().position(|w| (*w).window == window) {
            None => Err(FullscreenWMError::UnknownWindow(window)),
            Some(window_index) => {
                // unwrap() is used because Some(window_index) ensures
                // there is at least one element
                let minised_window = self.windows.get(window_index).unwrap().clone();
                if minised_window.float_or_tile == FloatOrTile::Tile {
                    match self.get_master_index() {
                        None => Ok(()),
                        Some(master_index) => {
                            if minised_window.minimised {
                                self.remove_minimised_window(window);
                            };
                            self.windows.swap(master_index, window_index);
                            self.update_geometries();
                            self.focus_window(Some(window))
                        }
                    }
                } else {
                    Err(FullscreenWMError::NoTiledWindow(window))
                }
            }
        }
    }


    /// Simlar approach than cycle_focus, but now the Vec is affect, hence
    /// the order in which the windows were added is changed.
    ///
    /// If there is no focused window or if the number of tiled windows is
    /// less than 2 the funtion has no effects.
    ///
    /// The minimised windows are hidden in this function.
    ///
    /// After the sucessful swap, the geometries should be updated
    /// accordingly.
    ///
    /// The fulscreen is removed whe this method is used
    fn swap_windows(&mut self, dir: PrevOrNext) {
        // remove fullscreen, if any
        for f_w in self.windows.iter_mut().filter(|x| (*x).fullscreen) {
            f_w.fullscreen = false;
        }
        if self.windows.len() > 1 {
            match self.index_foused_window {
                // no focused window = nothing
                None => (),

                Some(index) => {
                    // unwrap() is used because Some(index) ensures there is
                    // at least one element
                    if self.windows.get(index).unwrap().float_or_tile == FloatOrTile::Tile {
                        match dir {
                            PrevOrNext::Prev => {
                                match self.get_prev_tile_index(index, index) {
                                    Some(prev_index) => {
                                        self.index_foused_window = Some(prev_index);
                                        self.windows.swap(index, prev_index);
                                        self.update_geometries();
                                    }
                                    None => (),
                                }
                            }

                            PrevOrNext::Next => {
                                match self.get_next_tile_index(index, index) {
                                    Some(next_index) => {
                                        self.index_foused_window = Some(next_index);
                                        self.windows.swap(index, next_index);
                                        self.update_geometries();
                                    }
                                    None => (),
                                }
                            }
                        };
                        // remove minised window, if applied
                        match self.index_foused_window {
                            None => (),
                            Some(index_f_w) => {
                                // unwrap() is used because Some(index_f_w)
                                // ensures there is at least one element
                                let focused_window = self.windows.get(index_f_w).unwrap().clone();
                                self.remove_minimised_window(focused_window.window);
                            }
                        }
                    } else {
                        ()
                    }
                }
            }
        }
    }
}

impl FloatSupport for FullscreenWM {
    /// return a vector that contains the floating windows managed by
    /// this window manager
    ///
    /// I have to iterate over the whole collection to filter out the
    /// non-floating windows.
    ///
    /// The minised and floating windows are filtered out too.
    fn get_floating_windows(&self) -> Vec<Window> {
        let mut temp_windows = Vec::new();

        for window_with_info_floating in
            self.windows
                .iter()
                .filter(|x| (*x).float_or_tile == FloatOrTile::Float && !(*x).minimised) {
            temp_windows.push(window_with_info_floating.window.clone());
        }

        temp_windows
    }


    /// set the given window to a floating window if it was a tiled window
    /// or viceversa
    ///
    /// if the given window was minised, after toggle_floating succeeds,
    /// such window should be unminimised
    ///
    /// after the a sucessful toogle, the geometries of the tiled windows
    /// should be updated
    ///
    /// if the given window is the fullscreen window, the it is removed
    fn toggle_floating(&mut self, window: Window) -> Result<(), Self::Error> {
        match self.windows.iter().position(|w| (*w).window == window) {
            None => Err(FullscreenWMError::UnknownWindow(window)),

            Some(i) => {
                // unwrap() is used because Some(i) ensures there is at
                // least one element
                let win_info = self.windows.get(i).unwrap().clone();
                // if the given structure is fullscreen then it should be
                // removed and
                // added as floating or tiled window depending of the
                // previous window type
                // and focused
                if win_info.fullscreen {
                    self.remove_window(win_info.window).unwrap();
                    let window_info;
                    if win_info.float_or_tile == FloatOrTile::Tile {

                        window_info = WindowWithInfo {
                            window: win_info.window,
                            geometry: win_info.saved_geometry,
                            float_or_tile: FloatOrTile::Float,
                            fullscreen: false,
                        };

                    } else {
                        window_info = WindowWithInfo {
                            window: win_info.window,
                            geometry: win_info.saved_geometry,
                            float_or_tile: FloatOrTile::Tile,
                            fullscreen: false,
                        };
                    };
                    self.add_window(window_info).unwrap();
                    Ok(())
                } else {
                    if win_info.minimised {
                        self.remove_minimised_window(win_info.window);
                    };

                    // store the current focused and
                    let mut focused_window = None;
                    let fullscreen_window = self.get_fullscreen_window();
                    match self.index_foused_window {
                        None => (),
                        Some(index) => {
                            // unwrap() is used because Some(index) ensures
                            // there is at least one element
                            let window_info = self.windows.get(index).unwrap().clone();
                            focused_window = Some(window_info.window);
                        }
                    };
                    self.remove_window(win_info.window).unwrap();
                    let window_info;
                    if win_info.float_or_tile == FloatOrTile::Tile {

                        window_info = WindowWithInfo {
                            window: win_info.window,
                            geometry: win_info.saved_geometry,
                            float_or_tile: FloatOrTile::Float,
                            fullscreen: win_info.fullscreen,
                        };

                    } else {
                        window_info = WindowWithInfo {
                            window: win_info.window,
                            geometry: win_info.saved_geometry,
                            float_or_tile: FloatOrTile::Tile,
                            fullscreen: win_info.fullscreen,
                        };
                    };
                    self.add_window(window_info).unwrap();

                    // after toogle the window, the geometries and the
                    // focused window should be updated if that it the case
                    // focus_window will always sucees since the window is
                    // already managed and it was removed in this toggle
                    // process
                    self.focus_window(focused_window).unwrap();
                    if !win_info.fullscreen {
                        match fullscreen_window {
                            None => (),
                            Some(windows_id) => self.set_fullscreen_window(windows_id),
                        }
                    };
                    self.update_geometries();
                    Ok(())
                }
            }
        }
    }

    /// set the given window to a floating window if it was a tiled window
    ///  or viceversa
    ///
    /// this function iterates over the windows until the given window is
    /// found, then if it is a floating window
    /// the corresponding FloatingWindow structure is mutated, otherwise
    /// a NoFloatingWindow error is thrown.
    ///
    /// if the given window is minised, it should be unminimised
    ///
    /// The fullscreen window it is not affected by this function
    fn set_window_geometry(&mut self,
                           window: Window,
                           new_geometry: Geometry)
                           -> Result<(), Self::Error> {
        match self.windows.iter().position(|w| (*w).window == window) {
            None => Err(FullscreenWMError::UnknownWindow(window)),

            Some(i) => {
                // it was already check that there is a window, so unwrap
                // can be used
                let window_min = self.windows.get(i).unwrap().clone();

                if window_min.float_or_tile == FloatOrTile::Tile {
                    Err(FullscreenWMError::NoFloatingWindow(window_min.window))
                } else {
                    if window_min.minimised {
                        self.remove_minimised_window(window_min.window);
                    };
                    let window_with_info = self.windows.get_mut(i).unwrap();
                    (*window_with_info).saved_geometry = new_geometry;
                    (*window_with_info).geometry = new_geometry;
                    Ok(())
                }
            }
        }
    }
}

impl MinimiseSupport for FullscreenWM {
    /// Returns a vector of all the minimised windows.
    ///
    /// the order in which the vec is returned as the order in whih minised
    /// elements were minised
    fn get_minimised_windows(&self) -> Vec<Window> {
        self.minimised_windows.clone()
    }


    /// Returns `true` if the given window is minimised.
    fn is_minimised(&self, window: Window) -> bool {
        self.get_minimised_windows().contains(&window)
    }

    /// if the given window is minimised is unminimised or viceversa
    ///
    /// after toggle_minimised succeeds, the geometries should be updated
    ///
    /// if the given window is the fullscreen, the it is removed
    ///
    /// if the given window was focused and should be minised, then
    /// focused_window is set to None
    fn toggle_minimised(&mut self, window: Window) -> Result<(), Self::Error> {
        if let Some(index_window) = self.windows.iter().position(|w| (*w).window == window) {
            // remove fullscreen if the given window is a fullscreen
            self.remove_fullscreen_window(window);
            if self.is_minimised(window) {
                self.remove_minimised_window(window)
            } else {
                // Non focused if is minised the current focused_window
                match self.index_foused_window {
                    None => (),
                    Some(index_focused) => {
                        if index_focused == index_window {
                            self.index_foused_window = None
                        }
                    }
                };
                self.set_minimised_window(window)
            };
            Ok(self.update_geometries())
        } else {
            Err(FullscreenWMError::UnknownWindow(window))
        }
    }
}

impl FullscreenSupport for FullscreenWM {
    /// Return the current fullscreen, if any.
    fn get_fullscreen_window(&self) -> Option<Window> {
        let mut fullscreen = None;
        for f_w in self.windows.iter().filter(|x| (*x).fullscreen) {
            fullscreen = Some(f_w.window.clone());
        }
        fullscreen
    }

    /// Make the given window fullscreen, or when it is already fullscreen,
    /// undo it.
    ///
    // If the given window is minised, it should be unminimised
    fn toggle_fullscreen(&mut self, window: Window) -> Result<(), Self::Error> {
        match self.windows.iter().position(|w| (*w).window == window) {
            None => Err(FullscreenWMError::UnknownWindow(window)),
            Some(i) => {
                let window_min = self.windows.get(i).unwrap().clone();
                if window_min.minimised {
                    self.remove_minimised_window(window_min.window);
                };
                match self.get_fullscreen_window() {
                    None => {
                        self.focus_window(Some(window)).unwrap();
                        self.set_fullscreen_window(window)
                    }
                    Some(full_window) => {
                        self.remove_fullscreen_window(window_min.window);
                        if full_window != window {
                            self.focus_window(Some(window)).unwrap();
                            self.set_fullscreen_window(window)
                        }
                    }
                };
                self.update_geometries();
                Ok(())
            }
        }
    }
}


#[cfg(test)]
mod tests {

    // We have to import `FullscreenWM` from the super module.
    use super::FullscreenWM;
    // We have to repeat the imports we did in the super module.
    use cplwm_api::wm::WindowManager;
    use cplwm_api::wm::TilingSupport;
    use cplwm_api::wm::FloatSupport;
    use cplwm_api::wm::MinimiseSupport;
    use cplwm_api::wm::FullscreenSupport;
    use cplwm_api::types::*;

    // We define a static variable for the screen we will use in the tests.
    // You can just as well define it as a local variable in your tests.
    static SCREEN: Screen = Screen {
        width: 800,
        height: 600,
    };

    static SCREEN2: Screen = Screen {
        width: 1000,
        height: 800,
    };
    // We define a static variable for the geometry of a fullscreen window.
    // Note that it matches the dimensions of `SCREEN`.
    static SCREEN_GEOM: Geometry = Geometry {
        x: 0,
        y: 0,
        width: 800,
        height: 600,
    };

    // We define a static variable for some random geometry that we will use
    // when adding windows to a window manager.
    static SOME_GEOM: Geometry = Geometry {
        x: 10,
        y: 10,
        width: 100,
        height: 100,
    };

    static FULLSCREEN: Geometry = Geometry {
        x: 0,
        y: 0,
        width: 800,
        height: 600,
    };

    // Now let's write our test.
    //
    // Note that tests are annotated with `#[test]`, and cannot take arguments
    // nor return anything.

    #[test]
    fn test_adding_and_removing_some_windows() {
        // Let's make a new `FullscreenWM` with `SCREEN` as screen.
        let mut wm = FullscreenWM::new(SCREEN);

        assert_eq!(WindowLayout::new(), wm.get_window_layout());

        wm.add_window(WindowWithInfo::new_tiled(1, SOME_GEOM)).unwrap();

        assert!(wm.is_managed(1));
        // and be present in the `Vec` of windows.
        assert_eq!(vec![1], wm.get_windows());
        // According to the window layout
        let wl1 = wm.get_window_layout();
        // it should be focused
        assert_eq!(Some(1), wl1.focused_window);
        // and fullscreen.
        assert_eq!(vec![(1, SCREEN_GEOM)], wl1.windows);

        // Let's add another window.
        wm.add_window(WindowWithInfo::new_tiled(2, SOME_GEOM)).unwrap();
        // It should now be managed by the WM.
        assert!(wm.is_managed(2));
        // The `Vec` of windows should now contain both windows 1 and 2.
        assert_eq!(vec![1, 2], wm.get_windows());
        // According to the window layout
        let wl2 = wm.get_window_layout();
        // window 2 should be focused
        assert_eq!(Some(2), wl2.focused_window);
        // and should be half of the screen.
        let first_half = Geometry {
            x: 0,
            y: 0,
            width: 400,
            height: 600,
        };

        let second_half = Geometry {
            x: 400,
            y: 0,
            width: 400,
            height: 600,
        };

        assert_eq!(vec![(1, first_half), (2, second_half)], wl2.windows);

        // Now let's remove window 2
        wm.remove_window(2).unwrap();
        // It should no longer be managed by the WM.
        assert!(!wm.is_managed(2));
        // The `Vec` of windows should now just contain window 1.
        assert_eq!(vec![1], wm.get_windows());
        // According to the window layout
        let wl3 = wm.get_window_layout();
        // because the new behavior, window 1 should not be focused, No
        // window is focused
        assert_eq!(None, wl3.focused_window);
        // and fullscreen.
        assert_eq!(vec![(1, SCREEN_GEOM)], wl3.windows);

        let third_half = Geometry {
            x: 400,
            y: 0,
            width: 400,
            height: 150,
        };

        let fourth_half = Geometry {
            x: 400,
            y: 150,
            width: 400,
            height: 150,
        };

        let fifth_half = Geometry {
            x: 400,
            y: 300,
            width: 400,
            height: 150,
        };

        let sixth_half = Geometry {
            x: 400,
            y: 450,
            width: 400,
            height: 150,
        };
        // I add more window, which shoudl be allocated in the right side of
        // the window
        wm.add_window(WindowWithInfo::new_tiled(3, SOME_GEOM)).unwrap();
        wm.add_window(WindowWithInfo::new_tiled(4, SOME_GEOM)).unwrap();
        wm.add_window(WindowWithInfo::new_tiled(5, SOME_GEOM)).unwrap();
        wm.add_window(WindowWithInfo::new_tiled(6, SOME_GEOM)).unwrap();

        let wl4 = wm.get_window_layout();
        // Due to the new added windows, now window 6 should be focused
        assert_eq!(Some(6), wl4.focused_window);
        // The windows should be allocated in the correct window
        assert_eq!(vec![(1, first_half),
                        (3, third_half),
                        (4, fourth_half),
                        (5, fifth_half),
                        (6, sixth_half)],
                   wl4.windows);
    }

    #[test]
    fn test_focus_window() {

        let mut wm = FullscreenWM::new(SCREEN);

        // Add some windows
        wm.add_window(WindowWithInfo::new_tiled(1, SOME_GEOM)).unwrap();
        wm.add_window(WindowWithInfo::new_tiled(2, SOME_GEOM)).unwrap();
        wm.add_window(WindowWithInfo::new_tiled(3, SOME_GEOM)).unwrap();
        wm.add_window(WindowWithInfo::new_tiled(4, SOME_GEOM)).unwrap();
        wm.add_window(WindowWithInfo::new_tiled(5, SOME_GEOM)).unwrap();

        // Now an action should be applied, when None is given to
        // focus_window the current focused window should be unfocused.
        wm.focus_window(None).unwrap();

        // Focused window should return None
        let wl1 = wm.get_window_layout();
        assert_eq!(None, wl1.focused_window);

        // Window 10 is not in manager an UnknownWindow error should be thrown
        assert!(wm.focus_window(Some(10)).is_err());

        // Focus to window 4
        wm.focus_window(Some(4)).unwrap();
        let wl2 = wm.get_window_layout();
        assert_eq!(Some(4), wl2.focused_window);

        // Focus to window 2
        wm.focus_window(Some(2)).unwrap();
        let wl3 = wm.get_window_layout();
        assert_eq!(Some(2), wl3.focused_window);

        // Focus to window 5
        wm.focus_window(Some(5)).unwrap();
        let wl4 = wm.get_window_layout();
        assert_eq!(Some(5), wl4.focused_window);

        wm.remove_window(5).unwrap();
        wm.focus_window(Some(1)).unwrap();
        wm.remove_window(1).unwrap();
        // Because the last focused window was removed, the focused_window
        // attribute should be None
        let wl5 = wm.get_window_layout();
        assert_eq!(None, wl5.focused_window);
    }

    #[test]
    fn test_cycle_focus() {

        let mut wm = FullscreenWM::new(SCREEN);

        // Do nothing
        wm.cycle_focus(PrevOrNext::Next);

        // Add some both type of windows
        wm.add_window(WindowWithInfo::new_tiled(1, SOME_GEOM)).unwrap();
        wm.add_window(WindowWithInfo::new_float(2, SOME_GEOM)).unwrap();
        wm.add_window(WindowWithInfo::new_tiled(3, SOME_GEOM)).unwrap();
        wm.add_window(WindowWithInfo::new_float(4, SOME_GEOM)).unwrap();
        wm.add_window(WindowWithInfo::new_tiled(5, SOME_GEOM)).unwrap();


        // previous should be 3
        wm.cycle_focus(PrevOrNext::Prev);
        let wl1 = wm.get_window_layout();
        assert_eq!(Some(3), wl1.focused_window);

        // previous should be 1
        wm.cycle_focus(PrevOrNext::Prev);
        let wl2 = wm.get_window_layout();
        assert_eq!(Some(1), wl2.focused_window);

        // previous should be 3
        wm.cycle_focus(PrevOrNext::Next);
        let wl3 = wm.get_window_layout();
        assert_eq!(Some(3), wl3.focused_window);

        // previous should be 5
        wm.cycle_focus(PrevOrNext::Next);
        let wl4 = wm.get_window_layout();
        assert_eq!(Some(5), wl4.focused_window);

        // Focus should be 2
        wm.cycle_focus(PrevOrNext::Next);
        let wl4 = wm.get_window_layout();
        assert_eq!(Some(2), wl4.focused_window);

        // Focus should be 4
        wm.cycle_focus(PrevOrNext::Next);
        let wl5 = wm.get_window_layout();
        assert_eq!(Some(4), wl5.focused_window);

        // Focus should be in window 6, since is added
        wm.add_window(WindowWithInfo::new_tiled(6, SOME_GEOM)).unwrap();
        let wl6 = wm.get_window_layout();
        assert_eq!(Some(6), wl6.focused_window);

        // Now previous should be 5, since that was the last tiled window
        // before adding 6
        wm.cycle_focus(PrevOrNext::Prev);
        let wl7 = wm.get_window_layout();
        assert_eq!(Some(5), wl7.focused_window);
    }

    #[test]
    fn test_get_window_info() {

        let mut wm = FullscreenWM::new(SCREEN);

        // Add some windows
        wm.add_window(WindowWithInfo::new_tiled(1, SOME_GEOM)).unwrap();
        wm.add_window(WindowWithInfo::new_tiled(2, SOME_GEOM)).unwrap();
        wm.add_window(WindowWithInfo::new_float(3, SCREEN_GEOM)).unwrap();
        wm.add_window(WindowWithInfo::new_tiled(4, SOME_GEOM)).unwrap();
        wm.add_window(WindowWithInfo::new_float(5, SOME_GEOM)).unwrap();
        wm.add_window(WindowWithInfo::new_tiled(6, SOME_GEOM)).unwrap();


        // All screens should have different size, since now we are dealing
        // with tiled funtions, it should be reflec since they are added,
        // removed or toggled

        let master_half = Geometry {
            x: 0,
            y: 0,
            width: 400,
            height: 600,
        };

        let first_half = Geometry {
            x: 400,
            y: 0,
            width: 400,
            height: 200,
        };

        let second_half = Geometry {
            x: 400,
            y: 200,
            width: 400,
            height: 200,
        };

        let third_half = Geometry {
            x: 400,
            y: 400,
            width: 400,
            height: 200,
        };

        assert_eq!(wm.get_window_info(1).unwrap().geometry, master_half);
        assert_eq!(wm.get_window_info(2).unwrap().geometry, first_half);
        assert_eq!(wm.get_window_info(3).unwrap().geometry, SCREEN_GEOM);
        assert_eq!(wm.get_window_info(4).unwrap().geometry, second_half);
        assert_eq!(wm.get_window_info(5).unwrap().geometry, SOME_GEOM);
        assert_eq!(wm.get_window_info(6).unwrap().geometry, third_half);
    }

    #[test]
    fn test_get_resize_screen() {

        let mut wm = FullscreenWM::new(SCREEN);

        // swm screen should be the same as SCREEN
        assert_eq!(wm.get_screen(), SCREEN);

        // now, swm screen should be the same as SCREEN
        wm.resize_screen(SCREEN2);
        assert_eq!(wm.get_screen(), SCREEN2);
    }

    #[test]
    fn test_tiling_support() {

        let mut wm = FullscreenWM::new(SCREEN);

        // No window yet
        assert_eq!(None, wm.get_master_window());

        // Add some windows
        wm.add_window(WindowWithInfo::new_tiled(1, SOME_GEOM)).unwrap();
        wm.add_window(WindowWithInfo::new_tiled(2, SOME_GEOM)).unwrap();
        wm.add_window(WindowWithInfo::new_tiled(3, SOME_GEOM)).unwrap();
        wm.add_window(WindowWithInfo::new_tiled(4, SOME_GEOM)).unwrap();


        // First window added is the master window
        assert_eq!(Some(1), wm.get_master_window());

        // Focused window is 4, since was the last added
        let wl1 = wm.get_window_layout();
        assert_eq!(Some(4), wl1.focused_window);

        // swapping from master to master, no swap action is taken. Focused
        // window is changed, though.

        wm.swap_with_master(1).unwrap();
        let wl2 = wm.get_window_layout();

        assert_eq!(Some(1), wl2.focused_window);

        // swapping from master to window 3, now window 3 is master window
        // and is focused
        wm.swap_with_master(3).unwrap();
        let wl3 = wm.get_window_layout();
        assert_eq!(Some(3), wm.get_master_window());

        assert_eq!(Some(3), wl3.focused_window);

        // Traying to swap from master to an unknown window, erro is thrown
        assert!(wm.swap_with_master(10).is_err());

        // Since previously I used  swap_with_master the collection was
        // updated to [3,2,1,4], where 3 is the master window
        // using swap_windows(PrevOrNext::Prev) should be allocate windows
        // 4 as master window and put window 3 in the bottom right corner
        // of the window. Moreover windows 3 should be focused.
        let master_half = Geometry {
            x: 0,
            y: 0,
            width: 400,
            height: 600,
        };

        let first_half = Geometry {
            x: 400,
            y: 0,
            width: 400,
            height: 200,
        };

        let second_half = Geometry {
            x: 400,
            y: 200,
            width: 400,
            height: 200,
        };

        let third_half = Geometry {
            x: 400,
            y: 400,
            width: 400,
            height: 200,
        };

        let wl4a = wm.get_window_layout();
        assert_eq!(vec![(3, master_half), (2, first_half), (1, second_half), (4, third_half)],
                   wl4a.windows);


        wm.swap_windows(PrevOrNext::Prev);
        let wl4 = wm.get_window_layout();
        assert_eq!(Some(4), wm.get_master_window());
        assert_eq!(Some(3), wl4.focused_window);
        assert_eq!(vec![(4, master_half), (2, first_half), (1, second_half), (3, third_half)],
                   wl4.windows);

        // I change focused to master window and apply
        // swap_windows(PrevOrNext::Next), the result should be window 2 as
        // master window while windows 4 is focused
        wm.focus_window(Some(4)).unwrap();
        wm.swap_windows(PrevOrNext::Next);
        let wl5 = wm.get_window_layout();
        assert_eq!(Some(2), wm.get_master_window());
        assert_eq!(Some(4), wl5.focused_window);
        assert_eq!(vec![(2, master_half), (4, first_half), (1, second_half), (3, third_half)],
                   wl5.windows);

        // **Invariant**: if `swap_with_master(w)` succeeds,
        // `get_master_window() == Some(w)`.
        wm.swap_with_master(1).unwrap();
        assert_eq!(wm.get_master_window(), Some(1));

        // **Invariant**: `get_master_window() == Some(w)`, then `w`
        //  must occur in the vector returned by `get_windows()`.

        let master_window = wm.get_master_window().unwrap();
        let windows = wm.get_windows();
        assert!(windows.contains(&master_window));

        // **Invariant**: if the vector returned by `get_windows()`
        // is empty => `get_master_window() == None`.
        wm.remove_window(2).unwrap();
        wm.remove_window(3).unwrap();
        wm.remove_window(4).unwrap();
        wm.remove_window(1).unwrap();

        assert!(wm.get_windows().is_empty());
        assert_eq!(wm.get_master_window(), None);

        // The other direction of the arrow must
        // not hold, e.g., there could floating windows (see `FloatSupport`),
        // but no tiled windows.
        wm.add_window(WindowWithInfo::new_float(90, SOME_GEOM)).unwrap();
        wm.add_window(WindowWithInfo::new_float(80, SOME_GEOM)).unwrap();
        assert_eq!(wm.get_master_window(), None);
        assert!(!wm.get_windows().is_empty());

    }


    #[test]
    fn test_floating_support() {

        let mut wm = FullscreenWM::new(SCREEN);

        // Add some windows
        wm.add_window(WindowWithInfo::new_tiled(1, SOME_GEOM)).unwrap();
        wm.add_window(WindowWithInfo::new_tiled(2, SOME_GEOM)).unwrap();
        wm.add_window(WindowWithInfo::new_tiled(3, SOME_GEOM)).unwrap();
        wm.add_window(WindowWithInfo::new_tiled(4, SOME_GEOM)).unwrap();
        wm.add_window(WindowWithInfo::new_tiled(5, SOME_GEOM)).unwrap();

        // No floating window manager
        assert!(wm.get_floating_windows().is_empty());

        // Now let's do 4 and 1 floating elementes
        wm.toggle_floating(4).unwrap();
        wm.toggle_floating(1).unwrap();
        wm.toggle_floating(5).unwrap();

        // the order of the vector changed to [2,3,4,1,5], where 4 and 1 are
        // floating windows
        assert_eq!(vec![4, 1, 5], wm.get_floating_windows());

        // focused should remind to the last window added, 5
        let wl = wm.get_window_layout();
        assert_eq!(Some(5), wl.focused_window);

        // Now I return window 5 to a tiled one and focuse window 4
        wm.toggle_floating(5).unwrap();
        wm.focus_window(Some(4)).unwrap();

        // now let check the layout, where tiled windows should be at the
        // begining of the vec while floating elements
        // should at the last, both floating elements should have SOME_GEOM
        // as geometry.
        // The remining elements should have a proper geometry depending in
        // its position and focused floating element should be the
        // last element [2,3,5,1,4]
        let master_half = Geometry {
            x: 0,
            y: 0,
            width: 400,
            height: 600,
        };

        let first_half = Geometry {
            x: 400,
            y: 0,
            width: 400,
            height: 300,
        };

        let second_half = Geometry {
            x: 400,
            y: 300,
            width: 400,
            height: 300,
        };

        let wl1 = wm.get_window_layout();
        assert_eq!(vec![(2, master_half),
                        (3, first_half),
                        (5, second_half),
                        (1, SOME_GEOM),
                        (4, SOME_GEOM)],
                   wl1.windows);

        // now let's change the geometry of a titled window, an error should
        // be occur
        assert!((wm.set_window_geometry(2, SCREEN_GEOM)).is_err());


        // now let's change the geometry of window 4,
        wm.set_window_geometry(4, SCREEN_GEOM).unwrap();

        // this change should be reflected in the window layout
        let wl2 = wm.get_window_layout();
        assert_eq!(vec![(2, master_half),
                        (3, first_half),
                        (5, second_half),
                        (1, SOME_GEOM),
                        (4, SCREEN_GEOM)],
                   wl2.windows);

        // now we use toggle_floating again in window 1, allocated at the
        // end of the tiled windows.
        // The appropiate modification of the entire tiled layout should be
        // reflected
        // the focuse element should remind as 4
        wm.toggle_floating(1).unwrap();

        let first_half_a = Geometry {
            x: 400,
            y: 0,
            width: 400,
            height: 200,
        };

        let second_half_a = Geometry {
            x: 400,
            y: 200,
            width: 400,
            height: 200,
        };

        let third_half_a = Geometry {
            x: 400,
            y: 400,
            width: 400,
            height: 200,
        };

        let wl3 = wm.get_window_layout();
        assert_eq!(vec![(2, master_half),
                        (3, first_half_a),
                        (5, second_half_a),
                        (1, third_half_a),
                        (4, SCREEN_GEOM)],
                   wl3.windows);
        assert_eq!(Some(4), wl3.focused_window);

        // **Invariant**: if `is_floating(w) == true` for some window `w`,
        // then `is_managed(w) == true`.
        assert_eq!(wm.is_floating(4), true);
        assert_eq!(wm.is_managed(4), true);

        // **Invariant**: `is_floating(w) == true` for some window `w`, iff
        // the vector returned by the `get_floating_windows` method contains
        // `w`.
        assert_eq!(wm.is_floating(4), true);
        assert_eq!(vec![4], wm.get_floating_windows());

        // **Invariant**: if calling `toggle_floating(w)` with a tiled window
        //  `w` succeeds, `is_floating(w)` must return `true`.
        assert_eq!(wm.is_floating(1), false);
        wm.toggle_floating(1).unwrap();
        assert_eq!(wm.is_floating(1), true);

        // **Invariant**: if calling `toggle_floating(w)` with a floating
        // window `w` succeeds, `is_floating(w)` must return `false`.
        assert_eq!(wm.is_floating(4), true);
        wm.toggle_floating(4).unwrap();
        assert_eq!(wm.is_floating(4), false);

        // **Invariant**: the result of `is_floating(w)` must be the same
        // before and after calling `toggle_floating(w)` twice.
        assert_eq!(wm.is_floating(5), false);
        wm.toggle_floating(5).unwrap();
        wm.toggle_floating(5).unwrap();
        assert_eq!(wm.is_floating(5), false);

    }

    #[test]
    fn test_minimise_support() {

        let mut wm = FullscreenWM::new(SCREEN);

        let master_half = Geometry {
            x: 0,
            y: 0,
            width: 400,
            height: 600,
        };

        // Add some windows
        wm.add_window(WindowWithInfo::new_float(1, SOME_GEOM)).unwrap();
        wm.add_window(WindowWithInfo::new_float(2, SOME_GEOM)).unwrap();
        wm.add_window(WindowWithInfo::new_tiled(3, SOME_GEOM)).unwrap();
        wm.add_window(WindowWithInfo::new_tiled(4, SOME_GEOM)).unwrap();
        wm.add_window(WindowWithInfo::new_tiled(5, SOME_GEOM)).unwrap();
        wm.add_window(WindowWithInfo::new_tiled(6, SOME_GEOM)).unwrap();

        assert_eq!(wm.get_window_info(3).unwrap().geometry, master_half);
        // No minimised windows
        assert!(wm.get_minimised_windows().is_empty());

        assert_eq!(wm.get_window_info(3).unwrap().geometry, master_half);

        // Now let's minise window 5, 1 and 4
        wm.toggle_minimised(5).unwrap();
        assert_eq!(wm.get_window_info(3).unwrap().geometry, master_half);
        wm.toggle_minimised(1).unwrap();
        assert_eq!(wm.get_window_info(3).unwrap().geometry, master_half);
        wm.toggle_minimised(4).unwrap();
        assert_eq!(wm.get_window_info(3).unwrap().geometry, master_half);

        // we have some minimised_windows, the array is given in the order
        // the windows were minimised
        assert_eq!(vec![5, 1, 4], wm.get_minimised_windows());


        assert_eq!(true, wm.is_minimised(5));
        assert_eq!(true, wm.is_minimised(1));
        assert_eq!(true, wm.is_minimised(4));

        // the window layout shows window 2 and 3 and 6, where 2 is floating
        // and 3, 6 are tiled
        let master_half = Geometry {
            x: 0,
            y: 0,
            width: 400,
            height: 600,
        };

        let second_half = Geometry {
            x: 400,
            y: 0,
            width: 400,
            height: 600,
        };

        // assert_eq!(wm.get_window_info(3).unwrap().geometry, master_half);
        assert_eq!(wm.get_window_info(3).unwrap().float_or_tile,
                   FloatOrTile::Tile);
        assert_eq!(wm.get_window_info(3).unwrap().fullscreen, false);

        let wl1 = wm.get_window_layout();
        assert_eq!(vec![(3, master_half), (6, second_half), (2, SOME_GEOM)],
                   wl1.windows);

        // window 6 remains as the focused window
        assert_eq!(Some(6), wl1.focused_window);

        //  **Invariant**: if calling `toggle_minimised(w)` with an
        // unminimised window `w` succeeds, `w` may no longer be visible
        // according to `get_window_layout` and `is_minimised(w)` must
        // return `true`.
        wm.toggle_minimised(6).unwrap();
        let wl2 = wm.get_window_layout();
        assert_eq!(vec![(3, FULLSCREEN), (2, SOME_GEOM)], wl2.windows);
        assert_eq!(wm.is_minimised(6), true);

        // **Invariant**: if calling `toggle_minimised(w)` with an already
        // minimised window `w` succeeds, `w` must be visible according to
        // `get_window_layout` and `is_minimised(w)` must return `false`.
        // now let's change the geometry of window 4,
        wm.toggle_minimised(1).unwrap();
        let wl3 = wm.get_window_layout();
        assert_eq!(vec![(3, FULLSCREEN), (1, SOME_GEOM), (2, SOME_GEOM)],
                   wl3.windows);
        assert_eq!(wm.is_minimised(1), false);


        // **Invariant**: if `is_minimised(w) == true` for some window `w`,
        // then `is_managed(w) == true`.
        assert_eq!(wm.is_minimised(5), true);
        assert_eq!(wm.is_managed(5), true);

        // **Invariant**: `is_minimised(w) == true` for some window `w`, iff
        // the vector returned by the `get_minimised_windows` method contains
        // `w`.
        assert_eq!(wm.is_minimised(5), true);
        assert_eq!(vec![5, 4, 6], wm.get_minimised_windows());
    }


    #[test]
    fn test_fullscreen_support() {

        let mut wm = FullscreenWM::new(SCREEN);

        // Add some windows
        wm.add_window(WindowWithInfo::new_float(1, SOME_GEOM)).unwrap();
        wm.add_window(WindowWithInfo::new_float(2, SOME_GEOM)).unwrap();
        wm.add_window(WindowWithInfo::new_tiled(3, SOME_GEOM)).unwrap();
        wm.add_window(WindowWithInfo::new_tiled(4, SOME_GEOM)).unwrap();
        wm.add_window(WindowWithInfo::new_float(5, SOME_GEOM)).unwrap();
        wm.add_window(WindowWithInfo::new_tiled(6, SOME_GEOM)).unwrap();

        assert_eq!(wm.get_fullscreen_window(), None);


        // Return the current fullscreen, if any.
        //
        // **Invariant**: if `get_fullscreen_window() == Some(w)`, then
        // `is_managed(w) == true`.
        wm.add_window(WindowWithInfo::new_fullscreen(7, SOME_GEOM)).unwrap();
        assert_eq!(wm.get_fullscreen_window(), Some(7));
        assert_eq!(wm.is_managed(7), true);

        // the only window given by the get_window_layout is the fullscreen
        // window
        let wl1 = wm.get_window_layout();
        assert_eq!(vec![(7, FULLSCREEN)], wl1.windows);

        // **Invariant**: if `get_fullscreen_window() == Some(w)`, then
        // `get_focused_window() == Some(w)`.
        assert_eq!(wm.get_fullscreen_window(), Some(7));
        assert_eq!(wm.get_focused_window(), Some(7));

        // Make the given window fullscreen, or when it is already fullscreen,
        // undo it.
        //
        // When called on a window that is already fullscreen, it should
        // restore the window to the state before, e.g. float at the same
        // place.
        // **Hint**: you could use the `float_or_tile` field of
        // `WindowWithInfo`.

        // **Invariant**: if calling `toggle_fullscreen(w)` with a window `w`
        // that is not yet fullscreen, `w` should be the only visible window
        // according to `get_window_layout`, its geometry should be the
        // same size as the screen, and `get_fullscreen_window(w) == Some(w)`.
        wm.toggle_fullscreen(4).unwrap();

        let wl1 = wm.get_window_layout();
        assert_eq!(vec![(4, FULLSCREEN)], wl1.windows);
        assert_eq!(wm.get_focused_window(), Some(4));

        // if I add new window then the new window is focused and the
        // fullscreen is no longer enable
        wm.add_window(WindowWithInfo::new_float(8, SOME_GEOM)).unwrap();
        assert_eq!(wm.get_focused_window(), Some(8));
        assert_eq!(wm.get_fullscreen_window(), None);

        // I use toggle_fullscreen window 6 fullscreen
        wm.toggle_fullscreen(6).unwrap();
        assert_eq!(wm.get_fullscreen_window(), Some(6));
        assert_eq!(wm.get_focused_window(), Some(6));

        // I focused window 1 and no fullscreen is found
        wm.focus_window(Some(1)).unwrap();
        assert_eq!(wm.get_focused_window(), Some(1));
        assert_eq!(wm.get_fullscreen_window(), None);

        // I used swap_with_master with window 4
        // window 4 is now the master and the focused window
        wm.swap_with_master(4).unwrap();
        assert_eq!(wm.get_focused_window(), Some(4));
        assert_eq!(wm.get_master_window(), Some(4));

        // I use toggle_fullscreen window 4 fullscreen
        wm.toggle_fullscreen(4).unwrap();
        assert_eq!(wm.get_fullscreen_window(), Some(4));
        assert_eq!(wm.get_focused_window(), Some(4));

        // I use swap_windows to next element since the focuse window is
        // the fullscreen window after use¡ing it, no fullscreen should
        // be there and 4 is still focused
        wm.swap_windows(PrevOrNext::Next);
        assert_eq!(wm.get_fullscreen_window(), None);
        assert_eq!(wm.get_focused_window(), Some(4));

        // I use toggle_fullscreen window 1 fullscreen
        wm.toggle_fullscreen(1).unwrap();
        assert_eq!(wm.get_fullscreen_window(), Some(1));
        assert_eq!(wm.get_focused_window(), Some(1));

        // minised window 4, since if can be minised in second plan the
        // fullscreen is still available
        wm.toggle_minimised(4).unwrap();
        assert_eq!(wm.is_minimised(4), true);
        assert_eq!(wm.get_fullscreen_window(), Some(1));
        assert_eq!(wm.get_focused_window(), Some(1));

        // minised window 1, then no fullscreen is available and no focused
        // window
        wm.toggle_minimised(1).unwrap();
        assert_eq!(wm.is_minimised(4), true);
        assert_eq!(wm.is_minimised(1), true);
        assert_eq!(wm.get_fullscreen_window(), None);
        assert_eq!(wm.get_focused_window(), None);

        // I use toggle_fullscreen window 4 fullscreen
        wm.toggle_fullscreen(4).unwrap();
        assert_eq!(wm.get_fullscreen_window(), Some(4));
        assert_eq!(wm.get_focused_window(), Some(4));

        // Making floating window 1 should not affect the fullscreen window
        wm.toggle_floating(1).unwrap();
        assert_eq!(wm.get_fullscreen_window(), Some(4));
        assert_eq!(wm.get_focused_window(), Some(4));

        // it is not the case if 4 is transform to a tiled window
        wm.toggle_floating(4).unwrap();
        assert_eq!(wm.get_fullscreen_window(), None);
        assert_eq!(wm.get_focused_window(), Some(4));

        // I use toggle_fullscreen window 2 fullscreen
        wm.toggle_fullscreen(2).unwrap();
        assert_eq!(wm.get_fullscreen_window(), Some(2));
        assert_eq!(wm.get_focused_window(), Some(2));

        // I removed window 1, previous fullscreen should remain
        wm.remove_window(1).unwrap();
        assert_eq!(wm.get_fullscreen_window(), Some(2));
        assert_eq!(wm.get_focused_window(), Some(2));

        // I removed window 2, no fullscreen and no fucused window is managed
        wm.remove_window(2).unwrap();
        assert_eq!(wm.get_fullscreen_window(), None);
        assert_eq!(wm.get_focused_window(), None);

        // I use toggle_fullscreen window 2 fullscreen
        wm.toggle_fullscreen(6).unwrap();
        assert_eq!(wm.get_fullscreen_window(), Some(6));
        assert_eq!(wm.get_focused_window(), Some(6));

        // I resize the window, then fullscreen should update its geometry
        wm.resize_screen(SCREEN2);
        assert_eq!(wm.get_screen(), SCREEN2);
        let wl1 = wm.get_window_layout();
        assert_eq!(vec![(6, wm.screen.to_geometry())], wl1.windows);

    }
}
