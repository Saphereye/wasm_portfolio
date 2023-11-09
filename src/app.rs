#[derive(serde::Deserialize, serde::Serialize)]
enum Window {
    About,
    Resume,
    Projects,
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct Website {
    // Example stuff:
    label: String,

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,

    current_window: Window,
}

impl Default for Website {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Adarsh Das".to_owned(),
            value: 0.0,
            current_window: Window::About,
        }
    }
}

impl Website {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for Website {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                egui::widgets::global_dark_light_mode_switch(ui);

                ui.separator();
            
                if ui.button("About").clicked() {
                    self.current_window = Window::About;
                }
                if ui.button("Resume").clicked() {
                    self.current_window = Window::Resume;
                }
                if ui.button("Projects").clicked() {
                    self.current_window = Window::Projects;
                }

                ui.add_space(16.0);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.current_window {
                Window::About => {
                    ui.heading("About Me");
                }
                Window::Resume => {
                    // The central panel the region left after adding TopPanel's and SidePanel's
                    ui.heading("Resume");

                    ui.horizontal(|ui| {
                        ui.label("Go on write something");
                        ui.text_edit_singleline(&mut self.label);
                    });

                    ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("value"));
                    if ui.button("Increase number").clicked() {
                        self.value += 0.1_f32;
                    }

                    ui.separator();

                    ui.add(egui::github_link_file!(
                        "https://github.com/emilk/eframe_template/blob/master/",
                        "Source code."
                    ));

                    ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                        powered_by_egui_and_eframe(ui);
                        egui::warn_if_debug_build(ui);
                    });
                }
                Window::Projects => {
                    ui.heading("Projects");
                }
            }
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
