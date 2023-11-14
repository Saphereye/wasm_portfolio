use egui::*;

use std::f64::consts::TAU;

use egui_plot::{CoordinatesFormatter, Corner, Legend, Line, Plot, PlotPoints};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct GraphingCalculatorApp {
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

    pub equations: Vec<String>,
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
    pub fn circle(&self) -> Line {
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

    pub fn sin(&self) -> Line {
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

    pub fn thingy(&self) -> Line {
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

    pub fn ui(&mut self, ui: &mut Ui) -> Response {
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

    /*
    pub fn parse_equation(&self) {
        todo!();
    }
    */
}
