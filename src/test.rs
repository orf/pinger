#[cfg(test)]
mod tests {
    use crate::linux::LinuxParser;
    use crate::macos::MacOSParser;
    #[cfg(windows)]
    use crate::windows::WindowsParser;
    use crate::{Parser, PingResult};

    fn test_parser<T>(contents: &str)
    where
        T: Parser,
    {
        let parser = T::default();
        let test_file: Vec<&str> = contents.split("-----").collect();
        let input = test_file[0].trim().split("\n");
        let expected: Vec<&str> = test_file[1].trim().split("\n").collect();
        let parsed: Vec<Option<PingResult>> = input.map(|l| parser.parse(l.to_string())).collect();

        assert_eq!(parsed.len(), expected.len());

        for (output, expected) in parsed.into_iter().zip(expected) {
            if let Some(value) = output {
                assert_eq!(format!("{:?}", value).trim(), expected.trim())
            } else {
                assert_eq!("None", expected.trim())
            }
        }
    }

    #[test]
    fn macos() {
        test_parser::<MacOSParser>(include_str!("tests/macos.txt"));
    }

    #[test]
    fn ubuntu() {
        test_parser::<LinuxParser>(include_str!("tests/ubuntu.txt"));
    }

    #[cfg(windows)]
    #[test]
    fn windows() {
        test_parser::<WindowsParser>(include_str!("tests/windows.txt"));
    }
}
