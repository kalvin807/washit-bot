use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref URL_RE: Regex =
        Regex::new(r"^(http(s)?://)?(www.)?([a-zA-Z0-9])+([\-\.]{1}[a-zA-Z0-9]+)*\.[a-zA-Z]{2,5}(:[0-9]{1,5})?(/[^\s]*)?$").unwrap();
}

fn is_url(url: String) -> bool {
    URL_RE.is_match(&url)
}
