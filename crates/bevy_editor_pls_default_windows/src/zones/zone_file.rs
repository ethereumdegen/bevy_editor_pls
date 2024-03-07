


use serde::{Serialize,Deserialize};
use bevy::prelude::*;


#[derive(Serialize,Deserialize)]
pub struct ZoneFile {


	entities: Vec<ZoneEntity>

}


impl ZoneFile {



	pub fn new(entities: Vec<Entity>, zone_entity_query:&Query<(&Name,&Transform)>) -> Self {
		let mut zone_entities = Vec::new();


		for entity in entities {

			if let Some(zone_entity) = ZoneEntity::from_entity(entity, &zone_entity_query){
				zone_entities.push(zone_entity);
			}
		}


		Self {

			entities: zone_entities
		}


	}

	pub fn load() -> Self {

		Self {

			entities: vec![]
		}

	}

}




#[derive(Serialize,Deserialize)]
pub struct ZoneEntity{


	name: String,

	transform: TransformSimple,


	// custom properties ? 

}


impl ZoneEntity {

	fn from_entity(entity: Entity,zone_entity_query:&Query<(&Name,&Transform)>) -> Option<Self> {
		 

		if let Some((name,xform)) = zone_entity_query.get(entity).ok() {

			 
			return Some(Self{
				name : name.as_str() .to_string(), 
				transform: xform.clone().into()
			})
		} 
		 

		None

	}

}




#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransformSimple {
    pub translation: Vec3,
    pub rotation: Vec3, //euler 
    pub scale: Vec3,
} 

impl From<Transform> for TransformSimple {
    fn from(transform: Transform) -> Self {
        // Extract translation directly
        let translation = transform.translation;

        // Convert quaternion to Euler angles (in radians)
        let (roll, pitch, yaw) = transform.rotation.to_euler(EulerRot::XYZ);

        // Extract scale directly
        let scale = transform.scale;

        // Create and return a new instance of TransformSimple
        TransformSimple {
            translation,
            rotation: Vec3::new(roll, pitch, yaw), // Assuming XYZ order for Euler angles
            scale,
        }
    }
}