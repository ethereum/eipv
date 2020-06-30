use crate::validators;

use anyhow::{anyhow, Error, Result};
use std::str::FromStr;
use url::Url;

#[derive(Debug)]
pub struct Eip {
    pub preamble: Preamble,
    pub body: String,
}

impl FromStr for Eip {
    type Err = Vec<Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match Preamble::from_str(s) {
            Ok((preamble, body)) => Ok(Eip { preamble, body }),
            Err(e) => Err(e),
        }
    }
}

#[derive(Debug, Default)]
pub struct Preamble {
    pub eip: Option<u64>,
    pub title: Option<String>,
    pub author: Option<String>,
    pub discussions_to: Option<Url>,
    pub status: Option<Status>,
    pub review_period_end: Option<String>,
    pub ty: Option<Type>,
    pub category: Option<Category>,
    pub created: Option<String>,
    pub updated: Option<String>,
    pub requires: Option<Vec<u64>>,
    pub replaces: Option<Vec<u64>>,
    pub superseded_by: Option<Vec<u64>>,
    pub resolution: Option<Url>,
}

macro_rules! insert {
    ($preamble: expr, $validator: expr, $value: expr, $errors: ident) => {{
        let res = $validator($value);
        match res {
            Ok(v) => $preamble = Some(v),
            Err(e) => $errors.push(e),
        }
    }};
}

impl Preamble {
    pub fn from_str(s: &str) -> Result<(Self, String), Vec<Error>> {
        let mut preamble = Preamble::default();
        let mut errors: Vec<Error> = vec![];

        // first line of preamble should denote beginning
        let (block, rest) = match s.starts_with("---\n") {
            false => return Err(vec![anyhow!("missing initial '---' in preamble")]),
            true => match s[4..].find("---\n") {
                Some(idx) => (&s[4..idx], &s[idx..]),
                None => return Err(vec![anyhow!("missing trailing '---' in preamble")]),
            },
        };

        for line in block.lines() {
            let key_value: Vec<&str> = line.split(": ").collect();
            if key_value.len() != 2 {
                errors.push(anyhow!("malformed key-value pair: {}", line));
                continue;
            }

            let (key, value) = (key_value[0], key_value[1]);
            if key != key.trim() {
                errors.push(anyhow!("extra whitespace"));
            }
            if value != value.trim() {
                errors.push(anyhow!("extra whitespace"));
            }

            let key = key.trim();
            let value = value.trim();

            match key {
                "eip" => insert!(preamble.eip, validators::eip, value, errors),
                "title" => insert!(preamble.title, validators::title, value, errors),
                "author" => insert!(preamble.author, validators::author, value, errors),
                "discussions-to" => insert!(
                    preamble.discussions_to,
                    validators::discussions_to,
                    value,
                    errors
                ),
                "status" => insert!(preamble.status, validators::status, value, errors),
                "review-period-end" => insert!(
                    preamble.review_period_end,
                    validators::review_period_end,
                    value,
                    errors
                ),
                "type" => insert!(preamble.ty, validators::ty, value, errors),
                "category" => insert!(preamble.category, validators::category, value, errors),
                "created" => insert!(preamble.created, validators::created, value, errors),
                "updated" => insert!(preamble.updated, validators::updated, value, errors),
                "requires" => insert!(preamble.requires, validators::requires, value, errors),
                "replaces" => insert!(preamble.replaces, validators::replaces, value, errors),
                "superseded-by" => insert!(
                    preamble.superseded_by,
                    validators::superseded_by,
                    value,
                    errors
                ),
                "resolution" => insert!(preamble.resolution, validators::resolution, value, errors),
                _ => errors.push(anyhow!("unknown preamble key: {}", key)),
            }
        }

        match errors.is_empty() {
            true => Ok((preamble, rest.to_string())),
            false => Err(errors),
        }
    }
}

#[derive(Debug)]
pub enum Status {
    Draft,
    LastCall,
    Accepted,
    Final,
    Active,
    Abandoned,
    Superseded,
    Rejected,
}

impl Status {
    pub fn from_str(s: &str) -> Result<Self> {
        match s {
            "Draft" => Ok(Self::Draft),
            "Last Call" => Ok(Self::LastCall),
            "Accepted" => Ok(Self::Accepted),
            "Final" => Ok(Self::Final),
            "Active" => Ok(Self::Active),
            "Abandoned" => Ok(Self::Abandoned),
            "Superseded" => Ok(Self::Superseded),
            "Rejected" => Ok(Self::Rejected),
            _ => Err(anyhow!("Unknown status type: {}", s)),
        }
    }
}

#[derive(Debug)]
pub enum Type {
    Standards,
    Informational,
    Meta,
}

impl Type {
    pub fn from_str(s: &str) -> Result<Self> {
        match s {
            "Standards Track" => Ok(Self::Standards),
            "Informational" => Ok(Self::Informational),
            "Meta" => Ok(Self::Meta),
            _ => Err(anyhow!("Unknown type: {}", s)),
        }
    }
}

#[derive(Debug)]
pub enum Category {
    Core,
    Networking,
    Interface,
    Erc,
}

impl Category {
    pub fn from_str(s: &str) -> Result<Self> {
        match s {
            "Core" => Ok(Self::Core),
            "Networking" => Ok(Self::Networking),
            "Interface" => Ok(Self::Interface),
            "ERC" => Ok(Self::Erc),
            _ => Err(anyhow!("Unknown category: {}", s)),
        }
    }
}
