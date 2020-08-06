use crate::error::Error;
use crate::{ctx::Context, validators};

use anyhow::{anyhow, Result};
use chrono::NaiveDate;
use url::Url;

#[derive(Debug)]
pub struct Eip {
    pub preamble: Preamble,
    pub body: String,
}

impl Eip {
    pub fn from_str(ctx: &Context, s: &str) -> Result<Self, Vec<Error>> {
        match Preamble::from_str(ctx, s) {
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
    ($preamble: expr, $validator: expr, $t: expr) => {{
        let res = $validator($t.0);

        match res {
            Ok(v) => $preamble = Some(Ok(v)),
            Err(e) => {
                $preamble = Some(Err((anyhow!(""))));

                if !$t.2.should_exclude(&e) {
                    $t.1.push(e);
                }
            }
        }
    }};
}

impl Preamble {
    pub fn from_str(ctx: &Context, s: &str) -> Result<(Self, String), Vec<Error>> {
        let mut preamble = Preamble::default();
        let mut errors: Vec<Error> = vec![];

        let (block, rest) = validators::preamble(s).map_err(|e| vec![e])?;

        for (i, line) in block.lines().enumerate() {
            let split_idx = line.find(":");
            if split_idx.is_none() {
                errors.push(Error::MalformedField);
                continue;
            }

            let (key, mut value) = line.split_at(split_idx.unwrap());
            value = value.strip_prefix(":").unwrap();

            if &value[1..] != value.trim() {
                errors.push(Error::MissingSpaceAfterColon);
            }

            value = &value[1..];

            if key != key.trim() {
                errors.push(Error::ExtraWhitespace);
            }
            if value != value.trim() {
                errors.push(Error::ExtraWhitespace);
            }

            let key = key.trim();
            let value = value.trim();

            // tuple to simplify macro calls
            let t = (value, &mut errors, &ctx);

            match key {
                "eip" => insert!(preamble.eip, validators::eip, t),
                "title" => insert!(preamble.title, validators::title, t),
                "author" => insert!(preamble.author, validators::author, t),
                "discussions-to" => insert!(preamble.discussions_to, validators::discussions_to, t),
                "status" => insert!(preamble.status, validators::status, t),
                "review-period-end" => {
                    insert!(preamble.review_period_end, validators::review_period_end, t)
                }
                "type" => insert!(preamble.ty, validators::ty, t),
                "category" => insert!(preamble.category, validators::category, t),
                "created" => insert!(preamble.created, validators::created, t),
                "updated" => insert!(preamble.updated, validators::updated, t),
                "requires" => insert!(preamble.requires, validators::requires, t),
                "replaces" => insert!(preamble.replaces, validators::replaces, t),
                "superseded-by" => insert!(preamble.superseded_by, validators::superseded_by, t),
                "resolution" => insert!(preamble.resolution, validators::resolution, t),
                _ => errors.push(Error::UnknownPreambleField),
            }
        }

        if preamble.eip.is_none() {
            errors.push(Error::MissingEipField);
        }

        if preamble.title.is_none() {
            errors.push(Error::MissingTitleField);
        }

        if preamble.author.is_none() {
            errors.push(Error::MissingAuthorField);
        }

        if preamble.discussions_to.is_none() {
            errors.push(Error::MissingDiscussionsToField);
        }

        if preamble.status.is_none() {
            errors.push(Error::MissingStatusField);
        }

        if let Some(Ok(ty)) = preamble.ty {
            if ty == Type::Standards && preamble.category.is_none() {
                errors.push(Error::MissingCategoryField);
            }
        } else {
            errors.push(Error::MissingTypeField);
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
    pub fn from_str(s: &str) -> std::result::Result<Self, Error> {
        match s {
            "Draft" => Ok(Self::Draft),
            "Last Call" => Ok(Self::LastCall),
            "Accepted" => Ok(Self::Accepted),
            "Final" => Ok(Self::Final),
            "Active" => Ok(Self::Active),
            "Abandoned" => Ok(Self::Abandoned),
            "Superseded" => Ok(Self::Superseded),
            "Rejected" => Ok(Self::Rejected),
            _ => Err(Error::UnknownStatus),
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
    pub fn from_str(s: &str) -> std::result::Result<Self, Error> {
        match s {
            "Standards Track" => Ok(Self::Standards),
            "Informational" => Ok(Self::Informational),
            "Meta" => Ok(Self::Meta),
            _ => Err(Error::UnknownType),
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
    pub fn from_str(s: &str) -> std::result::Result<Self, Error> {
        match s {
            "Core" => Ok(Self::Core),
            "Networking" => Ok(Self::Networking),
            "Interface" => Ok(Self::Interface),
            "ERC" => Ok(Self::Erc),
            _ => Err(Error::UnknownCategory),
        }
    }
}
