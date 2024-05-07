use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::path::Path;

const DEBUG: bool = false;
static DEFAULT_GITIGNORE_LOC_FROM_HOME: &str = ".local/share/gitignore";

fn gitignore_repo_dir() -> Option<String> {
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

fn run(gitignore_dir: &str, ftstr: &str, ft_aliases: &HashMap<&str, &str>) {
    let ft: &str = if let Some(t) = ft_aliases.get(ftstr) {
        // if there's a defined aliase for ftstr, prefer that
        t
    } else {
        &(ftstr.to_lowercase())
    };
    println!("by {}, searching for {} from {}", ftstr, ft, gitignore_dir);
    if let Some(f) = search_gitignore_file(gitignore_dir, ft) {
        if DEBUG {
            println!("Found {:?}", f)
        };
        add_gitignore_from(f)
    } else {
        println!("gitignore not found for {}: aborting", ft);
        return;
    }
}

fn add_gitignore_from(f: File) {
    let mut bufreader = BufReader::new(f);
    let mut content = vec![];
    let _ = bufreader.read_to_end(&mut content).unwrap();
    if DEBUG {
        println!("adding {:?}", content);
    }
    let local_gitignore = local_gitignore_or_create();
    if DEBUG {
        println!("{:?}", local_gitignore);
    }
    let mut bufwriter = BufWriter::new(local_gitignore);
    bufwriter.write_all(&content).unwrap();
    bufwriter.flush().unwrap();
}

fn search_gitignore_file(dir: &str, ft: &str) -> Option<File> {
    // for example, "haskell.gitignore"
    let target_filename = format!("{}.gitignore", ft);
    for entry in fs::read_dir(dir).ok()? {
        let entry = entry.ok()?;
        if entry.file_name().to_str()?.to_lowercase() == target_filename {
            return File::open(entry.path()).ok();
        }
    }
    None
}

fn gitignore_repo_not_found() {
    println!(
        "\
        The direcrtory where .gitignore-s are stored is not found:\n\
        (0) You have to manually clone it somewhere:\n\
            `git clone https://github.com/github/gitignore`\n\
        (1) Use the default '~/.local/share/gitignore', or \n\
        (2) Specify the location with GITIGNORE_REPO_DIR\
        "
    );
}

fn help() {}

fn main() {
    let gitignore_dir = gitignore_repo_dir();
    let fts = env::args().nth(1);
    let ft_aliases = ign::ft_aliases();
    match (gitignore_dir, fts) {
        (_, None) => {
            // no file type is given
            help()
        }
        (None, _) => gitignore_repo_not_found(),
        (Some(ig), Some(ft)) => run(&ig, &ft, &ft_aliases),
    }
}
