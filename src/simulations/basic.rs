use ashscript_types::{
    components::{
        energy::Energy, factory::Factory, owner::Owner, storage::Storage, substation::Substation, tile::Tile, turbine::Turbine, turret::Turret
    },
    objects::GameObjectKind,
    player::Player,
    resource::Resource,
};
use hexx::hex;
use uuid::Uuid;

use crate::{
    engine::generate::structures::{spawn_factory, spawn_substation, spawn_turbine, spawn_turret},
    game_state::GameState,
};

pub fn generate(game_state: &mut GameState) {
    for i in 0..2 {
        let id = Uuid::new_v4();
        game_state.global.players.insert(
            id,
            Player {
                id,
                name: format!("Player {}", i),
                ..Default::default()
            },
        );

        println!("generating player {} with id {}", i, id);
    }

    let factory_hexes = [hex(14, -6), hex(-8, 4)];
    let turret_hexes = [hex(17, -7), hex(-11, 5)];
    let substation_hexes = [hex(20, -8), hex(-13, 6)];
    let turbine_hexes = [hex(23, -9), hex(-16, 7)];

    let player_ids = game_state
        .global
        .players
        .keys()
        .cloned()
        .collect::<Vec<Uuid>>();
    for (i, player_id) in player_ids.iter().enumerate() {
        // factories

        let hex = factory_hexes[i];
        let factory_entity = spawn_factory(game_state, hex, *player_id);
        let (_, factory_storage) = game_state
            .world
            .query_one_mut::<(&Factory, &mut Storage)>(factory_entity)
            .unwrap();
        factory_storage.capacity = 10_000;

        factory_storage
            .add_checked(&Resource::Metal, &1000)
            .unwrap();

        // turrets

        let hex = turret_hexes[i];
        let turret_entity = spawn_turret(game_state, hex, *player_id);
        let (_, turret_energy) = game_state
            .world
            .query_one_mut::<(&Turret, &mut Energy)>(turret_entity)
            .unwrap();

        turret_energy.current = 1000;

        // substations

        let hex = substation_hexes[i];
        let substation_entity = spawn_substation(game_state, hex, *player_id);

        // turbines

        let hex = turbine_hexes[i];
        let turbine_entity = spawn_turbine(game_state, hex, *player_id);
    }
}

pub fn update(game_state: &mut GameState) {
    for (entity, (factory, storage)) in &mut game_state.world.query::<(&Factory, &mut Storage)>() {
        let _ = storage.add_checked(&Resource::Metal, &1000);
    }

    for (entity, (turret, energy)) in &mut game_state.world.query::<(&Turret, &mut Energy)>() {
        let _ = energy.current.saturating_add(1000);
    }
}
