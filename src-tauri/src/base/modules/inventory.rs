use azalea::prelude::*;
use azalea::inventory::operations::{SwapClick, ThrowClick};
use serde::{Serialize, Deserialize};

use crate::base::*;
use crate::emit::*;
use crate::common::get_inventory;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryModule;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryOptions {
  pub slot: Option<u16>,
  pub target_slot: Option<u16>,
  pub state: String
}

impl InventoryModule { 
  pub fn new() -> Self {
    Self
  }

  pub async fn interact(&self, bot: &Client, options: &InventoryOptions) {
    if let Some(s) = options.slot {
      let nickname = bot.username();

      if options.state.as_str() == "select" {
        if s <= 8 {
          bot.set_selected_hotbar_slot(s as u8);
        } else {
          emit_event(EventType::Log(LogEventPayload {
            name: "error".to_string(),
            message: format!("Бот {} не смог взять слот {} (индекс слота не должен превышать 8)", nickname, s)
          }));
        }
      } else {
        if let Some(inventory) = get_inventory(bot) {
          match options.state.as_str() {
            "drop" => {
              inventory.click(ThrowClick::All { slot: s });
            },
            "left-click" => {
              inventory.left_click(s);
            },
            "right-click" => {
              inventory.right_click(s);
            },
            "swap" => {
              inventory.click(SwapClick { source_slot: s, target_slot: if let Some(t) = options.target_slot { t as u8 } else { 0 }});
            },
            _ => {}
          }

          STATES.set_state(&nickname, "can_walking", true);
          STATES.set_state(&nickname, "can_sprinting", true);
          STATES.set_state(&nickname, "can_interacting", true);
          STATES.set_state(&nickname, "can_attacking", true);
        }
      }
    } 
  }
}
