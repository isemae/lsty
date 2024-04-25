use super::status_symbols::{status_symbol, Status::*};
use camino::Utf8PathBuf;
use crossterm::style::Stylize;

#[derive(Default)]
pub struct MsgArgs {
    pub primary_keyword: String,
    pub secondary_keyword: String,
    pub primary_path: String,
    pub secondary_path: String,
}

// category - action - condition
pub enum MsgKind {
    TargetChangePath(MsgArgs),
    TargetChangeKeyword(MsgArgs),
    NoKeywordOrPathForReplace(MsgArgs),
    FromPath(MsgArgs),
    ExistingAlias(MsgArgs),
    UpdatingAlias(MsgArgs),
    UpdatedAlias(MsgArgs),
    DeleteRule(MsgArgs),
    NoRuleShowAvailable(MsgArgs),
    ListRule(MsgArgs),
    RuleInfo(MsgArgs),
    AlreadyExistsTryEdit(MsgArgs),
}

pub enum ErrorKind {
    InvalidAlias,
    NotFoundAlias,
    NotFoundRuleForPath,
}

pub fn msg_format(kind: MsgKind) -> String {
    match kind {
        MsgKind::TargetChangePath(args) => {
            format!(
        "{0} change target path '\x1b[4m{1}\x1b[0m\x1b[0m' -> '\x1b[4m{2}\x1b[0m\x1b[0m' for keyword '{3}'?",
        status_symbol(&YN), args.primary_path, args.secondary_path, args.primary_keyword)
        }
        MsgKind::TargetChangeKeyword(args) => {
            format!(
                "{0} change keyword '{1}' -> '{2}'?",
                status_symbol(&YN),
                args.primary_keyword,
                args.secondary_keyword
            )
        }
        MsgKind::NoKeywordOrPathForReplace(args) => {
            format!("please add a new keyword or path for a replace.\nCURRENT:\n keyword - {},\n target - {}", args.primary_keyword, Utf8PathBuf::from(args.primary_path))
        }
        MsgKind::FromPath(args) => {
            format!("from \x1b[4m{}\x1b[0m\x1b[0m?", args.primary_path)
        }
        MsgKind::ExistingAlias(args) => {
            format!(
                "{} '{}' is an existing alias for path '\x1b[4m{}\x1b[0m\x1b[0m'",
                status_symbol(&Error),
                args.primary_keyword,
                args.primary_path
            )
        }
        MsgKind::ListRule(args) => {
            format!("Keywords for current directory:\n{}", args.primary_keyword)
        }
        MsgKind::DeleteRule(args) => {
            format!(
                "{0} delete rule for keyword '{1}', target path '\x1b[4m{2}\x1b[0m\x1b[0m'?",
                status_symbol(&YN),
                args.primary_keyword,
                args.primary_path
            )
        }
        MsgKind::NoRuleShowAvailable(args) => {
            format!(
                "{} no such rule for the keyword in the current path. \nkeywords available for current path:\n{}",
                status_symbol(&NotFound),
                    args.primary_keyword.cyan(),
            )
        }

        MsgKind::RuleInfo(args) => {
            format!(
                "KEYWORD: {}\n SOURCE : \x1b[4m{}\x1b[0m\n TARGET : └─> \x1b[4m{}\x1b[0m \n",
                args.primary_keyword, args.primary_path, args.secondary_path
            )
        }

        MsgKind::UpdatingAlias(args) => {
            format!(
                "{} update alias '{}' -> '{}'?",
                status_symbol(&YN),
                args.primary_keyword,
                args.secondary_keyword
            )
        }
        MsgKind::UpdatedAlias(args) => {
            format!(
                "{} updated alias: {} -> {}",
                status_symbol(&Safe),
                args.primary_keyword,
                args.secondary_keyword
            )
        }
        MsgKind::AlreadyExistsTryEdit(args) => {
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
        ErrorKind::NotFoundAlias => "NOT FOUND: no rule for the alias found.".to_string(),
        ErrorKind::NotFoundRuleForPath => "NOT FOUND: no rule for the path found.".to_string(),
    }
}
