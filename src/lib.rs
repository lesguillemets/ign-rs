use std::collections::HashMap;
use std::default::Default;
use std::env;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::{stdout, BufReader, BufWriter};
use std::path::Path;

pub const DEBUG: bool = false;
static DEFAULT_GITIGNORE_LOC_FROM_HOME: &str = ".local/share/gitignore";

pub fn ft_aliases() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        ("hs", "haskell"),
        ("pl", "perl"),
        ("py", "python"),
        ("rb", "ruby"),
        ("rs", "rust"),
        ("latex", "tex"),
    ])
}

pub fn run(c: Command) {
    // gitignore_dir: &str, ftstr: &str, ft_aliases: &HashMap<&str, &str>, write_to_file: bool) {
    let ft: &str = if let Some(t) = c.ft_aliases.get(c.ft_str) {
        // if there's a defined aliase for ftstr, prefer that
        t
    } else {
        &(c.ft_str.to_lowercase())
    };
    eprintln!(
        "by {}, searching for {} from {}",
        c.ft_str, ft, c.gitignore_dir
    );
    if let Some(f) = search_gitignore_file(c.gitignore_dir, ft) {
        // we have found the .gitignore!
        if DEBUG {
            eprintln!("Found {:?}", f)
        };
        if c.option.write_to_file {
            add_gitignore_from(f);
        } else {
            print_gitignore_from(f);
        }
    } else {
        eprintln!("gitignore not found for {}: aborting", ft)
    }
}
fn add_gitignore_from(the_gitignore: File) {
    let mut bufreader = BufReader::new(the_gitignore);
    let mut content = vec![];
    let _ = bufreader.read_to_end(&mut content).unwrap();
    if DEBUG {
        eprintln!("adding {:?}", content);
    }
    let local_gitignore = local_gitignore_or_create();
    if DEBUG {
        eprintln!("{:?}", local_gitignore);
    }
    let mut bufwriter = BufWriter::new(local_gitignore);
    bufwriter.write_all(&content).unwrap();
    bufwriter.flush().unwrap();
}

fn search_gitignore_file(dir: &str, ft: &str) -> Option<File> {
    // for example, "haskell.gitignore"
    // TODO: skip. git dir
    let target_filename = format!("{}.gitignore", ft);
    for entry in fs::read_dir(dir).ok()? {
        let entry = entry.ok()?;
        if entry.file_name().to_str()?.to_lowercase() == target_filename {
            return File::open(entry.path()).ok();
        } else if entry.file_type().ok()?.is_dir() {
            // dig deeper into directory
            if DEBUG {
                eprintln!(
                    "debug: trying to read this file {:?}",
                    entry.path().to_str()
                );
            }
            if let Some(f) = search_gitignore_file(entry.path().to_str()?, ft) {
                return Some(f);
            }
        }
    }
    None
}

pub fn gitignore_repo_not_found() {
    eprintln!(
        "\
        The direcrtory where .gitignore-s are stored is not found:\n\
        (0) You have to manually clone it somewhere:\n\
            `git clone https://github.com/github/gitignore`\n\
        (1) Use the default '~/.local/share/gitignore', or \n\
        (2) Specify the location with GITIGNORE_REPO_DIR\
        "
    );
}

pub fn gitignore_repo_dir() -> Option<String> {
    let mut default_dir = home::home_dir();
    default_dir
        .as_mut()
        .map(|d| d.push(DEFAULT_GITIGNORE_LOC_FROM_HOME));
    let path = env::var("GITIGNORE_REPO_DIR").unwrap_or(
        default_dir
            .unwrap_or_default()
            .into_os_string()
            .into_string()
            .unwrap_or_default(),
    );
    if DEBUG {
        println!("DEBUG: gitignore repo dir found to be: {:?}", path);
    }
    if Path::exists(path.as_ref()) {
        Some(path)
    } else {
        None
    }
}

fn local_gitignore_or_create() -> File {
    if let Ok(f) = OpenOptions::new().append(true).open(".gitignore") {
        f
    } else {
        File::create(".gitignore").unwrap()
    }
}

fn print_gitignore_from(the_gitignore: File) {
    let out = stdout();
    let mut out = BufWriter::new(out.lock());
    for line in BufReader::new(the_gitignore).lines() {
        writeln!(out, "{}", line.unwrap()).unwrap();
    }
}

pub struct Command<'a> {
    pub gitignore_dir: &'a str,
    pub ft_str: &'a str,
    pub ft_aliases: &'a HashMap<&'a str, &'a str>,
    pub option: CommandOption,
}

pub struct CommandOption {
    write_to_file: bool,
}
impl Default for CommandOption {
    fn default() -> Self {
        CommandOption {
            write_to_file: false,
        }
    }
}
