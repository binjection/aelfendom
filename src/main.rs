extern crate staticfile;
extern crate mount;
extern crate iron;

use std::path::Path;
use staticfile::Static;
use mount::Mount;
use iron::Iron;

fn main() {
    let mut mount = Mount::new();
    mount.mount("/", Static::new(Path::new("public")));
    Iron::new(mount).http("0.0.0.0:80").unwrap();
}
