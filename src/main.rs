mod app;

use app::*;
use leptos::{logging, mount_to_body};

pub fn main() {
    console_error_panic_hook::set_once();
    logging::log!("csr mode - mounting to body");
    mount_to_body(App);
}
