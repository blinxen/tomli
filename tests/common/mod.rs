macro_rules! generic_test {
    ($test_name:ident, $args:expr, $expected:literal) => {
        #[test]
        fn $test_name() {
            let result = Command::new(env!("CARGO_BIN_EXE_tomli"))
                .arg("-n")
                .args($args)
                .output();

            if let Ok(result) = result {
                // Depending on the status code, we check a different buffer
                let command_output = if result.status.success() {
                    String::from_utf8_lossy(&result.stdout)
                } else {
                    String::from_utf8_lossy(&result.stderr)
                };

                assert_eq!(command_output, $expected);
            } else {
                panic!("Command could not be executed --> {}", result.unwrap_err());
            }
        }
    };
}

pub(crate) use generic_test;
