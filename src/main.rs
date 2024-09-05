use gtk::prelude::*;
use gtk::{glib, Application};
mod main_window;

const APP_ID: &str = "org.gtk_rs.HelloWorld2";

fn main() -> glib::ExitCode 
{
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(main_window::build_ui);

    // Run the application
    app.run()
}
