use std::sync::Arc;

use pumpkin_core::{
    entity::{Entity, EntityBase, vehicle::minecart::MinecartEntity},
    server::Server,
};
use pumpkin_data::{entity::EntityType, item_stack::ItemStack};
use pumpkin_util::{GameMode, Hand, math::vector3::Vector3};

use crate::bot::HeadlessBot;

// Vanilla's default attack speed of 4 gives the fist a 5 tick attack strength delay.
const FIST_ATTACK_STRENGTH_DELAY_TICKS: usize = 5;
// A full strength fist hit deals 1 damage, and vehicles store damage times 10.
const FULL_FIST_HIT_VEHICLE_DAMAGE: f32 = 10.0;

pub fn run_fist_attack_charge_test(
    server: &Arc<Server>,
    bot: &HeadlessBot,
    summary_lines: &mut Vec<String>,
) -> Result<(), String> {
    let player = server.get_player_by_name(&bot.name).ok_or("missing bot")?;
    let world = player.world();
    // Vehicles have no hurt cooldown, so their damage shows the strength of every hit.
    let minecart = Arc::new(MinecartEntity::new(Entity::new(
        world.clone(),
        player.position() + Vector3::new(1.5, 0.0, 0.0),
        &EntityType::MINECART,
    )));
    world.spawn_entity(minecart.clone());
    let target: Arc<dyn EntityBase> = minecart.clone();
    let original_mode = player.gamemode.load();
    player.gamemode.store(GameMode::Survival);
    player
        .inventory()
        .set_stack_in_hand(Hand::Right, ItemStack::EMPTY.clone());

    player.attack(&target);
    for _ in 0..FIST_ATTACK_STRENGTH_DELAY_TICKS {
        server.tick();
    }
    let before = minecart.vehicle.get_damage();
    player.attack(&target);
    let hit = minecart.vehicle.get_damage() - before;

    minecart.get_entity().remove();
    player.gamemode.store(original_mode);
    if (hit - FULL_FIST_HIT_VEHICLE_DAMAGE).abs() > f32::EPSILON {
        return Err(format!(
            "A fist hit {FIST_ATTACK_STRENGTH_DELAY_TICKS} ticks after the last one added {hit} minecart damage, expected a full strength hit ({FULL_FIST_HIT_VEHICLE_DAMAGE})"
        ));
    }
    summary_lines.push("[PASS] bot:fist_attack_charge".to_string());
    Ok(())
}
