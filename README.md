# Watch HN

[![WatchHN Crates](https://img.shields.io/crates/v/watchhn)](https://crates.io/crates/watchhn)

CLI tool (Rust) to watch hackernews for new comments and keywords

Supports output to console and notification to slack.

![screenshot](https://github.com/monokh/watchhn/raw/master/screenshot.png)

## Run

`cargo install watchhn`

or

[Download](https://github.com/monokh/watchhn/releases) (mac & windows)

### Watch a post for new comments
`watchhn post https://news.ycombinator.com/item?id=23796580 --slack-webhook <optional slack webhook url>`

### Watch hackernews for posts with keywords
`keywords rust bitcoin GPT-3 --new --slack-webhook <optional slack webhook url>`

## Options

### Global

`-i, --interval <SECONDS>`          Time in seconds to wait between each check [default: 10]

`-s, --slack-webhook <HOOK URL>`    Slack webhook url to send notifications

### Post `watchhn post`

Watches a post for new comments. First indexes the post to save current comments, then watches for new comments anywhere in the hierarchy. 

`<link>    Link to the HN post to watch`

Example: `watchhn post https://news.ycombinator.com/item?id=23796580`

### Keywords `watchhn keywords`

Watches top, new or both HN index page for posts containing any of a given set of keywords

`-n, --new`        Include new stories

`-t, --top`        Include top stories

`-l, --limit <limit>`               Limit the number of posts to check for each kind (new and top) [default: 40]
