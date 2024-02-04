// ─── Registry ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Registry {
    Npm,
    Git,
}

// ─────────────────────────────────────────────────────────────────────────────

impl Registry {
    pub fn is_git(version: &str) -> bool {
        let git_prefixes = [
            "git+",
            "git://",
            "git@",
            "git::",
            "git+ssh://",
            "git+http://",
            "git+https://",
            "http",
            "github",
            "bitbucket",
            "gitlab",
            "ssh",
            "gitea",
        ];

        git_prefixes
            .iter()
            .any(|&prefix| version.starts_with(prefix))
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
