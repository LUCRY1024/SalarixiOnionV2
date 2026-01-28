use azalea::prelude::*;
use azalea::ecs::prelude::*;
use azalea::Vec3;
use azalea::entity::{Dead, LocalEntity, Position, metadata::{Player, AbstractAnimal, AbstractMonster}};
use azalea::registry::builtin::ItemKind;
use azalea::world::MinecraftEntityId;
use serde::{Serialize, Deserialize};

use crate::TASKS;
use crate::state::STATES;
use crate::tools::*;
use crate::common::{convert_inventory_slot_to_hotbar_slot, find_empty_slot_in_hotbar, find_empty_slot_in_invenotry};


#[derive(Debug)]
struct Weapon {
  slot: Option<usize>,
  priority: u8
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KillauraModule;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KillauraOptions {
  pub mode: String,
  pub settings: String,
  pub target: String,
  pub slot: Option<u8>,
  pub distance: Option<f64>,
  pub delay: Option<usize>,
  pub state: bool
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct KillauraConfig {
  target: String,
  slot: Option<u8>,
  distance: f64,
  delay: usize
}

impl KillauraModule {
  fn create_adaptive_config(target: String) -> KillauraConfig {
    KillauraConfig {
      target: target,
      slot: None,
      distance: 3.1,
      delay: 1
    }
  }

  fn find_nearest_entity(bot: &Client, target: String, distance: f64) -> Option<Entity> {
    let mut nearest_entity = None;

    let bot_position = bot.eye_position();

    match target.as_str() {
      "player" => { 
        nearest_entity = bot.nearest_entity_by::<&Position, (With<Player>, Without<LocalEntity>, Without<Dead>)>(|position: &Position| {
          bot_position.distance_to(**position) <= distance
        });
      },
      "monster" => {
        nearest_entity = bot.nearest_entity_by::<&Position, (With<AbstractMonster>, Without<LocalEntity>, Without<Dead>)>(|position: &Position| {
          bot_position.distance_to(**position) <= distance
        });
      },
      "animal" => {
        nearest_entity = bot.nearest_entity_by::<&Position, (With<AbstractAnimal>, Without<LocalEntity>, Without<Dead>)>(|position: &Position| {
          bot_position.distance_to(**position) <= distance
        });
      },
      _ => {}
    }

    nearest_entity
  }

  fn get_entity_position(bot: &Client, entity: Entity) -> Vec3 {
    let mut ecs = bot.ecs.lock(); 
    let pos = ecs.get_mut::<Position>(entity).unwrap().clone();

    Vec3::new(pos.x, pos.y, pos.z)
  }

  async fn auto_weapon(bot: &Client) {
    let mut weapons = vec![];

    let menu = bot.menu();

    for (slot, item) in menu.slots().iter().enumerate() {
      if !item.is_empty() {
        match item.kind() {
          ItemKind::WoodenSword => { weapons.push(Weapon { slot: Some(slot), priority: 0 }); },
          ItemKind::GoldenSword => { weapons.push(Weapon { slot: Some(slot), priority: 1 }); },
          ItemKind::StoneSword => { weapons.push(Weapon { slot: Some(slot), priority: 2 }); },
          ItemKind::CopperSword => { weapons.push(Weapon { slot: Some(slot), priority: 3 }); },
          ItemKind::IronSword => { weapons.push(Weapon { slot: Some(slot), priority: 4 }); },
          ItemKind::DiamondSword => { weapons.push(Weapon { slot: Some(slot), priority: 5 }); },
          ItemKind::NetheriteSword => { weapons.push(Weapon { slot: Some(slot), priority: 6 }); },
          _ => {}
        }
      }
    }

    let mut best_weapon = Weapon { slot: None, priority: 0 };

    for weapon in weapons {
      if weapon.priority > best_weapon.priority {
        best_weapon = weapon;
      }
    }

    if let Some(slot) = best_weapon.slot {
      if let Some(hotbar_slot) = convert_inventory_slot_to_hotbar_slot(slot) {
        if bot.selected_hotbar_slot() != hotbar_slot {
          bot.set_selected_hotbar_slot(hotbar_slot);
        }
      } else {
        let inventory = bot.get_inventory();

        if let Some(empty_slot) = find_empty_slot_in_hotbar(bot) {
          inventory.left_click(slot);
          bot.wait_ticks(randticks(1, 2)).await;
          inventory.left_click(empty_slot);

          if let Some(slot) = convert_inventory_slot_to_hotbar_slot(empty_slot as usize) {
            if bot.selected_hotbar_slot() != slot {
              bot.set_selected_hotbar_slot(slot);
            }
          }
        } else {
          let random_slot = randuint(36, 44) as usize;

          inventory.shift_click(random_slot);

          bot.wait_ticks(1).await;

          inventory.left_click(slot);
          bot.wait_ticks(randticks(1, 2)).await;
          inventory.left_click(random_slot);

          let hotbar_slot = convert_inventory_slot_to_hotbar_slot(random_slot).unwrap_or(0);

          if bot.selected_hotbar_slot() != hotbar_slot {
            bot.set_selected_hotbar_slot(hotbar_slot);
          }
        }
      }
    }
  }

  async fn moderate_killaura(bot: &Client, options: KillauraOptions) {
    let config = if options.settings.as_str() == "adaptive" {
      Self::create_adaptive_config(options.target.clone())
    } else {
      KillauraConfig { 
        target: options.target.clone(), 
        slot: options.slot, 
        distance: options.distance.unwrap_or(3.1), 
        delay: options.delay.unwrap_or(1)
      }
    };

    loop {
      if !STATES.get_plugin_activity(&bot.username(), "auto-eat") && !STATES.get_plugin_activity(&bot.username(), "auto-potion") {
        if options.settings.as_str() == "adaptive" {
          Self::auto_weapon(bot).await;
        } else {
          if let Some(slot) = config.slot {
            if slot <= 8 {
              bot.set_selected_hotbar_slot(slot);
            }
          } else {
            Self::auto_weapon(bot).await;
          }
        }

        let nearest_entity = Self::find_nearest_entity(bot, config.target.clone(), config.distance);

        if let Some(entity) = nearest_entity {
          if let Some(bot_id) = bot.get_entity_component::<MinecraftEntityId>(bot.entity) {
            if let Some(entity_id) = bot.get_entity_component::<MinecraftEntityId>(entity) {
              if bot_id != entity_id {
                let should_jump = randchance(0.5);
                let should_shift = randchance(0.5);

                if should_jump {
                  bot.jump();
                }

                if should_shift {
                  bot.set_crouching(true);
                }

                bot.wait_ticks(randticks(1, 2)).await;

                bot.look_at(Self::get_entity_position(bot, entity));
                bot.wait_ticks(randticks(3, 5)).await;
                bot.attack(entity);

                if should_shift {
                  bot.set_crouching(false);
                }
              }
            }
          }
        }
      }
      
      bot.wait_ticks(config.delay).await;
    }
  }

  async fn aggressive_killaura(bot: &Client, options: KillauraOptions) {
    let config = if options.settings.as_str() == "adaptive" {
      Self::create_adaptive_config(options.target.clone())
    } else {
      KillauraConfig { 
        target: options.target.clone(), 
        slot: options.slot, 
        distance: options.distance.unwrap_or(3.1), 
        delay: options.delay.unwrap_or(1)
      }
    };

    loop {
      if !STATES.get_plugin_activity(&bot.username(), "auto-eat") && !STATES.get_plugin_activity(&bot.username(), "auto-potion") {
        if options.settings.as_str() == "adaptive" {
          Self::auto_weapon(bot).await;
        } else {
          if let Some(slot) = config.slot {
            if slot <= 8 {
              bot.set_selected_hotbar_slot(slot);
            }
          } else {
            Self::auto_weapon(bot).await;
          }
        }

        let nearest_entity = Self::find_nearest_entity(bot, config.target.clone(), config.distance);

        if let Some(entity) = nearest_entity {
          if let Some(bot_id) = bot.get_entity_component::<MinecraftEntityId>(bot.entity) {
            if let Some(entity_id) = bot.get_entity_component::<MinecraftEntityId>(entity) {
              if bot_id != entity_id {
                bot.wait_ticks(randticks(3, 4)).await;
                bot.attack(entity);
              }
            }
          }
        }
      }

      bot.wait_ticks(config.delay).await;
    }
  }

  pub async fn enable(bot: &Client, options: KillauraOptions) {
    match options.mode.as_str() {
      "moderate" => { Self::moderate_killaura(bot, options).await; },
      "aggressive" => { Self::aggressive_killaura(bot, options).await; },
      _ => {}
    }
  }

  pub fn stop(nickname: &String) {
    TASKS.get(nickname).unwrap().write().unwrap().stop_task("killaura");
  }
}