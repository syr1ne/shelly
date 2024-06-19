use gtk::prelude::*;

fn on_activate(application: &gtk::Application) {
  let window = gtk::ApplicationWindow::new(application);
  window.set_default_size(900, 600);
  window.set_size_request(320, 240);
  window.present();
}

fn main() {
  let app = gtk::Application::builder().build();
  app.connect_activate(on_activate);
  app.run();
}