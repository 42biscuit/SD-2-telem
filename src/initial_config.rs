use eframe::egui;

#[derive(Default)]
pub struct InitialConfig {}

impl InitialConfig {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for InitialConfig {
    // TODO : have a combo box of settings  you can select. and select the stroke of the front and rear shock
   fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
       egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("inital Config!!");

            ui.horizontal(|ui|{
                ui.label("front stroke".to_string());
            });
        });
   }
}