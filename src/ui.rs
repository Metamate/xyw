use bevy::prelude::{App, Plugin, Res, Update};
use bevy_egui::{egui, EguiContexts, EguiPlugin};

use crate::boid::BoidSettings;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin).add_systems(Update, show_stats);
    }
}

pub fn show_stats(mut contexts: EguiContexts, boid_settings: Res<BoidSettings>) {
    egui::Window::new("Stats").show(contexts.ctx_mut(), |ui| {
        ui.label(format!("Alignment: {}", boid_settings.alignment));
        ui.label(format!("Cohesion: {}", boid_settings.cohesion));
        ui.label(format!("Separation: {}", boid_settings.separation));
    });
}
