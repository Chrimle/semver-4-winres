use regex;
use regex::Regex;

pub enum SemVerError {
    InvalidFormat(String),
    MissingVersion(String),
    InvalidVersion(String),
}

pub fn parse_to_winres_version_as_result(semver_str: &str) -> Result<u64, SemVerError> {
    let semver_regex: Regex =
        Regex::new("^(?<major>\\d+)\\.(?<minor>\\d+)\\.(?<patch>\\d+)$").unwrap();

    let captures = semver_regex
        .captures(semver_str)
        .ok_or_else(|| SemVerError::InvalidFormat(semver_str.to_string()))?;

    let _major: u64 = captures
        .name("major")
        .ok_or_else(|| SemVerError::MissingVersion("major".to_string()))?
        .as_str()
        .parse::<u64>()
        .map_err(|_| SemVerError::InvalidVersion("major".to_string()))?;

    let _minor: u64 = captures
        .name("minor")
        .ok_or_else(|| SemVerError::MissingVersion("minor".to_string()))?
        .as_str()
        .parse::<u64>()
        .map_err(|_| SemVerError::InvalidVersion("minor".to_string()))?;

    let _patch: u64 = captures
        .name("patch")
        .ok_or_else(|| SemVerError::MissingVersion("patch".to_string()))?
        .as_str()
        .parse::<u64>()
        .map_err(|_| SemVerError::InvalidVersion("patch".to_string()))?;

    return Ok(1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_str_fails() {
        assert!(matches!(
            parse_to_winres_version_as_result(""),
            Err(SemVerError::InvalidFormat(_))
        ));
    }

    #[test]
    fn test_invalid_str_fails() {
        assert!(matches!(
            parse_to_winres_version_as_result("invalid"),
            Err(SemVerError::InvalidFormat(_))
        ));
    }

    #[test]
    fn test_another_invalid_str_fails() {
        assert!(matches!(
            parse_to_winres_version_as_result("another.invalid.string"),
            Err(SemVerError::InvalidFormat(_))
        ));
    }

    #[test]
    fn test_incomplete_str_fails() {
        assert!(matches!(
            parse_to_winres_version_as_result("1.2."),
            Err(SemVerError::InvalidFormat(_))
        ));
    }

    #[test]
    fn test_valid_str_passes() {
        assert!(parse_to_winres_version_as_result("1.2.3").is_ok_and(|x| x == 1));
    }
}
