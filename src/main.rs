use open;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Builder, FileDialog, Label, gio, glib};
use glib::clone;

const APP_ID: &str = "org.keienb.file_dialog_demo";

fn main() -> glib::ExitCode {
    gio::resources_register_include!("org_keienb_file_dialog_demo.gresource")
        .expect("Failed to register resources.");
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_startup(setup_shortcuts);
    app.connect_activate(build_ui);
    app.run()
}

fn setup_shortcuts(app: &Application) {
    app.set_accels_for_action("win.open", &["<Ctrl>o"]);
    app.set_accels_for_action("window.close", &["<Ctrl>q"]);
}

fn build_ui(app: &Application) {
    let builder = Builder::from_resource("/org/keienb/file_dialog_demo/window.ui");
    let window :ApplicationWindow = builder.object("app_window")
        .expect("Failed to load application window from resource");
    setup_actions(&window);
    app.add_window(&window);
    window.present();
}

fn setup_actions(window: &ApplicationWindow) {
    let action_open = gio::SimpleAction::new("open", None);
    action_open.connect_activate(clone!(@weak window => move |_, _| {
        let file_dialog = FileDialog::builder().modal(false).build();
        file_dialog.open(Some(&window), None::<&gio::Cancellable>, clone!(@weak window => move |result| {
            let text :String;
            match result {
                 Ok(file) => {
                    let path = file.path().unwrap();
                    let _ = open::that(&path);
                    text = format!("Opening: {path:#?}")
                 },
                 Err(e) => text = format!("Error: {e:#?}")
            };
            let label = window.child()
                .unwrap()
                .downcast::<Label>()
                .unwrap();
            label.set_text(&text);
            println!("{}", text);
        }));
    }));
    window.add_action(&action_open);
}