/*
 *   Copyright 2026 Chrimle
 *
 *   Licensed under the Apache License, Version 2.0 (the "License");
 *   you may not use this file except in compliance with the License.
 *   You may obtain a copy of the License at
 *
 *       http://www.apache.org/licenses/LICENSE-2.0
 *
 *   Unless required by applicable law or agreed to in writing, software
 *   distributed under the License is distributed on an "AS IS" BASIS,
 *   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *   See the License for the specific language governing permissions and
 *   limitations under the License.
 */

use regex;
use regex::Regex;

#[derive(Debug)]
///
/// Represents a parsing error of a [Semantic Versioning](https://semver.org/) String.
///
pub enum SemVerError {
    /// A _SemVer String_ has an invalid format.
    InvalidFormat(String),
    /// A _SemVer String_ missing a required sub-version.
    MissingVersion(String),
    /// A _SemVer String_ has an invalid sub-version.
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
