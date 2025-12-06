use crate::app::App;
use crate::open::Open;

mod app;
mod open;

fn main() {
    let finder = App {
        bundle_id: "com.apple.Finder".to_string(),
    };
    finder.open().unwrap();
}
