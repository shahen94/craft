#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Registry {
    Npm,
    Git,
}

impl Registry {
    pub fn is_git(version: &str) -> bool {
        version.starts_with("git+")
            || version.starts_with("git://")
            || version.starts_with("git@")
            || version.starts_with("git::")
            || version.starts_with("git+ssh://")
            || version.starts_with("git+http://")
            || version.starts_with("git+https://")
            || version.starts_with("http")
            || version.starts_with("github")
            || version.starts_with("bitbucket")
            || version.starts_with("gitlab")
            || version.starts_with("ssh")
            || version.starts_with("gitea")
    }
}

impl ToString for Registry {
    fn to_string(&self) -> String {
        match self {
            Registry::Npm => "npm".to_string(),
            Registry::Git => "git".to_string(),
        }
    }
}
