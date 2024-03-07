use bevy::prelude::*;
use bevy_editor_pls_core::editor_window::{EditorWindow, EditorWindowContext};
use bevy_inspector_egui::egui::{self, RichText};


  

#[derive(Resource,Default)]
pub struct PlacementResource {
	pub random_scale_multiplier: Option<f32>,  //usually like 0.2 
	pub randomize_yaw: bool ,

	pub translation_grid_lock_step: Option<f32> 
}


 
#[derive(Default)]
pub struct PlacementWindowState {
   // filename: String,
   // zone_create_result: Option<Result<(), Box<dyn std::error::Error + Send + Sync>>>,
}

pub struct PlacementWindow;

impl EditorWindow for PlacementWindow {
    type State = PlacementWindowState;
    const NAME: &'static str = "Placement";

    fn ui(world: &mut World, mut cx: EditorWindowContext, ui: &mut egui::Ui) {
        let state = cx.state_mut::<PlacementWindow>().unwrap();

        let placement_resource = world.resource::<PlacementResource>();
      

		 ui.vertical(|ui| {
 


		 //create zone 
	        ui.horizontal(|ui| { 

 

	        })
	        // ----- h
	    }); // ---- v

        
    

     }
}



   

 