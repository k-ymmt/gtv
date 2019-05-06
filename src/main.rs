mod app;

use crate::app::App;
use crate::app::screen::TermionScreen;


fn main() {
    let app = App::new(TermionScreen::new().unwrap());
    let _ = app.run();
}
