use gtk::prelude::*;
use gio::prelude::*;

fn build_ui(gtkapp: &gtk::Application) {
    let file = include_str!("main.glade");
    let gt = gtk::Builder::from_string(file);

    let app: gtk::ApplicationWindow = gt.get_object("rodovia.aarchitect").unwrap();
    let connect_button: gtk::Button = gt.get_object("connect_button_button").unwrap();

    connect_button.connect("clicked", true, |val| {
        connect_step();
        None
    });

    app.set_application(Some(gtkapp));
    app.show_all();
}

fn connect_step() {

}

fn main() {
    let ap = gtk::Application::new(Some("rodovia.aarchitect"),
        Default::default())
        .unwrap();
    ap.connect_startup(|app| {
        build_ui(app);
    });

    ap.run(&std::env::args().collect::<Vec<_>>());
}
