mod app;

use crate::app::{App};
use crate::app::screen::TermionScreen;
use crate::app::view::list::List;


fn main() {
    let list = List::new();
    let mut app = App::new(TermionScreen::new().unwrap(), list);
    app.run().unwrap();
}
