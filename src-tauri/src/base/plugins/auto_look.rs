use azalea::{Vec3, prelude::*};
use azalea::ecs::prelude::*;
use azalea::entity::{Dead, LocalEntity, Position};
use std::time::Duration;
use tokio::time::sleep;

use crate::base::get_flow_manager;
use crate::state::STATES;
use crate::tasks::TASKS;
use crate::tools::randfloat;


pub struct AutoLookPlugin;

impl AutoLookPlugin {
  pub fn enable(bot: Client) {
    tokio::spawn(async move {
      loop {
        if let Some(arc) = get_flow_manager() {
          if !arc.read().active {
            break;
          }
        }

        Self::look(&bot).await;

        sleep(Duration::from_millis(50)).await;
      }
    });
  } 

  async fn look(bot: &Client) {
    let eye_pos = bot.eye_position();

    let nearest_entity = bot.nearest_entity_by::<&Position, (Without<LocalEntity>, Without<Dead>)>(|position: &Position| {
      eye_pos.distance_to(**position) <= 14.0
    });

    if let Some(entity) = nearest_entity {
      if let Some(entity_pos) = bot.get_entity_component::<Position>(entity) {
        let nickname = bot.username();

        if let Some(arc) = TASKS.get(&nickname) {
          if !STATES.get_plugin_activity(&nickname, "auto-potion") && !arc.write().unwrap().get_task_activity("bow-aim") && !arc.write().unwrap().get_task_activity("killaura") && !arc.write().unwrap().get_task_activity("scaffold") && !arc.write().unwrap().get_task_activity("miner") {
            STATES.set_plugin_activity(&nickname, "auto-look", true);

            let pos = Vec3::new(
              entity_pos.x + randfloat(-0.1, 0.1), 
              entity_pos.y + randfloat(-0.1, 0.1), 
              entity_pos.z + randfloat(-0.1, 0.1)
            );

            bot.look_at(pos);

            bot.wait_ticks(1).await;

            STATES.set_plugin_activity(&nickname, "auto-look", false);
          }
        }
      }
    }
  }
}
