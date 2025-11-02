use graphql_client::{self, GraphQLQuery};
use reqwest::{self, header};
use serde_json::json;
use std::collections::HashSet;

use super::graphql::{get_un_merged_commits, GetUnMergedCommits};

type GitHubGraphQLResponse =
    Result<graphql_client::Response<get_un_merged_commits::ResponseData>, reqwest::Error>;

#[derive(Debug)]
pub struct Client {
    host: String,
    owner: String,
    repo: String,
    base: String,
    head: String,
    token: String,
}

#[derive(Debug)]
pub struct PullRequest {
    pub author: String,
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
        }
    }

    pub async fn upsert_pull_request(&self) -> Result<(), reqwest::Error> {
        let Client {
            host,
            owner,
            repo,
            base,
            head,
            token,
        } = self;

        let client = reqwest::Client::new();
        let base_url = if host == "api.github.com" {
            format!("https://{host}/repos/{owner}/{repo}")
        } else {
            format!("https://{host}/api/v3/repos/{owner}/{repo}")
        };
        let list_url = format!("{base_url}/pulls?head={owner}:{head}&base={base}&state=open");
        let existing: Vec<serde_json::Value> = client
            .get(&list_url)
            .header(header::USER_AGENT, "pr-note")
            .header(header::AUTHORIZATION, format!("Bearer {token}"))
            .send()
            .await?
            .json()
            .await?;

        if let Some(pr) = existing.first() {
            let number = pr["number"].as_i64().unwrap();
            let patch_url = format!("{base_url}/pulls/{number}");

            let res: serde_json::Value = client
                .patch(&patch_url)
                .bearer_auth(token)
                .header(header::USER_AGENT, "pr-note")
                .json(&json!({
                    "title": "Updated Pull Request",
                    "body": "This pull request has been updated.",
                    "state": "open"
                }))
                .send()
                .await?
                .json()
                .await?;

            print!("Updated PR: {}", res["html_url"]);
        } else {
            let create_url = format!("{base_url}/pulls");

            let res: serde_json::Value = client
                .post(&create_url)
                .bearer_auth(token)
                .header(header::USER_AGENT, "pr-note")
                .json(&json!({
                    "title": "New Pull Request",
                    "head": head,
                    "base": base,
                    "body": "This is a new pull request created by pr-note."
                }))
                .send()
                .await?
                .json()
                .await?;

            print!("Created PR: {}", res["html_url"]);
        }

        Ok(())
    }

    pub fn extract_pr_info(&self, data: GitHubGraphQLResponse) -> Vec<PullRequest> {
        let mut prs = Vec::new();
        let mut seen = HashSet::new();

        let data = match data {
            Ok(response) => response.data.unwrap(),
            Err(_) => return prs,
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
                    number: pr.number,
                    title: pr.title.clone(),
                    body: pr.body.clone(),
                });
            }
        }

        prs
    }

    pub async fn get_un_merged_commits(&self) -> GitHubGraphQLResponse {
        let Client {
            host,
            owner,
            repo,
            base,
            head,
            token,
        } = self;

        let variables = get_un_merged_commits::Variables {
            owner: owner.clone(),
            repo: repo.clone(),
            base: base.clone(),
            head: head.clone(),
        };

        let client = reqwest::Client::new();
        let url = if host == "api.github.com" {
            format!("https://{host}/graphql")
        } else {
            format!("https://{host}/api/graphql")
        };
        let request_body = GetUnMergedCommits::build_query(variables);
        let response = client
            .post(&url)
            .header(header::USER_AGENT, "pr-note")
            .header(header::AUTHORIZATION, format!("Bearer {token}"))
            .json(&request_body)
            .send()
            .await?
            .json::<graphql_client::Response<get_un_merged_commits::ResponseData>>()
            .await?;

        // println!("Response: {:#?}", response);
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
        });

        let response = client.get_un_merged_commits().await;
        assert!(response.is_ok());
    }
}
