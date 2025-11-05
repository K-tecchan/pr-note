use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, Parser)]
#[command(
    version,
    about = "Generate or update a GitHub Pull Request (PR) note summarizing unmerged PRs between two branches.",
    long_about = "Generate or update a GitHub Pull Request (PR) note summarizing unmerged PRs between two branches."
)]
pub struct Args {
    #[arg(
        long,
        env = "PR_NOTE_GITHUB_HOST",
        default_value = "api.github.com",
        help = "API host domain for GitHub. Default is \"api.github.com\".
For GitHub Enterprise, set your enterprise host domain."
    )]
    pub host: String,

    #[arg(
        short,
        long,
        env = "PR_NOTE_REPO_OWNER",
        help = "The owner of the repository."
    )]
    pub owner: String,

    #[arg(
        short,
        long,
        env = "PR_NOTE_REPO_NAME",
        help = "The name of the repository."
    )]
    pub repo: String,

    #[arg(
        short,
        long,
        env = "PR_NOTE_BASE_BRANCH",
        help = "The name of the base branch."
    )]
    pub base: String,

    #[arg(
        short = 'a',
        long,
        env = "PR_NOTE_HEAD_BRANCH",
        help = "The name of the head branch."
    )]
    pub head: String,

    #[arg(
        short,
        long,
        env = "PR_NOTE_GITHUB_TOKEN",
        hide_env_values = true,
        help = "GitHub API token with appropriate repository permissions:
  - Contents: read
  - Metadata: read
  - Pull requests: write"
    )]
    pub token: String,

    #[arg(
        short = 'p',
        long,
        env = "PR_NOTE_TEMPLATE_PATH",
        default_value = "src/doc/template.tera",
        help = "Template file path for PR body.
If not specified, the default template will be used."
    )]
    pub template_path: String,

    #[arg(
        short,
        long,
        env = "PR_NOTE_COMMITS", 
        value_parser = clap::value_parser!(i64).range(1..),
        default_value_t = 100,
        help = "Number of commits to retrieve per request when checking unmerged commits.
Default is 100.")]
    pub commits: i64,

    #[arg(
        short,
        long,
        env = "PR_NOTE_GROUP_BY",
        help = "Grouping options for unmerged PRs.
Options are \"label\" or \"title\""
    )]
    pub group_by: Option<Group>,

    #[arg(
        short,
        long = "dry-run",
        env = "PR_NOTE_DRY_RUN",
        default_value_t = false,
        help = "Dry run mode.
With this option, no PR will be created or updated, only output the generated text to stdout"
    )]
    pub dry_run: bool,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum Group {
    Label,
    Title,
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
            "--dry-run",
        ]);

        assert_eq!(args.host, "sample.github.enterprise");
        assert_eq!(args.owner, "octocat");
        assert_eq!(args.repo, "Hello-World");
        assert_eq!(args.base, "main");
        assert_eq!(args.head, "feature-branch");
        assert_eq!(args.token, "ghp_exampletoken1234567890");
        assert_eq!(args.template_path, "./custom/template.md");
        assert_eq!(args.commits, 1000);
        assert!(args.dry_run);
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
        assert_eq!(args.template_path, "src/doc/template.tera");
        assert_eq!(args.commits, 100);
        assert!(!args.dry_run);
    }

    // #[test]
    // fn cli_parsing_with_env_vars() {
    //     std::env::set_var("GITHUB_HOST", "env.github.enterprise");
    //     std::env::set_var("REPO_OWNER", "env-octocat");
    //     std::env::set_var("REPO_NAME", "Env-Hello-World");
    //     std::env::set_var("BASE_BRANCH", "env-main");
    //     std::env::set_var("HEAD_BRANCH", "env-feature-branch");
    //     std::env::set_var("GITHUB_API_TOKEN", "env_exampletoken0987654321");
    //     std::env::set_var("TEMPLATE_PATH", "env/src/doc/template.md");
    //     std::env::set_var("COMMITS", "50");
    //     std::env::set_var("DRY_RUN", "true");

    //     let args = Args::parse_from(["pr-note"]);

    //     assert_eq!(args.host, "env.github.enterprise");
    //     assert_eq!(args.owner, "env-octocat");
    //     assert_eq!(args.repo, "Env-Hello-World");
    //     assert_eq!(args.base, "env-main");
    //     assert_eq!(args.head, "env-feature-branch");
    //     assert_eq!(args.token, "env_exampletoken0987654321");
    //     assert_eq!(args.template_path, "env/src/doc/template.md");
    //     assert_eq!(args.commits, 50);
    //     assert!(args.dry_run);

    //     std::env::remove_var("GITHUB_HOST");
    //     std::env::remove_var("REPO_OWNER");
    //     std::env::remove_var("REPO_NAME");
    //     std::env::remove_var("BASE_BRANCH");
    //     std::env::remove_var("HEAD_BRANCH");
    //     std::env::remove_var("GITHUB_API_TOKEN");
    //     std::env::remove_var("TEMPLATE_PATH");
    //     std::env::remove_var("COMMITS");
    //     std::env::remove_var("DRY_RUN");
    // }
}
