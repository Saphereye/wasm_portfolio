use std::{f64::consts::TAU, fmt::format};
use egui::*;


use egui_plot::{
    Arrows, AxisBools, AxisHints, Bar, BarChart, BoxElem, BoxPlot, BoxSpread, CoordinatesFormatter,
    Corner, GridInput, GridMark, HLine, Legend, Line, LineStyle, MarkerShape, Plot, PlotImage,
    PlotPoint, PlotPoints, PlotResponse, Points, Polygon, Text, VLine,
};

#[derive(serde::Deserialize, serde::Serialize, PartialEq)]
enum Window {
    About,
    Resume,
    Projects,
    GraphingCalculator,
}

#[derive(serde::Deserialize, serde::Serialize)]
struct GraphingCalculatorApp {
    circle_radius: f64,
    circle_center: Pos2,
    time: f64,
    // line_style: LineStyle,
    animate: bool,
    show_axes: bool,
    show_grid: bool,

    square: bool,
    proportional: bool,

    coordinates: bool,

    equations: Vec<String>,
}

impl Default for GraphingCalculatorApp {
    fn default() -> Self {
        GraphingCalculatorApp {
            circle_center: Pos2 { x: 0.0, y: 0.0 },
            circle_radius: 1.5,
            time: 31.4,
            // line_style: LineStyle::Solid,
            animate: false,
            show_axes: true,
            show_grid: true,
            square: false,
            proportional: true,
            coordinates: true,
            equations: vec!["sin(x)".to_string(), "cos(x)".to_string()],
        }
        // Self { circle_radius: Default::default(), circle_center: Default::default(), time: Default::default(), line_style: Default::default(), animate: Default::default(), show_axes: Default::default(), show_grid: Default::default(), square: Default::default(), proportional: Default::default(), coordinates: Default::default(), equations: Default::default() }
    }
}

impl GraphingCalculatorApp {
    fn circle(&self) -> Line {
        let n = 512;
        let circle_points: PlotPoints = (0..=n)
            .map(|i| {
                let t = remap(i as f64, 0.0..=(n as f64), 0.0..=TAU);
                let r = self.circle_radius;
                [
                    r * t.cos() + self.circle_center.x as f64,
                    r * t.sin() + self.circle_center.y as f64,
                ]
            })
            .collect();
        Line::new(circle_points)
            .color(Color32::from_rgb(100, 200, 100))
            // .style(self.line_style)
            .name("circle")
    }

    fn sin(&self) -> Line {
        let time = self.time;
        Line::new(PlotPoints::from_explicit_callback(
            move |x| 0.5 * (2.0 * x).sin() * time.sin(),
            ..,
            512,
        ))
        .color(Color32::from_rgb(200, 100, 100))
        // .style(self.line_style)
        .name("sin")
    }

    fn thingy(&self) -> Line {
        let time = self.time;
        Line::new(PlotPoints::from_parametric_callback(
            move |t| ((2.0 * t + time).sin(), (3.0 * t).sin()),
            0.0..=TAU,
            256,
        ))
        .color(Color32::from_rgb(100, 150, 250))
        // .style(self.line_style)
        .name("x = sin(2t), y = sin(3t)")
    }

    fn ui(&mut self, ui: &mut Ui) -> Response {
        if self.animate {
            ui.ctx().request_repaint();
            self.time += ui.input(|i| i.unstable_dt).at_most(1.0 / 30.0) as f64;
            // self.time += 0.1;
        };
        let mut plot = Plot::new("lines_demo")
            .legend(Legend::default())
            .y_axis_width(4)
            .show_axes(self.show_axes)
            .show_grid(self.show_grid);
        if self.square {
            plot = plot.view_aspect(1.0);
        }
        if self.proportional {
            plot = plot.data_aspect(1.0);
        }
        if self.coordinates {
            plot = plot.coordinates_formatter(Corner::LeftBottom, CoordinatesFormatter::default());
        }
        plot.show(ui, |plot_ui| {
            plot_ui.line(self.circle());
            plot_ui.line(self.sin());
            plot_ui.line(self.thingy());
        })
        .response
    }

    fn parse_equation(&self) {
        todo!();
    }
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
}

impl Default for Website {
    fn default() -> Self {
        Self {
            label: "Adarsh Das".to_owned(),
            value: 0.0,
            current_window: Window::About,
            graphing_calculator_app: GraphingCalculatorApp::default(),
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
                ui.selectable_value(&mut self.current_window, Window::About, "👨 About");
                ui.selectable_value(&mut self.current_window, Window::Resume, "📄 Resume");
                ui.selectable_value(&mut self.current_window, Window::Projects, "🚀 Projects");
                ui.selectable_value(
                    &mut self.current_window,
                    Window::GraphingCalculator,
                    "📈 Graphing Calculator",
                );
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
                Window::GraphingCalculator => {
                    egui::SidePanel::right("graph_panel")
                        .default_width(50.0)
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
