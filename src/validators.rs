use crate::eip::{Category, Status, Type};

use anyhow::{anyhow, Context, Result};
use url::Url;

const TITLE_MAX_LEN: usize = 44;

pub fn preamble(s: &str) -> Result<(&str, &str)> {
    match s.starts_with("---\n") {
        false => Err(anyhow!("missing initial '---' in preamble")),
        true => match s[4..].find("---\n") {
            Some(idx) => Ok((&s[4..idx + 4], &s[idx + 4..])),
            None => Err(anyhow!("missing trailing '---' in preamble")),
        },
    }
}

pub fn eip(s: &str) -> Result<u64> {
    Ok(s.parse::<u64>().with_context(|| "EIP should be an unsigned integer")?)
}

pub fn title(s: &str) -> Result<String> {
    if TITLE_MAX_LEN < s.len() {
        return Err(anyhow!(
            "title length of {} exceeds max length of {}",
            s.len(),
            TITLE_MAX_LEN
        ));
    }

    return Ok(s.to_string());
}

pub fn author(s: &str) -> Result<String> {
    // TODO
    return Ok(s.to_string());
}

pub fn discussions_to(s: &str) -> Result<Url> {
    Ok(Url::parse(s).map_err(|_| anyhow!("discussions-to must be a URL"))?)
}

pub fn status(s: &str) -> Result<Status> {
    Status::from_str(s)
}

pub fn review_period_end(s: &str) -> Result<String> {
    // TODO
    Ok(s.to_string())
}

pub fn ty(s: &str) -> Result<Type> {
    Type::from_str(s)
}

pub fn category(s: &str) -> Result<Category> {
    Category::from_str(s)
}

pub fn created(s: &str) -> Result<String> {
    // TODO
    Ok(s.to_string())
}

pub fn updated(s: &str) -> Result<String> {
    // TODO
    Ok(s.to_string())
}

pub fn requires(s: &str) -> Result<Vec<u64>> {
    // TODO
    Ok(vec![])
}

pub fn replaces(s: &str) -> Result<Vec<u64>> {
    // TODO
    Ok(vec![])
}

pub fn superseded_by(s: &str) -> Result<Vec<u64>> {
    // TODO
    Ok(vec![])
}

pub fn resolution(s: &str) -> Result<Url> {
    Ok(Url::parse(s)?)
}
