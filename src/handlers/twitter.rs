use lazy_static::lazy_static;
use regex::Regex;
use serenity::{model::prelude::Message, prelude::Context};
use tracing::debug;

lazy_static! {
    static ref URL_RE: Regex =
        Regex::new(r"^https:\/\/(www.|)(twitter|x)\.com\/(#!\/)?(\w+)\/status(es)*\/(\d+)")
            .unwrap();
}

fn is_twitter_url(url: &str) -> bool {
    if URL_RE.is_match(url) {
        return true;
    }
    false
}

fn replace_twitter_url_with_vxtwitter(url: &str) -> String {
    let replaced = URL_RE.replace_all(url, "https://vxtwitter.com/$4/status/$6");
    replaced.to_string()
}

pub async fn twitter_handler(ctx: &Context, new_message: &Message) {
    let content = new_message.content.clone();

    if is_twitter_url(&content) {
        debug!("Twitter URL detected");
        let _ = new_message
            .reply(&ctx.http, replace_twitter_url_with_vxtwitter(&content))
            .await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_x_url_pass() {
        let test_url = "https://x.com/JonAiart/status/1714415995484622866?s=20".to_string();
        assert!(is_twitter_url(&test_url));
        assert!(
            replace_twitter_url_with_vxtwitter(&test_url)
                == "https://vxtwitter.com/JonAiart/status/1714415995484622866?s=20".to_string()
        )
    }

    #[test]
    fn test_twitter_url_pass() {
        let test_url = "https://twitter.com/JonAiart/status/1714415995484622866?s=20".to_string();
        assert!(is_twitter_url(&test_url));
        assert!(
            replace_twitter_url_with_vxtwitter(&test_url)
                == "https://vxtwitter.com/JonAiart/status/1714415995484622866?s=20".to_string()
        )
    }

    #[test]
    fn test_full_twitter_url_pass() {
        let test_url =
            "https://www.twitter.com/JonAiart/status/1714415995484622866?s=20".to_string();
        assert!(is_twitter_url(&test_url));
        assert!(
            replace_twitter_url_with_vxtwitter(&test_url)
                == "https://vxtwitter.com/JonAiart/status/1714415995484622866?s=20".to_string()
        )
    }

    #[test]
    fn test_vxtwitter_url_fail() {
        let test_url = "https://vxtwitter.com/JonAiart/status/1714415995484622866?s=20".to_string();
        assert!(is_twitter_url(&test_url) == false)
    }
}
