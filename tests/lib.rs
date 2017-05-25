// This name matches the `mod` above
extern crate mathematical;

#[test]
fn it_works() {
    assert_eq!(mathematical::mathml_to_svg("123"), "123");
}
