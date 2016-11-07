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
//! ## something
//! 
//! Vec changed to VecDeque
//!
//! - FullscreenWM structure was extended to not only contains Window but 
//! WindowWithInfo
//!

// Because not all methods are implemented yet, some arguments are unused,
// which generates warnings. The annotation below disables this warning.
// Remove this annotation when you have implemented all methods, so you get
// warned about variables that you did not use by mistake.
// #![allow(unused_variables)]

// We import std::error and std::format so we can say error::Error instead of
// std::error::Error, etc.
use std::error;
use std::fmt;
use std::collections::VecDeque;
// Motivation: 
// "You want a Vec that supports efficient insertion at both ends of the sequence.
// You want a queue.
// You want a double-ended queue (deque)."
// From: https://doc.rust-lang.org/std/collections/#use-a-vecdeque-when


// Import some types and the WindowManager trait from the cplwm_api crate
// (defined in the api folder).
use cplwm_api::types::{PrevOrNext, Screen, Window, WindowLayout, WindowWithInfo};
use cplwm_api::wm::WindowManager;

/// You are free to choose the name for your window manager. As we will use
/// automated tests when grading your assignment, indicate here the name of
/// your window manager data type so we can just use `WMName` instead of
/// having to manually figure out your window manager name.
pub type WMName = FullscreenWM;


/// The FullscreenWM struct
///
/// The first thing to do when writing a window manager, is to define a struct
/// (or enum) that will contain the state of the window manager, e.g. the
/// managed windows along with their geometries, the focused window, etc.
///
/// Depending on the layout and the functionality the window manager provides,
/// this can vary from simple `Vec`s to trees, hashmaps, etc. You can have a
/// look at the [collections](https://doc.rust-lang.org/std/collections/) Rust
/// provides.
///
/// Remember that you are free to add additional dependencies to your project,
/// e.g., for another type of data structure. But this is certainly not
/// required. For more information, see the Hints & Tricks section of the
/// assignment.
///
/// # Example Representation
///
/// The fullscreen window manager that we are implementing is very simple: it
/// just needs to keep track of all the windows that were added and remember
/// which one is focused. It is not even necessary to remember the geometries
/// of the windows, as they will all be resized to the size of the screen.
///
/// A possible data structure to keep track of the windows is a simple `Vec`:
/// the last element in the vector is the window on top, which is also the
/// only window to display. Why not the first element? Because it is easier to
/// add an element to the end of a vector. This is convenient, as adding a new
/// window should also put it on top of the other windows.
///
/// Another thing we need to keep track of is the `Screen`, because we must
/// resize the windows the size of the screen. A `Screen` is passed via the
/// `new` method of the trait and the `resize_screen` method of the trait
/// updates the screen with a new one.
///
/// These two fields are enough to get started, which does not mean that they
/// are enough to correctly implement this window manager. As you will notice
/// in a short while, there is a problem with this representation. Feel free
/// to add/replace/remove fields.
///
/// To understand the `#derive[(..)]` line before the struct, read the
/// [Supertraits] section of the `WindowManager` trait.
///
/// [Supertraits]: ../../cplwm_api/wm/trait.WindowManager.html#supertraits
#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct FullscreenWM {
    /// A vector of windows, the first one is on the bottom, the last one is
    /// on top, and also the only visible window.
    //	pub windows: Vec<Window>,
    pub windows: VecDeque<WindowWithInfo>,
    /// We need to know which size the fullscreen window must be.
    pub screen: Screen,
}


/// The errors that this window manager can return.
///
/// For more information about why you need this, read the documentation of
/// the associated [Error] type of the `WindowManager` trait.
///
/// In the code below, we would like to return an error when we are asked to
/// do something with a window that we do not manage, so we define an enum
/// `FullscreenWMError` with one variant: `UnknownWindow`.
///
/// Feel free to add or remove variants from this enum. You may also replace
/// it with a type or struct if you wish to do so.
///
/// [Error]: ../../cplwm_api/wm/trait.WindowManager.html#associatedtype.Error
#[derive(Debug)]
pub enum FullscreenWMError {
    /// This window is not known by the window manager.
    UnknownWindow(Window),
}

// This code is explained in the documentation of the associated [Error] type
// of the `WindowManager` trait.
impl fmt::Display for FullscreenWMError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FullscreenWMError::UnknownWindow(ref window) => write!(f, "Unknown window: {}", window),
        }
    }
}

// This code is explained in the documentation of the associated [Error] type
// of the `WindowManager` trait.
impl error::Error for FullscreenWMError {
    fn description(&self) -> &'static str {
        match *self {
            FullscreenWMError::UnknownWindow(_) => "Unknown window",
        }
    }
}

// Now we start implementing our window manager
impl WindowManager for FullscreenWM {
    /// We use `FullscreenWMError` as our `Error` type.
    type Error = FullscreenWMError;

    /// The constructor is straightforward.
    ///
    /// Track the given screen and make a new empty `Vec`.
    fn new(screen: Screen) -> FullscreenWM {
        FullscreenWM {
            windows: VecDeque::new(),
            screen: screen,
        }
    }

    /// The `windows` field contains all the windows we manage.
    ///
    /// Why do we need a `clone` here?
    /// We need it because otherwise we will let our struture (windows) and we 
    //  will not able to use again ultil the client returns it.
    fn get_windows(&self) -> Vec<Window> {
        // self.windows.clone()
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

    /// The last window in the list is the focused one.
    ///
    /// Note that the `last` method of `Vec` returns an `Option`.
    fn get_focused_window(&self) -> Option<Window> {
        //	self.windows.last().map(|w| *w)
        // I get the obtain the WindowWithInfo 
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

    /// To add a window, just push it onto the end the `windows` `Vec`.
    ///
    /// We could choose to return an error when the window is already managed
    /// by the window manager, but in this case we just do nothing. You are
    /// free to define another error to handle this case.
    ///
    /// Note that we completely ignore the information that comes along with
    /// the info, this *could* lead to issues in later assignments.
    fn add_window(&mut self, window_with_info: WindowWithInfo) -> Result<(), Self::Error> {
        if !self.is_managed(window_with_info.window) {
            //	self.windows.push(window_with_info.window);
            self.windows.push_back(window_with_info);
        }
        Ok(())
    }

    /// To remove a window, just remove it from the `windows` `Vec`.
    ///
    /// First we look up the position (or index) of the window in `windows`,
    /// and then remove it unless the window does not occur in the `Vec`, in
    /// which case we return an error.
    fn remove_window(&mut self, window: Window) -> Result<(), Self::Error> {
        //	match self.windows.iter().position(|w| *w == window) {
        // Now the position parameter was update to extract window from windowWithInfo 
       	match self.windows.iter().position(|w| (*w).window == window) {
            None => Err(FullscreenWMError::UnknownWindow(window)),
            Some(i) => {
                self.windows.remove(i);
                Ok(())
            }
        }
    }

    /// Now the most important part: calculating the `WindowLayout`.
    ///
    /// First we build a `Geometry` for a fullscreen window using the
    /// `to_geometry` method: it has the same width and height as the screen.
    ///
    /// Then we look at the last window, remember that the `last()` method of
    /// `Vec` returns an `Option`.
    ///
    /// * When the `Option` contains `Some(w)`, we know that there was at
    ///   least one window, and `w`, being the last window in the `Vec` should
    ///   be focused. As the other windows will not be visible, the `windows`
    ///   field of `WindowLayout` can just be a `Vec` with one element: the
    ///   one window along with the fullscreen `Geometry`.
    ///
    /// * When the `Option` is `None`, we know that there are no windows, so
    ///   we can just return an empty `WindowLayout`.
    ///
    fn get_window_layout(&self) -> WindowLayout {
        let fullscreen_geometry = self.screen.to_geometry();
        //	match self.windows.last() {
        //	Updated to substract window from WindowWithInfo
        if !self.windows.is_empty(){
	        let last_index = self.windows.len() - 1;
	        match self.windows.get(last_index) {
	            // If there is at least one window.
	            Some(w) => {
	                WindowLayout {
	                    // The last window is focused ...
	                    focused_window: Some((*w).window),
	                    // ... and should fill the screen. The other windows are
	                    // simply hidden.
	                    windows: vec![((*w).window, fullscreen_geometry)],
	                }
	            }
	            // Otherwise, return an empty WindowLayout
	            None => WindowLayout::new(),
	        }
        }else {
            WindowLayout::new()
        } 
    }

    // Before you continue any further, first have a look at the bottom of
    // this file, where we show you how to write unit tests.

    /// Try this yourself
    ///
    /// Don't forget that when the argument is `None`, i.e. no window should
    /// be focused, `get_focused_window()` must return `None` afterwards. The
    /// `focused_window` field of the `WindowLayout` must also be `None`.
    ///
    /// You will probably have to change the code above (method
    /// implementations as well as the `FullscreenWM` struct) to achieve this.

    /*** Approach ***/
    // Once can look over *windows* for given *window*, extract 
	// *window_with_info* and remove it.
	// Then *window_with_info* will be pushed once again at the top of 
	// *windows*.
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
		                self.windows.remove(i);
		                // Fortunatly, *add_window* can helps us out to add the 
						// *window_with_info*
		                self.add_window(window_with_info)
		            }
		        }
	    	}
   		}
    }

    /*** Approach ***/
    // I take advantage of the new structure, so I use pop_back() and push_front()
    // for prev action and  pop_front() and push_back() for next action
    // Condition when *windows* is empty or a singleton is covered with a naive 
    // if statement. Focus *window* by definition is always the last element, no
    // need to test whether there is a focus *window* when *windows* is a 
    // singleton, the only *window* is always focus.
    fn cycle_focus(&mut self, dir: PrevOrNext) {
        if self.windows.len() > 1{
        	match dir {
        		// I use unwrap() because we already test that *windows*
	            // is not empty, temp will be always receive a *window_with_info*
	            // element
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

    /*** Approach ***/
    // Since now *windows* stores the whole *window_with_info*, I only iterate
    // over, found the corresponding *window* and return the *window_with_info*,
    // otherwise an *UnknownWindow* error is thrown.
    fn get_window_info(&self, window: Window) -> Result<WindowWithInfo, Self::Error> {
    	match self.windows.iter().position(|w| (*w).window == window) {
    		None => Err(FullscreenWMError::UnknownWindow(window)),
    		Some(i) => Ok(self.windows.get(i).unwrap().clone()),
    	}
    }

    /*** Approach ***/
    // I just take the screen attribute of FullscreenWM
    fn get_screen(&self) -> Screen {
        self.screen
    }

    /*** Approach ***/
    // I just take update screen attribute of FullscreenWM
    fn resize_screen(&mut self, screen: Screen) {
        self.screen = screen
    }
}

// Here we define a submodule, called `tests`, that will contain the unit
// tests of this module.
//
// The `#[cfg(test)]` annotation means that this code is only compiled when
// we're testing the code.
/*
#[cfg(test)]
mod tests {

    // We have to import `FullscreenWM` from the super module.
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

        //No action is done
        wm.focus_window(None).unwrap();

        //Focus should be kept in window 5, since was the last insertion
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
    }

    #[test]
    fn test_cycle_focus() {

        let mut wm = FullscreenWM::new(SCREEN);

        //Do nothing
        wm.cycle_focus(PrevOrNext::Next);

        //Add some windows
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

		//Focus should be in window 6
        wm.add_window(WindowWithInfo::new_tiled(6, SOME_GEOM)).unwrap();
        let wl6 = wm.get_window_layout();
        assert_eq!(Some(6), wl6.focused_window);

        //Focus should be in window 2
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
        wm.add_window(WindowWithInfo::new_tiled(3, SCREEN_GEOM)).unwrap();
        wm.add_window(WindowWithInfo::new_tiled(4, SOME_GEOM)).unwrap();
        wm.add_window(WindowWithInfo::new_tiled(5, SCREEN_GEOM)).unwrap();


        //screen 1 and 2 should have the smae geometry
        assert_eq!(wm.get_window_info(1).unwrap().geometry, wm.get_window_info(2).unwrap().geometry);

        //As well as window 3 5
        assert_eq!(wm.get_window_info(3).unwrap().geometry, wm.get_window_info(5).unwrap().geometry);

		//window 5 has geometry SCREEN_GEOM
        assert_eq!(SCREEN_GEOM, wm.get_window_info(5).unwrap().geometry);

        //window 1 has geometry SOME_GEOM
        assert_eq!(SOME_GEOM, wm.get_window_info(1).unwrap().geometry);
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
