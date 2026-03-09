use anyhow::Result;

pub fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) -> Result<()> {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_matches() {
        let content = "Hello, world!\nThis is a test.\nHello again.";
        let pattern = "Hello";
        let mut output = Vec::new();

        find_matches(content, pattern, &mut output).unwrap();
        assert_eq!(output, b"Hello, world!\nHello again.\n");
    }
}
