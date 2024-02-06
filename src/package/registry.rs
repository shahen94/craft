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

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_git() {
        assert!(Registry::is_git("git+https://github.com/owner/repo"));
        assert!(Registry::is_git("git+https://github.com/owner/repo"));
        assert!(Registry::is_git("git://github.com/owner/repo"));
        assert!(Registry::is_git("git@https://github.com/owner/repo"));
        assert!(Registry::is_git("git::github.com/owner/repo"));
        assert!(Registry::is_git("git+ssh://https://github.com/owner/repo"),);
        assert!(Registry::is_git("git+http://https://github.com/owner/repo"),);
        assert!(Registry::is_git(
            "git+https://https://github.com/owner/repo"
        ),);
        assert!(Registry::is_git("https://github.com/owner/repo"));
        assert!(Registry::is_git("github.com/owner/repo"));
        assert!(Registry::is_git("bitbucket/owner/repo"));
        assert!(Registry::is_git("gitlab/owner/repo"));
        assert!(Registry::is_git("ssh:https://github.com/owner/repo"));
        assert!(Registry::is_git("gitea.com/owner/repo"));

        assert!(!Registry::is_git("1.5.6"));
        assert!(!Registry::is_git("*"));
        assert!(!Registry::is_git("> 2"));
    }

    #[test]
    fn test_to_string() {
        assert_eq!(Registry::Npm.to_string(), "npm");
        assert_eq!(Registry::Git.to_string(), "git");
    }
}
