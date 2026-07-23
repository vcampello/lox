mod common;
use common::Trap;

#[test]
fn print_hello() {
    let (trap, mut rt) = Trap::new_runtime();
    rt.run("TEST", r#"print "hello world!";"#).unwrap();

    assert_eq!(trap.to_string(), "hello world!\n");
}
