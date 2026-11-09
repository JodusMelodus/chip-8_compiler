use chip8::{parser::ParserError, scan::scan};

#[test]
fn test_scan() -> Result<(), ParserError> {
    let lines = vec!["LDVN 0 2", "LDIF 4", "DRW 0 1 5", "JP 0x20A"];

    for line in lines {
        let (opcode, values) = scan(line)?;
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
            ("JP", [nnn]) => {
                assert_eq!(*nnn, 0x20A);
            }
            _ => panic!("Unknown instruction"),
        }
    }

    Ok(())
}
