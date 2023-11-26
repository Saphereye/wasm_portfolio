// use egui::{self, CentralPanel, Context, Image, TextStyle, TopBottomPanel};
// use ehttp::Request;
// use poll_promise::{Promise, Sender};

// // Minimal representation of an HTTP response.
// struct HttpResponse {
//     url: String,
//     status: u16,
//     content_type: String,
//     text: Option<String>,
// }

// // Minimal representation of an HTTP application state.
// pub struct HttpApp {
//     pub url: String,
//     pub promise: Option<Promise<String>>,
//     pub response: Option<String>,
//     pub sender: Option<Sender<String>>,
// }

// impl Default for HttpApp {
//     fn default() -> Self {
//         let (sender, promise) = Promise::new();
//         Self {
//             url: "https://www.skjdfhgsdkjhf.com".to_owned(),
//             promise: Some(promise),
//             response: None,
//             sender: Some(sender),
//         }
//     }
// }

// impl HttpApp {
//     pub fn fetch_data(&mut self, ui: &mut egui::Ui) {
//         ui.heading("Starting now");
//         let request = ehttp::Request::get(&self.url);
//         ui.heading("Sending request");
//         ehttp::fetch(request, move |result: ehttp::Result<ehttp::Response>| {
//             let output = match result {
//                 Ok(response) => format!("{:?}", response),
//                 Err(error) => format!("Error: {:?}", error),
//             };
//             self.sender.as_ref().unwrap().send(output);
//         });
//         ui.heading("Request received ???");
//     }
//   }
