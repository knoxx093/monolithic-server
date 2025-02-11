use ashscript_types::components::{body::UnitBody, health::Health};
use hecs::Entity;

use crate::game_state::GameState;

pub fn delete_0_health(game_state: &mut GameState) {
    let mut remove_entities: Vec<Entity> = Vec::new();

    for (entity, (health)) in &mut game_state.world.query::<&Health>() {
        if health.current == 0 {
            remove_entities.push(entity);
            continue;
        }
    }

    for entity in remove_entities {
        let result = game_state.despawn_entity(entity);
        println!("despawn result: {result:?}");
    }
}
