use gtk::prelude::*;
use gtk::{ApplicationWindow, Application};

pub fn build_ui(app: &Application) 
{
    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Cube")
        .height_request(640)
        .width_request(960)
        .build();

    // Present window
    window.present();
}