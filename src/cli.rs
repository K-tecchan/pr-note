use clap::Parser;

#[derive(Debug, Clone, Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// API host URL
    #[arg(long, env = "GITHUB_HOST", default_value = "api.github.com")]
    pub host: String,

    /// GitHub repository owner
    #[arg(long, env = "REPO_OWNER")]
    pub owner: String,

    /// GitHub repository name
    #[arg(long, env = "REPO_NAME")]
    pub repo: String,

    /// The name of the base(PR target) branch
    #[arg(long, env = "BASE_BRANCH")]
    pub base: String,

    /// The name of the head branch
    #[arg(long, env = "HEAD_BRANCH")]
    pub head: String,

    /// GitHub API token
    #[arg(long, env = "GITHUB_API_TOKEN", hide_env_values = true)]
    pub token: String,

    /// Template file path for PR body
    #[arg(
        long = "template-path",
        env = "TEMPLATE_PATH",
        default_value = "src/doc/template.md"
    )]
    pub template_path: String,

    /// Number of commits retrieved per request when checking unmerged commits
    #[arg(long, env = "COMMITS", value_parser = clap::value_parser!(i64).range(1..), default_value_t = 100)]
    pub commits: i64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn verify_cli() {
        Args::command().debug_assert();
    }

    #[test]
    fn cli_parsing() {
        let args = Args::parse_from([
            "pr-note",
            "--host",
            "sample.github.enterprise",
            "--owner",
            "octocat",
            "--repo",
            "Hello-World",
            "--base",
            "main",
            "--head",
            "feature-branch",
            "--token",
            "ghp_exampletoken1234567890",
            "--template-path",
            "./custom/template.md",
            "--commits",
            "1000",
        ]);

        assert_eq!(args.host, "sample.github.enterprise");
        assert_eq!(args.owner, "octocat");
        assert_eq!(args.repo, "Hello-World");
        assert_eq!(args.base, "main");
        assert_eq!(args.head, "feature-branch");
        assert_eq!(args.token, "ghp_exampletoken1234567890");
        assert_eq!(args.template_path, "./custom/template.md");
        assert_eq!(args.commits, 1000);
    }

    #[test]
    fn cli_parsing_with_defaults() {
        let args = Args::parse_from([
            "pr-note",
            "--owner",
            "octocat",
            "--repo",
            "Hello-World",
            "--base",
            "main",
            "--head",
            "feature-branch",
            "--token",
            "ghp_exampletoken1234567890",
        ]);

        assert_eq!(args.host, "api.github.com");
        assert_eq!(args.template_path, "src/doc/template.md");
        assert_eq!(args.commits, 100);
    }

    #[test]
    fn cli_parsing_with_env_vars() {
        std::env::set_var("GITHUB_HOST", "env.github.enterprise");
        std::env::set_var("REPO_OWNER", "env-octocat");
        std::env::set_var("REPO_NAME", "Env-Hello-World");
        std::env::set_var("BASE_BRANCH", "env-main");
        std::env::set_var("HEAD_BRANCH", "env-feature-branch");
        std::env::set_var("GITHUB_API_TOKEN", "env_exampletoken0987654321");
        std::env::set_var("TEMPLATE_PATH", "env/src/doc/template.md");
        std::env::set_var("COMMITS", "50");

        let args = Args::parse_from(["pr-note"]);

        assert_eq!(args.host, "env.github.enterprise");
        assert_eq!(args.owner, "env-octocat");
        assert_eq!(args.repo, "Env-Hello-World");
        assert_eq!(args.base, "env-main");
        assert_eq!(args.head, "env-feature-branch");
        assert_eq!(args.token, "env_exampletoken0987654321");
        assert_eq!(args.template_path, "env/src/doc/template.md");
        assert_eq!(args.commits, 50);

        std::env::remove_var("GITHUB_HOST");
        std::env::remove_var("REPO_OWNER");
        std::env::remove_var("REPO_NAME");
        std::env::remove_var("BASE_BRANCH");
        std::env::remove_var("HEAD_BRANCH");
        std::env::remove_var("GITHUB_API_TOKEN");
        std::env::remove_var("TEMPLATE_PATH");
        std::env::remove_var("COMMITS");
    }
}
