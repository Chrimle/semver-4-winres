pub fn parse_to_winres_version_as_result(semver_str: &str) -> Result<u64, &'static str> {
    Err("Not implemented")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(parse_to_winres_version_as_result("ignored").is_err());
    }
}
