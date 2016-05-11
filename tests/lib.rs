extern crate lightfield_loader;

use lightfield_loader::{Lightfield, LightfieldError};


fn test_load(filename: &str) {
    match Lightfield::from_zip(filename) {
        Ok(lf) => {
            assert!(lf.views.len() == 16);
        },
        Err(e) => {
            panic!("cannot load lf: {:?}", e);
        }
    }
}
#[test]
fn ok_jpg() {
    test_load("tests/images/dummy.jpg.zip");
}
#[test]
fn ok_png() {
    test_load("tests/images/dummy.png.zip");
}
#[test]
fn filenotfound() {
    let res = Lightfield::from_zip("does-not-exist.jpg");
    assert!(res.is_err());
    match res {
        Err(LightfieldError::IoError(_))  => {},
        Err(e) => {panic!(format!("{:?}", e))},
        _ => {panic!(format!("failed to fail"))},
    }
}
