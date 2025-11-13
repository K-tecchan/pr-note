use std::collections::HashSet;

use graphql_client::{self, GraphQLQuery};
use serde::Serialize;
use serde_json::json;

use super::graphql::{GetUnMergedCommits, get_un_merged_commits};

type GetUnMergedCommitsResponse = graphql_client::Response<get_un_merged_commits::ResponseData>;
type GetUnMergedCommitsResult = Result<GetUnMergedCommitsResponse, reqwest::Error>;

#[derive(Debug)]
pub struct Client {
    host: String,
    owner: String,
    repo: String,
    base: String,
    head: String,
    token: String,
    client: reqwest::Client,
}

#[derive(Clone, Debug, Serialize)]
pub struct PullRequest {
    pub author: String,
    pub labels: Vec<String>,
    pub number: i64,
    pub title: String,
    pub body: String,
}

impl Client {
    pub fn new(args: crate::cli::Args) -> Self {
        Client {
            host: args.host,
            owner: args.owner,
            repo: args.repo,
            base: args.base,
            head: args.head,
            token: args.token,
            client: reqwest::ClientBuilder::new()
                .user_agent("pr-note")
                .build()
                .unwrap(),
        }
    }

    pub async fn upsert_pull_request(&self, note: &str) -> Result<(), reqwest::Error> {
        let Client {
            host,
            owner,
            repo,
            base,
            head,
            token,
            client,
        } = self;

        let base_url = if host == "api.github.com" {
            format!("https://{host}/repos/{owner}/{repo}")
        } else {
            format!("https://{host}/api/v3/repos/{owner}/{repo}")
        };
        let list_url = format!("{base_url}/pulls?head={owner}:{head}&base={base}&state=open");
        let existing: Vec<serde_json::Value> = client
            .get(&list_url)
            .bearer_auth(token)
            .send()
            .await?
            .json()
            .await?;

        let mut note = note.splitn(2, "\n");
        let title = note.next().unwrap_or("Release");
        let body = note.next().unwrap_or("");

        if let Some(pr) = existing.first() {
            let number = pr["number"].as_i64().unwrap();
            let patch_url = format!("{base_url}/pulls/{number}");

            let res: serde_json::Value = client
                .patch(&patch_url)
                .bearer_auth(token)
                .json(&json!({
                    "title": title,
                    "body": body,
                    "state": "open",
                }))
                .send()
                .await?
                .json()
                .await?;

            println!("Updated PR: {}", res["html_url"]);
        } else {
            let create_url = format!("{base_url}/pulls");

            let res: serde_json::Value = client
                .post(&create_url)
                .bearer_auth(token)
                .json(&json!({
                    "title": title,
                    "head": head,
                    "base": base,
                    "body": body,
                }))
                .send()
                .await?
                .json()
                .await?;

            println!("Created PR: {}", res["html_url"]);
        }

        Ok(())
    }

    pub fn extract_pr_info(&self, data: GetUnMergedCommitsResult) -> Vec<PullRequest> {
        let mut prs = Vec::new();
        let mut seen = HashSet::new();

        let data = match data {
            Ok(response) => match response.data {
                Some(data) => data,
                None => {
                    eprintln!("GraphQL errors occurred. Could not extract PR info.");
                    return prs;
                }
            },
            Err(_) => {
                eprintln!("Failed to get a valid response from GitHub GraphQL API.");
                return prs;
            }
        };

        let commits = opt_ref(&data.repository)
            .and_then(|r| opt_ref(&r.ref_))
            .and_then(|r| opt_ref(&r.compare))
            .and_then(|c| opt_ref(&c.commits.nodes));

        for commit in commits.into_iter().flatten().flatten() {
            let pr_nodes =
                opt_ref(&commit.associated_pull_requests).and_then(|apr| opt_ref(&apr.nodes));

            for pr in pr_nodes.into_iter().flatten().flatten() {
                if !seen.insert(pr.number) {
                    continue;
                }

                prs.push(PullRequest {
                    author: pr
                        .author
                        .as_ref()
                        .map(|a| a.login.clone())
                        .unwrap_or_else(|| "unknown".to_string()),
                    labels: pr
                        .labels
                        .as_ref()
                        .and_then(|l| opt_ref(&l.nodes))
                        .map(|nodes| {
                            nodes
                                .iter()
                                .flatten()
                                .map(|label| label.name.clone())
                                .collect()
                        })
                        .unwrap_or_else(Vec::new),
                    number: pr.number,
                    title: pr.title.clone(),
                    body: pr.body.clone(),
                });
            }
        }

        prs
    }

    pub async fn get_un_merged_commits(&self) -> GetUnMergedCommitsResult {
        let Client {
            host,
            owner,
            repo,
            base,
            head,
            token,
            client,
        } = self;

        let variables = get_un_merged_commits::Variables {
            owner: owner.clone(),
            repo: repo.clone(),
            base: base.clone(),
            head: head.clone(),
        };

        let url = if host == "api.github.com" {
            format!("https://{host}/graphql")
        } else {
            format!("https://{host}/api/graphql")
        };
        let request_body = GetUnMergedCommits::build_query(variables);
        let response: GetUnMergedCommitsResponse = client
            .post(&url)
            .bearer_auth(token)
            .json(&request_body)
            .send()
            .await?
            .json()
            .await?;

        Ok(response)
    }
}

fn opt_ref<T>(opt: &Option<T>) -> Option<&T> {
    opt.as_ref()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_new() {
        let args = crate::cli::Args {
            host: "api.github.com".to_string(),
            owner: "octocat".to_string(),
            repo: "Hello-World".to_string(),
            base: "main".to_string(),
            head: "feature-branch".to_string(),
            token: "your_github_token".to_string(),
            template_path: Some("src/doc/template.md".to_string()),
            group_by: None,
            dry_run: false,
        };

        let client = Client::new(args);
        assert_eq!(client.host, "api.github.com");
        assert_eq!(client.owner, "octocat");
        assert_eq!(client.repo, "Hello-World");
        assert_eq!(client.base, "main");
        assert_eq!(client.head, "feature-branch");
        assert_eq!(client.token, "your_github_token");
    }

    #[tokio::test]
    async fn test_get_un_merged_commits() {
        let client = Client::new(crate::cli::Args {
            host: "api.github.com".to_string(),
            owner: "octocat".to_string(),
            repo: "Hello-World".to_string(),
            base: "main".to_string(),
            head: "feature-branch".to_string(),
            token: "your_github_token".to_string(),
            template_path: Some("src/doc/template.md".to_string()),
            group_by: None,
            dry_run: false,
        });

        let response = client.get_un_merged_commits().await;
        assert!(response.is_ok());
    }
}
