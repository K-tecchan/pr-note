use clap::Parser;

mod cli;
mod github;

#[tokio::main]
async fn main() {
    let args = cli::Args::parse();

    let client = github::Client::new(args);
    client.get_un_merged_commits().await.unwrap();
}
