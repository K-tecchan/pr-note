use clap::Parser;

mod cli;
mod github;

#[tokio::main]
async fn main() {
    let args = cli::Args::parse();

    let client = github::Client::new(args);
    let response = client.get_un_merged_commits().await;
    let prs = client.extract_pr_info(response);
    println!("PRs: {:#?}", prs);
    client.upsert_pull_request().await.unwrap();
}
