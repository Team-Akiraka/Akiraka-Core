pub struct VersionInfo {
    pub version_name: String,
    pub version_type: String,
}

#[cfg(test)]
mod test {
    use crate::VersionInfo;

    #[test]
    fn test() {
        let version_info = VersionInfo {
            version_name: "1.20.1".parse().unwrap(),
            version_type: "release".parse().unwrap(),
        };
    }
}
