//! Floating Windows
//!
//! Extend your window manager with support for floating windows, i.e. windows
//! that do not tile but that you move around and resize with the mouse. These
//! windows will *float* above the tiles, e.g. dialogs, popups, video players,
//! etc. See the documentation of the [`FloatSupport`] trait for the precise
//! requirements.
//!
//! Either make a copy of the tiling window manager you developed in the
//! previous assignment and let it implement the [`FloatSupport`] trait as
//! well, or implement the [`FloatSupport`] trait by building a wrapper around
//! your tiling window manager. This way you won't have to copy paste code.
//! Note that this window manager must still implement the [`TilingSupport`]
//! trait.
//!
//! [`FloatSupport`]: ../../cplwm_api/wm/trait.FloatSupport.html
//! [`TilingSupport`]: ../../cplwm_api/wm/trait.TilingSupport.html
//!
//! # Status
//!
//! **TODO**: Replace the question mark below with YES, NO, or PARTIAL to
//! indicate the status of this assignment. If you want to tell something
//! about this assignment to the grader, e.g., you have a bug you can't fix,
//! or you want to explain your approach, write it down after the comments
//! section.
//!
//! COMPLETED: Yes
//!
//! COMMENTS:
//!
//! ## General approach
//!
//! The approach now is quite similar to b_tillin_vm, but now
//! update_geometries is changed. First processing tiled windows
//! and then floating windows.
//!
//! Now the WindowWithInfo structure is decomposed and a new structured is
//! created since now it is important to save the floating geometry
//! of the window, even when it is a tiled one.
//!
//! In case a floating window is changed to tiled window, the order is kept
//! it so get_window_layout will return such windows in the order
//! of the current *windows* vec.

// Add imports here
use std::error;
use std::fmt;
use cplwm_api::types::{FloatOrTile, Geometry, PrevOrNext, Screen, Window, WindowLayout,
                       WindowWithInfo};
use cplwm_api::wm::WindowManager;
use cplwm_api::wm::TilingSupport;
use cplwm_api::wm::FloatSupport;


/// Window manager aliase.
pub type WMName = FloatingWM;

/// The FloatingWindow struct
#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct FloatingWindow {
    /// The window.
    pub window: Window,
    /// The geometry of the window.
    pub geometry: Geometry,
    /// The saved floating geometry of the window.
    pub saved_geometry: Geometry,
    /// Indicate whether the window should float or tile.
    pub float_or_tile: FloatOrTile,
    /// Indicate whether the window should be displayed fullscreen or not.
    /// Although it is not supported in this WM
    pub fullscreen: bool,
}

/// The FloatingWM struct
#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct FloatingWM {
    /// A vector of FloatingWindow, now the first part contains tiled windows
    /// while the second
    /// part contains floatign windows
    pub windows: Vec<FloatingWindow>,
    /// The size of the screen.
    pub screen: Screen,
    /// The index of the focused window in the *windows*, if there is no
    /// focused window a None is found.
    pub index_foused_window: Option<usize>,
}

/// Supported functions
impl FloatingWM {
    /// return the next tiled window from the given index
    ///
    /// it loops over the *windows* vec in its current order, it loops over
    /// it until either the next right side tile index is found or
    /// it reached the given index (in which a None is returned)
    pub fn get_next_tile_index(&self, index: usize, saved: usize) -> Option<usize> {
        let mut next_index = 0;
        if self.windows.len() - 1 > index {
            next_index = index + 1;
        }
        if next_index == saved {
            None
        } else {
            if self.windows.get(next_index).unwrap().float_or_tile == FloatOrTile::Tile {
                Some(next_index)
            } else {
                self.get_next_tile_index(next_index, saved)
            }
        }
    }

    /// return the previous tiled window from the given index
    ///
    /// it loops over the *windows* vec in its current order, it loops over
    /// it until either the next left side tile index is found or
    /// it reached the given index (in which a None is returned)
    pub fn get_prev_tile_index(&self, index: usize, saved: usize) -> Option<usize> {
        let mut prev_index = self.windows.len() - 1;
        if index != 0 {
            prev_index = index - 1;
        };
        if prev_index == saved {
            None
        } else {
            if self.windows.get(prev_index).unwrap().float_or_tile == FloatOrTile::Tile {
                Some(prev_index)
            } else {
                self.get_prev_tile_index(prev_index, saved)
            }
        }
    }

    /// get the master window index
    ///
    /// the logic of the b_tilling_wm reminds, the first tiled element of the
    /// list is the master window,
    fn get_master_index(&self) -> Option<usize> {
        if !self.windows.is_empty() {
            self.windows.iter().position(|w| (*w).float_or_tile == FloatOrTile::Tile)
        } else {
            None
        }
    }

    /// returns the index where the first floating window starts
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
    fn update_geometries(&mut self) {
        if !self.windows.is_empty() {

            // Divisor now should be update to just the tiled windows
            let non_tiled_windows = self.get_floating_windows().len() + 1;
            let total_windows = self.windows.len();
            // let divisor = self.windows.len() - non_tiled_windows;

            // if the divisor is greater than 0 we need to calculate slave
            // windows
            // if divisor > 0{
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

                for floating_window in self.windows
                    .iter_mut()
                    .filter(|x| (*x).float_or_tile == FloatOrTile::Tile) {
                    if master_window != floating_window.window {
                        // I calculate the values of the secondary windows
                        // (right windows)
                        let rigth_geometry = Geometry {
                            x: x_point,
                            y: y_point,
                            width: width_side,
                            height: height_side,
                        };
                        floating_window.geometry = rigth_geometry;
                        y_point += (height_side) as i32;

                    } else {
                        // I calculate the values for master window
                        let master_geometry = Geometry {
                            x: 0,
                            y: 0,
                            width: width_side,
                            height: self.screen.height,
                        };
                        floating_window.geometry = master_geometry;
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
}

/// The errors that this window manager can return.
#[derive(Debug)]
pub enum FloatingWMError {
    /// This window is not known by the window manager.
    UnknownWindow(Window),
    /// This window is already managed by this window manager
    ManagedWindow(Window),
    /// This window is not a floating window
    NoFloatingWindow(Window),
    /// This window is not a tiled window
    NoTiledWindow(Window),
}

impl fmt::Display for FloatingWMError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FloatingWMError::UnknownWindow(ref window) => write!(f, "Unknown window: {}", window),
            FloatingWMError::ManagedWindow(ref window) => {
                write!(f, "Window {} is already managed", window)
            }
            FloatingWMError::NoFloatingWindow(ref window) => {
                write!(f, "Window {} is not floating", window)
            }
            FloatingWMError::NoTiledWindow(ref window) => {
                write!(f, "Window {} is not tiled", window)
            }
        }
    }
}

impl error::Error for FloatingWMError {
    fn description(&self) -> &'static str {
        match *self {
            FloatingWMError::UnknownWindow(_) => "Unknown window",
            FloatingWMError::ManagedWindow(_) => "Window is already managed",
            FloatingWMError::NoFloatingWindow(_) => "Window is not floating",
            FloatingWMError::NoTiledWindow(_) => "Window is not tiled",
        }
    }
}


impl WindowManager for FloatingWM {
    type Error = FloatingWMError;

    /// The FloatingWM constructor.
    ///
    /// windows is initialised as empty vec, screen as the given screen and
    ///  focused index as None.
    fn new(screen: Screen) -> FloatingWM {
        FloatingWM {
            windows: Vec::new(),
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

    /// adds new window_with_info to the vec windows
    ///
    /// the fullscreen windows are accepted and added, but they are treated
    /// as tiled or floating window. The given geometry by the
    /// window_with_info is set as the saved_geometry and geometry attributes
    /// of the FloatingWindow structure.
    ///
    /// if the given window has an attribute FloatOrTile::Tile then the
    /// geometries for tiled windows should be updated.
    ///
    /// Now add_window adds the tiled and floating windows in order, so the
    /// first part of the vec contains
    /// the tiled windows while the second part are the floating windows
    ///
    /// returns an ManagedWindow error if the given window_with_info is
    /// already managed by the window manager.
    fn add_window(&mut self, window_with_info: WindowWithInfo) -> Result<(), Self::Error> {
        if !self.is_managed(window_with_info.window) {
            let floating_window = FloatingWindow {
                window: window_with_info.window,
                geometry: window_with_info.geometry,
                saved_geometry: window_with_info.geometry,
                float_or_tile: window_with_info.float_or_tile,
                fullscreen: window_with_info.fullscreen,
            };
            if window_with_info.float_or_tile == FloatOrTile::Float {
                // if there is a floating window, it is inserted at the end
                // of the vec
                self.windows.push(floating_window);
                let temp = self.windows.len() - 1;
                self.index_foused_window = Some(temp);
            } else {
                // if not, I get the index where the first floating window
                // start and insert the tiled window there, also the
                // geometries should be updated.
                match self.get_partion_index() {
                    None => {
                        self.windows.push(floating_window);
                        let temp = self.windows.len() - 1;
                        self.index_foused_window = Some(temp);
                    }
                    Some(partion_index) => {
                        self.windows.insert(partion_index, floating_window);
                        self.index_foused_window = Some(partion_index);
                    }
                };
                self.update_geometries();
            };
            Ok(())
        } else {
            Err(FloatingWMError::ManagedWindow(window_with_info.window))
        }
    }

    /// removes the given window from the window manager.
    ///
    /// Every time that an element is remove the index_foused_window should
    /// be updated if it is necessary.
    /// Important to noticy here is that when the focused element is the same
    /// as the removed element, no focused window is set.
    fn remove_window(&mut self, window: Window) -> Result<(), Self::Error> {
        match self.windows.iter().position(|w| (*w).window == window) {
            None => Err(FloatingWMError::UnknownWindow(window)),
            Some(i) => {
                // unwrap() is used because the some(i) statement ensures at
                // least one element
                let temp_window = self.windows.get(i).unwrap().clone();
                self.windows.remove(i);

                if temp_window.float_or_tile == FloatOrTile::Tile {
                    self.update_geometries();
                };
                match self.index_foused_window {
                    None => Ok(()),

                    Some(index) => {
                        // If the focused_element is the one that is erased,
                        // None focused element
                        if index == i {
                            self.index_foused_window = None;
                            Ok(())
                        } else {
                            // If the focused_element in the right side of
                            // the vector the focused elemente is kept it,
                            // otherwise the focused index is decreased by
                            // one.
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
    /// The array is already order so, no need to rearrange the vector except
    /// when there is a focused floating window
    fn get_window_layout(&self) -> WindowLayout {

        if !self.windows.is_empty() {

            let mut temp_windows = Vec::new();

            for window_info in self.windows.iter() {
                temp_windows.push((window_info.window.clone(), window_info.geometry.clone()))
            }

            let temp_focused_window = match self.index_foused_window {
                None => None,
                Some(index) => {
                    // unwrap() is used because the some(i) statement ensures
                    // at least one element
                    let window_type = self.windows.get(index).unwrap();
                    match self.get_partion_index() {
                        None => Some(self.windows.get(index).unwrap().window),
                        // if a focused window is a floating one, then the
                        // temp_windows is reorder to put the focused window
                        // at the very end of the vec
                        Some(partion_index) => {
                            if partion_index >= index &&
                               window_type.float_or_tile == FloatOrTile::Float {
                                match temp_windows.iter().position(|w| (*w)
                                    == (window_type.window,window_type.
                                        geometry)) {
										None => (),
                                        Some(real_index) => {
											let (temp_window,temp_geometry)
                                            = temp_windows.get(real_index).
                                            unwrap().clone();
											temp_windows.remove(real_index);
											temp_windows.push((temp_window,
                                                temp_geometry));
										},
								};
                            };
                            // unwrap() is used because the
                            // some(partion_index) statement ensures at least
                            // one element
                            Some(self.windows.get(index).unwrap().window)
                        }
                    }
                }
            };

            WindowLayout {
                focused_window: temp_focused_window,
                windows: temp_windows,
            }

        } else {
            WindowLayout::new()
        }
    }

    /// set the focused window in the window manager with the given window.
    fn focus_window(&mut self, window: Option<Window>) -> Result<(), Self::Error> {
        match window {
            None => {
                self.index_foused_window = None;
                Ok(())
            }

            Some(gw) => {
                match self.windows.iter().position(|w| (*w).window == gw) {
                    None => Err(FloatingWMError::UnknownWindow(gw)),

                    Some(i) => {
                        self.index_foused_window = Some(i);
                        Ok(())
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
        }
    }

    /// gets the complete current information of the given window.
    ///
    /// If the given window is not managed by the window manager,
    /// UnknownWindow error is shown.
    fn get_window_info(&self, window: Window) -> Result<WindowWithInfo, Self::Error> {
        match self.windows.iter().position(|w| (*w).window == window) {
            None => Err(FloatingWMError::UnknownWindow(window)),
            Some(i) => {
                // unwrap() is used because the some(i) statement ensures at
                // least one element
                let floating_window = self.windows.get(i).unwrap().clone();
                let window_with_info = WindowWithInfo {
                    window: floating_window.window,
                    geometry: floating_window.geometry,
                    float_or_tile: floating_window.float_or_tile,
                    fullscreen: floating_window.fullscreen,
                };
                Ok(window_with_info)
            }
        }
    }

    /// gets the current window screen size.
    fn get_screen(&self) -> Screen {
        self.screen
    }

    /// set the given screen as new screen size.
    ///
    /// The geometries should be updated accordingly with the new given
    /// screen.
    fn resize_screen(&mut self, screen: Screen) {
        self.screen = screen;
        self.update_geometries()
    }
}


impl TilingSupport for FloatingWM {
    /// if *windows* is not empty it returns the master window, otherwise
    /// return None.
    ///
    /// In this window manager the first tiled element of the *windows* vec
    /// is always the master window.
    fn get_master_window(&self) -> Option<Window> {
        if !self.windows.is_empty() {
            match self.get_master_index() {
                None => None,
                Some(index) => Some(self.windows.get(index).unwrap().window),
            }
        } else {
            None
        }
    }

    /// swap the position of the given window with the master window.
    ///
    /// This functions actually affects the order in which the windows were
    /// added, also the geometries should be updated accordingly.
    ///
    /// In case the given window is the master window  and the window master
    /// is not focused, the only effect of this funtions is
    /// changing the focused windows to the master window.
    fn swap_with_master(&mut self, window: Window) -> Result<(), Self::Error> {
        match self.windows.iter().position(|w| (*w).window == window) {
            None => Err(FloatingWMError::UnknownWindow(window)),
            Some(window_index) => {
                // unwrap() is used because the some(window_index) statement
                // ensures at least one element
                if self.windows.get(window_index).unwrap().float_or_tile == FloatOrTile::Tile {
                    match self.get_master_index() {
                        None => Ok(()),
                        Some(master_index) => {
                            self.windows.swap(master_index, window_index);
                            self.update_geometries();
                            self.focus_window(Some(window))
                        }
                    }
                } else {
                    Err(FloatingWMError::NoTiledWindow(window))
                }
            }
        }
    }


    /// Simlar approach than cycle_focus, but now the Vec is affected, hence
    /// the order in which the windows were added is changed.
    ///
    /// If there is no focused window or if the number of tiled windows is
    /// less than 2 the funtion has no effects.
    ///
    /// After the sucessful swap, the geometries should be updated
    /// accordingly.
    fn swap_windows(&mut self, dir: PrevOrNext) {
        let tiled_windows = self.windows.len() - self.get_floating_windows().len();
        if tiled_windows > 1 {
            match self.index_foused_window {
                // no focused window = nothing
                None => (),

                Some(index) => {
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
                        }
                    } else {
                        ()
                    }
                }
            }
        }
    }
}

impl FloatSupport for FloatingWM {
    /// return a vector that contains the floating windows managed by this
    /// window manager
    ///
    /// I have to iterate over the whole collection to filter out the
    /// non-floating windows.
    fn get_floating_windows(&self) -> Vec<Window> {
        let mut temp_windows = Vec::new();

        for window_with_info_floating in
            self.windows.iter().filter(|x| (*x).float_or_tile == FloatOrTile::Float) {
            temp_windows.push(window_with_info_floating.window.clone());
        }

        temp_windows
    }

    /// set the given window to a floating window if it was a tiled window or
    /// viceversa
    ///
    /// after a sucessful toogle, the geometries of the tiled windows should
    /// be updated
    fn toggle_floating(&mut self, window: Window) -> Result<(), Self::Error> {
        match self.windows.iter().position(|w| (*w).window == window) {
            None => Err(FloatingWMError::UnknownWindow(window)),

            Some(i) => {

                let win_info = self.windows.get(i).unwrap().clone();
                let mut focused_window = None;
                match self.index_foused_window {
                    None => (),
                    Some(index) => {
                        // unwrap() is used because the some(index) statement
                        // ensures at least one element
                        let window_info = self.windows.get(index).unwrap().clone();
                        focused_window = Some(window_info.window);
                    }
                };

                if win_info.float_or_tile == FloatOrTile::Tile {
                    // unwrap() is used because the some(index) statement
                    // ensures at least one element
                    self.remove_window(win_info.window).unwrap();
                    let window_info = WindowWithInfo {
                        window: win_info.window,
                        geometry: win_info.saved_geometry,
                        float_or_tile: FloatOrTile::Float,
                        fullscreen: win_info.fullscreen,
                    };
                    // unwrap() is used because the some(index) statement
                    // ensures at least one element
                    self.add_window(window_info).unwrap();
                } else {
                    // unwrap() is used because the some(index) statement
                    // ensures at least one element
                    self.remove_window(win_info.window).unwrap();
                    let window_info = WindowWithInfo {
                        window: win_info.window,
                        geometry: win_info.saved_geometry,
                        float_or_tile: FloatOrTile::Tile,
                        fullscreen: win_info.fullscreen,
                    };
                    // unwrap() is used because the some(index) statement
                    // ensures at least one element
                    self.add_window(window_info).unwrap();
                };
                // after toogle the window, the geometries and the focused
                // window should be updated if that is the case
                // focus_window will always succes since the window is
                // already managed and it was removed in this toggle process
                self.focus_window(focused_window).unwrap();
                self.update_geometries();
                Ok(())
            }
        }
    }

    /// set the given window to a floating window if it was a tiled window or
    /// viceversa
    ///
    /// this function iterates over the windows until the given window is
    /// found, then if it is a floating window
    /// the corresponding FloatingWindow structure is mutated, otherwise a
    /// NoFloatingWindow error is thrown.
    fn set_window_geometry(&mut self,
                           window: Window,
                           new_geometry: Geometry)
                           -> Result<(), Self::Error> {
        match self.windows.iter().position(|w| (*w).window == window) {
            None => Err(FloatingWMError::UnknownWindow(window)),

            Some(i) => {
                // it was already check that there is a window, so unwrap can
                // be used
                let window_with_info = self.windows.get_mut(i).unwrap();

                if window_with_info.float_or_tile == FloatOrTile::Tile {
                    Err(FloatingWMError::NoFloatingWindow(window_with_info.window))
                } else {
                    (*window_with_info).geometry = new_geometry;
                    Ok(())
                }
            }

        }
    }
}

#[cfg(test)]
mod tests {

    // We have to import `FloatingWM` from the super module.
    use super::FloatingWM;
    // We have to repeat the imports we did in the super module.
    use cplwm_api::wm::WindowManager;
    use cplwm_api::wm::TilingSupport;
    use cplwm_api::wm::FloatSupport;
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

    // Now let's write our test.
    //
    // Note that tests are annotated with `#[test]`, and cannot take arguments
    // nor return anything.

    #[test]
    fn test_adding_and_removing_some_windows() {
        // Let's make a new `FloatingWM` with `SCREEN` as screen.
        let mut wm = FloatingWM::new(SCREEN);

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

        let mut wm = FloatingWM::new(SCREEN);

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

        let mut wm = FloatingWM::new(SCREEN);

        // Do nothing
        wm.cycle_focus(PrevOrNext::Next);

        // Add some both type of windows
        // the order it is important depending the type of the window
        // so internal vector should be [1,2,3,5,2,4], whe 5 is focused
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

        let mut wm = FloatingWM::new(SCREEN);

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

        let mut wm = FloatingWM::new(SCREEN);

        // swm screen should be the same as SCREEN
        assert_eq!(wm.get_screen(), SCREEN);

        // now, swm screen should be the same as SCREEN
        wm.resize_screen(SCREEN2);
        assert_eq!(wm.get_screen(), SCREEN2);
    }

    #[test]
    fn test_tiling_support() {

        let mut wm = FloatingWM::new(SCREEN);

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
        // `get_master_window()
        // == Some(w)`.
        wm.swap_with_master(1).unwrap();
        assert_eq!(wm.get_master_window(), Some(1));

        // **Invariant**: `get_master_window() == Some(w)`, then `w` must
        // occur in the vector returned by `get_windows()`.

        let master_window = wm.get_master_window().unwrap();
        let windows = wm.get_windows();
        assert!(windows.contains(&master_window));

        // **Invariant**: if the vector returned by `get_windows()` is
        // empty => `get_master_window() == None`.
        wm.remove_window(2).unwrap();
        wm.remove_window(3).unwrap();
        wm.remove_window(4).unwrap();
        wm.remove_window(1).unwrap();

        assert!(wm.get_windows().is_empty());
        assert_eq!(wm.get_master_window(), None);

        // The other direction of the arrow must
        // not hold, e.g., there could floating windows (see `FloatSupport`),
        //  but no tiled windows.
        wm.add_window(WindowWithInfo::new_float(90, SOME_GEOM)).unwrap();
        wm.add_window(WindowWithInfo::new_float(80, SOME_GEOM)).unwrap();
        assert_eq!(wm.get_master_window(), None);
        assert!(!wm.get_windows().is_empty());

    }


    #[test]
    fn test_floating_support() {

        let mut wm = FloatingWM::new(SCREEN);

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
        // as geometry. The remining elements should have a proper geometry
        // depending in its position and focused floating element should be
        // the last element [2,3,5,1,4]
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

        // now we use toggle_floating again in window 1, allocated at the end
        // of the tiled windows.
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
        // `w` succeeds, `is_floating(w)` must return `true`.
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
    fn advance_test_floating_support() {

        let mut wm = FloatingWM::new(SCREEN);

        // Add some windows
        wm.add_window(WindowWithInfo::new_tiled(1, SOME_GEOM)).unwrap();
        wm.add_window(WindowWithInfo::new_float(4, SOME_GEOM)).unwrap();
        wm.add_window(WindowWithInfo::new_float(5, SOME_GEOM)).unwrap();
        wm.add_window(WindowWithInfo::new_tiled(2, SOME_GEOM)).unwrap();
        wm.add_window(WindowWithInfo::new_float(6, SOME_GEOM)).unwrap();
        wm.add_window(WindowWithInfo::new_float(7, SOME_GEOM)).unwrap();
        wm.add_window(WindowWithInfo::new_tiled(3, SOME_GEOM)).unwrap();

        // In this case the internal structure is [1,2,3,4,5,6,7], where 1,2
        // and 3 are tiled windows and 3 is fosued

        // the floating elements
        assert_eq!(vec![4, 5, 6, 7], wm.get_floating_windows());

        // focused should remin to the last window added, 3
        assert_eq!(Some(3), wm.get_focused_window());

        //* iterating over floating windows **
        //* focused window 4
        wm.cycle_focus(PrevOrNext::Next);
        assert_eq!(Some(4), wm.get_focused_window());

        // focused window 5
        wm.cycle_focus(PrevOrNext::Next);
        assert_eq!(Some(5), wm.get_focused_window());

        // focused window 6
        wm.cycle_focus(PrevOrNext::Next);
        assert_eq!(Some(6), wm.get_focused_window());

        // focused window 7
        wm.cycle_focus(PrevOrNext::Next);
        assert_eq!(Some(7), wm.get_focused_window());

        // the floating windows were passed, iteration start over with the
        // tiled windows * iterating over tiled windows **
        // focused window 1
        wm.cycle_focus(PrevOrNext::Next);
        assert_eq!(Some(1), wm.get_focused_window());

        // focused window 2
        wm.cycle_focus(PrevOrNext::Next);
        assert_eq!(Some(2), wm.get_focused_window());

        // focused window 3
        wm.cycle_focus(PrevOrNext::Next);
        assert_eq!(Some(3), wm.get_focused_window());

        // focused window 2
        wm.cycle_focus(PrevOrNext::Prev);
        assert_eq!(Some(2), wm.get_focused_window());

        // focused window 1
        wm.cycle_focus(PrevOrNext::Prev);
        assert_eq!(Some(1), wm.get_focused_window());

        // * iterating over floating windows **
        // focused window 7
        wm.cycle_focus(PrevOrNext::Prev);
        assert_eq!(Some(7), wm.get_focused_window());

        // focused window 6
        wm.cycle_focus(PrevOrNext::Prev);
        assert_eq!(Some(6), wm.get_focused_window());

        // focused window 5
        wm.cycle_focus(PrevOrNext::Prev);
        assert_eq!(Some(5), wm.get_focused_window());

        // focused window 4
        wm.cycle_focus(PrevOrNext::Prev);
        assert_eq!(Some(4), wm.get_focused_window());

        // * return to the initial focused window **
        // focused window 3
        wm.cycle_focus(PrevOrNext::Prev);
        assert_eq!(Some(3), wm.get_focused_window());

    }
}
