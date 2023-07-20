#[derive(Clone)]
pub struct VersionInfo {
    pub version_name: String,
    pub version_type: String,
}

impl VersionInfo {
    pub fn new(version_name: String, version_type: String) -> VersionInfo {
        VersionInfo {
            version_name,
            version_type,
        }
    }

    pub fn empty() -> VersionInfo {
        VersionInfo::new(String::new(), String::new())
    }
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
