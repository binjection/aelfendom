extern crate staticfile;
extern crate mount;
extern crate iron;

use std::env;
use std::path::Path;
use staticfile::Static;
use mount::Mount;
use iron::Iron;

fn main() {
    let mut mount = Mount::new();
    let root = env::var("ROOT").unwrap();
    mount.mount("/", Static::new(Path::new(&*root).join("html")));
    Iron::new(mount).http("0.0.0.0:80").unwrap();
}
