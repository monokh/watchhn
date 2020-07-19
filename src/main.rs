mod post_watch;
mod keyword_watch;
mod post;

use tokio;
use clap::{Arg, App, SubCommand};
use post_watch::PostWatch;
use keyword_watch::KeywordWatch;
use regex::Regex;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = App::new("Watch HN")
    .version("1.0")
    .author("monokh")
    .about("Watch HackerNews for changes")
    .arg(Arg::with_name("slack-webhook")
        .help("Slack webhook url to send notifications")
        .short("sl")
        .long("slack-webhook")
        .value_name("HOOK URL")
        .global(true)
    )
    .arg(Arg::with_name("interval")
        .help("Time in seconds to wait between each check")
        .short("i")
        .long("interval")
        .value_name("SECONDS")
        .default_value("10")
        .global(true)
    )
    .subcommand(SubCommand::with_name("post")
                .about("Watch a post for new comments")
                .arg(Arg::with_name("link")
                    .help("Link to the HN post to watch")
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
                .arg(Arg::with_name("new")
                    .help("Include new stories")
                    .short("n")
                    .long("new")
                )
                .arg(Arg::with_name("top")
                    .help("Include top stories")
                    .short("t")
                    .long("top")
                )
                .arg(Arg::with_name("limit")
                    .help("Limit the number of posts to check for each kind (new and top)")
                    .short("l")
                    .long("limit")
                    .default_value("40")
                )
    );
    
    let matches = cli.get_matches();

    let hook_arg = matches.value_of("slack-webhook");
    let hook_url = if hook_arg.is_some() { Some(String::from(hook_arg.unwrap())) } else { None };
    let interval: u32 = matches.value_of("interval").unwrap().parse().unwrap();

    if let Some(cmd) = matches.subcommand_matches("post") {
        let link: String = String::from(cmd.value_of("link").unwrap());
        let re = Regex::new(r"id=(?P<id>\d{8})").unwrap();
        let id: u32 = re.captures(&link).unwrap()["id"].parse().unwrap();
        let mut watch = PostWatch::new(id, hook_url.clone(), interval);
        watch.start().await?;
    }

    if let Some(cmd) = matches.subcommand_matches("keywords") {
        let keywords: Vec<String> = cmd.values_of("keywords").unwrap().map(|s| String::from(s)).collect();
        let new = cmd.is_present("new");
        let top = cmd.is_present("top");
        let limit: u32 = cmd.value_of("limit").unwrap().parse().unwrap();
        let mut watch = KeywordWatch::new(keywords, hook_url.clone(), new, top, limit, interval);
        watch.start().await?;
    }

    Ok(())
}
