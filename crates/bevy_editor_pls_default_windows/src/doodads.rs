use bevy::{
    asset::ReflectAsset,
    prelude::{AppTypeRegistry, World},
    reflect::TypeRegistry,
};
use bevy_editor_pls_core::editor_window::{EditorWindow, EditorWindowContext};
use bevy_inspector_egui::egui;

use crate::inspector::{InspectorSelection, InspectorWindow};

pub struct DoodadsWindow;

impl EditorWindow for DoodadsWindow {
    type State = ();
    const NAME: &'static str = "Doodads";

    fn ui(world: &mut World, mut cx: EditorWindowContext, ui: &mut egui::Ui) {
        let selection = &mut cx.state_mut::<InspectorWindow>().unwrap().selected;
        let type_registry = world.resource::<AppTypeRegistry>();
        let type_registry = type_registry.read();

        select_doodad(ui, &type_registry, world, selection);
    }
}

fn select_doodad(
    ui: &mut egui::Ui,
    type_registry: &TypeRegistry,
    world: &World,
    selection: &mut InspectorSelection,
) {

    //read manifest file instead .. .
    let mut doodads: Vec<_> = type_registry
        .iter()
        .filter_map(|registration| {
            let reflect_asset = registration.data::<ReflectAsset>()?;
            Some((
                registration.type_info().type_path_table().short_path(),
                registration.type_id(),
                reflect_asset,
            ))
        })
        .collect();
    doodads.sort_by(|(name_a, ..), (name_b, ..)| name_a.cmp(name_b));

    for (asset_name, asset_type_id, reflect_asset) in doodads {
        let handles: Vec<_> = reflect_asset.ids(world).collect();

        ui.collapsing(format!("{asset_name} ({})", handles.len()), |ui| {
            for handle in handles {
                let selected = match *selection {
                    InspectorSelection::Asset(_, _, selected_id) => selected_id == handle,
                    _ => false,
                };

                if ui
                    .selectable_label(selected, format!("{:?}", handle))
                    .clicked()
                {
                    *selection =
                        InspectorSelection::Asset(asset_type_id, asset_name.to_owned(), handle);
                }
            }
        });
    }
}
