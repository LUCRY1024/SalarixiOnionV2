use serde_json::Value;

use crate::base::get_flow_manager;
use crate::tasks::TASKS;
use crate::state::STATES;
use super::*;


// Структура ModuleManager
pub struct ModuleManager;

impl ModuleManager {
  pub async fn control(name: String, options: Value, group: String) {
    if let Some(arc) = get_flow_manager() {
      let fm = arc.write();

      let bots = fm.bots.clone();

      if fm.bots.len() > 0 {
        match name.as_str() {
          "chat" => {
            let o: ChatOptions = serde_json::from_value(options).map_err(|e| format!("Ошибка парсинга опций: {}", e)).unwrap();

            for bot in bots.into_values() {
              if let Some(state) = STATES.get(&bot.username()) {
                if state.read().unwrap().group != group {
                  continue;
                }
              }

              let options_clone = o.clone();  
              let options_task = o.clone();
              let nickname = bot.username().clone();

              if o.mode.as_str() == "spamming" {
                ChatModule::stop(&bot.username());
              }

              if options_clone.state {
                let task = tokio::spawn(async move {
                  match options_task.mode.as_str() {
                    "message" => { let _ = ChatModule::message(&bot, options_task).await; },
                    "spamming" => { let _ = ChatModule::spamming(&bot, options_task).await; },
                    _ => {}
                  }
                });

                if options_clone.mode.as_str() == "spamming" {
                  TASKS.get(&nickname).unwrap().write().unwrap().run_task("spamming", task);
                }
              } else {
                if options_clone.mode.as_str() == "spamming" {
                  ChatModule::stop(&nickname);
                }
              }
            }
          },
          "action" => {
            let o: ActionOptions = serde_json::from_value(options).map_err(|e| format!("Ошибка парсинга опций: {}", e)).unwrap();

            for bot in bots.into_values() {
              if let Some(state) = STATES.get(&bot.username()) {
                if state.read().unwrap().group != group {
                  continue;
                }
              }

              let options_clone = o.clone();  
              let options_task = o.clone();
              let nickname = bot.username().clone();

              ActionModule::stop(&bot, &options_clone.action);

              if options_clone.state {
                let task = tokio::spawn(async move {
                  match options_task.action.as_str() {
                    "jumping" => { ActionModule::jumping(&bot, options_task).await; },
                    "shifting" => { ActionModule::shifting(&bot, options_task).await; },
                    "waving" => { ActionModule::waving(&bot, options_task).await; },
                    _ => {}
                  }
                });

                TASKS.get(&nickname).unwrap().write().unwrap().run_task(&options_clone.action, task);
              } else {
                ActionModule::stop(&bot, &options_clone.action);
              }
            }
          },
          "inventory" => {
            let o: InventoryOptions = serde_json::from_value(options).map_err(|e| format!("Ошибка парсинга опций: {}", e)).unwrap();

            for bot in bots.into_values() {
              if let Some(state) = STATES.get(&bot.username()) {
                if state.read().unwrap().group != group {
                  continue;
                }
              }

              let options_task = o.clone();

              tokio::spawn(async move {
                InventoryModule::action(&bot, options_task).await;
              });
            }
          },
          "movement" => {
            let o: MovementOptions = serde_json::from_value(options).map_err(|e| format!("Ошибка парсинга опций: {}", e)).unwrap();

            for bot in bots.into_values() {
              if let Some(state) = STATES.get(&bot.username()) {
                if state.read().unwrap().group != group {
                  continue;
                }
              }

              let options_clone = o.clone();  
              let nickname = bot.username().clone();

              MovementModule::stop(&bot);

              if options_clone.state {
                let task = tokio::spawn(async move {
                  MovementModule::enable(&bot, options_clone).await;
                });

                TASKS.get(&nickname).unwrap().write().unwrap().run_task("movement", task);
              } else {
                MovementModule::stop(&bot);
              }
            }
          },
          "anti-afk" => {
            let o: AntiAfkOptions = serde_json::from_value(options).map_err(|e| format!("Ошибка парсинга опций: {}", e)).unwrap();

            for bot in bots.into_values() {
              if let Some(state) = STATES.get(&bot.username()) {
                if state.read().unwrap().group != group {
                  continue;
                }
              }

              let options_clone = o.clone();  
              let nickname = bot.username().clone();

              AntiAfkModule::stop(&bot);

              if options_clone.state {
                let task = tokio::spawn(async move {
                  AntiAfkModule::enable(&bot, options_clone).await;
                });

                TASKS.get(&nickname).unwrap().write().unwrap().run_task("anti-afk", task);
              } else {
                AntiAfkModule::stop(&bot);
              }
            }
          },
          "flight" => {
            let o: FlightOptions = serde_json::from_value(options).map_err(|e| format!("Ошибка парсинга опций: {}", e)).unwrap();

            for bot in bots.into_values() {
              if let Some(state) = STATES.get(&bot.username()) {
                if state.read().unwrap().group != group {
                  continue;
                }
              }

              let options_clone = o.clone();  
              let nickname = bot.username().clone();

              FlightModule::stop(&nickname);

              if options_clone.state {
                let task = tokio::spawn(async move {
                  FlightModule::enable(&bot, options_clone).await;
                });

                TASKS.get(&nickname).unwrap().write().unwrap().run_task("flight", task);
              } else {
                FlightModule::stop(&nickname);
              }
            }
          },
          "killaura" => {
            let o: KillauraOptions = serde_json::from_value(options).map_err(|e| format!("Ошибка парсинга опций: {}", e)).unwrap();

            for bot in bots.into_values() {
              if let Some(state) = STATES.get(&bot.username()) {
                if state.read().unwrap().group != group {
                  continue;
                }
              }

              let options_clone = o.clone();  
              let nickname = bot.username().clone();

              KillauraModule::stop(&bot);

              if options_clone.state {
                let task = tokio::spawn(async move {
                  KillauraModule::enable(&bot, options_clone).await;
                });

                TASKS.get(&nickname).unwrap().write().unwrap().run_task("killaura", task);
              } else {
                KillauraModule::stop(&bot);
              }
            }
          },
          "scaffold" => {
            let o: ScaffoldOptions = serde_json::from_value(options).map_err(|e| format!("Ошибка парсинга опций: {}", e)).unwrap();

            for bot in bots.into_values() {
              if let Some(state) = STATES.get(&bot.username()) {
                if state.read().unwrap().group != group {
                  continue;
                }
              }

              let options_clone = o.clone();  
              let nickname = bot.username().clone();

              ScaffoldModule::stop(&bot);

              if options_clone.state {
                let task = tokio::spawn(async move {
                  ScaffoldModule::enable(&bot, options_clone).await;
                });

                TASKS.get(&nickname).unwrap().write().unwrap().run_task("scaffold", task);
              } else {
                ScaffoldModule::stop(&bot);
              }
            }
          },
          "anti-fall" => {
            let o: AntiFallOptions = serde_json::from_value(options).map_err(|e| format!("Ошибка парсинга опций: {}", e)).unwrap();

            for bot in bots.into_values() {
              if let Some(state) = STATES.get(&bot.username()) {
                if state.read().unwrap().group != group {
                  continue;
                }
              }

              let options_clone = o.clone();  
              let nickname = bot.username().clone();

              AntiFallModule::stop(&nickname);

              if options_clone.state {
                let task = tokio::spawn(async move {
                  AntiFallModule::enable(&bot, options_clone).await;
                });

                TASKS.get(&nickname).unwrap().write().unwrap().run_task("anti-fall", task);
              } else {
                AntiFallModule::stop(&nickname);
              }
            }
          },
          "bow-aim" => {
            let o: BowAimOptions = serde_json::from_value(options).map_err(|e| format!("Ошибка парсинга опций: {}", e)).unwrap();

            for bot in bots.into_values() {
              if let Some(state) = STATES.get(&bot.username()) {
                if state.read().unwrap().group != group {
                  continue;
                }
              }
              
              let options_clone = o.clone();  
              let nickname = bot.username().clone();

              BowAimModule::stop(&bot);

              if options_clone.state {
                let task = tokio::spawn(async move {
                  BowAimModule::enable(&bot, options_clone).await;
                });

                TASKS.get(&nickname).unwrap().write().unwrap().run_task("bow-aim", task);
              } else {
                BowAimModule::stop(&bot);
              }
            }
          },
          "stealer" => {
            let o: StealerOptions = serde_json::from_value(options).map_err(|e| format!("Ошибка парсинга опций: {}", e)).unwrap();

            for bot in bots.into_values() {
              if let Some(state) = STATES.get(&bot.username()) {
                if state.read().unwrap().group != group {
                  continue;
                }
              }

              let options_clone = o.clone();  
              let nickname = bot.username().clone();

              StealerModule::stop(&bot);

              if options_clone.state {
                let task = tokio::spawn(async move {
                  StealerModule::enable(&bot, options_clone).await;
                });

                TASKS.get(&nickname).unwrap().write().unwrap().run_task("stealer", task);
              } else {
                StealerModule::stop(&bot);
              }
            }
          },
          "miner" => {
            let o: MinerOptions = serde_json::from_value(options).map_err(|e| format!("Ошибка парсинга опций: {}", e)).unwrap();

            for bot in bots.into_values() {
              if let Some(state) = STATES.get(&bot.username()) {
                if state.read().unwrap().group != group {
                  continue;
                }
              }
              
              let options_clone = o.clone();  
              let nickname = bot.username().clone();

              MinerModule::stop(&bot);

              if options_clone.state {
                let task = tokio::spawn(async move {
                  MinerModule::enable(&bot, options_clone).await;
                });

                TASKS.get(&nickname).unwrap().write().unwrap().run_task("miner", task);
              } else {
                MinerModule::stop(&bot);
              }
            }
          },
          "farmer" => {
            let o: FarmerOptions = serde_json::from_value(options).map_err(|e| format!("Ошибка парсинга опций: {}", e)).unwrap();

            for bot in bots.into_values() {
              if let Some(state) = STATES.get(&bot.username()) {
                if state.read().unwrap().group != group {
                  continue;
                }
              }
              
              let options_clone = o.clone();  
              let nickname = bot.username().clone();

              FarmerModule::stop(&bot);

              if options_clone.state {
                let task = tokio::spawn(async move {
                  FarmerModule::enable(&bot, options_clone).await;
                });

                TASKS.get(&nickname).unwrap().write().unwrap().run_task("farmer", task);
              } else {
                FarmerModule::stop(&bot);
              }
            }
          },
          _ => {}
        }
      }
    }
  }
}