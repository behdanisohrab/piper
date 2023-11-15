use eframe::egui;
use egui::{Vec2, FontId, FontFamily, TextStyle};
use crate::utils::export_piper_link;

pub(crate) async fn run_gui(){
    let options = eframe::NativeOptions {
        always_on_top: false,
        maximized: false,
        decorated: true,
        drag_and_drop_support: false,
        icon_data: None,
        initial_window_pos: None,
        initial_window_size: Option::from(
            Vec2::new(500_f32, 150.0)
        ),
        min_window_size: Option::from(
            Vec2::new(500_f32, 150.0)
        ),
        max_window_size: Option::from(
            Vec2::new(500_f32, 150.0)
        ),
        resizable: false,
        transparent: false,
        vsync: false,
        multisampling: 0,
        depth_buffer: 0,
        stencil_buffer: 0,

        ..Default::default()
    };
    eframe::run_native(
        "Youtube -> Piper",
        options,
        Box::new(|cc| Box::new(MyApp::new(cc))),
    ).unwrap();

}


fn configure_text_styles(ctx: &egui::Context) {
    use FontFamily::{Monospace, Proportional};

    let mut style = (*ctx.style()).clone();
    style.text_styles = [
        (TextStyle::Heading, FontId::new(18.0, Proportional)),
        (TextStyle::Body, FontId::new(16.0, Proportional)),
        (TextStyle::Monospace, FontId::new(12.0, Monospace)),
        (TextStyle::Button, FontId::new(23.0, Proportional)),
        (TextStyle::Small, FontId::new(8.0, Proportional)),
    ]
        .into();
    ctx.set_style(style);
}
struct MyApp {
    yt_link:String,
    py_link:String
}
impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>)->Self {
        configure_text_styles(&cc.egui_ctx);
        Self {
            yt_link: "".to_string(),
            py_link:"".to_string()
        }
    }
}



impl eframe::App for MyApp {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal_centered(|ui|{
                ui.vertical_centered_justified (|ui|{
                    ui.add_space(15.0);
                    ui.text_edit_singleline(&mut(self.yt_link));
                    ui.add_space(9.0);
                    ui.vertical_centered(|ui|{
                        if ui.button("Conver Link").clicked() {

                            let youtube_url = self.yt_link.trim();
                            self.py_link = export_piper_link(youtube_url).unwrap_or("error!".to_string());

                        }
                    });

                    if !self.py_link.is_empty(){
                        ui.add_space(28.0);
                        if self.py_link=="error!" {
                            ui.label("error!");
                        }
                        else{
                            ui.hyperlink(self.py_link.clone());

                        }
                    }

                });


            });



        });
    }

}

