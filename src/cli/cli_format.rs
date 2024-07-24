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

pub struct ConfirmationResult {
    pub bool: bool,
    pub message: String,
}

// category - action - condition
pub enum MsgKind {
    ChangeTargetPath(MsgArgs),
    ChangeTargetKeyword(MsgArgs),
    NoKeywordOrPathForReplace(MsgArgs),
    ImportFromPath(MsgArgs),
    ImportYN,
    ImportedRules,
    KeywordAndTarget(MsgArgs),
    AliasExisting(MsgArgs),
    AliasUpdating(MsgArgs),
    AliasUpdated(MsgArgs),
    DeleteRule(MsgArgs),
    DeletedRule,
    NoRuleShowAvailable(MsgArgs),
    ListRule(MsgArgs),
    RuleInfo(MsgArgs),
    RuleNonExistsForKeyword,
    AlreadyExistsTryEdit(MsgArgs),
    ActualPathNonExists(MsgArgs),
    ActualPathWillBeCreated,
    PathNotProvided(MsgArgs),
    AddedRule,
    PathNonExistsCreating(MsgArgs),
    DisplaySource(MsgArgs),
    DisplayTarget(MsgArgs),
    AlreadyExistsInTarget(MsgArgs),
    NoItemsToMoveInSource,
    SimpleDone,
}

pub fn msg_format(kind: MsgKind) -> String {
    match kind {
        MsgKind::ChangeTargetPath(args) => {
            format!(
        "{0} change target path '\x1b[4m{1}\x1b[0m\x1b[0m' -> '\x1b[4m{2}\x1b[0m\x1b[0m' for keyword '{3}'?",
        status_symbol(&YN), args.primary_path, args.secondary_path, args.primary_keyword)
        }
        MsgKind::ChangeTargetKeyword(args) => {
            format!(
                "{0} change keyword '{1}' -> '{2}'?",
                status_symbol(&YN),
                args.primary_keyword,
                args.secondary_keyword
            )
        }
        MsgKind::ActualPathNonExists(args) => {
            format!(
                "{0} change path '{1}' -> '{2}'? \n(path '{3}' doesn't exist on system. the new path will be created when moving files).",
                status_symbol(&YN),
                args.primary_path,
                args.secondary_path,
                args.secondary_path,
            )
        }
        MsgKind::ActualPathWillBeCreated => {
            "Note: the actual directory doesn't exist yet. it will be created later when the files are moved.".to_string()
        }
        MsgKind::NoKeywordOrPathForReplace(args) => {
            format!("please add a new keyword or path for replacement.\nCURRENT:\n keyword - {},\n target - {}", args.primary_keyword, Utf8PathBuf::from(args.primary_path))
        }
        MsgKind::ImportFromPath(args) => {
            format!("from \x1b[4m{}\x1b[0m\x1b[0m?", args.primary_path)
        }
        MsgKind::ImportYN => {
            format!("{} do you want to import rules: ", status_symbol(&YN))
        }
        MsgKind::ImportedRules => {
            "rules imported.".to_string()
        }
        MsgKind::AliasExisting(args) => {
            format!(
                "{} '{}' is an existing alias for path '\x1b[4m{}\x1b[0m\x1b[0m'",
                status_symbol(&Error),
                args.primary_keyword,
                args.primary_path
            )
        }
        MsgKind::KeywordAndTarget(args) => {
            format!(" - keyword: {}, target path: \x1b[4m{}\x1b[0m\x1b[0m", args.primary_keyword, args.primary_path)
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
        MsgKind::DeletedRule => {
            "deleted rule successfully.".to_string()
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

        MsgKind::AliasUpdating(args) => {
            format!(
                "{} update alias '{}' -> '{}'?",
                status_symbol(&YN),
                args.primary_keyword,
                args.secondary_keyword
            )
        }
        MsgKind::AliasUpdated(args) => {
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
        MsgKind::PathNotProvided(args) => {
            format!("{} target path not provided. make a new target path to the keyword in the current directory? ({})",status_symbol(&YN), args.primary_path)
        }
        MsgKind::AddedRule => {
            "rule added.".to_string()
        }
        MsgKind::RuleNonExistsForKeyword => {
            "no such rule for the keyword".to_string()
        }
        MsgKind::PathNonExistsCreating(args) => {
            format!(
            "{} \x1b[0;33mtarget path '{}' doesn't exist. Creating the directory...\x1b[0m",
            status_symbol(&Caution), args.primary_path)
        }
        MsgKind::DisplaySource(args) => {
            format!("\nSOURCE: \x1b[4m{}\x1b[0m\x1b[0m", args.primary_path)
        }
        MsgKind::DisplayTarget(args) => {
            format!("\r└→ TARGET: \x1b[4m{}\x1b[0m\x1b[0m ", args.primary_path)
        }
        MsgKind::AlreadyExistsInTarget(args) => {
            format!("  {0} {1} '{2}' \x1b[4malready exists.\x1b[0m\x1b[0m", status_symbol(&Caution), args.secondary_keyword, args.primary_path)
        }
        MsgKind::NoItemsToMoveInSource => {
            format!("{} No items to move.", status_symbol(&Safe))
        }
        MsgKind::SimpleDone => {
            "Done.".to_string()
        }
    }
}

pub enum ErrorKind {
    InvalidAlias,
    NotFoundAlias,
    NotFoundRuleForPath,
    PathShouldBeGiven,
    NoRuleForPath,
    CreateTargetDirFail,
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
        ErrorKind::PathShouldBeGiven => "INVALID INPUT: target path should be given.".to_string(),
        ErrorKind::NoRuleForPath => "NOT FOUND: no rule for the current path".to_string(),
        ErrorKind::CreateTargetDirFail => {
            "Error: failed to create target directory on disk.".to_string()
        }
    }
}
