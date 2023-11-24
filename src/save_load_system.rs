use std::fs::File;
use std::path::Path;
use specs::{Builder, World, WorldExt};
use specs::saveload::{MarkedBuilder, SimpleMarker};
use crate::{SerializationHelper, SerializeMe};

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
            Player, Name, Item, Stored, Impassable, Door, RequiresItem, PermanentItem, SerializationHelper
        );
    }

    // Clean up
    ecs.delete_entity(save_helper).expect("Crash on cleanup");
}

pub fn save_exists() -> bool {
    Path::new(SAVE_PATH).exists()
}