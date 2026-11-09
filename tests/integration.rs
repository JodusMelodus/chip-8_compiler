use chip8_parser::scan::{scan, scanf};

#[test]
fn test_scanf() {
    if let Some([x, y]) = scanf("ADD 3 2", "ADD {} {}").as_deref() {
        assert_eq!(*x, 3);
        assert_eq!(*y, 2);
    }
}

#[test]
fn test_scan() {
    let lines = vec!["LDVN 0 2", "LDIF 4", "DRW 0 1 5"];

    for line in lines {
        if let Some((opcode, values)) = scan(line) {
            match (opcode, values.as_slice()) {
                ("LDVN", [a, b]) => {
                    assert_eq!(*a, 0);
                    assert_eq!(*b, 2);
                }
                ("LDIF", [a]) => {
                    assert_eq!(*a, 4);
                }
                ("DRW", [a, b, c]) => {
                    assert_eq!(*a, 0);
                    assert_eq!(*b, 1);
                    assert_eq!(*c, 5);
                }
                _ => panic!("Unknown instruction"),
            }
        }
    }
}
