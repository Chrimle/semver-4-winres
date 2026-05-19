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

///
/// Parses a [SemVer](https://semver.org/) -string into the equivalent [winres](https://crates.io/crates/semver-4-winres)-compatible `u64`-value.
///
/// # Arguments
///
/// * `semver_str`: _SemVer_-string to be parsed and converted. **MUST** match the RegEx: `^\d+\.\d+\.\d+$`.
///
/// returns: `Result<u64, SemVerError>` the parsed _SemVer_-string converted to the _winres_-equivalent `u64`, if successful. Otherwise, a [`SemVerError`](SemVerError) is returned.
///
/// # Examples
///
/// ```
/// use semver_4_winres::parse_to_winres_version;
///
/// let winres_version: u64 = parse_to_winres_version("1.2.3").unwrap();
/// assert_eq!(winres_version, 281483566841856);
/// ```
pub fn parse_to_winres_version(semver_str: &str) -> Result<u64, SemVerError> {
    let semver_regex: Regex =
        Regex::new("^(?<major>\\d+)\\.(?<minor>\\d+)\\.(?<patch>\\d+)$").unwrap();

    let captures = semver_regex
        .captures(semver_str)
        .ok_or_else(|| SemVerError::InvalidFormat(semver_str.to_string()))?;

    let _major: u16 = captures
        .name("major")
        .map(|m| m.as_str())
        .ok_or_else(|| SemVerError::MissingVersion("major".to_string()))?
        .parse::<u16>()
        .map_err(|_| SemVerError::InvalidVersion("major".to_string()))?;

    let _minor: u16 = captures
        .name("minor")
        .map(|m| m.as_str())
        .ok_or_else(|| SemVerError::MissingVersion("minor".to_string()))?
        .parse::<u16>()
        .map_err(|_| SemVerError::InvalidVersion("minor".to_string()))?;

    let _patch: u16 = captures
        .name("patch")
        .map(|m| m.as_str())
        .ok_or_else(|| SemVerError::MissingVersion("patch".to_string()))?
        .parse::<u16>()
        .map_err(|_| SemVerError::InvalidVersion("patch".to_string()))?;

    let winres_version: u64 = u64::from(_major) << 48 | u64::from(_minor) << 32 | u64::from(_patch) << 16;

    Ok(winres_version)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("" => matches Err(SemVerError::InvalidFormat(_)))]
    #[test_case("invalid" => matches Err(SemVerError::InvalidFormat(_)))]
    #[test_case("another.invalid.string" => matches Err(SemVerError::InvalidFormat(_)))]
    #[test_case("1.2." => matches Err(SemVerError::InvalidFormat(_)))]
    fn test_invalid_format_error(semver_str: &str) -> Result<u64, SemVerError> {
        parse_to_winres_version(semver_str)
    }

    #[test_case("0.0.65536" => matches Err(SemVerError::InvalidVersion(_)))]
    #[test_case("0.65536.0" => matches Err(SemVerError::InvalidVersion(_)))]
    #[test_case("65536.0.0" => matches Err(SemVerError::InvalidVersion(_)))]
    fn test_u16_overflow_invalid_version_error(semver_str: &str) -> Result<u64, SemVerError> {
        parse_to_winres_version(semver_str)
    }

    #[test_case("0.0.0" => 0)]
    #[test_case("1.2.3" => 281483566841856)]
    #[test_case("65535.65535.65535" => 18446744073709486080)]
    fn test_valid_equals_expected(semver_str: &str) -> u64 {
        parse_to_winres_version(semver_str).unwrap()
    }

}
