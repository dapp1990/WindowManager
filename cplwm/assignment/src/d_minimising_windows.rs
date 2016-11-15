//! Minimising Windows
//!
//! Extend your window manager with support for (un)minimising windows. i.e.
//! the ability to temporarily hide windows and to reveal them again later.
//! See the documentation of the [`MinimiseSupport`] trait for the precise
//! requirements.
//!
//! Either make a copy of the tiling window manager with support for floating
//! windows you developed in the previous assignment and let it implement the
//! [`MinimiseSupport`] trait as well, or implement this trait by building a
//! wrapper around the previous window manager. Note that this window manager
//! must still implement all the traits from previous assignments.
//!
//! [`MinimiseSupport`]: ../../cplwm_api/wm/trait.MinimiseSupport.html
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
//! it is a requesto to keep track of the position of them minimised elements, the best way I concude to actually do this ordering
//! is with a collection, in this case a vec is use to keep track the position minimised elements, such vector it is par of the
//! MinimisingWM structure. Moreover it is important to *remove* the minimised element from the windows vector, however if the removed
//! is used as it was implemented in the others window managers, we actually deleted from the windows vector whcih becomes pretty inconvinient
//! since the windows vector keep the order in which the windows were added, so when the minimised window is unminimised the previous location is lost
//! so the naive solution is added to push it to the top of the window vec, then the order is not longer the as the windows wehre added.
//! 
//! Since it is not possible to modify the WindowWithInfo structure to implement and additional flag to mark minimised windows and then don't show 
//! them when the get_window_layout is called, I decided to stored in windows a tupple (indow_with_info, uniminimised_window). In this way I can easily
//! hide the windows on the fly when get_window_layout is used while I keep the order of the windows when ever the minimised window is unminimised. It is worth
//! to notice that minimised_windows vector will keep track of the order in which the windows where minimised.
//! Since windows was updated, several small upgrades should be done in some functions to suppor a vector with the new structure.
//! 
//! 
//! PERSONAL NOTES:
//! let (windows_with_info, minimised_window) = tuple.clone(); -> no tuple is reference while
//! tuple.clone() ara actually the values.
//! 
// Add imports here
use std::error;
use std::fmt;

use cplwm_api::types::{PrevOrNext, Screen, Window, WindowLayout, WindowWithInfo, Geometry, FloatOrTile};
use cplwm_api::wm::WindowManager;
use cplwm_api::wm::TilingSupport;
use cplwm_api::wm::FloatSupport;
use cplwm_api::wm::MinimiseSupport;

/// **TODO**: Documentation
pub type WMName = MinimisingWM;

/// The MinimisingWM struct
///
/// # Example Representation
/// Now MinimisingWM contains an extra attribute to keep track the order of the minimised windows
/// 
#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct MinimisedWindow {
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
pub struct MinimisingWM {
	/// Vector where the elementes are minimised windows
	pub windows: Vec<MinimisedWindow>,  
	/// Vector that stores the order in which the windows were minimised
	pub minimised_windows: Vec<Window>,
	/// **TODO**: Documentation
	pub screen: Screen,
	/// The index of the focused window in the collection, if there is no focused window a None is placed
	pub index_foused_window: Option<usize>,
}

/// Supported functions
impl MinimisingWM {
	
	/// The method was upgraded to skip not only floating but also minimised windows
	pub fn get_next_tile_index(& self, index:usize, saved:usize) -> Option<usize> {
		let mut next_index = 0;		
		if self.windows.len()-1 > index{
			next_index = index + 1;			
		}
		if next_index == saved {
			None
		}else{
			match self.windows.get(next_index){
				None => None,
				Some(window) => {
					if window.float_or_tile == FloatOrTile::Tile && !window.minimised{
						Some(next_index)
					}else{
						self.get_next_tile_index(next_index,saved)
					}
				}
			}
		}
	}

	/// The method was upgraded to skip not only floating but also minimised windows
	pub fn get_prev_tile_index(& self, index:usize, saved:usize) -> Option<usize> {
		let mut prev_index = self.windows.len() -1;
		if index != 0{
			prev_index = index - 1;
		};		
		if prev_index == saved {
			None
		}else{
			match self.windows.get(prev_index){
				None => None,
				Some(window) => {
					if window.float_or_tile == FloatOrTile::Tile && !window.minimised{
						Some(prev_index)
					}else{
						self.get_prev_tile_index(prev_index,saved)
					}
				}
			}
		}
	}

	/// The method was upgraded to skip not only floating but also minimised windows
	fn get_master_index(&self) -> Option<usize>{
		if !self.windows.is_empty(){
			self.windows.iter().position(|w| (*w).float_or_tile == FloatOrTile::Tile)
		}else{
			None			
		}
	}

	/// *TODO*
	fn get_minimised_tiled_windows(&self) -> Vec<Window>{
		let mut temp_windows = Vec::new();
		for minimised_window in self.windows.iter().filter(|x| (*x).float_or_tile == FloatOrTile::Tile && (*x).minimised){
			temp_windows.push(minimised_window.window.clone())
		};
		temp_windows
	}

	/// This method calculated the tiled window's geometries in the order of the
	/// windows vector
	//*** Improvement: here you calculate first thar windows is greater than 1, but could be thecase
	// and not necessary it is a tiled window 
	fn calculate_tiled_geometries(&mut self){
		if !self.windows.is_empty(){

				//Divisor now should be update to just the tiled windows
				let non_tiled_windows = self.get_floating_windows().len() + self.get_minimised_tiled_windows().len() + 1;
				let total_windows = self.windows.len();
				//let divisor = self.windows.len() - non_tiled_windows;

				// if the divisor is greater than 0 we need to calculate slave windows
				//if divisor > 0{
				if total_windows > non_tiled_windows{
					let divisor = total_windows - non_tiled_windows;
					let divisor_2 = divisor as u32;
					let height_side = self.screen.height / divisor_2;
					let width_side = self.screen.width / 2;
					let x_point = (self.screen.width / 2) as i32;
					// It is already tested that there is more than 1 window, hence one can use unwrap method being sure that a 
					// Some intance of option will be returned
					let master_window = self.get_master_window().unwrap();

					let mut y_point = 0 as i32;

					for minimised_window in self.windows.iter_mut().filter(|x| (*x).float_or_tile == FloatOrTile::Tile && !(*x).minimised){
						if master_window != minimised_window.window {
							// I calculate the values of the secondary windows (right windows)
							let rigth_geometry = Geometry {
								x: x_point,
								y: y_point,
								width: width_side,
								height: height_side,
							};
							minimised_window.geometry = rigth_geometry;
							y_point += (height_side) as i32;

						}else{
							// I calculate the values for master window
							let  master_geometry = Geometry { 
								x: 0,
								y: 0,
								width: width_side,
								height: self.screen.height,
							};

							minimised_window.geometry = master_geometry;
						}
					};
				}else{
					// It could be the posibility that there is just one windwo (the master window)
					match self.get_master_index(){
						None => (),
						Some(i) => {
							let window = self.windows.get_mut(i).unwrap();
							window.geometry = self.screen.to_geometry();
						}
					}
				}
		};
	}

	/// Removes a minimised window from the minimised vector

	/**** Improvement: you should handle in a better way the None/error ****/
	fn remove_minimised_window(&mut self, window:Window){
		match self.minimised_windows.iter().position(|w| *w == window) {
			None => (),
			Some(i) => {
				self.minimised_windows.remove(i); 
				self.windows.get_mut(i).unwrap().minimised = false;
			}
		}
	}
}

/// **TODO**: Documentation
#[derive(Debug)]
pub enum MinimisingWMError {
	/// **TODO**: Documentation
	UnknownWindow(Window),
	/// **TODO**: Documentation
	ManagedWindow(Window),
	/// **TODO**: Documentation
	NoFloatingWindow(Window),
	/// **TODO**: Documentation
	NoTiledWindow(Window),
}

impl fmt::Display for MinimisingWMError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			MinimisingWMError::UnknownWindow(ref window) => write!(f, "Unknown window: {}", window),
			MinimisingWMError::ManagedWindow(ref window) => write!(f, "Window {} is already managed", window),
			MinimisingWMError::NoFloatingWindow(ref window) => write!(f, "Window {} is not floating", window),
			MinimisingWMError::NoTiledWindow(ref window) => write!(f, "Window {} is not tiled", window),
		}
	}
}

impl error::Error for MinimisingWMError {
	fn description(&self) -> &'static str {
		match *self {
			MinimisingWMError::UnknownWindow(_) => "Unknown window",
			MinimisingWMError::ManagedWindow(_) => "Window is already managed",
			MinimisingWMError::NoFloatingWindow(_) => "Window is not floating",
			MinimisingWMError::NoTiledWindow(_) => "Window is not tiled",
		}
	}
}


impl WindowManager for MinimisingWM {

	type Error = MinimisingWMError;

	// Upgrade to support the new minimised_windows vector
	fn new(screen: Screen) -> MinimisingWM {
		MinimisingWM {
			windows: Vec::new(),
			minimised_windows: Vec::new(),
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

	// get focused work for both floating, tiled and minimised windows
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

	// now add_window converts every window_with_info to minimised_window, settin the minimised
	// attribute as false
	fn add_window(&mut self, window_with_info: WindowWithInfo) -> Result<(), Self::Error> {
		if !self.is_managed(window_with_info.window) {
			//All new added windows are set to minimised = false by default
			let minimised_window = MinimisedWindow {
				window: window_with_info.window, 
				geometry: window_with_info.geometry, 
				saved_geometry: window_with_info.geometry, 
				float_or_tile: window_with_info.float_or_tile, 
				fullscreen: window_with_info.fullscreen, 
				minimised: false,};
			self.windows.push(minimised_window);
			let temp = self.windows.len() - 1;
			self.index_foused_window = Some(temp);
			if window_with_info.float_or_tile == FloatOrTile::Tile{
				self.calculate_tiled_geometries();
			}
			Ok(())
		}else{
			Err(MinimisingWMError::ManagedWindow(window_with_info.window))
		}
	}

	// method updated to removed the minimised window in the minimised_windows vector if that is the case
	fn remove_window(&mut self, window: Window) -> Result<(), Self::Error> {
		match self.windows.iter().position(|w| (*w).window == window) {
			None => Err(MinimisingWMError::UnknownWindow(window)),
			Some(i) => { 
				let temp_window = self.windows.get(i).unwrap().clone();
				self.windows.remove(i);
				
				if temp_window.minimised {
					self.remove_minimised_window(temp_window.window);
				};

				if temp_window.float_or_tile == FloatOrTile::Tile{
					self.calculate_tiled_geometries();
				};
				match self.index_foused_window {
					None => Ok(()),

					Some(index) => {
						//If the focused_element is the one that is erased,
						// None focused elemente
						if index == i {
							self.index_foused_window = None;
							Ok(())
						}else{
							// If the focused_element in the right side of the vector
							// the focused elemente is keep it, otherwise the focused
							// index is decreased by one.
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

	/// I opt to filter the vec of the MinimisingWM structure. First I filter the tiled windows, they are processed exactly like 
	/// in b_tilling_wm and attache them to the temporal vec. Then I get the windows with the attribute FloatOrTile::Float, I 
	/// obtain the Geometries (which are already saved in the vector) and add them to the temporal vec. 
	/// I keep the order in which the windows were added, no matter if they are tiled or floating, this becomes pretty handy
	/// because then the geometry of each individual window is dynamically adapted according with the type and it is return to
	// te same position (in the tiled set) where the window was left out. So if window x is the master and the toggle_floating 
	/// is used twice continuosly, x should be remaind as master window in the tiled layout, no matter the numbers of windows
	/// the window manager is handling.
	fn get_window_layout(&self) -> WindowLayout {

		if !self.windows.is_empty(){

			let mut temp_windows = Vec::new();

			for tiled_window in self.windows.iter().filter(|x| (*x).float_or_tile == FloatOrTile::Tile && (*x).minimised == false){
				temp_windows.push((tiled_window.window.clone(), tiled_window.geometry.clone()))
			};

			for floating_window in self.windows.iter().filter(|x| (*x).float_or_tile == FloatOrTile::Float && (*x).minimised == false){
				temp_windows.push((floating_window.window.clone(), floating_window.geometry.clone()))
			};

			let temp_focused_window = 
			match self.index_foused_window {
				None => None,
				Some(index) => Some(self.windows.get(index).unwrap().window),
			};

			WindowLayout {
				focused_window: temp_focused_window,
				windows: temp_windows,
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
					None => Err(MinimisingWMError::UnknownWindow(gw)),

					Some(i) => {
						let minimised_window = self.windows.get(i).unwrap().clone();
						self.index_foused_window = Some(i);
						if minimised_window.minimised{
							self.toggle_minimised(window.unwrap())
						}else{
							Ok(())
						}
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
		};

		match self.index_foused_window {
		    Some(index) => {
		    	let minimised_window = self.windows.get(index).unwrap().clone();
		    	/*** Improvement: do you really don't care about error? ***/
		    	match self.toggle_minimised(minimised_window.window){
		    		_ => (),
		    	}
		    },
		    None => (),
		}
	}

	//now get_window_info extrant the correct WindowWithInfo attributes, create a new structure
	//and return the proper resutls
	fn get_window_info(&self, window: Window) -> Result<WindowWithInfo, Self::Error> {
		match self.windows.iter().position(|w| (*w).window == window) {
			None => Err(MinimisingWMError::UnknownWindow(window)),
			Some(i) => {
				let minimised_window = self.windows.get(i).unwrap().clone();
				let window_with_info = WindowWithInfo{
					window: minimised_window.window,
				    geometry: minimised_window.geometry,
				    float_or_tile: minimised_window.float_or_tile,
				    fullscreen: minimised_window.fullscreen,
				};
				Ok(window_with_info)
			},
		}
	}

	fn get_screen(&self) -> Screen {
		self.screen
	}

	// When the scren is resized, the tiled windows should be updated accordingly
	fn resize_screen(&mut self, screen: Screen) {
		self.screen = screen;
		self.calculate_tiled_geometries()
	}

}

// This methods where update to only be applied to tiled windows, a no tiled window is given a 
// NoTiledWindow error is thrown
impl TilingSupport for MinimisingWM {

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
			None => Err(MinimisingWMError::UnknownWindow(window)),
			Some(window_index) => {
				match self.get_master_index(){
					None => Ok(()),
					Some(master_index) => {
						self.windows.swap(master_index, window_index);
						self.calculate_tiled_geometries();
						self.focus_window(Some(window))
					}
				}
			}
		}
	}


	// Simlar approach than cycle_focus, but now the structure should be updated accordingly, that behavior can be done
	// with the swap built-in method 
	/// *** Improvement: here the minimised window should be unminimised if that is the case ***/
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
										self.calculate_tiled_geometries();
									},
									None => (),
								}
							}

							PrevOrNext::Next => {
								match self.get_next_tile_index(index,index) {
									Some(next_index) => {
										self.index_foused_window = Some(next_index);
										self.windows.swap(index, next_index);
										self.calculate_tiled_geometries();
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

impl FloatSupport for MinimisingWM {

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
			None => Err(MinimisingWMError::UnknownWindow(window)),

			Some(i) => {

				if let Some(win) = self.windows.get_mut(i) {
					if win.float_or_tile == FloatOrTile::Tile{
						(*win).geometry = (*win).saved_geometry;	
						(*win).float_or_tile = FloatOrTile::Float;	
					}else{
						(*win).float_or_tile = FloatOrTile::Tile;
					}

				};
				self.calculate_tiled_geometries();
				Ok(())
			}
		}
	}

	/// The approach is iterate over the windows until I found the given window, the if it is a floating window
	/// the corresponding window_with_info structure is mutated, otherwise a NoFloatingWindow error is thrown.
	/// Once again the trade off of have two vectors for each window type, now if have to iterate over the whole vec
	/// but at least the location of every window is saved with no extra structure
	/// *** Improvement: here the minimised window should be unminimised if that is the case ***/
	fn set_window_geometry(&mut self, window: Window, new_geometry: Geometry)-> Result<(), Self::Error>
	{
		match self.windows.iter().position(|w| (*w).window == window) {
			None => Err(MinimisingWMError::UnknownWindow(window)),

			Some(i) => {
				// it was already check that there is a window, so unwrap can be used
				let window_with_info = self.windows.get_mut(i).unwrap(); 

				if window_with_info.float_or_tile == FloatOrTile::Tile{
					Err(MinimisingWMError::NoFloatingWindow(window_with_info.window))
				}else{
					(*window_with_info).saved_geometry = new_geometry;	
					(*window_with_info).geometry = new_geometry;
					Ok(())
				}	           
			}
		}
	}
}

impl MinimiseSupport for MinimisingWM {
	/// Return a vector of all the minimised windows.
    ///
    /// The order of the windows in the vector *does* matter.
    ///
    /// The windows must occur in the order they were minimised: the window
    /// that was minimised first must occur first in the vector, the window
    /// that was minimised last must occur last. This makes it easy to define
    /// a function that unminimises the last minimised window.
    fn get_minimised_windows(&self) -> Vec<Window>{
    	self.minimised_windows.clone()
    }


    /// Return `true` if the given window is minimised.
    ///
    /// This function must always return false when the given window is not
    /// managed by the window manager.
    ///
    /// **Invariant**: if `is_minimised(w) == true` for some window `w`, then
    /// `is_managed(w) == true`.
    ///
    /// **Invariant**: `is_minimised(w) == true` for some window `w`, iff the
    /// vector returned by the `get_minimised_windows` method contains `w`.
    ///
    /// A default implementation is provided in terms of
    /// `get_minimised_windows()`. Override this implementation if you have a
    /// more efficient one.
    fn is_minimised(&self, window: Window) -> bool {
        self.get_minimised_windows().contains(&window)
    }

    /// Minimise the given window, or when it is already minimised, unminise
    /// it.
    ///
    /// When a minimised floating window is unminimised, it should float again
    /// and have the same geometry as before. Hint: you could use the
    /// `float_or_tile` field of `WindowWithInfo`. Analogously for fullscreen
    /// windows.
    ///
    /// **Invariant**: if calling `toggle_minimised(w)` with an unminimised
    /// window `w` succeeds, `w` may no longer be visible according to
    /// `get_window_layout` and `is_minimised(w)` must return `true`.
    ///
    /// **Invariant**: if calling `toggle_minimised(w)` with an already
    /// minimised window `w` succeeds, `w` must be visible according to
    /// `get_window_layout` and `is_minimised(w)` must return `false`.
    ///
    /// The window layout before and after minimising and directly
    /// unminimising the currently focused window should be the same. This
    /// cannot hold for a window manager that implements
    /// [`TilingSupport`](trait.TilingSupport.html). Try to figure out why.
    /**** Improvement: There is a remove_minimised_window that should be reuse here ****/
    /**** Improvement: Improve those awful nested if :S ****/
    fn toggle_minimised(&mut self, window: Window) -> Result<(), Self::Error>{
    	if self.is_minimised(window){
    		match self.minimised_windows.iter().position(|w| *w == window) {
				None => Err(MinimisingWMError::UnknownWindow(window)),
				Some(i) => { 
					self.minimised_windows.remove(i);
					match self.windows.iter().position(|w| (*w).window == window) {
						None => Err(MinimisingWMError::UnknownWindow(window)),
						Some(i) => { 
							{
								let unminimised_window = self.windows.get_mut(i).unwrap();
								unminimised_window.minimised = false;
							};
							self.calculate_tiled_geometries();
							Ok(())						
						},
					}
				},
			}
    	}else{
    		match self.windows.iter().position(|w| (*w).window == window) {
				None => Err(MinimisingWMError::UnknownWindow(window)),
				Some(i) => { 
					{
						let minimised_window = self.windows.get_mut(i).unwrap();
						minimised_window.minimised = true;
						self.minimised_windows.push(minimised_window.window.clone());
					}
					self.calculate_tiled_geometries();
					Ok(())
				},
			}
    	}
    }
}

/*
#[cfg(test)]
mod tests {

	// We have to import `MinimisingWM` from the super module.
	use super::MinimisingWM;
	// We have to repeat the imports we did in the super module.
	use cplwm_api::wm::WindowManager;
	use cplwm_api::wm::TilingSupport;
	use cplwm_api::wm::FloatSupport;
	use cplwm_api::wm::MinimiseSupport;
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
		// Let's make a new `MinimisingWM` with `SCREEN` as screen.
		let mut wm = MinimisingWM::new(SCREEN);

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

		let mut wm = MinimisingWM::new(SCREEN);

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

		let mut wm = MinimisingWM::new(SCREEN);

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

		let mut wm = MinimisingWM::new(SCREEN);

		//Add some windows
		wm.add_window(WindowWithInfo::new_tiled(1, SOME_GEOM)).unwrap();
		wm.add_window(WindowWithInfo::new_tiled(2, SOME_GEOM)).unwrap();
		wm.add_window(WindowWithInfo::new_float(3, SCREEN_GEOM)).unwrap();
		wm.add_window(WindowWithInfo::new_tiled(4, SOME_GEOM)).unwrap();
		wm.add_window(WindowWithInfo::new_float(5, SOME_GEOM)).unwrap();
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
		assert_eq!(wm.get_window_info(3).unwrap().geometry, SCREEN_GEOM);
		assert_eq!(wm.get_window_info(4).unwrap().geometry, second_half);
		assert_eq!(wm.get_window_info(5).unwrap().geometry, SOME_GEOM);
		assert_eq!(wm.get_window_info(6).unwrap().geometry, third_half);
	}

	#[test]
	fn test_get_resize_screen() {

		let mut wm = MinimisingWM::new(SCREEN);

		//swm screen should be the same as SCREEN
		assert_eq!(wm.get_screen(), SCREEN);

		//now, swm screen should be the same as SCREEN
		wm.resize_screen(SCREEN2);
		assert_eq!(wm.get_screen(), SCREEN2);
	}

	#[test]
	fn test_tiling_support() {

		let mut wm = MinimisingWM::new(SCREEN);

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

    	// The other direction of the arrow must
    	// not hold, e.g., there could floating windows (see `FloatSupport`), but
    	// no tiled windows.
		wm.add_window(WindowWithInfo::new_float(90, SOME_GEOM)).unwrap();
		wm.add_window(WindowWithInfo::new_float(80, SOME_GEOM)).unwrap();
    	assert_eq!(wm.get_master_window(),None);
    	assert!(!wm.get_windows().is_empty());

	}


	#[test]
	fn test_floating_support() {

		let mut wm = MinimisingWM::new(SCREEN);

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
		// The remining elements should have a proper geometry depending in its position [2,3,5]
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

    	// **Invariant**: if `is_floating(w) == true` for some window `w`, then
    	// `is_managed(w) == true`.
		assert_eq!(wm.is_floating(4), true);
		assert_eq!(wm.is_managed(4), true);

    	// **Invariant**: `is_floating(w) == true` for some window `w`, iff the
    	// vector returned by the `get_floating_windows` method contains `w`.
    	assert_eq!(wm.is_floating(4), true);
		assert_eq!(vec![4], wm.get_floating_windows());

		// **Invariant**: if calling `toggle_floating(w)` with a tiled window `w`
    	// succeeds, `is_floating(w)` must return `true`.
    	assert_eq!(wm.is_floating(1), false);
    	wm.toggle_floating(1).unwrap();
		assert_eq!(wm.is_floating(1), true);

		// **Invariant**: if calling `toggle_floating(w)` with a floating window
    	// `w` succeeds, `is_floating(w)` must return `false`.
    	assert_eq!(wm.is_floating(4), true);
    	wm.toggle_floating(4).unwrap();
    	assert_eq!(wm.is_floating(4), false);

		// **Invariant**: the result of `is_floating(w)` must be the same before
    	// and after calling `toggle_floating(w)` twice.
    	assert_eq!(wm.is_floating(5), false);
    	wm.toggle_floating(5).unwrap();
    	wm.toggle_floating(5).unwrap();
    	assert_eq!(wm.is_floating(5), false);

	}

	#[test]
	fn test_minimise_support() {

		let mut wm = MinimisingWM::new(SCREEN);

		//Add some windows
		wm.add_window(WindowWithInfo::new_float(1, SOME_GEOM)).unwrap();
		wm.add_window(WindowWithInfo::new_float(2, SOME_GEOM)).unwrap();
		wm.add_window(WindowWithInfo::new_tiled(3, SOME_GEOM)).unwrap();
		wm.add_window(WindowWithInfo::new_tiled(4, SOME_GEOM)).unwrap();
		wm.add_window(WindowWithInfo::new_tiled(5, SOME_GEOM)).unwrap();
		wm.add_window(WindowWithInfo::new_tiled(6, SOME_GEOM)).unwrap();

		//No minimised windows
		assert!(wm.get_minimised_windows().is_empty());

		//Now let's minise window 5, 1 and 4
		wm.toggle_minimised(5).unwrap();
		wm.toggle_minimised(1).unwrap();
		wm.toggle_minimised(4).unwrap();

		// we have some minimised_windows, the array is given in the order the windows were minimised
		assert_eq!(vec![5,1,4], wm.get_minimised_windows());

		// the window layout shows window 2 and 3 and 6, where 2 is floating and 3, 6 are tiled
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

		let wl1 = wm.get_window_layout();
		assert_eq!(vec![(3, master_half),(6, second_half),(2, SOME_GEOM)], wl1.windows);

		// window 6 remains as the focused window
		assert_eq!(Some(6), wl1.focused_window);

		//  **Invariant**: if calling `toggle_minimised(w)` with an unminimised
    	// window `w` succeeds, `w` may no longer be visible according to
    	// `get_window_layout` and `is_minimised(w)` must return `true`.
    	wm.toggle_minimised(6).unwrap();
    	let wl2 = wm.get_window_layout();
		assert_eq!(vec![(3, FULLSCREEN),(2, SOME_GEOM)], wl2.windows);
		assert_eq!(wm.is_minimised(6), true);

		// **Invariant**: if calling `toggle_minimised(w)` with an already
    	// minimised window `w` succeeds, `w` must be visible according to
    	// `get_window_layout` and `is_minimised(w)` must return `false`.
		// now let's change the geometry of window 4,
		wm.toggle_minimised(1).unwrap();
		let wl3 = wm.get_window_layout();
		assert_eq!(vec![(3, FULLSCREEN),(1, SOME_GEOM),(2, SOME_GEOM)], wl3.windows);
		assert_eq!(wm.is_minimised(1), false);


		// **Invariant**: if `is_minimised(w) == true` for some window `w`, then
    	// `is_managed(w) == true`.
    	assert_eq!(wm.is_minimised(5), true);
    	assert_eq!(wm.is_managed(5), true);

    	// **Invariant**: `is_minimised(w) == true` for some window `w`, iff the
    	// vector returned by the `get_minimised_windows` method contains `w`.
    	assert_eq!(wm.is_minimised(5), true);
    	assert_eq!(vec![5,4,6], wm.get_minimised_windows());
	}
}*/