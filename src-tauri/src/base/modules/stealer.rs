use std::time::Duration;

use azalea::prelude::*;
use azalea::Vec3;
use azalea::core::position::BlockPos;
use azalea::container::ContainerHandle;
use serde::{Serialize, Deserialize};
use tokio::time::sleep;

use crate::base::*;
use crate::tools::*;
use crate::common::get_block_state;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StealerModule;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StealerOptions {
  pub target: String,
  pub radius: Option<i32>,
  pub delay: Option<u64>,
  pub state: bool
}

impl StealerModule {
  pub fn new() -> Self {
    Self
  }

  fn check_block_id(&self, block_id: u16, target: &String) -> bool {
    match target.as_str() {
      "chest" => {
        return vec![3793, 3787, 3805, 3799].contains(&block_id);
      },
      "barrel" => {
        return vec![20547, 20543, 20541, 20549, 20545].contains(&block_id);
      },
      "shulker" => {
        return vec![
          14666, 14672, 14678, 14720, 
          14756, 14714, 14762, 14744,
          14702, 14696, 14708, 14750,
          14684, 14726, 14738, 14732
        ].contains(&block_id);
      },
      _ => {}
    }

    false
  }

  fn find_nearest_targets(&self, bot: &Client, center: Vec3, target: &String, radius: i32) -> Vec<azalea::BlockPos> {
    let mut positions = Vec::new();  
        
    for x in -radius..=radius {  
      for y in -radius..=radius {  
        for z in -radius..=radius {  
          let block_pos = BlockPos::new(  
            (center.x as i32 + x) as i32,  
            (center.y as i32 + y) as i32,  
            (center.z as i32 + z) as i32,  
          );  
                    
          if let Some(state) = get_block_state(bot, block_pos) {  
            if self.check_block_id(state.id(), target) {  
              positions.push(block_pos);  
            }  
          } 
        } 
      }  
    }
      
    positions  
  }

  async fn extract_all_items(&self, container: &ContainerHandle) {  
    let menu = container.menu().unwrap();  
       
    for slot in 0..=26 {  
      if let Some(item) = menu.slot(slot) {  
        if !item.is_empty() {  
          container.shift_click(slot); 
        }  
      }  
    }  
  }

  pub async fn enable(&self, bot: &Client, options: StealerOptions) {
    loop {
      let position = bot.position();  
      let direction = bot.direction();

      let target_positions = self.find_nearest_targets(bot, position, &options.target, if let Some(radius) = options.radius { radius } else { 4 });
        
      for pos in target_positions {  
        bot.look_at(pos.center());  

        sleep(Duration::from_millis(randuint(50, 100))).await;
            
        if let Some(container) = bot.open_container_at(pos).await {  
          self.extract_all_items(&container).await;  
          container.close();
          sleep(Duration::from_millis(randuint(200, 300))).await;
        }  
      } 

      bot.set_direction(direction.0 + randfloat(-2.5, 2.5) as f32, direction.1 + randfloat(-2.5, 2.5) as f32);

      sleep(Duration::from_millis(options.delay.unwrap_or(1000))).await;
    }
  } 

  pub fn stop(&self, bot: &Client) {
    kill_task(&bot.username(), "stealer");
    bot.get_inventory().close();
  }
}