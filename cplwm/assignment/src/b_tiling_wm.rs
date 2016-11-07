//! Tiling Window Manager
//!
//! Write a more complex window manager that will *tile* its windows. Tiling
//! is described in the first section of the assignment. Your window manager
//! must implement both the [`WindowManager`] trait and the [`TilingSupport`]
//! trait. See the documentation of the [`TilingSupport`] trait for the
//! precise requirements and an explanation of the tiling layout algorithm.
//!
//! [`WindowManager`]: ../../cplwm_api/wm/trait.WindowManager.html
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
//! COMPLETED: PARTIAL
//!
//! COMMENTS:
//!
//! General approach
//!
//! Since now we are dealing with a master window, the way the windows are added matter, according with the algorithm 
//! new window should be added to the bottom right of the window. It is convenient to keep the order in our collection to
//! adapt the different layout of the no-master windows. Also there is a new function that swap the layout of the master
//! window with another window, so it is also handy to just look over the index of the given window and then make the change
//! using the swap build-in method with the first element of the collection (which will be always represents the master window).
//! Finally, if I want to keep the order of the collection and swap windows position without focused windows, the focused element
//! then should be swap with the internal element of the collection and reorder the collection to get the correct focused element.
//! To avoid such behavior, I store the index of focused window in the TillingWM structure. 


// self.index_foused_window = temp <-.-----------------------------------------

#![allow(unused_variables)]

// Add imports here
use std::error;
use std::fmt;
//use std::collections::VecDeque;

use cplwm_api::types::{PrevOrNext, Screen, Window, WindowLayout, WindowWithInfo, Geometry};
use cplwm_api::wm::WindowManager;
use cplwm_api::wm::TilingSupport;



/// **TODO**: Documentation
pub type WMName = TillingWM;

/// The TillingWM struct
///
/// # Example Representation
/// Similar as previous exercises, once again I use VecDeque due to the power to push and pop in both
/// sides of the vector, likewise, it stores the whole *window_with_info* which will keep track the individual
/// geometry of each window, screen was kept as previous exercise. Additionally TillingWM should remember the master 
/// 
#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct TillingWM {
	/// **TODO**: Documentation
    pub windows: Vec<WindowWithInfo>,  
    /// **TODO**: Documentation
    pub screen: Screen,
	/// The index of the focused window in the collection
    pub index_foused_window: usize,
}

/// **TODO**: Documentation
#[derive(Debug)]
pub enum TillingWMError {
	/// **TODO**: Documentation
    UnknownWindow(Window),
    /// **TODO**: Documentation
    ManagedWindow(Window),
}

impl fmt::Display for TillingWMError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TillingWMError::UnknownWindow(ref window) => write!(f, "Unknown window: {}", window),
            TillingWMError::ManagedWindow(ref window) => write!(f, "Window {} is already managed", window),
        }
    }
}

impl error::Error for TillingWMError {
    fn description(&self) -> &'static str {
        match *self {
            TillingWMError::UnknownWindow(_) => "Unknown window",
            TillingWMError::ManagedWindow(_) => "Window is already managed",
        }
    }
}


impl WindowManager for TillingWM {
    
    type Error = TillingWMError;

    // Method modified
    fn new(screen: Screen) -> TillingWM {
        TillingWM {
            windows: Vec::new(),
            screen: screen,
            // by the fault the master windows is focused, get_focused_window handles the case when there are no windows
            index_foused_window: 0,
        }
    }

    fn get_windows(&self) -> Vec<Window> {
        let mut temp_windows = Vec::new();
        for window_with_info in self.windows.iter() {
   			temp_windows.push(window_with_info.window.clone());
		}
       	temp_windows
    }

    // get_focused_window was simplyfied due to the index_foused_window element
    fn get_focused_window(&self) -> Option<Window> {
        if !self.windows.is_empty(){ 
        	Some(self.windows.get(self.index_foused_window).unwrap().window)
    	}else{
    		None
    	}
    }

    // Method modified
    // add_window still focused the new added window
    fn add_window(&mut self, window_with_info: WindowWithInfo) -> Result<(), Self::Error> {
        if !self.is_managed(window_with_info.window) {
            self.windows.push(window_with_info);
            let temp = self.windows.len() - 1;
            self.index_foused_window = temp;
            Ok(())
        }else{
        	Err(TillingWMError::ManagedWindow(window_with_info.window))
        }
    }

	// Now we need to keep track of the focused element, every time that a element is remove the index_foused_window should be
	// decrease by one. Important to noticy here is that when the focused element is the same as the remove element, focused
	// element now will be the previous window. If master windows is removed and was focused and len of the collection is > 0
	//  the next element of it will be focused (which will be the new master window)
    fn remove_window(&mut self, window: Window) -> Result<(), Self::Error> {
       	match self.windows.iter().position(|w| (*w).window == window) {
            None => Err(TillingWMError::UnknownWindow(window)),
            Some(i) => {
                self.windows.remove(i);
                if i != 0 {
                	let temp = self.index_foused_window - 1;
                	self.index_foused_window = temp;
                }
                Ok(())
            }
        }
    }

    // this should modify to get a window attribute


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
    

    fn get_window_layout(&self) -> WindowLayout {

        if !self.windows.is_empty(){

        	if self.windows.len() > 1 {

        		let divisor = (self.windows.len() - 1) as u32;
        		let last_index = self.windows.len() - 1;
        		let height_side = self.screen.height / divisor;
        		let width_side = self.screen.width / 2;
        		let x_point = (self.screen.width / 2) as i32;
				let master_window = self.get_master_window().unwrap();

        		let mut temp_windows = Vec::new();
        		let mut y_point = 0 as i32;

		       	for window_with_info in self.windows.iter(){
		       		if master_window != window_with_info.window {
		       			// I calculate the values of the secondary windows (right windows)
			        	let rigth_geometry = Geometry {
		            		x: x_point,
		            		y: y_point,
		            		width: width_side,
		            		height: height_side,
	        			};

			   			temp_windows.push((window_with_info.window.clone(), rigth_geometry));

	        			y_point += (height_side) as i32;

        			}else{
        				// I calculate the values for master window
						let  master_geometry = Geometry { 
							x: 0,
			            	y: 0,
			            	width: width_side,
			            	height: self.screen.height,
		        		};

        				temp_windows.push((window_with_info.window.clone(), master_geometry));
        			}
				}

		       	 WindowLayout {
		       	 	focused_window: Some(self.windows.get(self.index_foused_window).unwrap().window),
		       	 	windows: temp_windows,
		       	 }

        	}else{

        		// here we ensure that we have at least one lement so no match is necessary
        		let fullscreen_geometry = self.screen.to_geometry();

        		match self.windows.get(self.index_foused_window) {

		            Some(w) => {
		                WindowLayout {
		                    focused_window: Some((*w).window),
		                    windows: vec![((*w).window, fullscreen_geometry)],
		                }
		            }
		            None => WindowLayout::new(),
		        }
        	}	        
        }else {
            WindowLayout::new()
        } 
    }

	// Method modified
    fn focus_window(&mut self, window: Option<Window>) -> Result<(), Self::Error> {
    	match window{
    		None => Ok(()),

    		Some(gw) => {

		    	match self.windows.iter().position(|w| (*w).window == gw) {
		            None => Err(TillingWMError::UnknownWindow(gw)),

		            Some(i) => {
		            	self.index_foused_window = i;
		            	Ok(())
		            }
		        }
	    	}
   		}
    }

	// Method modified
    fn cycle_focus(&mut self, dir: PrevOrNext) {
        if self.windows.len() > 1{
        	match dir {
	            PrevOrNext::Prev => {
	            	if self.index_foused_window != 0{
						let temp = self.index_foused_window.clone() - 1;
	            		self.index_foused_window = temp;
	            	}else{
	            		let temp = self.windows.len() - 1;
	            		self.index_foused_window = temp;
	            	}
	            }

	            PrevOrNext::Next => {
	            	let last_index = self.windows.len() - 1;
	            	if self.index_foused_window != last_index{
						let temp = self.index_foused_window + 1;
						self.index_foused_window = temp;
	            	}else{
	            		self.index_foused_window = 0;
	            	}
	            }
	         }
        };
    }

    fn get_window_info(&self, window: Window) -> Result<WindowWithInfo, Self::Error> {
    	match self.windows.iter().position(|w| (*w).window == window) {
    		None => Err(TillingWMError::UnknownWindow(window)),
    		Some(i) => Ok(self.windows.get(i).unwrap().clone()),
    	}
    }

    fn get_screen(&self) -> Screen {
        self.screen
    }

    fn resize_screen(&mut self, screen: Screen) {
        self.screen = screen
    }

}


impl TilingSupport for TillingWM {

	fn get_master_window(&self) -> Option<Window>{
		if !self.windows.is_empty(){
			//Using unwrap because there should be at leat on element since the collection is not empty
			Some(self.windows.get(0).unwrap().window)
		}else{
			None
		}
	}

	fn swap_with_master(&mut self, window: Window) -> Result<(), Self::Error>{
		match self.windows.iter().position(|w| (*w).window == window) {
            None => Err(TillingWMError::UnknownWindow(window)),
            Some(i) => {
            	self.windows.swap(0, i);
            	let temp_master = self.get_master_window();
            	self.focus_window(temp_master)
            }
        }
	}


	// use swap method of the structure
	fn swap_windows(&mut self, dir: PrevOrNext){
		unimplemented!();
	}

}

#[cfg(test)]
mod tests {

    // We have to import `TillingWM` from the super module.
    use super::TillingWM;
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
        // Let's make a new `TillingWM` with `SCREEN` as screen.
        let mut wm = TillingWM::new(SCREEN);

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

        assert_eq!(vec![(1, first_half),(2, second_half)], wl2.windows);

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

        let mut wm = TillingWM::new(SCREEN);

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

        wm.remove_window(5).unwrap();
        wm.focus_window(Some(1)).unwrap();
        wm.remove_window(1).unwrap();
        let wl5 = wm.get_window_layout();
        assert_eq!(Some(2), wl5.focused_window);
    }

    #[test]
    fn test_cycle_focus() {

        let mut wm = TillingWM::new(SCREEN);

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

		//Focus should be in window 6, since is added 
        wm.add_window(WindowWithInfo::new_tiled(6, SOME_GEOM)).unwrap();
        let wl6 = wm.get_window_layout();
        assert_eq!(Some(6), wl6.focused_window);

        //Now focus should previous shpuld be 5, since the order in which were added now matters
        wm.cycle_focus(PrevOrNext::Prev);
        let wl7 = wm.get_window_layout();
        assert_eq!(Some(5), wl7.focused_window);

    }

     #[test]
    fn test_get_window_info() {

        let mut wm = TillingWM::new(SCREEN);

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

        let mut wm = TillingWM::new(SCREEN);

        //swm screen should be the same as SCREEN
        assert_eq!(wm.get_screen(), SCREEN);

        //now, swm screen should be the same as SCREEN
        wm.resize_screen(SCREEN2);
        assert_eq!(wm.get_screen(), SCREEN2);
     }

}
