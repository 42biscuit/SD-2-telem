use egui::{Id, Ui, Vec2, ViewportBuilder, ViewportId};

pub struct ConfigWindow {
    id: Id,
    builder: ViewportBuilder,
    pub open: bool,
}

impl ConfigWindow {
    pub fn new() -> ConfigWindow {
        let builder = ViewportBuilder::default()
            .with_inner_size(Vec2::new(400.0, 400.0))
            .with_resizable(false)
            .with_always_on_top()
            .with_active(true)
            .with_title("Config");

        ConfigWindow {
            id: "config_window".into(),
            builder,
            open: false,
        }
    }

    pub fn update(&mut self, ctx: &eframe::egui::Context) {
        if self.open {
            self.show(ctx);
        }

        //if ctx.viewport_id().
    }

    pub fn show(&self, ctx: &eframe::egui::Context) {
        ctx.show_viewport_deferred(ViewportId(self.id.clone()), self.builder.clone(), |ctx, _| {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.label("AAAAAAAAAAAAAAA");

                if ui.input(|i| i.viewport().close_requested()) {
                    //self.open = false;
                }
            });
        });
    }
}