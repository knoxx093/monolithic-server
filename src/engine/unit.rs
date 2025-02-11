use ashscript_types::{
    actions::UnitAttack,
    components::{body::UnitBody, energy::Energy, health::Health, storage::Storage, tile::Tile, unit::Unit},
    player::PlayerId,
    resource::Resource,
};
use hexx::Hex;

use crate::game_state::GameState;

pub fn age_units(game_state: &mut GameState) {
    for (_, (body, storage)) in game_state.world.query_mut::<(&mut UnitBody, &Storage)>() {
        body.age += 1;

        // Age also increases based on how much uranium is being carried
        body.age += storage.resources.get(&Resource::Uranium).unwrap_or(&0) / 100;
    }
}

pub fn units_generate_energy(game_state: &mut GameState) {
    for (_, (body, energy)) in game_state.world.query_mut::<(&UnitBody, &mut Energy)>() {
        energy.current = (energy.current.saturating_add(body.energy_income())).min(body.energy_capacity());
    }
}

pub fn delete_old_units(game_state: &mut GameState) {
    for (entity, body) in game_state.world.query_mut::<&UnitBody>() {
        /* if body.age >= body.max_age() {
            game_state.despawn_entity(entity);
            return;
        } */
    }
}

pub fn attack_intents(game_state: &mut GameState, attack_intents: &Vec<UnitAttack>) {
    for intent in attack_intents {}
}

pub fn can_attack(game_state: &GameState, intent: &UnitAttack) -> bool {
    true
}

pub fn attack(
    attacker: &mut Unit,
    attacker_tile: &Tile,
    attacker_body: &UnitBody,
    attacker_energy: &mut Energy,
    target: &mut Unit,
    target_tile: &Tile,
    target_health: &mut Health,
) {
    let cost = attacker_body.attack_cost();
    if attacker_energy.current < cost {
        return;
    }

    if attacker_tile.hex == target_tile.hex {
        return;
    }

    let distance = attacker_tile.hex.unsigned_distance_to(target_tile.hex);
    if distance > attacker_body.range() {
        return;
    }

    let damage = attacker_body.damage();
    if damage > target_health.current {
        target_health.current = 0
    } else {
        target_health.current -= damage
    }

    attacker_energy.current = attacker_energy.current.saturating_sub(cost);
}
