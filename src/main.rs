use clap::Parser;

mod cli;
mod doc;
mod github;

#[tokio::main]
async fn main() {
    let args = cli::Args::parse();

    let client = github::Client::new(args.clone());
    let commits = client.get_un_merged_commits().await;
    let prs = client.extract_pr_info(commits);
    if prs.is_empty() {
        println!(
            "No unmerged PRs found between {} and {}.",
            args.base, args.head
        );
        return;
    }

    let mut doc = doc::Doc::new();
    let note = match doc.render(&args, &prs) {
        Ok(note) => note,
        Err(e) => {
            eprintln!("Failed to render the PR note: {}", e);
            return;
        }
    };
    println!("{note}");

    if !args.dry_run && client.upsert_pull_request(&note).await.is_err() {
        eprintln!("Failed to create or update the PR.");
    }
}
