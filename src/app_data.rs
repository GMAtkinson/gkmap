use vizia::*;

use crate::alert;

#[derive(Debug, Lens)]
pub struct AppData {
    pub visible: bool,
    pub text_size: f32,
}

pub enum AppEvent {
    Debug(String),
    Test,
    SetTextSize(f32),
}

impl Model for AppData {
    fn event(&mut self, cx: &mut Context, event: &mut Event) {
        if let Some(app_event) = event.message.downcast() {
            match app_event {
                AppEvent::Debug(val) => {
                    alert(val);
                    println!("Done");
                },

                AppEvent::Test => {
                    self.visible = true;
                }

                AppEvent::SetTextSize(val) => {
                    self.text_size = *val;
                }
            }
        }
    }
}