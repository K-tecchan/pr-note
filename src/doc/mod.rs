use regex::Regex;
use serde::Serialize;
use tera::{Context, Result, Tera};

use crate::cli::{Args, Group};
use crate::github::PullRequest;

#[derive(Debug)]
pub struct Doc {
    tera: Tera,
}

impl Doc {
    pub fn new() -> Self {
        Doc {
            tera: Tera::default(),
        }
    }

    pub fn render(&mut self, args: &Args, prs: &[PullRequest]) -> Result<String> {
        self.tera
            .add_template_file(args.template_path.clone(), Some("template"))
            .unwrap();

        let prs = prs
            .iter()
            .map(|pr| pr.clone().into_extended(&args.group_by))
            .collect::<Vec<_>>();

        let mut context = Context::new();
        context.insert("prs", &prs);
        self.tera.render("template", &context)
    }
}

#[derive(Debug, Serialize)]
struct ExtendedPullRequest {
    author: String,
    labels: Vec<String>,
    number: i64,
    title: String,
    body: String,
    group: String,
}

impl PullRequest {
    fn into_extended(self, group_by: &Option<Group>) -> ExtendedPullRequest {
        let group = match group_by {
            Some(Group::Label) => {
                if self.labels.is_empty() {
                    "others".to_string()
                } else {
                    self.labels.join(" / ")
                }
            }
            Some(Group::Title) => {
                let re_head = Regex::new(r"^\s*(\[(?:[^\[\]]+)\])+").unwrap();

                if let Some(m) = re_head.find(&self.title) {
                    let head = m.as_str();
                    let re_tag = Regex::new(r"\[(?P<tag>[^\[\]]+)\]").unwrap();
                    let mut tags: Vec<_> = re_tag
                        .captures_iter(head)
                        .filter_map(|c| c.name("tag").map(|m| m.as_str().to_string()))
                        .collect();

                    if tags.is_empty() {
                        "others".to_string()
                    } else {
                        tags.sort();
                        tags.join(" / ")
                    }
                } else {
                    "others".to_string()
                }
            }
            None => "ungrouped".to_string(),
        };

        ExtendedPullRequest {
            author: self.author,
            labels: self.labels,
            number: self.number,
            title: self.title,
            body: self.body,
            group,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render() {
        let mut doc = Doc::new();
        let args = Args {
            host: "api.github.com".to_string(),
            owner: "owner".to_string(),
            repo: "repo".to_string(),
            base: "main".to_string(),
            head: "feature".to_string(),
            token: "token".to_string(),
            template_path: "src/doc/template.tera".to_string(),
            commits: 100,
            group_by: Some(Group::Label),
            dry_run: true,
        };

        let prs = vec![PullRequest {
            author: "octocat".to_string(),
            labels: vec!["bug".to_string(), "enhancement".to_string()],
            number: 1,
            title: "Add feature A".to_string(),
            body: "body".to_string(),
        }];

        let result = doc.render(&args, &prs);
        assert!(result.is_ok());
    }

    #[test]
    fn test_into_extended_when_group_by_title() {
        let prs = vec![PullRequest {
            author: "octocat".to_string(),
            labels: vec!["bug".to_string(), "enhancement".to_string()],
            number: 1,
            title: "[documentation][duplicate]Add[aaaa] feature A[aaaaaa]".to_string(),
            body: "body".to_string(),
        }];

        let extended_prs: Vec<ExtendedPullRequest> = prs
            .into_iter()
            .map(|pr| pr.into_extended(&Some(Group::Title)))
            .collect();
        assert_eq!(extended_prs[0].group, "documentation / duplicate");
    }
}
