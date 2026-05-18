use regex;
use regex::Regex;

#[derive(Debug)]
pub enum SemVerError {
    InvalidFormat(String),
    MissingVersion(String),
    InvalidVersion(String),
}

pub fn parse_to_winres_version(semver_str: &str) -> Result<u64, SemVerError> {
    let semver_regex: Regex =
        Regex::new("^(?<major>\\d+)\\.(?<minor>\\d+)\\.(?<patch>\\d+)$").unwrap();

    let captures = semver_regex
        .captures(semver_str)
        .ok_or_else(|| SemVerError::InvalidFormat(semver_str.to_string()))?;

    let _major: u16 = captures
        .name("major")
        .ok_or_else(|| SemVerError::MissingVersion("major".to_string()))?
        .as_str()
        .parse::<u16>()
        .map_err(|_| SemVerError::InvalidVersion("major".to_string()))?;

    let _minor: u16 = captures
        .name("minor")
        .ok_or_else(|| SemVerError::MissingVersion("minor".to_string()))?
        .as_str()
        .parse::<u16>()
        .map_err(|_| SemVerError::InvalidVersion("minor".to_string()))?;

    let _patch: u16 = captures
        .name("patch")
        .ok_or_else(|| SemVerError::MissingVersion("patch".to_string()))?
        .as_str()
        .parse::<u16>()
        .map_err(|_| SemVerError::InvalidVersion("patch".to_string()))?;

    let winres_version: u64 = u64::from(_major) << 48 | u64::from(_minor) << 32 | u64::from(_patch) << 16;

    Ok(winres_version)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_str_fails() {
        assert!(matches!(
            parse_to_winres_version(""),
            Err(SemVerError::InvalidFormat(_))
        ));
    }

    #[test]
    fn test_invalid_str_fails() {
        assert!(matches!(
            parse_to_winres_version("invalid"),
            Err(SemVerError::InvalidFormat(_))
        ));
    }

    #[test]
    fn test_another_invalid_str_fails() {
        assert!(matches!(
            parse_to_winres_version("another.invalid.string"),
            Err(SemVerError::InvalidFormat(_))
        ));
    }

    #[test]
    fn test_incomplete_str_fails() {
        assert!(matches!(
            parse_to_winres_version("1.2."),
            Err(SemVerError::InvalidFormat(_))
        ));
    }

    #[test]
    fn test_valid_str_passes() {
        assert_eq!(281483566841856, parse_to_winres_version("1.2.3").unwrap());
    }

    #[test]
    fn test_maximum_valid_str_passes() {
        assert_eq!(18446744073709486080, parse_to_winres_version("65535.65535.65535").unwrap());
    }

    #[test]
    fn test_u16_patch_overflow_str_fails() {
        assert!(matches!(
            parse_to_winres_version("0.0.65536"),
            Err(SemVerError::InvalidVersion(_))
        ));
    }

    #[test]
    fn test_u16_minor_overflow_str_fails() {
        assert!(matches!(
            parse_to_winres_version("0.65536.0"),
            Err(SemVerError::InvalidVersion(_))
        ));
    }

    #[test]
    fn test_u16_major_overflow_str_fails() {
        assert!(matches!(
            parse_to_winres_version("65536.0.0"),
            Err(SemVerError::InvalidVersion(_))
        ));
    }

}
