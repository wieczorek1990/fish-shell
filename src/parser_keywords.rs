//! Functions having to do with parser keywords, like testing if a function is a block command.

use crate::wchar::prelude::*;

const SKIP_KEYWORDS: &[&wstr] = &[L!("else"), L!("begin")];
const SUBCOMMAND_KEYWORDS: &[&wstr] = &[
    L!("and"),
    L!("begin"),
    L!("builtin"),
    L!("command"),
    L!("exec"),
    L!("if"),
    L!("not"),
    L!("or"),
    L!("time"),
    L!("while"),
];
const BLOCK_KEYWORDS: &[&wstr] = &[
    L!("begin"),
    L!("for"),
    L!("function"),
    L!("if"),
    L!("switch"),
    L!("while"),
];

// Don't forget to add any new reserved keywords to the documentation
const RESERVED_KEYWORDS: &[&wstr] = &[
    L!("["),
    L!("_"),
    L!("argparse"),
    L!("break"),
    L!("case"),
    L!("continue"),
    L!("else"),
    L!("end"),
    L!("eval"),
    L!("read"),
    L!("return"),
    L!("set"),
    L!("status"),
    L!("string"),
    L!("test"),
];

// The lists above are purposely implemented separately from the logic below, so that future
// maintainers may assume the contents of the list based off their names, and not off what the
// functions below require them to contain.

/// Tests if the specified commands parameters should be interpreted as another command, which will
/// be true if the command is either 'command', 'exec', 'if', 'while', or 'builtin'.  This does not
/// handle "else if" which is more complicated.
pub fn parser_keywords_is_subcommand(cmd: &wstr) -> bool {
    SUBCOMMAND_KEYWORDS.contains(&cmd) || SKIP_KEYWORDS.contains(&cmd)
}

/// Tests if the specified command is a reserved word, i.e. if it is the name of one of the builtin
/// functions that change the block or command scope, like 'for', 'end' or 'command' or 'exec'.
/// These functions may not be overloaded, so their names are reserved.
pub fn parser_keywords_is_reserved(word: &wstr) -> bool {
    SUBCOMMAND_KEYWORDS.contains(&word)
        || SKIP_KEYWORDS.contains(&word)
        || BLOCK_KEYWORDS.contains(&word)
        || RESERVED_KEYWORDS.contains(&word)
}
