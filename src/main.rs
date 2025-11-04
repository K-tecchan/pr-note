use clap::Parser;

mod cli;
mod doc;
mod github;

#[tokio::main]
async fn main() {
    let args = cli::Args::parse();

    let client = github::Client::new(args.clone());
    let response = client.get_un_merged_commits().await;
    let prs = client.extract_pr_info(response);
    println!("PRs: {:#?}", prs);

    let mut doc = doc::Doc::new();
    let text = doc.render(&args.template_path, &prs).unwrap();
    println!("Generated PR List:\n{}", text);

    if !args.dry_run {
        client.upsert_pull_request(&text).await.unwrap();
    }
}
