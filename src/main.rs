use gtk::prelude::*;
use gtk::{glib, Application};
mod main_window;
mod server_logic;

const APP_ID: &str = "org.gtk_rs.HelloWorld2";

fn main() -> glib::ExitCode 
{
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(main_window::build_ui);
    app.run()
}
