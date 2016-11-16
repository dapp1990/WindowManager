//! Fullscreen Window Manager
//!
//! Implement the [`WindowManager`] trait by writing a simple window manager
//! that displays every window fullscreen. When a new window is added, the
//! last window that was visible will become invisible.
//!
//! [`WindowManager`]: ../../cplwm_api/wm/trait.WindowManager.html
//!
//! Now have a look at the source code of this file, it contains a tutorial to
//! help you write the fullscreen window manager.
//!
//! You are free to remove the documentation in this file that is only part of
//! the tutorial or no longer matches the code after your changes.
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
//!
//! There is no need to actually keep track the focused window, the describetion states that the last element of the vec is focused 
//! by default, the only possible way to have a None focused window is when there is no windows, and that behaviour is already handled by
//! the WindowLayout constructor.
//!
//! It is convenient to store the complete WindowWithInfo instead of the Window, so the given functions implementations where updated to
//! the structure.
//!
//! Finally, for the cycle_focus it is more useful if one can push front/back and pop back/front (double-ended queue) to handle the rotation of the vec in order
//! to change the focused window, so a VecDeque is used instead of Vec
//! 

// Import modules
use std::error;
use std::fmt;
use std::collections::VecDeque;
use cplwm_api::types::{PrevOrNext, Screen, Window, WindowLayout, WindowWithInfo};
use cplwm_api::wm::WindowManager;

/// Window manager aliase
pub type WMName = FullscreenWM;


/// The FullscreenWM struct
#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct FullscreenWM {
    /// A vector of WindowWithInfo, the first one is on the bottom, the last one is
    /// on top, and also the only visible window.
    pub windows: VecDeque<WindowWithInfo>,
    /// The size of the screen
    pub screen: Screen,
}


/// Complementary functions for FullscreenWM
impl FullscreenWM {
    /// This method calculated the geometries of windows.
    ///
    /// This is a naive implementation since all windows in this window manager has the same size.
    fn update_geometries(&mut self){
        for full_screen_window in self.windows.iter_mut(){
            full_screen_window.geometry = self.screen.to_geometry();                        
        };
    }
}

/// The errors that this window manager can return.
#[derive(Debug)]
pub enum FullscreenWMError {
    /// This window is not known by the window manager.
    UnknownWindow(Window),
    /// This window is already managed by this window manager
    ManagedWindow(Window),
}

impl fmt::Display for FullscreenWMError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FullscreenWMError::UnknownWindow(ref window) => write!(f, "Unknown window: {}", window),
            FullscreenWMError::ManagedWindow(ref window) => write!(f, "Window {} is already managed", window),
        }
    }
}

impl error::Error for FullscreenWMError {
    fn description(&self) -> &'static str {
        match *self {
            FullscreenWMError::UnknownWindow(_) => "Unknown window",
            FullscreenWMError::ManagedWindow(_) => "Window is already managed",
        }
    }
}

impl WindowManager for FullscreenWM {
    /// We use `FullscreenWMError` as our `Error` type.
    type Error = FullscreenWMError;

    /// The FullscreenWM constructor
    ///
    /// windows is initialised as empty vec and screen as the given screen
    fn new(screen: Screen) -> FullscreenWM {
        FullscreenWM {
            windows: VecDeque::new(),
            screen: screen,
        }
    }

    /// Returns all the managed windows in the window manager
    fn get_windows(&self) -> Vec<Window> {
        // I cretae new vec, which temporaly saves the window elements
        let mut temp_windows = Vec::new();
        // I iterate over WindowManager to get *window_with_info*s to obtain the 
        // window which will be stored in temp_windows.
        for window_with_info in self.windows.iter() {
   			temp_windows.push(window_with_info.window.clone());
		}
		//return temp_windows with window elements
       	temp_windows
    }

    /// gets the current focused window
    ///
    /// If list is not empty I get the last element focused window, otherwise it is returned None
    fn get_focused_window(&self) -> Option<Window> {
        if !self.windows.is_empty(){ 
        	// I use unwrap() because the if test ensure that *windows* has
        	// *window*s
        	if self.windows.len() > 1{ 
		        let last_index = self.windows.len() - 1;      
		        Some(self.windows.get(last_index).unwrap().window)
		    }else{
		    	Some(self.windows.get(0).unwrap().window)
		    }
    	}else{
            
    		None
    	}

    }

    /// adds new window_with_info to the vec windows and set the geometry to fullscreen
    ///
    /// returns an ManagedWindow error if the given window_with_info is already managed by the window manager 
    fn add_window(&mut self, window_with_info: WindowWithInfo) -> Result<(), Self::Error> {
        if !self.is_managed(window_with_info.window) {
            let fullscreen_window = WindowWithInfo {
                window: window_with_info.window, 
                geometry: self.screen.to_geometry(), 
                float_or_tile: window_with_info.float_or_tile, 
                fullscreen: window_with_info.fullscreen,};
            //	self.windows.push(window_with_info.window);
            Ok(self.windows.push_back(fullscreen_window))
        }else {
            Err(FullscreenWMError::ManagedWindow(window_with_info.window))
        }
    }

    /// removes the given window form the window manager
    fn remove_window(&mut self, window: Window) -> Result<(), Self::Error> {
       	match self.windows.iter().position(|w| (*w).window == window) {
            None => Err(FullscreenWMError::UnknownWindow(window)),
            Some(i) => {
                self.windows.remove(i);
                Ok(())
            }
        }
    }

    /// returns the layout of the visible windows, in this case the focused window
    fn get_window_layout(&self) -> WindowLayout {
        if !self.windows.is_empty(){
	        let last_index = self.windows.len() - 1;
            // I used unwrap because it is already tested that there is at least one element in Vec
            let window_with_info = self.windows.get(last_index).unwrap();

            WindowLayout {
                        focused_window: Some(window_with_info.window),
                        windows: vec![(window_with_info.window, window_with_info.geometry)],
                    }
        }else {
            WindowLayout::new()
        } 
    }

    /// set the focused window in the window manager with the given window
    ///
    /// the function uses the remove_window and add_window as subroutines. As a consequence,
    /// the order in which the windows were added can be changed.
    fn focus_window(&mut self, window: Option<Window>) -> Result<(), Self::Error> {
    	// First I check if the given *window* is either a window or a None
    	match window{
    		// If None, nothing happens
    		None => Ok(()),
    		// If Some, focus operation starts
    		Some(gw) => {

		    	// By default *remove_window* only removes *window_with_info* 
		    	// without returns it, so a sliglty modification of that method 
		    	// is done.
		    	// Now the *window_with_info* element is temporaly stored
		    	match self.windows.iter().position(|w| (*w).window == gw) {
		            None => Err(FullscreenWMError::UnknownWindow(gw)),
		            Some(i) => {
		            	// we get a copy of the actually *window_with_info* that is
		            	// in *windows*, we used unwrap() here because we are already
		            	// tested that the actual structure exists
		            	let window_with_info = self.windows.get(i).unwrap().clone();
		            	// the given window is removed
		                self.remove_window(window_with_info.window).unwrap();
		                // Fortunatly, *add_window* can helps us out to add the 
						// *window_with_info*
		                self.add_window(window_with_info)
		            }
		        }
	    	}
   		}
    }

    /// back/forth to the next window from the current focused window.
    ///
    /// If there is no focused window, nothing is focused since that implies a empty windows vec.
    fn cycle_focus(&mut self, dir: PrevOrNext) {
        // I take advantage of the new structure, so I use pop_back() and push_front()
        // for prev action and  pop_front() and push_back() for next action
        // Condition when *windows* is empty or a singleton is covered with a naive 
        // if statement. Focus *window* by definition is always the last element, no
        // need to test whether there is a focus *window* when *windows* is a 
        // singleton, the only *window* is always focus.
        if self.windows.len() > 1{
        	match dir {
        		// I use unwrap() because we already test that *windows* is not empty
	            PrevOrNext::Prev => {
	            	let temp = self.windows.pop_back().unwrap();
	            	self.windows.push_front(temp);
	            }

	            PrevOrNext::Next => {
	            	let temp = self.windows.pop_front().unwrap();
	            	self.windows.push_back(temp);
	            }
	         }
        };
    }

    /// gets the complete current information of the given window.
    ///
    /// If the given window is not managed by the window manager, UnknownWindow error is shown
    fn get_window_info(&self, window: Window) -> Result<WindowWithInfo, Self::Error> {
        // Since now *windows* stores the whole *window_with_info*, I only iterate
        // over, found the corresponding *window* and return the *window_with_info*,
        // otherwise an *UnknownWindow* error is thrown.
    	match self.windows.iter().position(|w| (*w).window == window) {
    		None => Err(FullscreenWMError::UnknownWindow(window)),
    		Some(i) => Ok(self.windows.get(i).unwrap().clone()),
    	}
    }

    /// gets the current window screen size
    fn get_screen(&self) -> Screen {
        self.screen
    }

    /// set the given screen as new screen size
    ///
    /// The geometries should be updated accordingly with the new given screen
    fn resize_screen(&mut self, screen: Screen) {
        self.screen = screen;
        self.update_geometries()
    }
}


/*
#[cfg(test)]
mod tests {

    // We have to import `TillingWM` from the super module.
    use super::FullscreenWM;
    // We have to repeat the imports we did in the super module.
    use cplwm_api::wm::WindowManager;
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
        // Let's make a new `FullscreenWM` with `SCREEN` as screen.
        let mut wm = FullscreenWM::new(SCREEN);
        // Initially the window layout should be empty.
        assert_eq!(WindowLayout::new(), wm.get_window_layout());
        // `assert_eq!` is a macro that will check that the second argument,
        // the actual value, matches first value, the expected value.
        // Let's add a window
        wm.add_window(WindowWithInfo::new_tiled(1, SOME_GEOM)).unwrap();
        // Because `add_window` returns a `Result`, we use `unwrap`, which
        // tries to extract the `Ok` value from the result, but will panic
        // (crash) when it is an `Err`. You must be very careful when using
        // `unwrap` in your code. Here we can use it because we know for sure
        // that an `Err` won't be returned, and even if that were the case,
        // the panic will simply cause the test to fail.
        // The window should now be managed by the WM
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
        // and fullscreen.
        assert_eq!(vec![(2, SCREEN_GEOM)], wl2.windows);
        // Now let's remove window 2
        wm.remove_window(2).unwrap();
        // It should no longer be managed by the WM.
        assert!(!wm.is_managed(2));
        // The `Vec` of windows should now just contain window 1.
        assert_eq!(vec![1], wm.get_windows());
        // According to the window layout
        let wl3 = wm.get_window_layout();
        // window 1 should be focused again
        assert_eq!(Some(1), wl3.focused_window);
        // and fullscreen.
        assert_eq!(vec![(1, SCREEN_GEOM)], wl3.windows);
        // To run these tests, run the command `cargo test` in the `solution`
        // directory.
        //
        // To learn more about testing, check the Testing chapter of the Rust
        // Book: https://doc.rust-lang.org/book/testing.html
    }

    #[test]
    fn test_focus_window() {

        let mut wm = FullscreenWM::new(SCREEN);

        //Add some windows
        wm.add_window(WindowWithInfo::new_tiled(1, SOME_GEOM)).unwrap();
        wm.add_window(WindowWithInfo::new_tiled(2, SOME_GEOM)).unwrap();
        wm.add_window(WindowWithInfo::new_tiled(3, SOME_GEOM)).unwrap();
        wm.add_window(WindowWithInfo::new_tiled(4, SOME_GEOM)).unwrap();
        wm.add_window(WindowWithInfo::new_tiled(5, SOME_GEOM)).unwrap();

        //Now an action should be applied, even when it given in this wm if the vec windows is not empty
        // it must hold that there is a focused window.
        //The only way in which no focused window exists is when there is no widnows
        wm.focus_window(None).unwrap();

        //Focused window should return 5
        let wl1 = wm.get_window_layout();
        assert_eq!(Some(5), wl1.focused_window);

        //Window 10 is not in manager an UnknownWindow error should be thrown
        assert!(wm.focus_window(Some(10)).is_err());

        //Focus to window 4
        wm.focus_window(Some(4)).unwrap();
        let wl2 = wm.get_window_layout();
        assert_eq!(Some(4), wl2.focused_window);

        //Focus to window 2
        wm.focus_window(Some(2)).unwrap();
        let wl3 = wm.get_window_layout();
        assert_eq!(Some(2), wl3.focused_window);

        //Focus to window 5
        wm.focus_window(Some(5)).unwrap();
        let wl4 = wm.get_window_layout();
        assert_eq!(Some(5), wl4.focused_window);

        wm.remove_window(5).unwrap();
        wm.focus_window(Some(1)).unwrap();
        wm.remove_window(1).unwrap();
        //Because the last focused window was removed, the focused_window attribute should be the second botton element
        // in this case window 2
        let wl5 = wm.get_window_layout();
        assert_eq!(Some(2), wl5.focused_window);
    }

    #[test]
    fn test_cycle_focus() {

        let mut wm = FullscreenWM::new(SCREEN);

        //Do nothing
        wm.cycle_focus(PrevOrNext::Next);

        //Add some both type of windows
        wm.add_window(WindowWithInfo::new_tiled(1, SOME_GEOM)).unwrap();
        wm.add_window(WindowWithInfo::new_tiled(2, SOME_GEOM)).unwrap();
        wm.add_window(WindowWithInfo::new_tiled(3, SOME_GEOM)).unwrap();
        wm.add_window(WindowWithInfo::new_tiled(4, SOME_GEOM)).unwrap();
        wm.add_window(WindowWithInfo::new_tiled(5, SOME_GEOM)).unwrap();


        //Focus should be in window 4
        wm.cycle_focus(PrevOrNext::Prev);
        let wl1 = wm.get_window_layout();
        assert_eq!(Some(4), wl1.focused_window);

        //Focus should be in window 3
        wm.cycle_focus(PrevOrNext::Prev);
        let wl2 = wm.get_window_layout();
        assert_eq!(Some(3), wl2.focused_window);

        //Focus should be in window 4
        wm.cycle_focus(PrevOrNext::Next);
        let wl3 = wm.get_window_layout();
        assert_eq!(Some(4), wl3.focused_window);

        //Focus should be in window 5
        wm.cycle_focus(PrevOrNext::Next);
        let wl4 = wm.get_window_layout();
        assert_eq!(Some(5), wl4.focused_window);

        //Focus should be in window 1
        wm.cycle_focus(PrevOrNext::Next);
        let wl4 = wm.get_window_layout();
        assert_eq!(Some(1), wl4.focused_window);

        //Focus should be in window 2
        wm.cycle_focus(PrevOrNext::Next);
        let wl5 = wm.get_window_layout();
        assert_eq!(Some(2), wl5.focused_window);

        //Focus should be in window 6, since is added 
        wm.add_window(WindowWithInfo::new_tiled(6, SOME_GEOM)).unwrap();
        let wl6 = wm.get_window_layout();
        assert_eq!(Some(6), wl6.focused_window);

        //Now focus should previous should be 2, since was 6 was added at the bottom of the vec, then it was added when the vec
        // has a the following order [3,4,5,1,2]
        wm.cycle_focus(PrevOrNext::Prev);
        let wl7 = wm.get_window_layout();
        assert_eq!(Some(2), wl7.focused_window);

    }

    #[test]
    fn test_get_window_info() {

        let mut wm = FullscreenWM::new(SCREEN);

        //Add some windows
        wm.add_window(WindowWithInfo::new_tiled(1, SOME_GEOM)).unwrap();
        wm.add_window(WindowWithInfo::new_tiled(2, SOME_GEOM)).unwrap();
        wm.add_window(WindowWithInfo::new_tiled(4, SOME_GEOM)).unwrap();
        wm.add_window(WindowWithInfo::new_tiled(6, SOME_GEOM)).unwrap();


        //All screens should have the same size, fullscreen geometry

        let full_screen = Geometry {
            x: 0,
            y: 0,
            width: 800,
            height: 600,
        };

        assert_eq!(wm.get_window_info(1).unwrap().geometry, full_screen);
        assert_eq!(wm.get_window_info(2).unwrap().geometry, full_screen);
        assert_eq!(wm.get_window_info(4).unwrap().geometry, full_screen);
        assert_eq!(wm.get_window_info(6).unwrap().geometry, full_screen);
    }

    #[test]
    fn test_get_resize_screen() {

        let mut wm = FullscreenWM::new(SCREEN);

        //swm screen should be the same as SCREEN
        assert_eq!(wm.get_screen(), SCREEN);

        //now, swm screen should be the same as SCREEN
        wm.resize_screen(SCREEN2);
        assert_eq!(wm.get_screen(), SCREEN2);
    }
}*/