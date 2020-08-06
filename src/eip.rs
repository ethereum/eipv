use crate::validators;

use anyhow::{anyhow, Error, Result};
use chrono::NaiveDate;
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
    pub eip: Option<Result<u64>>,
    pub title: Option<Result<String>>,
    pub author: Option<Result<Vec<String>>>,
    pub discussions_to: Option<Result<Url>>,
    pub status: Option<Result<Status>>,
    pub review_period_end: Option<Result<NaiveDate>>,
    pub ty: Option<Result<Type>>,
    pub category: Option<Result<Category>>,
    pub created: Option<Result<NaiveDate>>,
    pub updated: Option<Result<NaiveDate>>,
    pub requires: Option<Result<Vec<u64>>>,
    pub replaces: Option<Result<Vec<u64>>>,
    pub superseded_by: Option<Result<Vec<u64>>>,
    pub resolution: Option<Result<Url>>,
}

macro_rules! insert {
    ($preamble: expr, $validator: expr, $value: expr, $errors: ident) => {{
        let res = $validator($value);

        match res {
            Ok(v) => $preamble = Some(Ok(v)),
            Err(e) => {
                $preamble = Some(Err((anyhow!(""))));
                $errors.push(e);
            }
        }
    }};
}

impl Preamble {
    pub fn from_str(s: &str) -> Result<(Self, String), Vec<Error>> {
        let mut preamble = Preamble::default();
        let mut errors: Vec<Error> = vec![];

        let (block, rest) = validators::preamble(s).map_err(|e| vec![e])?;

        for (i, line) in block.lines().enumerate() {
            let split_idx = line.find(":");
            if split_idx.is_none() {
                errors.push(anyhow!("malformed key-value pair: {}", line));
                continue;
            }

            let (key, mut value) = line.split_at(split_idx.unwrap());
            value = value.strip_prefix(":").unwrap();

            if &value[1..] != value.trim() {
                errors.push(anyhow!("missing a `space` between colon and value",));
            }

            value = &value[1..];

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

        if preamble.eip.is_none() {
            errors.push(anyhow!("missing eip field in preamble"));
        }

        if preamble.title.is_none() {
            errors.push(anyhow!("missing title field in preamble"));
        }

        if preamble.author.is_none() {
            errors.push(anyhow!("missing author field in preamble"));
        }

        if preamble.discussions_to.is_none() {
            errors.push(anyhow!("missing discussions-to field in preamble"));
        }

        if preamble.status.is_none() {
            errors.push(anyhow!("missing status field in preamble"));
        }

        if let Some(Ok(ty)) = preamble.ty {
            if ty == Type::Standards && preamble.category.is_none() {
                errors.push(anyhow!("missing category field in preamble"));
            }
        } else {
            errors.push(anyhow!("missing type field in preamble"));
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
            _ => Err(anyhow!("unknown status type: {}", s)),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
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
            _ => Err(anyhow!("unknown type: {}", s)),
        }
    }
}

#[derive(Clone, Copy, Debug)]
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
            _ => Err(anyhow!("unknown category: {}", s)),
        }
    }
}
