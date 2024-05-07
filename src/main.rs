use ign::{gitignore_repo_dir, gitignore_repo_not_found, run, Command, CommandOption};
use std::env;

fn help() {}

fn main() {
    let gitignore_dir = gitignore_repo_dir();
    let fts = env::args().nth(1);
    let ft_aliases = ign::ft_aliases();
    let def = CommandOption::default();
    match (gitignore_dir, fts) {
        (_, None) => {
            // no file type is given
            help()
        }
        (None, _) => gitignore_repo_not_found(),
        (Some(ig), Some(ft)) => run(Command {
            gitignore_dir: &ig,
            ft_str: &ft,
            ft_aliases: &ft_aliases,
            option: def,
        }),
    }
}
