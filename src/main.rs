use session::Session;

use crate::config::TomatoConfig;

mod config;
mod session;

fn main() {
    let config = TomatoConfig::new(25, 5, 15);
    let mut session = Session::new(config);
    session.run();
}
