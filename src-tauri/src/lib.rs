mod emit;
mod tools;
mod base;
mod common;
mod radar;
mod quick;
mod webhook;

use crate::base::*;
use crate::quick::*;
use crate::radar::*;
use crate::emit::*;
use crate::webhook::*;

// Botları başlatma fonksiyonu
#[tauri::command(async)]
async fn launch_bots(options: LaunchOptions) -> (String, String) {
  if let Some(arc) = get_flow_manager() {
    let mut fm = arc.write();

    if fm.active {
      return ("warning".to_string(), format!("Zaten aktif botlar var, tekrar başlatılamaz."));
    }

    let _ = fm.launch(options);

    return ("success".to_string(), format!("Botlar başlatıldı!"));
  }

  ("error".to_string(), format!("FlowManager başlatılamadı."))
}

// Botları durdurma fonksiyonu
#[tauri::command(async)]
async fn stop_bots() -> (String, String) {
  if let Some(arc) = get_flow_manager() {
    emit_message("Sistem", format!("{} bot durduruluyor...", get_active_bots_count()));
    return arc.write().stop();
  } else {
    return ("error".to_string(), format!("FlowManager başlatılamadı"));
  }
}

// Bot profillerini al
#[tauri::command]
fn get_bot_profiles() -> Option<std::collections::HashMap<String, Profile>> {
  Some(PROFILES.get_all())
}

// Bot mesajı gönder
#[tauri::command]
fn send_message(nickname: String, message: String) {
  if let Some(arc) = get_flow_manager() {
    arc.read().send_message(&nickname, &message);
  }
}

// Botu resetle
#[tauri::command]
fn reset_bot(nickname: String) -> (String, String) {
  if let Some(arc) = get_flow_manager() {
    if let Some(msg) = arc.read().reset_bot(&nickname) {
      return ("info".to_string(), msg);
    }
  }
  ("error".to_string(), format!("Bot ({}) görevleri sıfırlanamadı.", nickname))
}

// Bot bağlantısını kes
#[tauri::command]
fn disconnect_bot(nickname: String) -> (String, String) {
  if let Some(arc) = get_flow_manager() {
    if let Some(msg) = arc.read().disconnect_bot(&nickname) {
      return ("info".to_string(), msg);
    }
  }
  ("error".to_string(), format!("Bot ({}) bağlantısı kesilemedi.", nickname))
}

// Grup ayarla
#[tauri::command]
fn set_group(nickname: String, group: String) {
  if let Some(arc) = get_flow_manager() {
    arc.read().bots.get(&nickname).map(|_| {
      PROFILES.set_str(&nickname, "group", &group);
    });
  }
}

// Radar verisi al
#[tauri::command]
fn get_radar_data(target: String) -> Option<RadarInfo> {
  RADAR_MANAGER.find_target(target)
}

// Radar verisi kaydet
#[tauri::command]
fn save_radar_data(target: String, path: String, filename: String, x: f64, y: f64, z: f64) {
  RADAR_MANAGER.save_data(target, path, filename, x, y, z);
}

// Aktif bot sayısını al
#[tauri::command]
fn get_active_bots_count() -> i32 {
  if let Some(arc) = get_flow_manager() {
    let fm = arc.read();
    let mut count = 0;
    for (nickname, _) in &fm.bots {
      if let Some(profile) = PROFILES.get(&nickname) {
        if profile.status.to_lowercase().as_str() == "online" || profile.status.to_lowercase().as_str() == "онлайн" {
          count += 1;
        }
      }
    }
    return count;
  }
  0
}

// Bellek kullanımı
#[tauri::command]
fn get_memory_usage() -> f64 {
  if let Some(usage) = memory_stats::memory_stats() {
    return usage.physical_mem as f64 / 1_000_000.0;
  }
  0.0
}

// Bot kontrolü (Komutlar)
#[tauri::command]
async fn control(name: String, options: serde_json::Value, group: String) {
  if let Some(opts) = get_current_options() {
    if opts.use_webhook && opts.webhook_settings.actions {
      send_webhook(opts.webhook_settings.url, format!("'{}' grubu '{}' komutunu aldı. Seçenekler: {}", group, name, options));
    }
  }
  emit_event(EventType::Log(LogEventPayload { 
    name: "extended".to_string(), 
    message: format!("'{}' grubu '{}' komutunu aldı. Seçenekler: {}", group, name, options)
  }));
  emit_message("Kontrol", format!("'{}' grubu '{}' komutunu başarıyla aldı.", group, name));
  MODULE_MANAGER.control(name, options, group).await;
}

// Hızlı görevler
#[tauri::command]
async fn quick_task(name: String) {
  if let Some(opts) = get_current_options() {
    if opts.use_webhook && opts.webhook_settings.actions {
      send_webhook(opts.webhook_settings.url, format!("Hızlı görev çalıştırıldı: '{}'", name));
    }
  }
  emit_event(EventType::Log(LogEventPayload { 
    name: "extended".to_string(), 
    message: format!("Hızlı görev çalıştırıldı: '{}'", name)
  }));
  emit_message("Hızlı Görev", format!("{} bot '{}' görevini aldı.", get_active_bots_count(), name));
  QUICK_TASK_MANAGER.execute(name);
}

// Harita render
#[tauri::command]
async fn render_map(nickname: String) -> Option<String> {
  let mut base64_code = None;
  if let Some(arc) = get_flow_manager() {
    arc.read().bots.get(&nickname).map(|bot| {
      base64_code = Some(MAP_RENDERER.render(bot));
    });
  }
  base64_code
}

// Harita kaydet
#[tauri::command]
async fn save_map(nickname: String, path: Option<String>, base64code: String) {
  MAP_RENDERER.save_map(nickname, path, base64code);
}

// URL Aç
#[tauri::command]
fn open_url(url: String) {
  let _ = open::that(url);
}

// Çıkış
#[tauri::command]
fn exit() {
  std::process::exit(0x0);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .setup(|app| {
      init_flow_manager(FlowManager::new(app.handle().clone()));  
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      exit, launch_bots, stop_bots, get_bot_profiles, 
      send_message, reset_bot, disconnect_bot,
      get_radar_data, save_radar_data, set_group,
      get_active_bots_count, get_memory_usage,
      control, quick_task, render_map, save_map, open_url
    ])
    .run(tauri::generate_context!())
    .expect("İstemci başlatılamadı.");
}
