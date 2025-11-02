use graphql_client::{self, GraphQLQuery};
use reqwest::{self, header};

use super::graphql::{get_un_merged_commits, GetUnMergedCommits};

#[derive(Debug)]
pub struct Client {
    host: String,
    owner: String,
    repo: String,
    base: String,
    head: String,
    token: String,
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

    pub async fn get_un_merged_commits(
        &self,
    ) -> Result<graphql_client::Response<get_un_merged_commits::ResponseData>, reqwest::Error> {
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

        println!("Response: {:#?}", response);
        Ok(response)
    }
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
