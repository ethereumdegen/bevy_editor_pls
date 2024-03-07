use bevy::{
    asset::ReflectAsset,
   
    reflect::TypeRegistry,
};


use bevy::prelude::*;


use bevy::{
     gltf::{Gltf, GltfMesh, GltfNode} };


use bevy_editor_pls_core::editor_window::{EditorWindow, EditorWindowContext};
use bevy_inspector_egui::bevy_egui::EguiContexts;
use bevy_inspector_egui::{   egui::{self, ScrollArea}};

 
use bevy_common_assets::ron::RonAssetPlugin;

use bevy_mod_raycast::CursorRay;

use bevy_mod_raycast::prelude::Raycast;

use self::doodad::{DoodadComponent, DoodadPlugin, LoadedGltfAssets};
use self::doodad_manifest::{DoodadDefinition, DoodadManifest, DoodadManifestResource};


#[derive(Resource,Default)]
pub struct DoodadToolState  {
   pub selected: Option<DoodadDefinition> ,
   
}


mod doodad;
mod doodad_manifest;




#[derive(Event)]
 pub struct PlaceDoodadEvent {
    pub position: Vec3,
    pub doodad_definition: DoodadDefinition
 }


 




#[derive(Default)]
pub struct DoodadWindowState  {
  //  pub selected: Option<DoodadDefinition> ,
  //  rename_info: Option<RenameInfo>,
}


pub struct DoodadsWindow;

impl EditorWindow for DoodadsWindow {
    type State = DoodadWindowState;
    const NAME: &'static str = "Doodads";

     /// Necessary setup (resources, systems) for the window.
    fn app_setup(app: &mut App) {
       app
        .add_event::<PlaceDoodadEvent>()
            .add_plugins(RonAssetPlugin::<DoodadManifest>::new(&["manifest.ron"]))

              .add_plugins( DoodadPlugin  )
          .insert_resource( DoodadManifestResource::default()  ) 
          .insert_resource( DoodadToolState::default()  ) 
            .insert_resource( LoadedGltfAssets::default()  ) 
          .add_systems(Startup, load_doodad_manifest)
          .add_systems(Update, load_doodad_models)

          .add_systems(Update, update_place_doodads)
          .add_systems(Update, reset_place_doodads)

          .add_systems(Update, handle_place_doodad_events)

       ;
    }



    fn ui(
        world: &mut World,
         mut cx: EditorWindowContext,
          ui: &mut egui::Ui, 

          ) {


        let doodad_definition_resource = world.resource::<DoodadManifestResource>() ;

        //this releases the lock on World 
        let doodad_manifest_handle = &doodad_definition_resource.manifest.clone();


        let doodad_manifests_map = world.resource::<Assets<DoodadManifest>>();

        let doodad_manifest = doodad_manifest_handle.as_ref().and_then(|h|   doodad_manifests_map.get( h ) ) .cloned() ;


        let mut doodad_tool_resource = world.resource_mut::<DoodadToolState>();



         



/*
         let doodad_row_state = match cx.state_mut::<DoodadsWindow >() {
                Some(a) => a,
                None => {
                    let a = cx
                        .state_mut ::<DoodadsWindow   >()
                        .unwrap();
                    a
                }
            };
*/


        ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                 


                    let Some(doodad_manifest) = &doodad_manifest else {

                         ui.label(format!(" No doodad definitions found. "));
                         return;

                     };


                     if let Some(selected_doodad_def) = &doodad_tool_resource.selected {
                        ui.label(format!( "Placing: {:?}",  selected_doodad_def.name.clone() ))  ;
                      
                        ui.separator();

                        if ui.button("reset").clicked() {
                             doodad_tool_resource.selected = None;

                        }

                     }else{
                        ui.label( "---"  );
                     }


                      ui.separator();

              

                            for doodad_definition in &doodad_manifest.doodad_definitions {
                                 let label_text = doodad_definition.name.clone();
                                  let checked = false;

                                   if ui.selectable_label(checked, label_text.clone()).clicked() {
                                      //*selection = InspectorSelection::Entities ;

                                   println!("detected a doodad click  !! {:?}", label_text);

                                

                                   doodad_tool_resource.selected = Some( doodad_definition.clone() );

                                }
                            }
                           
                             
            });






    }
}


// --------------------------------------------------------

fn load_doodad_manifest(
   asset_server: Res<AssetServer> , 
   mut doodad_manifest_resource: ResMut<DoodadManifestResource>
){

    doodad_manifest_resource.manifest = Some( asset_server.load("doodad_manifest.manifest.ron")  ) ;
 

}
 
fn load_doodad_models(
     mut evt_asset: EventReader<AssetEvent<DoodadManifest>>,
       doodad_manifest_resource: Res<DoodadManifestResource>,
       doodad_manifest_assets: Res<Assets<DoodadManifest>>,


       mut loaded_gltf_resource: ResMut<LoadedGltfAssets>,

        asset_server: ResMut<AssetServer>
){

    let Some(  doodad_manifest_handle ) = &doodad_manifest_resource.manifest else {return};

    for evt  in evt_asset.read() {

        match evt {
            AssetEvent::LoadedWithDependencies { id } => {

                 if  id == &doodad_manifest_handle.id()  {

                     let manifest: &DoodadManifest = doodad_manifest_assets
                        .get( doodad_manifest_handle.id())
                        .unwrap();

                        println!("loading gltfs 1 ");

                        for doodad_definition in &manifest.doodad_definitions {

                                let model_path = &doodad_definition.model_path;

                                let gltf_model_handle:Handle<Gltf> = asset_server.load( model_path   ) ;

                                loaded_gltf_resource.gltf_models.insert(model_path.clone(), gltf_model_handle); 

                                println!("loaded gltf {:?}", model_path);

                        }

                  
                 }

            }
            _ => {}
        }

     
      

    }
     
 

}
 
 

fn handle_place_doodad_events(
    mut commands : Commands,

    mut evt_reader: EventReader<PlaceDoodadEvent>

) {

    for evt in evt_reader.read()  {


        let position = &evt.position;
        let doodad_spawned = commands.spawn(SpatialBundle{

            transform: Transform::from_xyz(position.x, position.y, position.z) ,
            ..default()
        })

        .insert(DoodadComponent::from_definition( &evt.doodad_definition ))
        .id();

        println!("doodad spawned {:?}", doodad_spawned);



    }





}


 fn update_place_doodads(
    mouse_input: Res<ButtonInput<MouseButton>>, //detect mouse click

    cursor_ray: Res<CursorRay>,
    mut raycast: Raycast,

    mut  event_writer: EventWriter<PlaceDoodadEvent>,
    
    doodad_tool_resource: Res<DoodadToolState>,
 
    mut contexts: EguiContexts,
) {
 

    let selected_doodad_definition = &doodad_tool_resource.selected;
 

    if !mouse_input.just_pressed(MouseButton::Left) {
        return;
    }

    let egui_ctx = contexts.ctx_mut();
    if egui_ctx.is_pointer_over_area() {
        return;
    }
 
 
    if let Some(cursor_ray) = **cursor_ray {
        if let Some((_intersection_entity, intersection_data)) =
            raycast.cast_ray(cursor_ray, &default()).first()
        {
            let hit_point = intersection_data.position();

            //offset this by the world psn offset of the entity !? would need to query its transform ?  for now assume 0 offset.
            let hit_coordinates = Vec3::new(hit_point.x, hit_point.y, hit_point.z);

            //use an event to pass the entity and hit coords to the terrain plugin so it can edit stuff there

            if let Some(doodad_definition) =  selected_doodad_definition .clone() {

                 
                 event_writer.send(PlaceDoodadEvent { 
                    position: hit_coordinates,
                    doodad_definition   
                 });
            }
           
        }
    }
}




 fn reset_place_doodads(
    mouse_input: Res<ButtonInput<MouseButton>>, //detect mouse click

      
    
   mut doodad_tool_resource: ResMut<DoodadToolState>,
 
     mut contexts: EguiContexts,
) {

   
 
 
   let egui_ctx = contexts.ctx_mut();

    if egui_ctx.is_pointer_over_area() {
        return;
    }
 

    if !mouse_input.just_pressed(MouseButton::Right) {
        return;
    }
 
    doodad_tool_resource.selected = None;
}


