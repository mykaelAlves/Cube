use gtk::{prelude::*, Button};
use gtk::{ApplicationWindow, Application};

use crate::server_logic;

pub fn build_ui(app: &Application) 
{
    let button_initiate_server = Button::builder()
        .label("Iniciar servidor")
        .margin_bottom(500)
        .build();

    let main_window = ApplicationWindow::builder()
        .application(app)
        .title("Cube")
        .height_request(640)
        .width_request(960)
        .child(&button_initiate_server)
        .build();

    button_initiate_server.connect_clicked(|_|
    {
        server_logic::init_server();
    });

    main_window.present();
}