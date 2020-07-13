mod post_watch;
mod keyword_watch;
mod post;

use tokio;
use clap::{Arg, App, SubCommand};
use post_watch::PostWatch;
use keyword_watch::KeywordWatch;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = App::new("Watch HN")
    .version("1.0")
    .author("monokh")
    .about("Watch HackerNews for changes")
    .arg(Arg::with_name("slack-webhook")
            .short("sl")
            .long("slack-webhook")
            .value_name("HOOK URL")
            .global(true)
    )
    .subcommand(SubCommand::with_name("post")
                .about("Watch a post for new comments")
                .arg(Arg::with_name("id")
                    .required(true)
                    .index(1)
                )
    )
    .subcommand(SubCommand::with_name("keywords")
                .about("Watch a set of keywords")
                .arg(Arg::with_name("keywords")
                    .short("kw")
                    .long("keywords")
                    .multiple(true)
                    .required(true)
                    .index(1)
                )
    );
    
    let matches = cli.get_matches();

    let hook_url = matches.value_of("slack-webhook").expect("Slack webhook mandatory for now");

    if let Some(cmd) = matches.subcommand_matches("post") {
        let id: u32 = cmd.value_of("id").unwrap().parse().unwrap();
        let mut watch = PostWatch::new(id, String::from(hook_url));
        watch.start().await?;
    }

    if let Some(cmd) = matches.subcommand_matches("keywords") {
        let keywords: Vec<String> = cmd.values_of("keywords").unwrap().map(|s| String::from(s)).collect();
        let mut watch = KeywordWatch::new(keywords, String::from(hook_url));
        watch.start().await?;
    }

    Ok(())
}
