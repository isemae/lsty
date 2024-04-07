use super::status_symbols::{status_symbol, Status::*};
use crossterm::style::Stylize;

pub struct MessageArgs {
    pub primary_keyword: String,
    pub secondary_keyword: String,
    pub primary_path: String,
    pub secondary_path: String,
}

impl Default for MessageArgs {
    fn default() -> Self {
        MessageArgs {
            primary_keyword: String::new(),
            secondary_keyword: String::new(),
            primary_path: String::new(),
            secondary_path: String::new(),
        }
    }
}

// category - action - condition
pub enum MessageKind {
    TargetChangePath,
    TargetChangeKeyword,
    NoKeywordOrPathForReplace,
    FromPath,
    ExistingAlias,
    UpdatingAlias,
    UpdatedAlias,
    DeleteRule,
    NoRuleShowAvailable,
    RuleInfo,
    AlreadyExistsTryEdit,
}

pub enum ErrorKind {
    InvalidAlias,
    NotFoundAlias,
    NotFoundRuleForPath,
}

pub fn message_format(kind: MessageKind, args: MessageArgs) -> String {
    match kind {
        MessageKind::TargetChangePath => {
            format!(
        "{0} change target path '\x1b[4m{1}\x1b[0m\x1b[0m' -> '\x1b[4m{2}\x1b[0m\x1b[0m' for keyword '{3}'?",
        status_symbol(&YN), args.primary_path, args.secondary_path, args.primary_keyword)
        }
        MessageKind::TargetChangeKeyword => {
            format!(
                "{0} change keyword '{1}' -> '{2}'?",
                status_symbol(&YN),
                args.primary_keyword,
                args.secondary_keyword
            )
        }
        MessageKind::NoKeywordOrPathForReplace => {
            format!("please add a new keyword or path for a replace.\nCURRENT:\n keyword - {},\n target - {}", args.primary_keyword, args.primary_path)
        }
        MessageKind::FromPath => {
            format!("from \x1b[4m{}\x1b[0m\x1b[0m?", args.primary_path)
        }
        MessageKind::ExistingAlias => {
            format!(
                "{} '{}' is an existing alias for path '\x1b[4m{}\x1b[0m\x1b[0m'",
                status_symbol(&Error),
                args.primary_keyword,
                args.primary_path
            )
        }
        MessageKind::DeleteRule => {
            format!(
                "{0} delete rule for keyword '{1}', target path '\x1b[4m{2}\x1b[0m\x1b[0m'?",
                status_symbol(&YN),
                args.primary_keyword,
                args.primary_path
            )
        }
        MessageKind::NoRuleShowAvailable => {
            format!(
                "{} no such rule for the keyword in the current path. \nkeywords available for current path:\n{}",
                status_symbol(&NotFound),
                    args.primary_keyword.cyan(),
            )
        }

        MessageKind::RuleInfo => {
            format!(
                "KEYWORD: {}\n SOURCE : \x1b[4m{}\x1b[0m\n TARGET : └─> \x1b[4m{}\x1b[0m \n",
                args.primary_keyword, args.primary_path, args.secondary_path
            )
        }

        MessageKind::UpdatingAlias => {
            format!(
                "{} update alias '{}' -> '{}'?",
                status_symbol(&YN),
                args.primary_keyword,
                args.secondary_keyword
            )
        }
        MessageKind::UpdatedAlias => {
            format!(
                "{} updated alias: {} -> {}",
                status_symbol(&Safe),
                args.primary_keyword,
                args.secondary_keyword
            )
        }
        MessageKind::AlreadyExistsTryEdit => {
            format!("rule already exists.\nNote: Try \"\x1b[4mlsty edit {0} {1}\x1b[0m\x1b[0m\" or \"\x1b[4mlsty -e {0} {1}\x1b[0m\x1b[0m\" to edit the path.", args.primary_keyword, args.primary_path)
        }
    }
}

pub fn error_format(kind: ErrorKind) -> String {
    match kind {
        ErrorKind::InvalidAlias => {
            format!(
                "{0} invalid alias: alias should not contain '/' or '\\'.",
                status_symbol(&Error)
            )
        }
        ErrorKind::NotFoundAlias => {
            format!("NOT FOUND: no rule for the alias found.")
        }
        ErrorKind::NotFoundRuleForPath => format!("NOT FOUND: no rule for the path found."),
    }
}
