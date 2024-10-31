use clap::Parser;
use tokio::task::JoinSet;

#[derive(Parser, Debug)]
#[command(version, about = "Count the total github stars for each user across all of their repos", long_about = None)]
struct Cli {
    /// Github username to count stars for
    #[arg(required = true)]
    users: Vec<String>,
}

async fn get_users_stars(user: String) -> (String, Result<u32, octocrab::Error>) {
    let instance = octocrab::instance();
    match instance.users(&user).repos().send().await {
        Ok(repos) => (
            user,
            Ok(repos
                .into_iter()
                .fold(0, |acc, repo| acc + repo.stargazers_count.unwrap_or(0))),
        ),
        Err(e) => (user, Err(e)),
    }
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    for result in JoinSet::from_iter(args.users.into_iter().map(get_users_stars))
        .join_all()
        .await
    {
        match result {
            (user_name, Ok(stars)) => println!("{user_name}\t{stars}"),
            (user_name, Err(e)) => {
                eprintln!("Error: {user_name} {e}")
            }
        }
    }
}
