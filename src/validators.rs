use crate::eip::{Category, Status, Type};
use crate::error::{Error, Result};

use chrono::NaiveDate;
use regex::Regex;
use url::Url;

const TITLE_MAX_LEN: usize = 44;
const DESCRIPTION_MAX_LEN: usize = 140;

pub fn preamble(s: &str) -> Result<(&str, &str)> {
    match s.starts_with("---\n") {
        false => Err(Error::StartDelimiterMissing),
        true => match s[4..].find("---\n") {
            Some(idx) => Ok((&s[4..idx + 4], &s[idx + 4..])),
            None => Err(Error::EndDelimiterMissing),
        },
    }
}

pub fn eip(s: &str) -> Result<u64> {
    Ok(s.parse::<u64>().map_err(|_| Error::MalformedEipNumber)?)
}

pub fn title(s: &str) -> Result<String> {
    if TITLE_MAX_LEN < s.len() {
        return Err(Error::TitleExceedsMaxLength);
    }

    return Ok(s.to_string());
}

pub fn description(s: &str) -> Result<String> {
    if DESCRIPTION_MAX_LEN < s.len() {
        return Err(Error::DescriptionExceedsMaxLength);
    }

    return Ok(s.to_string());
}

pub fn author(s: &str) -> Result<Vec<String>> {
    validate_csv(s, validate_author)
}

pub fn discussions_to(s: &str) -> Result<Url> {
    Ok(Url::parse(s).map_err(|_| Error::MalformedDiscussionsTo)?)
}

pub fn status(s: &str) -> Result<Status> {
    Status::from_str(s)
}

pub fn review_period_end(s: &str) -> Result<NaiveDate> {
    NaiveDate::parse_from_str(s, "%Y-%m-%d").map_err(|_| Error::MalformedReviewPeriodEnd)
}

pub fn ty(s: &str) -> Result<Type> {
    Type::from_str(s)
}

pub fn category(s: &str) -> Result<Category> {
    Category::from_str(s)
}

pub fn created(s: &str) -> Result<NaiveDate> {
    NaiveDate::parse_from_str(s, "%Y-%m-%d").map_err(|_| Error::MalformedCreated)
}
pub fn updated(s: &str) -> Result<Vec<NaiveDate>> {
    validate_csv(s, |acc, d| {
        match NaiveDate::parse_from_str(d, "%Y-%m-%d").map_err(|_| Error::MalformedUpdated) {
            Ok(d) => {
                acc.push(d);
                Ok(())
            }
            Err(e) => Err(e),
        }
    })
}

pub fn requires(s: &str) -> Result<Vec<u64>> {
    validate_csv(s, validate_eip)
}

pub fn replaces(s: &str) -> Result<Vec<u64>> {
    validate_csv(s, validate_eip)
}

pub fn superseded_by(s: &str) -> Result<Vec<u64>> {
    validate_csv(s, validate_eip)
}

pub fn resolution(s: &str) -> Result<Url> {
    Ok(Url::parse(s).map_err(|_| Error::MalformedResolution)?)
}

fn validate_csv<T, F: Fn(&mut Vec<T>, &str) -> Result<()>>(s: &str, f: F) -> Result<Vec<T>> {
    let csv: Vec<&str> = s.split(",").collect();

    let mut acc = vec![];

    for (i, x) in csv.iter().enumerate() {
        // the first element never has whitespace, so check trailing whitespace
        // all other elements should have only one whitespace at n[0]
        if (i == 0 && x.trim_start() != *x) || (i != 0 && x.len() > 2 && x.trim_start() != &x[1..])
        {
            return Err(Error::MissingSpaceAfterComma);
        }

        if x != &x.trim_end() {
            return Err(Error::ExtraWhitespaceBeforeComma);
        }

        f(&mut acc, x.trim())?;
    }

    Ok(acc)
}

fn validate_eip(acc: &mut Vec<u64>, s: &str) -> Result<()> {
    match s.parse() {
        Ok(n) => {
            if acc.len() != 0 && acc[acc.len() - 1] > n {
                Err(Error::OutOfOrderEips)
            } else {
                acc.push(n);
                Ok(())
            }
        }
        Err(e) => Err(Error::MalformedEipNumber),
    }
}

fn validate_author<'a>(acc: &mut Vec<String>, s: &str) -> Result<()> {
    let email_start = s.find('<');
    let email_end = s.find('>');

    let handle_start = s.find('(');
    let handle_end = s.find(')');

    if email_start.is_some() != email_end.is_some() {
        return Err(Error::UnmatchedEmailDelimiter);
    }

    if handle_start.is_some() != handle_end.is_some() {
        return Err(Error::UnmatchedHandleDelimiter);
    }

    if email_start.is_some() == true && handle_start.is_some() == true {
        return Err(Error::AuthorHasEmailAndHandle);
    }

    if email_start.is_some() {
        let start = email_start.unwrap();
        let end = email_end.unwrap();

        if end != s.len() - 1 {
            return Err(Error::TrailingInfoAfterEmail);
        }

        let re = Regex::new(r#"(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])"#).unwrap();
        if !re.is_match(&s[start + 1..end]) {
            return Err(Error::MalformedEmail);
        }
    }

    if handle_start.is_some() {
        let start = handle_start.unwrap();
        let end = handle_end.unwrap();

        let re = Regex::new(r#"(^@[a-zA-Z0-9]+(?:-[a-zA-Z0-9]+)*$)"#).unwrap();
        if !re.is_match(&s[start + 1..end]) {
            return Err(Error::MalformedHandle);
        }

        if end != s.len() - 1 {
            return Err(Error::TrailingInfoAfterHandle);
        }
    }

    acc.push(s.to_string());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn author_invalid_email() {
        let a = "John Doe <@handle>";
        let e = validate_author(&mut vec![], a);
        assert_eq!(e, Err(Error::MalformedEmail));

        let a = "John Doe <@handle.com>";
        let e = validate_author(&mut vec![], a);
        assert_eq!(e, Err(Error::MalformedEmail));
    }
}
