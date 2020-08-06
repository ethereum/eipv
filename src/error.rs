use anyhow::anyhow;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Error {
    // generic errors
    MalformedField,
    MissingSpaceAfterColon,
    LeadingWhitespace,
    ExtraWhitespace,
    TrailingWhitespace,
    UnknownPreambleField,

    // missing field
    MissingEipField,
    MissingTitleField,
    MissingAuthorField,
    MissingDiscussionsToField,
    MissingStatusField,
    MissingCategoryField,
    MissingTypeField,

    // validator level errors
    StartDelimiterMissing,
    EndDelimiterMissing,
    MalformedEipNumber,
    TitleExceedsMaxLength,
    MalformedDiscussionsTo,
    UnknownStatus,
    UnknownType,
    UnknownCategory,
    MalformedReviewPeriodEnd,
    MalformedCreated,
    MalformedUpdated,
    MalformedResolution,
    MissingSpaceAfterComma,
    ExtraWhitespaceBeforeComma,
    OutOfOrderEips,
    UnmatchedEmailDelimiter,
    UnmatchedHandleDelimiter,
    AuthorHasEmailAndHandle,
    TrailingInfoAfterEmail,
    TrailingInfoAfterHandle,
}

impl Error {
    pub fn from_str(s: &str) -> anyhow::Result<Self> {
        match s {
            "title_max_length" => Ok(Self::TitleExceedsMaxLength),
            "missing_discussions_to" => Ok(Self::MissingDiscussionsToField),
            _ => Err(anyhow!("unknown validator")),
        }
    }

    pub fn human_readable(&self) -> &'static str {
        match &self {
            // preamble level errors
            Self::MalformedField => "malformed field",
            Self::MissingSpaceAfterColon => "missing a `space` between colon and value",
            Self::ExtraWhitespace => "extra whitespace",
            Self::TrailingWhitespace => "trailing whitespace",
            Self::LeadingWhitespace => "leading whitespace",
            Self::UnknownPreambleField => "unknown preamble field",

            // missing required fields
            Self::MissingEipField => "missing EIP field in preamble",
            Self::MissingTitleField => "missing title field in preamble",
            Self::MissingAuthorField => "missing author field in preamble",
            Self::MissingDiscussionsToField => "missing discussions-to field in preamble",
            Self::MissingStatusField => "missing status field in preamble",
            Self::MissingCategoryField => "missing category field in preamble",
            Self::MissingTypeField => "missing type field in preamble",

            // validator level errors
            Self::StartDelimiterMissing => "missing initial '---' in preamble",
            Self::EndDelimiterMissing => "missing trailing '---' in preamble",
            Self::MalformedEipNumber => "EIP should be an unsigned integer",
            Self::TitleExceedsMaxLength => "title exceeds max length of 55 characters",
            Self::MalformedDiscussionsTo => "discussions-to must be a URL",
            Self::UnknownStatus => "unknown status",
            Self::UnknownType => "unknown type",
            Self::UnknownCategory => "unknown category",
            Self::MalformedReviewPeriodEnd => "malformed review-period-end date",
            Self::MalformedCreated => "malformed created date",
            Self::MalformedUpdated => "malformed updated date",
            Self::MalformedResolution => "resolution must be a URL",
            Self::MissingSpaceAfterComma => {
                "comma-separated values must have spaces following each comma"
            }
            Self::ExtraWhitespaceBeforeComma => {
                "comma-separated values must not have spaces before a comma"
            }
            Self::OutOfOrderEips => "numbers must be in ascending order",
            Self::UnmatchedEmailDelimiter => "unmatched email delimiter",
            Self::UnmatchedHandleDelimiter => "unmatched handle delimiter",
            Self::AuthorHasEmailAndHandle => "author can't include both an email and handle",
            Self::TrailingInfoAfterEmail => "trailing information after email",
            Self::TrailingInfoAfterHandle => "trailing information after handle",
        }
    }
}
