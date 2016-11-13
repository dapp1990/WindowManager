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
//!	The approach now is quite similar to b_tillin_vm, but now get_window_layout is changed. First processing floating windows 
//! and then processing and append it to the vector, that way the first windows in the array will be the floating onces,
//! hence the tiled windows will be cover by. No need to modify the windows attribute in the FloatingWM structure since
//! the title are already saved with the given geometry whether it is tiled or floating. Additionally the logic of the ordered
//! collection is keep it, so when there the cycle_focus change among tiled and floating windows. In case a floating window is
//! changed to tiled window, the order is kept it so get_window_layout will allocated such windows in the order in it was added.
//! tile_support has to be change to support only over windows with FloatOrTile::Tile

// Add imports here
use std::error;
use std::fmt;

use cplwm_api::types::{PrevOrNext, Screen, Window, WindowLayout, WindowWithInfo, Geometry, FloatOrTile};
use cplwm_api::wm::WindowManager;
use cplwm_api::wm::TilingSupport;
use cplwm_api::wm::FloatSupport;

/// **TODO**: Documentation
pub type WMName = FloatingWM;

/// The FloatingWM struct
///
/// # Example Representation
/// Similar as previous exercises
/// 
#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct FloatingWM {
	/// **TODO**: No need to used a complex collection
	pub windows: Vec<WindowWithInfo>,  
	/// **TODO**: Documentation
	pub screen: Screen,
	/// The index of the focused window in the collection, if there is no focused window a None is placed
	pub index_foused_window: Option<usize>,
}

/// Supported functions
impl FloatingWM {
	/// Get the next index by the given index of the tiled windows.
	// when the end of the list is reached, tthen start to loop
	pub fn get_next_tile_index(& self, index:usize, saved:usize) -> Option<usize> {
		let mut next_index = 0;		
		if self.windows.len()-1 > index{
			next_index = index + 1;			
		}
		if next_index == saved {
			None
		}else{
			if self.windows.get(next_index).unwrap().float_or_tile == FloatOrTile::Tile{
				Some(next_index)
			}else{
				self.get_next_tile_index(next_index,saved)
			}
		}
	}

	/// Get the previous index by the given index of the tiled windows.
	pub fn get_prev_tile_index(& self, index:usize, saved:usize) -> Option<usize> {
		let mut prev_index = self.windows.len() -1;
		if index != 0{
			prev_index = index - 1;
		};		
		if prev_index == saved {
			None
		}else{
			if self.windows.get(prev_index).unwrap().float_or_tile == FloatOrTile::Tile{
				Some(prev_index)
			}else{
				self.get_prev_tile_index(prev_index,saved)
			}
		}
	}

	// helper function to get the master index if it exists
	// in this case it is neccesary since it is possible that master windows is not the first element
	// of the vec
	// logic is similar to get_master_window
	fn get_master_index(&self) -> Option<usize>{
		if !self.windows.is_empty(){
			self.windows.iter().position(|w| (*w).float_or_tile == FloatOrTile::Tile)
		}else{
			None			
		}
	}
}

/// **TODO**: Documentation
#[derive(Debug)]
pub enum FloatingWMError {
	/// **TODO**: Documentation
	UnknownWindow(Window),
	/// **TODO**: Documentation
	ManagedWindow(Window),
	/// **TODO**: Documentation
	NoFloatingWindow(Window),
	/// **TODO**: Documentation
	NoTiledWindow(Window),
}

impl fmt::Display for FloatingWMError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			FloatingWMError::UnknownWindow(ref window) => write!(f, "Unknown window: {}", window),
			FloatingWMError::ManagedWindow(ref window) => write!(f, "Window {} is already managed", window),
			FloatingWMError::NoFloatingWindow(ref window) => write!(f, "Window {} is not floating", window),
			FloatingWMError::NoTiledWindow(ref window) => write!(f, "Window {} is not tiled", window),
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

	// Method modified
	fn new(screen: Screen) -> FloatingWM {
		FloatingWM {
			windows: Vec::new(),
			screen: screen,
			index_foused_window: None,
		}
	}

	fn get_windows(&self) -> Vec<Window> {
		let mut temp_windows = Vec::new();
		for window_with_info in self.windows.iter() {
			temp_windows.push(window_with_info.window.clone());
		}
		temp_windows
	}

	// get focused work for both floating and tiled windows
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

	// By default, add_window still focuses the new added window, no matter whether it is tiled or floating
	fn add_window(&mut self, window_with_info: WindowWithInfo) -> Result<(), Self::Error> {
		if !self.is_managed(window_with_info.window) {
			self.windows.push(window_with_info);
			let temp = self.windows.len() - 1;
			self.index_foused_window = Some(temp);
			Ok(())
		}else{
			Err(FloatingWMError::ManagedWindow(window_with_info.window))
		}
	}

	fn remove_window(&mut self, window: Window) -> Result<(), Self::Error> {
		match self.windows.iter().position(|w| (*w).window == window) {
			None => Err(FloatingWMError::UnknownWindow(window)),
			Some(i) => { 
				self.windows.remove(i);
				match self.index_foused_window {
					None => Ok(()),

					Some(index) => {
						if index == i {
							self.index_foused_window = None;
							Ok(())
						}else{
							let temp = index - 1;
							self.index_foused_window = Some(temp);
							Ok(())
						}
					}
				}                
			}
		}
	}

	/// I opt to filter the vec of the FloatingWM structure. First I filter the tiled windows, they are processed exactly like 
	/// in b_tilling_wm and attache them to the temporal vec. Then I get the windows with the attribute FloatOrTile::Float, I 
	/// obtain the Geometries (which are already saved in the vector) and add them to the temporal vec. 
	/// I keep the order in which the windows were added, no matter if they are tiled or floating, this becomes pretty handy
	/// because then the geometry of each individual window is dynamically adapted according with the type and it is return to
	// te same position (in the tiled set) where the window was left out. So if window x is the master and the toggle_floating 
	/// is used twice continuosly, x should be remaind as master window in the tiled layout, no matter the numbers of windows
	/// the window manager is handling.
	fn get_window_layout(&self) -> WindowLayout {

		if !self.windows.is_empty(){

			if self.windows.len() > 1 {
				//Divisor now should be update to just the tiled windows
				let divisor = (self.windows.len() - self.get_floating_windows().len() - 1) as u32;

				let height_side = self.screen.height / divisor;
				let width_side = self.screen.width / 2;
				let x_point = (self.screen.width / 2) as i32;
				// It is already tested that there is more than 1 window, hence one can use unwrap method being sure that a 
				// Some intance of option will be returned
				let master_window = self.get_master_window().unwrap();

				let mut temp_windows = Vec::new();
				let mut y_point = 0 as i32;

				for window_with_info in self.windows.iter().filter(|x| (*x).float_or_tile == FloatOrTile::Tile){
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

				for window_with_info_floating in self.windows.iter().filter(|x| (*x).float_or_tile == FloatOrTile::Float){
					temp_windows.push((window_with_info_floating.window.clone(), window_with_info_floating.geometry.clone()))
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

				let temp_focused_window = 
				match self.index_foused_window {
					None => None,
					Some(index) => Some(self.windows.get(index).unwrap().window),
				};

				// the geometry is taken depending whether the window is set to tiled or floating
				match self.windows.get(0).unwrap().float_or_tile {
					FloatOrTile::Tile =>{
						let fullscreen_geometry = self.screen.to_geometry();

						WindowLayout {
							focused_window: temp_focused_window,
							windows: vec![(self.windows.get(0).unwrap().window, fullscreen_geometry)],
						}

					},

					FloatOrTile::Float =>{
						WindowLayout {
							focused_window: temp_focused_window,
							windows: vec![(self.windows.get(0).unwrap().window, self.windows.get(0).unwrap().geometry)],
						}
					}
				}
			}	        
		}else {
			WindowLayout::new()
		} 
	}

	fn focus_window(&mut self, window: Option<Window>) -> Result<(), Self::Error> {
		match window{
			None => {
				self.index_foused_window = None;
				Ok(())
			},

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

	// I'm assuming that cycle_focus applies for both tiled and floating windows
	// I implemetned the navie way, so the cycle is done throug the windows in the order
	// they were added, so I transverse the vector, this is becasue I create the actually layout
	// on fly, whenever the get_window_layout function is called.
	fn cycle_focus(&mut self, dir: PrevOrNext) {
		if self.windows.len() > 1 {
			match self.index_foused_window{

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
			if self.windows.len() == 1 {
				self.index_foused_window = Some(0);
			}
		}
	}

	fn get_window_info(&self, window: Window) -> Result<WindowWithInfo, Self::Error> {
		match self.windows.iter().position(|w| (*w).window == window) {
			None => Err(FloatingWMError::UnknownWindow(window)),
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

// This methods where update to only be applied to tiled windows, a no tiled window is given a 
// NoTiledWindow error is thrown
impl TilingSupport for FloatingWM {

	fn get_master_window(&self) -> Option<Window>{

		if !self.windows.is_empty(){
			//Now we have to look over the vec and select the first tiled window
			match self.windows.iter().position(|w| (*w).float_or_tile == FloatOrTile::Tile) {  
				//now it could be the case that no tiled window exist
				None => None,
				Some(index) => Some(self.windows.get(index).unwrap().window)
			}
		}else{
			None			
		}
	}

	fn swap_with_master(&mut self, window: Window) -> Result<(), Self::Error>{
		match self.windows.iter().position(|w| (*w).window == window) {
			None => Err(FloatingWMError::UnknownWindow(window)),
			Some(window_index) => {
				match self.get_master_index(){
					None => Ok(()),
					Some(master_index) => {
						self.windows.swap(master_index, window_index);
						self.focus_window(Some(window))
					}
				}
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
					if self.windows.get(index).unwrap().float_or_tile == FloatOrTile::Tile{
						match dir {
							PrevOrNext::Prev => {
								match self.get_prev_tile_index(index,index) {
									Some(prev_index) => {
										self.index_foused_window = Some(prev_index);
										self.windows.swap(index, prev_index);
									},
									None => (),
								}
							}

							PrevOrNext::Next => {
								match self.get_next_tile_index(index,index) {
									Some(next_index) => {
										self.index_foused_window = Some(next_index);
										self.windows.swap(index, next_index);
									},
									None => (),
								}
							}
						}
					}else{
						()
					}
				}
			}
		}
	}
}

impl FloatSupport for FloatingWM {

	// this is probably a pitfall of having both tiled and floating windows in one vector, now we have to iterate over the whole 
	// collection filter out the non-floating windows and return it. Because the vec keeps the entire windows_with_info structure, 
	// we to extract the window form it anyway.
	fn get_floating_windows(&self) -> Vec<Window>{
		let mut temp_windows = Vec::new();

		for window_with_info_floating in self.windows.iter().filter(|x|  (*x).float_or_tile == FloatOrTile::Float){
			temp_windows.push(window_with_info_floating.window.clone());
		}

		temp_windows
	}


	// This method is specially because can be applied to both tiled and floating windows
	// so the approach is iterate ove thewhole windows, get the correcponding window and
	// mutate the element, in this case the FloatOrTile  window_with_info structure of the given
	// window
	fn toggle_floating(&mut self, window: Window) -> Result<(), Self::Error>{
		match self.windows.iter().position(|w| (*w).window == window) {
			None => Err(FloatingWMError::UnknownWindow(window)),

			Some(i) => {

				if let Some(win) = self.windows.get_mut(i) {
					if win.float_or_tile == FloatOrTile::Tile{
						(*win).float_or_tile = FloatOrTile::Float;	
					}else{
						(*win).float_or_tile = FloatOrTile::Tile;	
					}

				};

				Ok(())
			}
		}
	}

	/// The approach is iterate over the windows until I found the given window, the if it is a floating window
	/// the corresponding window_with_info structure is mutated, otherwise a NoFloatingWindow error is thrown.
	/// Once again the trade off of have two vectors for each window type, now if have to iterate over the whole vec
	/// but at least the location of every window is saved with no extra structure
	fn set_window_geometry(&mut self, window: Window, new_geometry: Geometry)-> Result<(), Self::Error>
	{
		match self.windows.iter().position(|w| (*w).window == window) {
			None => Err(FloatingWMError::UnknownWindow(window)),

			Some(i) => {
				// it was already check that there is a window, so unwrap can be used
				let window_with_info = self.windows.get_mut(i).unwrap(); 

				if window_with_info.float_or_tile == FloatOrTile::Tile{
					Err(FloatingWMError::NoFloatingWindow(window_with_info.window))
				}else{
					(*window_with_info).geometry = new_geometry;	
					Ok(())
				}	           
			}

		}
	}
}
/*
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

		let mut wm = FloatingWM::new(SCREEN);

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

		let mut wm = FloatingWM::new(SCREEN);

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

		let mut wm = FloatingWM::new(SCREEN);

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

		let mut wm = FloatingWM::new(SCREEN);

		//swm screen should be the same as SCREEN
		assert_eq!(wm.get_screen(), SCREEN);

		//now, swm screen should be the same as SCREEN
		wm.resize_screen(SCREEN2);
		assert_eq!(wm.get_screen(), SCREEN2);
	}

	#[test]
	fn test_tiling_support() {

		let mut wm = FloatingWM::new(SCREEN);

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
		
	}


	#[test]
	fn test_floating_support() {

		let mut wm = FloatingWM::new(SCREEN);

		//Add some windows
		wm.add_window(WindowWithInfo::new_tiled(1, SOME_GEOM)).unwrap();
		wm.add_window(WindowWithInfo::new_tiled(2, SOME_GEOM)).unwrap();
		wm.add_window(WindowWithInfo::new_tiled(3, SOME_GEOM)).unwrap();
		wm.add_window(WindowWithInfo::new_tiled(4, SOME_GEOM)).unwrap();
		wm.add_window(WindowWithInfo::new_tiled(5, SOME_GEOM)).unwrap();

		//No floating window manager
		assert!(wm.get_floating_windows().is_empty());

		//Now let's do 4 and 1 floating elementes
		wm.toggle_floating(4).unwrap();
		wm.toggle_floating(1).unwrap();

		// since we keeot the order of the windows, no matters the order in which the toggle_floating funtion is applied
		// for 1 and 4, the vector should return [1,4], since that was the order in which they were added.
		assert_eq!(vec![1,4], wm.get_floating_windows());

		// now let check the layout, where tiled windows should be at the begining of the vec while floating elements,
		// should at the last, both floating elements should have SOME_GEOM as geometry.
		// The remining elements should have a proper geometry dependin in its position [2,3,5]
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
		assert_eq!(vec![(2, master_half),(3, first_half),(5, second_half),(1, SOME_GEOM),(4,SOME_GEOM)], wl1.windows);

		// now let's change the geometry of a titled window, an error should be occur
		assert!((wm.set_window_geometry(2,SCREEN_GEOM)).is_err());


		// now let's change the geometry of window 4,
		wm.set_window_geometry(4,SCREEN_GEOM).unwrap(); 

		// this change should be reflected in the window layout
		let wl2 = wm.get_window_layout();
		assert_eq!(vec![(2, master_half),(3, first_half),(5, second_half),(1, SOME_GEOM),(4,SCREEN_GEOM)], wl2.windows);

		// now we use toggle_floating again in window 1, since windows one was the initial master window, now it should
		// be allocated as master windows in the tiled layout. The appropiate modification of the entire tiled layout should be
		// reflected
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
		assert_eq!(vec![(1, master_half),(2, first_half_a),(3, second_half_a),(5, third_half_a),(4,SCREEN_GEOM)], wl3.windows);

	}

}
*/