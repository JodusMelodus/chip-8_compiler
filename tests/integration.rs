use chip8_parser::scan::scan;

#[test]
fn test_scan() {
    if let Some([x, y]) = scan("ADD 3 2", "ADD {} {}").as_deref() {
        assert_eq!(*x, 3);
        assert_eq!(*y, 2);
    }
}
