use azalea::prelude::*;
use azalea::ecs::prelude::*;
use regex::Regex;


pub struct CaptchaCatcher;

impl CaptchaCatcher {
  pub fn catch_url_from_message(message: String, regex: &str, required_url_part: Option<String>) -> Option<String> {
    let re = Regex::new(regex).unwrap();

    for link_to_captcha in re.find_iter(&message) {
      if !link_to_captcha.is_empty() {
        if let Some(required) = required_url_part.clone() {
          if link_to_captcha.as_str().contains(required.as_str()) {
            return Some(link_to_captcha.as_str().to_string());
          }
        } else {
          return Some(link_to_captcha.as_str().to_string());
        }
      }
    }

    None  
  }

  pub fn catch_image_from_frame(bot: &Client, radius: f64) {
    // Логика получения изображения ...
  }
}