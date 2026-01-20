use azalea::prelude::*;
use azalea::prelude::ContainerClientExt;
use azalea::registry::builtin::ItemKind;

use crate::tools::randuint;


pub struct AutoTotemPlugin;

impl AutoTotemPlugin {
  pub fn take_totem(bot: Client) {
    tokio::spawn(async move {
      if Self::is_totem_slot_empty(&bot) {
        for (slot, item) in bot.menu().slots().iter().enumerate(){  
          if slot != 45 {
            if item.kind() == ItemKind::TotemOfUndying {
              let inventory = bot.get_inventory();

              inventory.left_click(slot);
              bot.wait_ticks(randuint(1, 2) as usize).await;
              inventory.left_click(45 as usize);
            }
          }
        }
      } 
    });
  }

  fn is_totem_slot_empty(bot: &Client) -> bool {
    let menu = bot.menu();

    if let Some(item) = menu.slot(45) {
      if item.is_empty() {
        return true;
      }
    }

    false
  }
}
