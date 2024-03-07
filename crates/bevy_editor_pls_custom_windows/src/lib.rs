#![allow(clippy::type_complexity)]
//! Custom windows for the editor
 
use bevy::prelude::*;
use doodads::{update_place_doodads,reset_place_doodads,handle_place_doodad_events};


pub mod doodads;
pub mod zones; 

pub struct CustomWindowsPlugin {
    
}
impl Plugin for CustomWindowsPlugin {
    fn build(&self, app: &mut App) {


    	app. add_systems(Update, update_place_doodads ) 

    	   .add_systems(Update, reset_place_doodads )

          .add_systems(Update, handle_place_doodad_events)

           ;

    }

}