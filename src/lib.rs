use dotenv::dotenv;
use github_flows::{listen_to_event, EventPayload, GithubLogin::Default};
use slack_flows::send_message_to_channel;
use std::env;

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() -> anyhow::Result<()> {
    dotenv().ok();
    let github_owner = env::var("github_owner").unwrap_or("alabulei1".to_string());
    let github_repo = env::var("github_repo").unwrap_or("a-test".to_string());

    listen_to_event(
        &Default,
        &github_owner,
        &github_repo,
        vec!["star"],
        |payload| handler(&github_repo, payload),
    )
    .await;

    Ok(())
}

async fn handler(repo: &str, payload: EventPayload) {
    let slack_workspace = env::var("slack_workspace").unwrap_or("secondstate".to_string());
    let slack_channel = env::var("slack_channel").unwrap_or("github-status".to_string());

    if let EventPayload::UnknownEvent(e) = payload {
        let stargazers_count = e["repository"]["stargazers_count"].as_i64().unwrap_or(-1);

        let text =
            format!("Congratulations on your repository {repo} with {stargazers_count} stars.");

        if stargazers_count % 10 == 0 {
            send_message_to_channel(&slack_workspace, &slack_channel, text);
        }
    }
}
