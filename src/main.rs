use chrono::Local;
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
    let title = doc
        .render_title(Local::now().format("%Y-%m-%d").to_string().as_str())
        .unwrap();
    let body = doc.render_body(&args.template_path ,&prs).unwrap();
    println!("Generated PR List:\n{}", body);

    if !args.dry_run {
        client.upsert_pull_request(&title, &body).await.unwrap();
    }
}
