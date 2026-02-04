use azalea::prelude::*;
use azalea::WalkDirection;
use azalea::pathfinder::goals::XZGoal;  
use azalea::pathfinder::PathfinderOpts;
use azalea::pathfinder::astar::PathfinderTimeout;
use azalea::pathfinder::moves;
use serde::{Serialize, Deserialize};
use std::time::Duration;
use tokio::time::sleep;

use crate::base::*;
use crate::tools::*;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovementModule;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovementOptions {
  pub mode: String,
  pub direction: String,
  pub use_sync: bool,
  pub use_impulsiveness: bool,
  pub x: Option<i32>,
  pub z: Option<i32>,
  pub state: bool
}

impl MovementModule {
  pub fn new() -> Self {
    Self
  }

  pub async fn enable(&self, bot: &Client, options: MovementOptions) {
    match options.mode.as_str() {
      "default" => {
        match options.use_impulsiveness {
          true => {
            if !options.use_sync {
              sleep(Duration::from_millis(randuint(500, 2000))).await;
            }

            loop {
              match options.direction.as_str() {
                "forward" => { bot.walk(WalkDirection::Forward); },
                "backward" => { bot.walk(WalkDirection::Backward); },
                "left" => { bot.walk(WalkDirection::Left); },
                "right" => { bot.walk(WalkDirection::Right); },
                _ => {}
              }

              if options.use_sync {
                sleep(Duration::from_millis(1200)).await;
              } else {
                sleep(Duration::from_millis(randuint(800, 1800))).await;
              }

              bot.walk(WalkDirection::None);
            }
          },
          false => {
            if !options.use_sync {
              sleep(Duration::from_millis(randuint(500, 2000))).await;
            }

            match options.direction.as_str() {
              "forward" => { bot.walk(WalkDirection::Forward); },
              "backward" => { bot.walk(WalkDirection::Backward); },
              "left" => { bot.walk(WalkDirection::Left); },
              "right" => { bot.walk(WalkDirection::Right); },
              _ => {}
            }
          }
        }
      },
      "pathfinder" => {
        if let Some(x) = options.x {
          if let Some(z) = options.z {
            if !options.use_sync {
              sleep(Duration::from_millis(randuint(500, 2000))).await;
            }

            bot.start_goto_with_opts(
              XZGoal { x: x, z: z },  
              PathfinderOpts::new()  
                .min_timeout(PathfinderTimeout::Time(Duration::from_millis(300)))  
                .max_timeout(PathfinderTimeout::Time(Duration::from_millis(1000)))  
                .allow_mining(false)  
                .successors_fn(moves::basic::basic_move)  
            );
          }
        }
      },
      _ => {}
    }
  } 

  pub fn stop(&self, bot: &Client) {
    TASKS.get(&bot.username()).unwrap().write().unwrap().kill_task("movement");
    bot.walk(WalkDirection::None);
    bot.stop_pathfinding();
  }
}