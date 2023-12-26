enum TextType {
    Heading,
    SubHeading,
    Paragraph,
    Project,
}

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
    main_menu_size: f32,
    // #[serde(skip)]
    // http_app: HttpApp,
}

impl Default for Website {
    fn default() -> Self {
        Self {
            label: "Adarsh Das".to_owned(),
            main_menu_size: 17.0,
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
        let mut window_width = 0.0;

        egui::TopBottomPanel::top("quote").show(ctx, |ui| {
            window_width = ui.available_width();
            ui.set_width(277.4);
            ui.set_min_width(277.4);
            ui.horizontal(|ui| {
                egui::widgets::global_dark_light_mode_switch(ui);
                ui.vertical_centered(|ui| {
                    ui.heading(
                        add_text(TextType::Heading, "Reden ist Silber, Schweigen ist Gold")
                            .italics(),
                    );
                    ui.heading(
                        add_text(
                            TextType::Paragraph,
                            "- Büchmann, Georg (1895). Geflügelte Worte",
                        )
                        .italics(),
                    );
                });
                ui.add_space(10.0);
            });
        });

        let mut about_me_button: Option<egui::Response> = None;
        let mut education_button: Option<egui::Response> = None;
        let mut strengths_button: Option<egui::Response> = None;
        let mut coursework_button: Option<egui::Response> = None;
        let mut research_projects_button: Option<egui::Response> = None;
        let mut work_experience_button: Option<egui::Response> = None;
        let mut personal_projects_button: Option<egui::Response> = None;

        if window_width > 972.0 {
            egui::SidePanel::left("table_of_contents").show(ctx, |ui| {
                // ui.heading(format!("{:?}", ui.available_size()));
                ui.heading(add_text(TextType::SubHeading, "Table of Contents"));
                about_me_button = Some( ui.link(add_text(TextType::Paragraph, "- About Me")));
                education_button = Some(ui.link(add_text(TextType::Paragraph, "- Education")));
                strengths_button = Some(ui.link(add_text(TextType::Paragraph, "- Strengths")));
                coursework_button = Some(ui.link(add_text(TextType::Paragraph, "- Coursework")));
                research_projects_button = Some(ui.link(add_text(TextType::Paragraph, "- Research Projects")));
                work_experience_button = Some(ui.link(add_text(TextType::Paragraph, "- Work Experience, Competetions and Club activities")));
                personal_projects_button = Some(ui.link(add_text(TextType::Paragraph, "- Hobby Projects")));
                ui.separator();

                egui::ScrollArea::vertical().show(ui, |ui| {
                    //
                    // Contact Me
                    //
                    ui.vertical(|ui| {
                        ui.set_max_width(ui.available_width());

                        ui.heading(add_text(TextType::SubHeading, "Contact Me"));
                        add_custom_hyperlink(ui, add_text(TextType::Paragraph, "Email: adarshdas950@gmail.com"), "mailto:adarshdas950@gmail.com"); ui.add_space(10.0);
                        add_custom_hyperlink(ui, add_text(TextType::Paragraph, "Phone Number: +91 85278 59660"), "tel:+918527859660"); ui.add_space(10.0);
                        add_custom_hyperlink(ui, add_text(TextType::Paragraph, "Github"), "https://github.com/Saphereye"); ui.add_space(10.0);
                        add_custom_hyperlink(ui, add_text(TextType::Paragraph, "Linkedin"), "https://www.linkedin.com/in/adarsh-das-8684ab240/"); ui.add_space(10.0);
                        add_custom_hyperlink(ui, add_text(TextType::Paragraph, "Résumé"), "https://drive.google.com/file/d/1MtJ129Ipuvl9u7SHRu3e2Ug_oekiMDBY/view?usp=sharing");
                        ui.separator();
                        ui.add_space(10.0);

                        ui.heading(add_text(TextType::SubHeading, "Linguistic Proficiency"));
                        ui.label(add_text(TextType::Paragraph, "English (A1)"));
                        ui.label(add_text(TextType::Paragraph, "German (B2)"));
                        ui.label(add_text(TextType::Paragraph, "Hindi (Native)"));
                        ui.add_space(10.0);
                        ui.separator();

                        ui.add_space(10.0);
                        ui.heading(add_text(TextType::SubHeading, "Misc."));
                        add_custom_hyperlink(ui, add_text(TextType::Paragraph, "Favorite Fungi: Spongiforma squarepantsii"), "https://en.wikipedia.org/wiki/Spongiforma_squarepantsii");
                        add_custom_hyperlink(ui, add_text(TextType::Paragraph, "Favorite Insect: Aha ha"), "https://en.wikipedia.org/wiki/Aha_ha");
                    });
                });
            });
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            egui_extras::install_image_loaders(ctx);
            egui::ScrollArea::vertical().show(ui, |ui| {
                //
                // About Me
                //
                ui.vertical(|ui| {
                    // ui.set_max_width(ui.available_width() * 0.5);

                    let response = ui.heading(add_text(TextType::Heading, "About Me"));
                    if let Some(about_me_button) = about_me_button {
                        if about_me_button.clicked() {
                            response.scroll_to_me(Some(egui::Align::Min));
                        }
                    }
                    ui.horizontal_wrapped(|ui| {
                        // ui.set_max_width(ui.available_width() * 0.5);
                        ui.label(add_text(TextType::Paragraph, "Hi, I am"));
                        // pronunciation: [ɑːˈd̪ɐɾ.ɕ/]
                        ui.label(add_text(TextType::Paragraph, "Adarsh Das").underline());
                        ui.label(add_text(TextType::Paragraph, ", a third year undergraduate student at BITS Pilani, Hyderabad Campus. I am a passionate programmer and a tech enthusiast. I am also a member of the"));
                    });
                    ui.label(add_text(TextType::Paragraph, "Furthermore, I am self-motivated, enthusiastic, reliable and a responsible team-spirited person with a strong foundation in ethics."));

                    add_custom_hyperlink(ui, add_text(TextType::Paragraph, "Résumé"), "https://drive.google.com/file/d/1MtJ129Ipuvl9u7SHRu3e2Ug_oekiMDBY/view?usp=sharing");

                    ui.separator();
                    ui.add_space(10.0);

                    let response = ui.heading(add_text(TextType::Heading, "Education"));
                    if let Some(education_button) = education_button {
                        if education_button.clicked() {
                            response.scroll_to_me(Some(egui::Align::Min));
                        }
                    }
                    ui.horizontal_wrapped(|ui| {
                        ui.label(add_text(TextType::Paragraph, "Currently pursuing my"));
                        ui.label(add_text(TextType::Paragraph, "B.E. Hons in Computer Science and Minor in Data Science").underline(),);
                        ui.label(add_text(TextType::Paragraph, "from"));
                        ui.label(add_text(TextType::Paragraph, "Birla Institute of Technology and Science, Hyderabad Campus.").underline(),);
                    });
                    ui.label(add_text(TextType::Paragraph, "I am currently in my third year of study."));
                    ui.add_space(10.0);
                    ui.separator();

                    let response = ui.heading(add_text(TextType::Heading, "Strengths"));
                    if let Some(strengths_button) = strengths_button {
                        if strengths_button.clicked() {
                            response.scroll_to_me(Some(egui::Align::Min));
                        }
                    }
                    ui.horizontal_wrapped(|ui| {
                        let _ = ui.button(add_text(TextType::Paragraph, "Python"));
                        let _ = ui.button(add_text(TextType::Paragraph, "C++"));
                        let _ = ui.button(add_text(TextType::Paragraph, "C"));
                        let _ = ui.button(add_text(TextType::Paragraph, "Git"));
                        let _ = ui.button(add_text(TextType::Paragraph, "Godot"));
                        let _ = ui.button(add_text(TextType::Paragraph, "OpenGL"));
                        let _ = ui.button(add_text(TextType::Paragraph, "Java"));
                        let _ = ui.button(add_text(TextType::Paragraph, "Rust"));
                        let _ = ui.button(add_text(TextType::Paragraph, "Sklearn"));
                        let _ = ui.button(add_text(TextType::Paragraph, "Pandas"));
                        let _ = ui.button(add_text(TextType::Paragraph, "Django"));
                    });
                    ui.add_space(10.0);
                    ui.separator();

                    let response = ui.heading(add_text(TextType::Heading, "Coursework"));
                    if let Some(coursework_button) = coursework_button {
                        if coursework_button.clicked() {
                            response.scroll_to_me(Some(egui::Align::Min));
                        }
                    }
                    ui.horizontal_wrapped(|ui| {
                        ui.set_max_width(ui.available_width());
                        let _ = ui.button(add_text(TextType::Paragraph, "Data Structures and Algorithms"));
                        let _ = ui.button(add_text(TextType::Paragraph, "Database Systems"));
                        let _ = ui.button(add_text(TextType::Paragraph, "Object Oriented Programming"));
                        let _ = ui.button(add_text(TextType::Paragraph, "Operating Systems"));
                        let _ = ui.button(add_text(TextType::Paragraph, "Computer Architecture"));
                        let _ = ui.button(add_text(TextType::Paragraph, "Deep Learning"));
                        let _ = ui.button(add_text(TextType::Paragraph, "Machine Learning"));
                        let _ = ui.button(add_text(TextType::Paragraph, "Artificial Intelligence"));
                        let _ = ui.button(add_text(TextType::Paragraph, "Image Processing"));
                        let _ = ui.button(add_text(TextType::Paragraph, "Theory of Computation"));
                        let _ = ui.button(add_text(TextType::Paragraph, "Principles of programming languages"));
                        let _ = ui.button(add_text(TextType::Paragraph, "Discrete Structure in Computer Science"));
                        let _ = ui.button(add_text(TextType::Paragraph, "Logic in Computer Science"));
                        let _ = ui.button(add_text(TextType::Paragraph, "Digital Design"));
                        let _ = ui.button(add_text(TextType::Paragraph, "Computer Programming"));
                        let _ = ui.button(add_text(TextType::Paragraph, "Probability and Statistics"));
                        let _ = ui.button(add_text(TextType::Paragraph, "Linear Algebra"));
                        let _ = ui.button(add_text(TextType::Paragraph, "Differential Calculus"));
                        let _ = ui.button(add_text(TextType::Paragraph, "Principles of Management"));
                        let _ = ui.button(add_text(TextType::Paragraph, "Technical Report Writing"));

                    });
                    ui.add_space(10.0);
                    ui.separator();

                    //
                    // Projects
                    //
                    let response = ui.heading(add_text(TextType::Heading, "Research Projects"));
                    if let Some(research_projects_button) = research_projects_button {
                        if research_projects_button.clicked() {
                            response.scroll_to_me(Some(egui::Align::Min));
                        }
                    }
                    add_project(ui, "Chess AI comparative analysis", "Aimed to explore search algorithms to create a novel chess engine. We use python3.10 programming language and chess module as an interace for handling the board. Furthermore chessboard library was used for gui display.", Some("https://github.com/Saphereye/ChessAI"), Some(egui::include_image!("../assets/projects/chess.png")));
                    add_project(ui, "Malaria Cell classification using state-of-the-art Vision Tranformer", "The project utilized vision transformer trained on various processed images of the training data such as green channel, green channle canny filtered and klahe filter. The individual models where then combined using a ensemble methods. The validation set gave 99.7% accuracy and the testing accuracy was ~94%", None, Some(egui::include_image!("../assets/projects/ip.png")));
                    add_project(ui, "Pneumonia diagnosis using chest X-ray", "The project leveraged vision transformers architecture for pneumonia diagnosis. The project also included implementing methods for improving upon the research paper on which it was implemented", None, Some(egui::include_image!("../assets/projects/dl.png")));

                    let response = ui.heading(add_text(TextType::Heading, "Work Experience, Competetions and Club activities"));
                    if let Some(work_experience_button) = work_experience_button {
                        if work_experience_button.clicked() {
                            response.scroll_to_me(Some(egui::Align::Min));
                        }
                    }
                    add_project(ui, "BC6 data analysis", "This was a project for my research internship at NCPOR, Goa. The project was made using Django. It supports a step by step research submission portal and features such as email verification for proposal acceptance. It also includes a page for visualizing BC6 carbon data.", Some("https://github.com/Saphereye/ncpor-portal-ps2"), Some(egui::include_image!("../assets/projects/data.png")));
                    add_project(ui, "ServiQuick: One touch emergency services app", "ServiQuik is a user-friendly mobile application designed to provide swift access to emergency services. With just a few taps, you can call for immediate assistance from hospitals, fire stations, or the police. The app employs Text-to-Speech (TTS) technology to convey essential information about the nearest service of your choice and provides a convenient route on the map for your destination.", Some("https://github.com/Saphereye/ServiQuick"), Some(egui::include_image!("../assets/projects/serviquick.png")));
                    add_project(ui, "Article: Free Software Movement and its Importance in the Modern World", "In today’s digital world, every aspect of our lives is intertwined with computers. A “computer” will play a considerable part in official work or leisure activities. With such a significant dependence on this technology, the idea of not having control over what we use is absurd. This is, sadly, the current situation with proprietary software. This article explores this problem in detail", Some("https://csabitsh.wordpress.com/2022/10/15/free-software-movement-and-its-importance-in-the-modern-world/"), Some(egui::include_image!("../assets/projects/fsm.webp")));
                    add_project(ui, "Article: A Brief History of Computer Graphics", "Millions of people watch movies every year, marveling at the impeccable CGI (Computer-generated imagery). According to some studies, teens use their phones for an average of about 8 hours a week, surfing social media and popular websites like YouTube. Knowingly or unknowingly, computer graphics is inherent everywhere around us. Consumers often overlook how much computer graphics is part of their lives, from bringing their favorite characters to life to providing realistic simulations. This article explores this topic in detail", Some("https://csabitsh.wordpress.com/a-brief-history-of-computer-graphics/"), Some(egui::include_image!("../assets/projects/graphics2.jpg")));
                    add_project(ui, "Dockerized E-Commerce with Spring Security and React.js", "A ready-to-scale, docker-ized web application that supported placing orders, order status, inventory management, Admin, Manager, and User functionality using and authentication using Spring Security and MySQL coupled with a React.js based frontend. Utilized a custom-made system for user payments and had coupon code functionality and email notification", None, None);
                    add_project(ui, "Graph Based Database Architecture in Multithreaded C", "A C based multithreaded graph database system which supported multiple concurrent requests for writing, reading and traversing the graphs in the database using DFS and BFS", Some("https://github.com/Divyateja04/ClientServer_CSF372"), None);
                    add_project(ui, "Handouts For You", "A dedicated website to facilitate the accessibility of almost 2000+ course handouts, expanded the website's functionality by incorporating features for sharing notes, resources, and questions related to the campus, and implemented a course review system, notes sharing system and CGPA cutoff system with 4000+ students handling 1000+ requests per day.", Some("https://handoutsforyou.vercel.app/"), None);

                    let response = ui.heading(add_text(TextType::Heading, "Hobby Projects"));
                    if let Some(personal_projects_button) = personal_projects_button {
                        if personal_projects_button.clicked() {
                            response.scroll_to_me(Some(egui::Align::Min));
                        }
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

                    if window_width < 972.0 {
                        ui.heading(add_text(TextType::Heading, "Contact Me"));
                        add_custom_hyperlink(ui, add_text(TextType::Paragraph, "Email: adarshdas950@gmail.com"), "mailto:adarshdas950@gmail.com");
                        ui.add_space(10.0);
                        add_custom_hyperlink(ui, add_text(TextType::Paragraph, "Phone Number: +91 85278 59660"), "tel:+918527859660");
                        ui.add_space(10.0);
                        add_custom_hyperlink(ui, add_text(TextType::Paragraph, "Github"), "https://github.com/Saphereye");
                        ui.add_space(10.0);
                        add_custom_hyperlink(ui, add_text(TextType::Paragraph, "Linkedin"), "https://www.linkedin.com/in/adarsh-das-8684ab240/");
                        ui.add_space(10.0);
                        add_custom_hyperlink(ui, add_text(TextType::Paragraph, "Résumé"), "https://drive.google.com/file/d/1MtJ129Ipuvl9u7SHRu3e2Ug_oekiMDBY/view?usp=sharing");
                    }
                });
            })
        });
    }
}

fn add_text(text_type: TextType, text: &str) -> egui::RichText {
    match text_type {
        TextType::Heading => egui::RichText::new(text).size(35.0),
        TextType::SubHeading => egui::RichText::new(text).size(25.0),
        TextType::Paragraph => egui::RichText::new(text).size(20.0),
        TextType::Project => egui::RichText::new(text).size(30.0),
    }
}

fn add_custom_hyperlink(ui: &mut egui::Ui, text: egui::RichText, url: &str) -> egui::Response {
    ui.add(egui::Hyperlink::from_label_and_url(text, url).open_in_new_tab(true))
}

fn add_project(
    ui: &mut egui::Ui,
    name: &str,
    description: &str,
    source_code_link: Option<&str>,
    image: Option<egui::ImageSource<'_>>,
) {
    // Image display on terminal
    ui.add_space(10.0);
    if let Some(source_code_link) = source_code_link {
        ui.hyperlink_to(add_text(TextType::Project, name), source_code_link);
    } else {
        ui.heading(add_text(TextType::Project, name));
    }
    ui.add_space(10.0);
    ui.label(add_text(TextType::Paragraph, description));
    ui.add_space(10.0);
    if let Some(image) = image {
        ui.add(egui::Image::new(image).fit_to_exact_size(egui::Vec2 {
            x: ui.available_width(),
            y: 500.0,
        }));
    }
    ui.add_space(10.0);
    ui.separator();
}
