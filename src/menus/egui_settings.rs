use egui::{DragValue, WidgetText};

use crate::menus::Menu;

pub struct EguiSettings;

impl Menu for EguiSettings {
    fn ui(&mut self, _app: &mut crate::app::GraphApp, ui: &mut egui::Ui) {
        let mut style = (*ui.ctx().style()).clone();
        ui.heading("Font Sizes");
        for (text_style, font_id) in style.text_styles.iter_mut() {
            ui.horizontal(|ui| {
                ui.label(WidgetText::from(text_style.to_string()).text_style(text_style.clone()));
                ui.add(DragValue::new(&mut font_id.size).clamp_range(5.0..=50.0));
            });
        }
        ui.ctx().set_style(style);
    }

    fn name(&self) -> &'static str {
        "UI Settings"
    }
}
