use std::fs;
use std::fs::File;
use std::path::Path;
use specs::{Builder, Entity, Join, World, WorldExt};
use specs::saveload::{MarkedBuilder, SimpleMarker, SerializeComponents, DeserializeComponents, SimpleMarkerAllocator};
use specs::error::NoError;
use super::components::*;

const SAVE_PATH: &str = "./save_game.json";
macro_rules! serialize_individually {
    ($ecs:expr, $ser:expr, $data:expr, $( $type:ty),*) => {
        $(
        SerializeComponents::<NoError, SimpleMarker<SerializeMe>>::serialize(
            &( $ecs.read_storage::<$type>(), ),
            &$data.0,
            &$data.1,
            &mut $ser,
        )
        .unwrap();
        )*
    };
}

macro_rules! deserialize_individually {
    ($ecs:expr, $de:expr, $data:expr, $( $type:ty),*) => {
        $(
        DeserializeComponents::<NoError, _>::deserialize(
            &mut ( &mut $ecs.write_storage::<$type>(), ),
            &mut $data.0, // entities
            &mut $data.1, // marker
            &mut $data.2, // allocater
            &mut $de,
        )
        .unwrap();
        )*
    };
}
pub fn save_game(ecs: &mut World) {
    let map_copy = ecs.get_mut::<super::map::Map>().unwrap().clone();
    let save_helper = ecs
        .create_entity()
        .with(SerializationHelper { map: map_copy })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
    {
        let data = (ecs.entities(), ecs.read_storage::<SimpleMarker<SerializeMe>>());

        let writer = File::create(SAVE_PATH).unwrap();
        let mut serializer = serde_json::Serializer::new(writer);
        serialize_individually!(ecs, serializer, data, Position, TargetedPosition, Renderable,
            Player, Name, Item, Stored, Impassable, RequiresItem, ContainsItem, PermanentItem, SerializationHelper,
            Portal, BelongsTo, Npc, Objective, Interaction
        );
    }

    // Clean up
    ecs.delete_entity(save_helper).expect("Crash on cleanup");
}

pub fn save_exists() -> bool {
    Path::new(SAVE_PATH).exists()
}

pub fn load_game(ecs: &mut World) {
    {
        // Delete everything
        let mut to_delete = Vec::new();
        for e in ecs.entities().join() {
            to_delete.push(e);
        }
        for del in to_delete.iter() {
            ecs.delete_entity(*del).expect("Deletion failed");
        }
    }

    let data = fs::read_to_string(SAVE_PATH).unwrap();
    let mut de = serde_json::Deserializer::from_str(&data);

    {
        let mut d = (&mut ecs.entities(), &mut ecs.write_storage::<SimpleMarker<SerializeMe>>(), &mut ecs.write_resource::<SimpleMarkerAllocator<SerializeMe>>());

        deserialize_individually!(ecs, de, d, Position, TargetedPosition, Renderable,
            Player, Name, Item, Stored, Impassable, RequiresItem, ContainsItem, PermanentItem, SerializationHelper,
            Portal, BelongsTo, Npc, Objective, Interaction
        );
    }

    let mut delete_me: Option<Entity> = None;
    {
        let entities = ecs.entities();
        let helper = ecs.read_storage::<SerializationHelper>();
        let player = ecs.read_storage::<Player>();
        let position = ecs.read_storage::<Position>();
        for (e, h) in (&entities, &helper).join() {
            let mut world_map = ecs.write_resource::<super::map::Map>();
            *world_map = h.map.clone();
            delete_me = Some(e);
        }
        for (e, _p, pos) in (&entities, &player, &position).join() {
            let mut player_pos = ecs.write_resource::<rltk::Point>();
            *player_pos = rltk::Point::new(pos.x, pos.y);
            let mut player_resource = ecs.write_resource::<Entity>();
            *player_resource = e;
        }
    }
    ecs.delete_entity(delete_me.unwrap()).expect("Unable to delete helper");
}
