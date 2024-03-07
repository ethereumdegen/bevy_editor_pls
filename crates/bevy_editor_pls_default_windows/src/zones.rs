use bevy::prelude::*;
use bevy_editor_pls_core::editor_window::{EditorWindow, EditorWindowContext};
use bevy_inspector_egui::egui::{self, RichText};



#[derive(Component)]
pub struct ZoneComponent {

}


#[derive(Event)]
pub enum ZoneEvent {
	SetZoneAsPrimary(Entity),
	ExportZone(Entity),
	ResetPrimaryZone
}

#[derive(Resource,Default)]
pub struct ZoneResource {
	pub primary_zone: Option<Entity>,

}




const DEFAULT_FILENAME: &str = "zone01";

#[derive(Default, Component)]
pub struct NotInScene;

#[derive(Default)]
pub struct ZoneWindowState {
    filename: String,
    zone_create_result: Option<Result<(), Box<dyn std::error::Error + Send + Sync>>>,
}

pub struct ZoneWindow;

impl EditorWindow for ZoneWindow {
    type State = ZoneWindowState;
    const NAME: &'static str = "Zones";

    fn ui(world: &mut World, mut cx: EditorWindowContext, ui: &mut egui::Ui) {
        let state = cx.state_mut::<ZoneWindow>().unwrap();

        let zone_resource = world.resource::<ZoneResource>();
        let primary_zone = zone_resource.primary_zone;

		 ui.vertical(|ui| {

		 	 ui.horizontal(|ui| {
				 ui.label( format!("Primary zone: {:?}", primary_zone   )) ;
				  if ui.button("Reset").clicked()   {
				  		world.send_event::<ZoneEvent>( ZoneEvent::ResetPrimaryZone ) ;
				  }
			});


		 //create zone 
	        ui.horizontal(|ui| { 


	               let res = egui::TextEdit::singleline(&mut state.filename)
	                .hint_text(DEFAULT_FILENAME)
	                .desired_width(120.0)
	                .show(ui);



	            if res.response.changed() {
	                state.zone_create_result = None;
	            }

	            let enter_pressed = ui.input(|input| input.key_pressed(egui::Key::Enter));

	            if ui.button("Create Zone").clicked() || enter_pressed {
	                let filename = if state.filename.is_empty() {
	                    DEFAULT_FILENAME
	                } else {
	                    &state.filename
	                };
	                let mut query = world.query_filtered::<Entity, Without<NotInScene>>();
	                let entitys = query.iter(world).collect();
	                state.zone_create_result = Some(create_zone(world, filename, entitys));
	            }

	        })
	        // ----- h
	    }); // ---- v

        if let Some(status) = &state.zone_create_result {
            match status {
                Ok(()) => {
                    ui.label(RichText::new("Success!").color(egui::Color32::GREEN));
                }
                Err(error) => {
                    ui.label(RichText::new(error.to_string()).color(egui::Color32::RED));
                }
            }
        }
    

     }
}




fn create_zone(
    world: &mut World,
    name: &str,
    entities: std::collections::HashSet<Entity>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    
   let zone = world.spawn(SpatialBundle::default())
   .insert(ZoneComponent{})
   .insert(Name::new( name.to_string() ))
   .id();

   /*
    let type_registry = world.get_resource::<AppTypeRegistry>().unwrap();
    let mut scene_builder = DynamicSceneBuilder::from_world(world);
    scene_builder = scene_builder.extract_entities(entities.into_iter());
    let scene = scene_builder.build();

    let ron = scene.serialize_ron(type_registry)?;
    std::fs::write(name, ron)?;
    */
    Ok(())
}


pub fn handle_zone_events( 


  mut evt_reader: EventReader<ZoneEvent>,

  mut zone_resource: ResMut<ZoneResource>

 ){


 	for evt in evt_reader.read(){



 		match evt {
    ZoneEvent::SetZoneAsPrimary(ent) =>  {

    	zone_resource.primary_zone = Some(ent.clone());

    },
    ZoneEvent::ResetPrimaryZone => {
    	zone_resource.primary_zone = None; 
    },
    ZoneEvent::ExportZone(ent) => {

    	println!("exporting zone ! {:?}", ent);

    	//find all children ?? 



    },

}



 	}




}