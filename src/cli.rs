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
        help = "Template file path for PR title and body.
First line is used as the title, rest as the body.
If not specified, the default template will be used."
    )]
    pub template_path: Option<String>,

    #[arg(
        short,
        long,
        env = "PR_NOTE_GROUP_BY",
        help = "Grouping options for unmerged PRs.
Options are \"label\" or \"title\"."
    )]
    pub group_by: Option<Group>,

    #[arg(
        short,
        long = "dry-run",
        env = "PR_NOTE_DRY_RUN",
        default_value_t = false,
        help = "Dry run mode.
With this option, no PR will be created or updated, only output the generated text to stdout."
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
            "--dry-run",
        ]);

        assert_eq!(args.host, "sample.github.enterprise");
        assert_eq!(args.owner, "octocat");
        assert_eq!(args.repo, "Hello-World");
        assert_eq!(args.base, "main");
        assert_eq!(args.head, "feature-branch");
        assert_eq!(args.token, "ghp_exampletoken1234567890");
        assert_eq!(args.template_path.unwrap(), "./custom/template.md");
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
        assert!(args.template_path.is_none());
        assert!(!args.dry_run);
    }
}
