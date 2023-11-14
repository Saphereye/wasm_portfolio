use std::fmt::format;

use egui::*;

mod graphing_calculator;
mod projects;
mod expense_calculator;

use graphing_calculator::*;
use projects::*;
use expense_calculator::*;

#[derive(serde::Deserialize, serde::Serialize, PartialEq)]
enum Window {
    About,
    Projects,
    GraphingCalculator,
    NoteMaker,
    ExpenseCalculator,
    Resume,
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

    graphing_calculator_app: GraphingCalculatorApp,

    main_menu_size: f32,

    expense_calculator_app: ExpenseCalcApp,
}

impl Default for Website {
    fn default() -> Self {
        Self {
            label: "Adarsh Das".to_owned(),
            value: 0.0,
            current_window: Window::About,
            graphing_calculator_app: GraphingCalculatorApp::default(),
            main_menu_size: 17.0,
            expense_calculator_app: ExpenseCalcApp::default(),
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
                // ctx.set_pixels_per_point(2.0);
                egui::widgets::global_dark_light_mode_switch(ui);

                ui.separator();
                egui::ScrollArea::horizontal().show(ui, |ui| {
                    ui.selectable_value(
                        &mut self.current_window,
                        Window::About,
                        RichText::new("👨 About Me").size(self.main_menu_size),
                    );
    
                    ui.selectable_value(
                        &mut self.current_window,
                        Window::Projects,
                        RichText::new("🎮 Projects").size(self.main_menu_size),
                    );
    
                    ui.selectable_value(
                        &mut self.current_window,
                        Window::GraphingCalculator,
                        RichText::new("📈 Graphing Calculator").size(self.main_menu_size),
                    );
    
                    ui.selectable_value(
                        &mut self.current_window,
                        Window::NoteMaker,
                        RichText::new("📝 Notemaker ").size(self.main_menu_size),
                    );
    
                    ui.selectable_value(
                        &mut self.current_window,
                        Window::ExpenseCalculator,
                        RichText::new("💸 Expense Calculator ").size(self.main_menu_size),
                    );

                    ui.hyperlink_to(
                        RichText::new("📄 Resume").size(self.main_menu_size),
                        "https://github.com/Saphereye/resume-and-details/files/12909438/Adarsh_resume.1._compressed.pdf",
                    );
                });
                    
            });
            // ui.add_space(20.0);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.current_window {
                Window::About => {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        ui.heading(RichText::new("About Me").size(40.0));
                        ui.label(
                            RichText::new(
                                "A passionate developer interested in all things computers",
                            )
                            .italics()
                            .size(20.0),
                        );
                        ui.add_space(10.0);
                        ui.label(
                            RichText::new("I am Adarsh Das, currently studying at BITS, Hyd")
                                .size(20.0),
                        )
                    });
                }
                Window::Projects => {
                    let mut project_app = ProjectsApp::default();
                    project_app.add_project(Project {
                        name: "Dicey Fate 2.0".to_string(),
                        image: "skdjfh".to_string(),
                        description: "skdjfhsjkdfh".to_string(),
                        year: 2021,
                    });
                    ui.heading("Projects");
                    ui.label(format!("{:?}", project_app.projects));
                }
                Window::NoteMaker => {
                    ui.heading("NoteMaker");
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
                Window::GraphingCalculator => {
                    egui::SidePanel::right("graph_panel")
                        .width_range(Rangef { min: 500.0, max: 800.0  })
                        .show(ctx, |ui| {
                            self.graphing_calculator_app.ui(ui);
                        });

                    egui::CentralPanel::default().show(ctx, |ui| {
                        // self.graphing_calculator_app.equations.push("69420".to_string());
                        ui.heading("Equations");
                        // ui.horizontal(|ui| {
                        //     ui.text_edit_singleline(&mut self.graphing_calculator_app.equations[0]);
                        //     // ui.text_edit_singleline(&mut self.graphing_calculator_app.equations[1]);
                        // });
                        ui.vertical(|ui| {
                            for equation in self.graphing_calculator_app.equations.iter_mut() {
                                ui.text_edit_singleline(equation);
                            }
                        });
                        ui.heading(format!("{:?}", self.graphing_calculator_app.equations));
                        ui.ctx().request_repaint();
                        // ui.heading(self.graphing_calculator_app.equations[1].clone());
                    });
                }
                Window::ExpenseCalculator => {
                    ui.heading("Expense Calculator");

                    if ui.text_edit_multiline(&mut self.expense_calculator_app.input).changed() {
                        let _ = self.expense_calculator_app.parse_data_from_input();
                    }

                    ui.heading(format!("{:?}", self.expense_calculator_app.contributions));
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
