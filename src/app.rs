use std::{default, fmt::format, task::Poll};

use egui::{*, text::LayoutJob};
use egui_extras::{Column, TableBuilder};

mod expense_calculator;
mod graphing_calculator;
mod http_app;

use expense_calculator::*;
use graphing_calculator::*;
use http_app::*;
use poll_promise::Promise;

#[derive(serde::Deserialize, serde::Serialize, PartialEq)]
enum Window {
    About,
    Projects,
    GraphingCalculator,
    NoteMaker,
    ExpenseCalculator,
    Resume,
    // Http,
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

    expense_calculator_app: ExpenseCalculatorApp,
    // #[serde(skip)]
    // http_app: HttpApp,
}

impl Default for Website {
    fn default() -> Self {
        Self {
            label: "Adarsh Das".to_owned(),
            value: 0.0,
            current_window: Window::About,
            main_menu_size: 17.0,
            graphing_calculator_app: GraphingCalculatorApp::default(),
            expense_calculator_app: ExpenseCalculatorApp::default(),
            // http_app: HttpApp::default(),
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
        egui_extras::install_image_loaders(ctx);
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                egui::widgets::global_dark_light_mode_switch(ui);
                ui.vertical_centered(|ui| {
                    // ui.set_max_width(ui.available_height());
                    ui.heading(RichText::new("Reden ist Silber, Schweigen ist Gold").size(35.0).italics());
                    ui.heading(RichText::new("- Büchmann, Georg (1895). Geflügelte Worte").size(18.0).italics());
                });
                
                ui.heading(RichText::new("Table of Contents").size(35.0));
                let about_me_button =  ui.link(RichText::new("About Me").size(18.0));
                let education_button = ui.link(RichText::new("Education").size(18.0));
                let strengths_button = ui.link(RichText::new("Strengths").size(18.0));
                let coursework_button = ui.link(RichText::new("Coursework").size(18.0));
                let research_projects_button = ui.link(RichText::new("Research Projects").size(18.0));
                let work_experience_button = ui.link(RichText::new("Work Experience").size(18.0));
                let personal_projects_button = ui.link(RichText::new("Personal Projects").size(18.0));
                let contact_me_button = ui.link(RichText::new("Contact Me").size(18.0));
                ui.separator();
                //
                // About Me
                //
                ui.vertical(|ui| {
                    ui.set_max_width(ui.available_width() * 0.5);

                    let response = ui.heading(RichText::new("About Me").size(35.0));
                    if about_me_button.clicked() {
                        response.scroll_to_me(Some(Align::Min));
                    }
                    ui.horizontal_wrapped(|ui| {
                        // ui.set_max_width(ui.available_width() * 0.5);
                        ui.label(RichText::new("I am").size(18.0));
                        ui.label(RichText::new("Adarsh Das").size(18.0).underline());
                        ui.label(RichText::new(", an inquisitive aspiring software developer with an interest in all ﬁelds of computer science ranging from the mathematical foundations to graphics.").size(18.0));
                    });
                    ui.label(RichText::new("Furthermore, I am self-motivated, enthusiastic, reliable and a responsible team-spirited person with a strong foundation in ethics.").size(18.0));
                    
                    ui.hyperlink_to("Résumé", "https://drive.google.com/file/d/1TnOysGFb8FreWxzyTqyW_RSVO3QrxpFR/view");


                    ui.separator();
                    ui.add_space(10.0);

                    let reponse = ui.heading(RichText::new("Education").size(35.0));
                    if education_button.clicked() {
                        reponse.scroll_to_me(Some(Align::Min));
                    }
                    ui.horizontal_wrapped(|ui| {
                        ui.label(RichText::new("Currently pursuing my",).size(18.0),);
                        ui.label(RichText::new("B.E. Hons in Computer Science and Minor in Data Science",).size(18.0).underline(),);
                        ui.label(RichText::new("from",).size(18.0),);
                        ui.label(RichText::new("Birla Institute of Technology and Science, Hyderabad Campus.",).size(18.0).underline(),);
                    });
                    ui.label(RichText::new("I am currently in my third year of study.").size(18.0));
                    ui.add_space(10.0);
                    ui.separator();

                    let response = ui.heading(RichText::new("Strengths").size(35.0));
                    if strengths_button.clicked() {
                        response.scroll_to_me(Some(Align::Min));
                    }
                    ui.horizontal(|ui| {
                        let _ = ui.button(RichText::new("Python").size(18.0));
                        let _ = ui.button(RichText::new("C++").size(18.0));
                        let _ = ui.button(RichText::new("C").size(18.0));
                        let _ = ui.button(RichText::new("Git").size(18.0));
                        let _ = ui.button(RichText::new("Godot").size(18.0));
                        let _ = ui.button(RichText::new("OpenGL").size(18.0));
                        let _ = ui.button(RichText::new("Java").size(18.0));
                        let _ = ui.button(RichText::new("Rust").size(18.0));
                        let _ = ui.button(RichText::new("Sklearn").size(18.0));
                        let _ = ui.button(RichText::new("Pandas").size(18.0));
                        let _ = ui.button(RichText::new("Django").size(18.0));
                    });
                    ui.add_space(10.0);
                    ui.separator();

                    let response = ui.heading(RichText::new("Coursework").size(35.0));
                    if coursework_button.clicked() {
                        response.scroll_to_me(Some(Align::Min));
                    }
                    ui.horizontal_wrapped(|ui| {
                        ui.set_max_width(ui.available_width());
                        let _ = ui.button(RichText::new("Data Structures and Algorithms").size(18.0));
                        let _ = ui.button(RichText::new("Database Systems").size(18.0));
                        let _ = ui.button(RichText::new("Object Oriented Programming").size(18.0));
                        let _ = ui.button(RichText::new("Operating Systems").size(18.0));
                        let _ = ui.button(RichText::new("Computer Architecture").size(18.0));
                        let _ = ui.button(RichText::new("Deep Learning").size(18.0));
                        let _ = ui.button(RichText::new("Machine Learning").size(18.0));
                        let _ = ui.button(RichText::new("Artificial Intelligence").size(18.0));
                        let _ = ui.button(RichText::new("Image Processing").size(18.0));
                        let _ = ui.button(RichText::new("Theory of Computation").size(18.0));
                        let _ = ui.button(RichText::new("Principles of programming languages").size(18.0));
                        let _ = ui.button(RichText::new("Discrete Structure in Computer Science").size(18.0));
                        let _ = ui.button(RichText::new("Logic in Computer Science").size(18.0));
                        let _ = ui.button(RichText::new("Digital Design").size(18.0));
                        let _ = ui.button(RichText::new("Computer Programming").size(18.0));
                        let _ = ui.button(RichText::new("Probability and Statistics").size(18.0));
                        let _ = ui.button(RichText::new("Linear Algebra").size(18.0));
                        let _ = ui.button(RichText::new("Differential Calculus").size(18.0));
                        let _ = ui.button(RichText::new("Principles of Management").size(18.0));
                        let _ = ui.button(RichText::new("Technical Report Writing").size(18.0));

                    });
                    ui.add_space(10.0);
                    ui.separator();
                });

                ui.vertical(|ui| {
                    //
                    // Projects
                    //
                    let response = ui.heading(RichText::new("Research Projects").size(35.0));
                    if research_projects_button.clicked() {
                        response.scroll_to_me(Some(Align::Min));
                    }
                    add_project(ui, "Chess AI comparative analysis", "Aimed to explore search algorithms to create a novel chess engine. We use python3.10 programming language and chess module as an interace for handling the board. Furthermore chessboard library was used for gui display.", Some("https://github.com/Saphereye/ChessAI"), Some(egui::include_image!("../assets/projects/chess.png")));                                
                    // add DL, IP project also
                        
                    
                    let response = ui.heading(RichText::new("Work Experience").size(35.0));
                    if work_experience_button.clicked() {
                        response.scroll_to_me(Some(Align::Min));
                    }
                    add_project(ui, "BC6 data analysis", "This was a project for my research internship at NCPOR, Goa. The project was made using Django. It supports a step by step research submission portal and features such as email verification for proposal acceptance. It also includes a page for visualizing BC6 carbon data.", Some("https://github.com/Saphereye/ncpor-portal-ps2"), Some(egui::include_image!("../assets/projects/data.png")));
                    add_project(ui, "ServiQuick: One touch emergency services app", "ServiQuik is a user-friendly mobile application designed to provide swift access to emergency services. With just a few taps, you can call for immediate assistance from hospitals, fire stations, or the police. The app employs Text-to-Speech (TTS) technology to convey essential information about the nearest service of your choice and provides a convenient route on the map for your destination.", Some("https://github.com/Saphereye/ServiQuick"), Some(egui::include_image!("../assets/projects/serviquick.png")));

                    let response = ui.heading(RichText::new("Personal Projects").size(35.0));
                    if personal_projects_button.clicked() {
                        response.scroll_to_me(Some(Align::Min));
                    }
                    add_project(ui, "Image display on terminal", "This program addresses the challenge of displaying images in a terminal, which lacks the ability to render small pixels. It achieves this by pixelating the image and leveraging the terminal's color coding capabilities to provide a more accurate representation", Some("https://github.com/Saphereye/image-to-terminal"), Some(egui::include_image!("../assets/projects/imgterm.png")));
                    add_project(
                        ui,
                        "Brainfuck Interpreter", 
                        "Implemented a brainf*ck interpreter in Rust with the brain of the code in about 150 loc. Supports intuitive command line support. A toy project finished in two hours.\nThe project support improved versions also.\nAs a demo, for the below input", 
                        Some("https://github.com/Saphereye/brainfuck-interpreter"), 
                        Some(egui::include_image!("../assets/projects/brainfuck.png")
                    ));
                    add_project(ui, "Multipurpose Telegram Bot", "A personal telegram bot implemeted using teloxide library in rust. Supports a wide variety of toy features such as reporting the weather and sending cat pics. Sends a greeting at 8am everyday and can also jot down todos for every user.", Some("https://github.com/Saphereye/herr-jr"), Some(egui::include_image!("../assets/projects/herrjr.png")));
                    add_project(ui, "NES Emulator", "Implemented a an NES emulator in rust. Supports screen switching and input mapping.", Some("https://github.com/Saphereye/nes_emulator"), Some(egui::include_image!("../assets/projects/nes.png")));
                    // add_project(ui, "Lan based chatting application", "gg", Some("https://github.com/Saphereye/lan-chat"), None);  
                });

                //
                // Contact Me
                //
                ui.vertical(|ui| {
                    ui.set_max_width(ui.available_width());
                    
                    let response = ui.heading(RichText::new("Contact Me").size(35.0));
                    if contact_me_button.clicked() {
                        response.scroll_to_me(Some(Align::Min));
                    }
                    ui.hyperlink_to(RichText::new("Email: adarshdas950@gmail.com",).size(18.0), "mailto:adarshdas950@gmail.com");
                    ui.hyperlink_to(RichText::new("Phone Number: +91 85278 5966",).size(18.0), "tel:+91852785966");
                    ui.hyperlink_to(RichText::new("Résumé",).size(18.0), "https://drive.google.com/file/d/1TnOysGFb8FreWxzyTqyW_RSVO3QrxpFR/view");
                    ui.add_space(10.0);
                    ui.hyperlink_to(RichText::new("Favorite Fungi: Spongiforma squarepantsii",).size(18.0), "https://en.wikipedia.org/wiki/Spongiforma_squarepantsii");
                });
            })              
        });
    }
}

fn add_project(
    ui: &mut Ui,
    name: &str,
    description: &str,
    source_code_link: Option<&str>,
    image: Option<ImageSource<'_>>,
) {
    // Image display on terminal
    ui.add_space(10.0);
    ui.hyperlink_to(RichText::new(name).size(30.0), source_code_link.unwrap_or("#"));
    ui.add_space(10.0);
    ui.label(RichText::new(description).size(18.0));
    ui.add_space(10.0);
    if let Some(image) = image {
        ui.add(
            egui::Image::new(image)
                .fit_to_original_size(1.0)
                .max_width(ui.available_width() * 0.75),
        );
    }
    ui.add_space(10.0);
    // if let Some(source_code_link) = source_code_link {
    //     ui.hyperlink_to("Source Code", source_code_link);
    // }
    // ui.add_space(10.0);
    ui.separator();
}
