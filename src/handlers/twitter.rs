use once_cell::sync::Lazy;
use regex::Regex;
use serenity::model::prelude::*;
use serenity::prelude::*;

static URL_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(https?://)?(?:www\.)?(twitter|x)\.com/([a-zA-Z0-9_]+)/status/([0-9]+)(?:\S+)?").unwrap()
});

fn is_twitter_url(url: &str) -> bool {
    URL_RE.is_match(url)
}

fn replace_twitter_url_with_vxtwitter(url: &str) -> String {
    URL_RE.replace_all(url, "https://vxtwitter.com/$3/status/$4").to_string()
}

pub async fn twitter_handler(ctx: &Context, new_message: &Message) -> Result<(), crate::Error> {
    let content = &new_message.content;
    if is_twitter_url(content) {
        let vx_url = replace_twitter_url_with_vxtwitter(content);
        new_message.reply(ctx, vx_url).await?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_twitter_url_pass() {
        let url = "twitter.com/user/status/123456789";
        assert!(is_twitter_url(url));
    }

    #[test]
    fn test_x_url_pass() {
        let url = "x.com/user/status/123456789";
        assert!(is_twitter_url(url));
    }

    #[test]
    fn test_full_twitter_url_pass() {
        let url = "https://twitter.com/user/status/123456789";
        assert!(is_twitter_url(url));
    }

    #[test]
    fn test_vxtwitter_url_fail() {
        let url = "vxtwitter.com/user/status/123456789";
        assert!(!is_twitter_url(url));
    }
}
