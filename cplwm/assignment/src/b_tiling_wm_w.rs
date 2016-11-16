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
//! COMPLETED: YES
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
//!
//! Even further according with the description of swap_windows() the focus window seems like an optional value, it is not 
//! necessary the case that some window should be focus, so intead of just store a plain number index, a Option<int> will be stored
//#![allow(unused_variables)]

// Imports
use std::error;
use std::fmt;
use cplwm_api::types::{PrevOrNext, Screen, Window, WindowLayout, WindowWithInfo, Geometry};
use cplwm_api::wm::WindowManager;
use cplwm_api::wm::TilingSupport;

// fullscren_window_manager = a_fullscree_n_wm_w::FullscreenWM;
//let mut fullscreen_wm = FullscreenWM;
/// The TillingWM struct
///
/// # Example Representation
/// Similar as previous exercises, once again I use VecDeque due to the power to push and pop in both
/// sides of the vector, likewise, it stores the whole *window_with_info* which will keep track the individual
/// geometry of each window, screen was kept as previous exercise. Additionally TillingWM should remember the master 
/// 
#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct TillingWM {
	/// **TODO**: No need to used a complex collection
    pub windows: Vec<WindowWithInfo>,  
    /// **TODO**: Documentation
    pub screen: Screen,
	/// The index of the focused window in the collection, if there is no focused window a None is placed
    pub index_foused_window: Option<usize>,
}

/// Supported functions
impl TillingWM {
    pub fullscren_window_manager = a_fullscree_n_wm_w::FullscreenWM;
    /// This method calculated the tiled window's geometries in the order of the
    /// windows vector
    //*** Improvement: here you calculate first thar windows is greater than 1, but could be thecase
    // and not necessary it is a tiled window 
    fn calculate_tiled_geometries(&mut self){
        if !self.windows.is_empty(){

                let divisor = (self.windows.len() - 1) as u32;
                //let divisor = self.windows.len() - non_tiled_windows;

                // if the divisor is greater than 0 we need to calculate slave windows
                //if divisor > 0{
                if divisor > 0{
                    let height_side = self.screen.height / divisor;
                    let width_side = self.screen.width / 2;
                    let x_point = (self.screen.width / 2) as i32;
                    // It is already tested that there is more than 1 window, hence one can use unwrap method being sure that a 
                    // Some intance of option will be returned
                    let master_window = self.get_master_window().unwrap();

                    let mut y_point = 0 as i32;

                    for tiled_window in self.windows.iter_mut(){
                        if master_window != tiled_window.window {
                            // I calculate the values of the secondary windows (right windows)
                            let rigth_geometry = Geometry {
                                x: x_point,
                                y: y_point,
                                width: width_side,
                                height: height_side,
                            };
                            tiled_window.geometry = rigth_geometry;
                            y_point += (height_side) as i32;

                        }else{
                            // I calculate the values for master window
                            let  master_geometry = Geometry { 
                                x: 0,
                                y: 0,
                                width: width_side,
                                height: self.screen.height,
                            };
                            tiled_window.geometry = master_geometry;
                        }
                    };
                }else{
                    // There is the master window
                    let window = self.windows.get_mut(0).unwrap();
                    window.geometry = self.screen.to_geometry();
                }
        };
    }
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
        FullscreenWM::new(screen);

        TillingWM {
            windows: Vec::new(),
            screen: screen,
            index_foused_window: None,
        }
    }

    fn get_windows(&self) -> Vec<Window> {
        FullscreenWM::get_windows(self)
    }

    // get_focused_window was simplyfied due to the index_foused_window element
    fn get_focused_window(&self) -> Option<Window> {
        if !self.windows.is_empty(){ 
        	match self.index_foused_window {
        		None => None,
        		Some(index) => Some(self.windows.get(index).unwrap().window)
        	}
    	}else{
    		None
    	}
    }

    // By default, add_window still focuses the new added window
    /*** Improvement: thrown error when the added window is not tilling ***/
    fn add_window(&mut self, window_with_info: WindowWithInfo) -> Result<(), Self::Error> {
        if !self.is_managed(window_with_info.window) {
            self.windows.push(window_with_info);
            let temp = self.windows.len() - 1;
            self.index_foused_window = Some(temp);
            self.calculate_tiled_geometries();
            Ok(())
        }else{
        	Err(TillingWMError::ManagedWindow(window_with_info.window))
        }
    }

	// Now we need to keep track of the focused element, every time that a element is remove the index_foused_window should be
	// decrease by one to keep tracking the correct focused window if that is the case. Important to noticy here is that when 
	// the focused element is the same as the removed element, no focused window is set.
    fn remove_window(&mut self, window: Window) -> Result<(), Self::Error> {
       	match self.windows.iter().position(|w| (*w).window == window) {
            None => Err(TillingWMError::UnknownWindow(window)),
            Some(i) => { 
                self.windows.remove(i);
                match self.index_foused_window {
                	None => Ok(()),

                	Some(index) => {
                		if index == i {
                			self.index_foused_window = None;
                			Ok(())
                		}else{
                			let mut temp = index;
                            if index > i{
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

    /// get_window_layout actually is the one that calculates the sizes of the different windows and arracnge the geometry of
    /// each one. When there is a unique window sam approach as a_fullscreen_wm is used, otherwise the calculation of every
    /// window is done.

    fn get_window_layout(&self) -> WindowLayout {

        if !self.windows.is_empty(){

        	if self.windows.len() > 1 {
        		// I start to calculate the different values of the windows, because of the approach used in this windows manager
        		// it is known that the windows are actually ordered in the collection, hence the first element of vec is the first
        		// added, second element of vec is the second one and so on. So I can iterate over vec and arrange the size of the
        		// height attribute, width attribute is a constant for all windows, even for the master window.
        		// on every iteration the y of every window is updated to the proper position. There is a special case with the
        		// master window which is handle with a if statement.
        		let divisor = (self.windows.len() - 1) as u32;
        		let height_side = self.screen.height / divisor;
        		let width_side = self.screen.width / 2;
        		let x_point = (self.screen.width / 2) as i32;
        		// It is already tested that there is more than 1 window, hence one can use unwrap method being sure that a 
        		// Some intance of option will be returned
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
				};

				//once again the unwrap is used because it is centrain that there are windows, the None mathc of the index_foused_window
				// prevents whenever there is no focused window
				let temp_focused_window = 
					match self.index_foused_window {
						None => None,
						Some(index) => Some(self.windows.get(index).unwrap().window),
					};	

		       	 WindowLayout {
		       	 	focused_window: temp_focused_window,
		       	 	windows: temp_windows,
		       	 }

        	}else{

        		let fullscreen_geometry = self.screen.to_geometry();

        		let temp_focused_window = 
					match self.index_foused_window {
						None => None,
						Some(index) => Some(self.windows.get(index).unwrap().window),
					};

        		WindowLayout {
		                    focused_window: temp_focused_window,
		                    windows: vec![(self.windows.get(0).unwrap().window, fullscreen_geometry)],
		                }
        	}	        
        }else {
            WindowLayout::new()
        } 
    }

	// focus_window was slightly modified to adapt to the new attirbute in the TillingWM structure, now when None is passed
	// there will be no focused window
    fn focus_window(&mut self, window: Option<Window>) -> Result<(), Self::Error> {
    	match window{
    		None => {
    			self.index_foused_window = None;
    			Ok(())}

    		Some(gw) => {
		    	match self.windows.iter().position(|w| (*w).window == gw) {
		            None => Err(TillingWMError::UnknownWindow(gw)),

		            Some(i) => {
		            	self.index_foused_window = Some(i);
		            	Ok(())
		            }
		        }
	    	}
   		}
    }

	// cycle_focus was modifed to support the next and previous methods accordign with the index, now the structure itself it is
	// not modified, index_foused_window is updated instead. When no window is focused, the Master window is focused
    fn cycle_focus(&mut self, dir: PrevOrNext) {
        if self.windows.len() > 1 {
        	match self.index_foused_window{
        		//No focused window, so master window is focused
        		None => self.index_foused_window = Some(0),

        		Some(index) => {
		        	match dir {
			            PrevOrNext::Prev => {
			            	if index != 0{
								let temp = index - 1;
			            		self.index_foused_window = Some(temp);
			            	}else{
			            		let temp = self.windows.len() - 1;
			            		self.index_foused_window = Some(temp);
			            	}
			            }

			            PrevOrNext::Next => {
			            	let last_index = self.windows.len() - 1;
			            	if index != last_index{
								let temp = index + 1;
								self.index_foused_window = Some(temp);
			            	}else{
			            		self.index_foused_window = Some(0);
			            	}
			            }
			         }
        		}

        	}
        }else{
        	//If there is only one window then that should be focused
        	if self.windows.len() == 1 {
				self.index_foused_window = Some(0);
        	}
        }
    }

    fn get_window_info(&self, window: Window) -> Result<WindowWithInfo, Self::Error> {
    	match self.windows.iter().position(|w| (*w).window == window) {
    		None => Err(TillingWMError::UnknownWindow(window)),
    		Some(i) => Ok(self.windows.get(i).unwrap().clone()),
    	}
    }

    fn get_screen(&self) -> Screen {
        //fullscreen_wm
        self.screen
    }

    fn resize_screen(&mut self, screen: Screen) {
        self.screen = screen
    }

}


impl TilingSupport for TillingWM {

    fn get_master_window(&self) -> Option<Window>{
        if !self.windows.is_empty(){
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
                self.calculate_tiled_geometries();
                self.focus_window(temp_master)
            }
        }
    }


    // Simlar approach than cycle_focus, but now the structure should be updated accordingly, that behavior can be done
    // with the swap built-in method 
    fn swap_windows(&mut self, dir: PrevOrNext){
        if self.windows.len() > 1 {
            match self.index_foused_window {
                // no focused window = nothing
                None => (),

                Some(index) => {
                    match dir {
                        PrevOrNext::Prev => {
                            if index != 0{
                                let temp = index - 1;
                                self.index_foused_window = Some(temp);
                                self.windows.swap(index, temp);
                                self.calculate_tiled_geometries();
                            }else{
                                let temp = self.windows.len() - 1;
                                self.index_foused_window = Some(temp);
                                self.windows.swap(0, temp);
                                self.calculate_tiled_geometries();
                            }
                        }

                        PrevOrNext::Next => {
                            let last_index = self.windows.len() - 1;
                            if index != last_index{
                                let temp = index + 1;
                                self.index_foused_window = Some(temp);
                                self.windows.swap(index, temp);
                                self.calculate_tiled_geometries();
                            }else{
                                self.windows.swap(last_index, 0);
                                self.index_foused_window = Some(0);
                                self.calculate_tiled_geometries();
                            }
                        }
                     }
                }
            }
        }
    }
}

/*
#[cfg(test)]
mod tests {

    // We have to import `TillingWM` from the super module.
    use super::TillingWM;
    // We have to repeat the imports we did in the super module.
    use cplwm_api::wm::WindowManager;
    use cplwm_api::wm::TilingSupport;
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

        assert_eq!(vec![(1, first_half),(2, second_half)], wl2.windows);

        // Now let's remove window 2
        wm.remove_window(2).unwrap();
        // It should no longer be managed by the WM.
        assert!(!wm.is_managed(2));
        // The `Vec` of windows should now just contain window 1.
        assert_eq!(vec![1], wm.get_windows());
        // According to the window layout
        let wl3 = wm.get_window_layout();
        // because the new behavior, window 1 should not be focused, No window is focused
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
        // I add more window, which shoudl be allocated in the right side of the window
        wm.add_window(WindowWithInfo::new_tiled(3, SOME_GEOM)).unwrap();
        wm.add_window(WindowWithInfo::new_tiled(4, SOME_GEOM)).unwrap();
        wm.add_window(WindowWithInfo::new_tiled(5, SOME_GEOM)).unwrap();
        wm.add_window(WindowWithInfo::new_tiled(6, SOME_GEOM)).unwrap();

        let wl4 = wm.get_window_layout();
        // Due to the new added windows, now window 6 should be focused
        assert_eq!(Some(6), wl4.focused_window);
        // The windows should be allocated in the correct window
        assert_eq!(vec![(1, first_half),(3, third_half),(4, fourth_half),(5, fifth_half),(6, sixth_half)], wl4.windows);
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

        //Now an action should be applied, when None is given to focus_window the current focused window should be unfocused.
        wm.focus_window(None).unwrap();

        //Focused window should return None
        let wl1 = wm.get_window_layout();
        assert_eq!(None, wl1.focused_window);

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
        //Because the last focused window was removed, the focused_window attribute should be None
        let wl5 = wm.get_window_layout();
        assert_eq!(None, wl5.focused_window);
    }

    #[test]
    fn test_cycle_focus() {

        let mut wm = TillingWM::new(SCREEN);

        //Do nothing
        wm.cycle_focus(PrevOrNext::Next);

        //Add some both type of windows
        wm.add_window(WindowWithInfo::new_tiled(1, SOME_GEOM)).unwrap();
        wm.add_window(WindowWithInfo::new_float(2, SOME_GEOM)).unwrap();
        wm.add_window(WindowWithInfo::new_tiled(3, SOME_GEOM)).unwrap();
        wm.add_window(WindowWithInfo::new_float(4, SOME_GEOM)).unwrap();
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
        wm.add_window(WindowWithInfo::new_tiled(4, SOME_GEOM)).unwrap();
        wm.add_window(WindowWithInfo::new_tiled(6, SOME_GEOM)).unwrap();


        //All screens should have different size, since now we are dealing with 
        // tiled funtions, it should be reflec since they are added, removed or toggled

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
        assert_eq!(wm.get_window_info(4).unwrap().geometry, second_half);
        assert_eq!(wm.get_window_info(6).unwrap().geometry, third_half);
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

    #[test]
    fn test_tiling_support() {

        let mut wm = TillingWM::new(SCREEN);

        // No window yet
        assert_eq!(None, wm.get_master_window());

        //Add some windows
        wm.add_window(WindowWithInfo::new_tiled(1, SOME_GEOM)).unwrap();
        wm.add_window(WindowWithInfo::new_tiled(2, SOME_GEOM)).unwrap();
        wm.add_window(WindowWithInfo::new_tiled(3, SOME_GEOM)).unwrap();
        wm.add_window(WindowWithInfo::new_tiled(4, SOME_GEOM)).unwrap();


        //First window added is the master window
        assert_eq!(Some(1), wm.get_master_window());

        //Focused window is 4, since was the last added
        let wl1 = wm.get_window_layout();
        assert_eq!(Some(4), wl1.focused_window);

        //swapping from master to master, no swap action is taken. Focused window is changed, though.
        
        wm.swap_with_master(1).unwrap();
        let wl2 = wm.get_window_layout();

        assert_eq!(Some(1), wl2.focused_window);

        //swapping from master to window 3, now window 3 is master window and is focused
        wm.swap_with_master(3).unwrap();
        let wl3 = wm.get_window_layout();
        assert_eq!(Some(3), wm.get_master_window());         

        assert_eq!(Some(3), wl3.focused_window);         

        //Traying to swap from master to an unknown window, erro is thrown
        assert!(wm.swap_with_master(10).is_err());

        //Since previously I used  swap_with_master the collection was updated to [3,2,1,4], where 3 is the master window
        //using swap_windows(PrevOrNext::Prev) should be allocate windows 4 as master window and put window 3 in the bottom right corner
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
        assert_eq!(vec![(3, master_half),(2, first_half),(1, second_half),(4, third_half)], wl4a.windows);


        wm.swap_windows(PrevOrNext::Prev);
        let wl4 = wm.get_window_layout();
        assert_eq!(Some(4), wm.get_master_window());
        assert_eq!(Some(3), wl4.focused_window);
        assert_eq!(vec![(4, master_half),(2, first_half),(1, second_half),(3, third_half)], wl4.windows);

        //I change focused to master window and apply swap_windows(PrevOrNext::Next), the result should be window 2 as 
        // master window while windows 4 is focused
        wm.focus_window(Some(4)).unwrap();
        wm.swap_windows(PrevOrNext::Next);
        let wl5 = wm.get_window_layout();
        assert_eq!(Some(2), wm.get_master_window());
        assert_eq!(Some(4), wl5.focused_window);
        assert_eq!(vec![(2, master_half),(4, first_half),(1, second_half),(3, third_half)], wl5.windows);
        
        // **Invariant**: if `swap_with_master(w)` succeeds, `get_master_window()
        // == Some(w)`.
        wm.swap_with_master(1).unwrap();
        assert_eq!(wm.get_master_window(),Some(1));

        // **Invariant**: `get_master_window() == Some(w)`, then `w` must occur
        // in the vector returned by `get_windows()`.
        
        let master_window = wm.get_master_window().unwrap();
        let windows = wm.get_windows();
        assert!(windows.contains(&master_window));

        // **Invariant**: if the vector returned by `get_windows()` is empty =>
        // `get_master_window() == None`.
        wm.remove_window(2).unwrap();
        wm.remove_window(3).unwrap();
        wm.remove_window(4).unwrap();
        wm.remove_window(1).unwrap();
        
        assert!(wm.get_windows().is_empty());
        assert_eq!(wm.get_master_window(),None);
    }
}
*/