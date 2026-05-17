pub fn parse_to_winres_version_as_result(semver_str: &str) -> Result<u64, &'static str> {
    let match_iterator = semver_str.matches("^\\d\\.\\d\\.\\d$");

    if match_iterator.count() == 0 {
        return Err("Invalid Format of semver_str!");
    }

    Err("Not implemented")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_str_fails() {
        assert!(parse_to_winres_version_as_result("").is_err());
    }

    #[test]
    fn test_invalid_str_fails() {
        assert!(parse_to_winres_version_as_result("invalid").is_err());
    }

    #[test]
    fn test_incomplete_str_fails() {
        assert!(parse_to_winres_version_as_result("1.2").is_err());
    }
}
