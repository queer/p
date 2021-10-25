use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

use ansi_term::Colour;

type Result<T> = core::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        if let Some("source") = args.get(1).map(|s| s.as_str()) {
            print_fish_source();
            return Ok(());
        }
    }
    let git_status = match git2::Repository::discover(".") {
        Ok(repo) => {
            let name = get_current_branch(&repo)?;
            let status = get_git_status(&repo)?;
            // Format git status in the format:
            //     git:(branch_name) <number of pending changes>
            // Example:
            //     git:(mistress) 4
            Some(format!(
                "{}{}{}{}",
                Colour::Blue.bold().paint("git:("),
                Colour::Red.bold().paint(name.trim()),
                Colour::Blue.bold().paint(")"),
                Colour::Yellow.bold().paint(status)
            ))
        }
        Err(_e) => None,
    };

    // TODO: Read last command status from P_LAST_CMD_STATUS
    let arrow = Colour::Red.bold().paint("▶");

    if let Some(repo_text) = git_status {
        print!("{} {} ", repo_text, Colour::Blue.bold().paint("|"));
    }
    print!("{}  ", arrow);

    Ok(())
}

fn get_current_branch(repo: &git2::Repository) -> Result<String> {
    match repo.head() {
        Ok(head) => {
            let name = head.name();
            match name {
                Some(name) => Ok(name.to_string().replace("refs/heads/", "")),
                None => Ok("unknown branch".to_string()),
            }
        }
        Err(_) => {
            // If we can't read the repo name correctly, try to parse it from
            // .git/HEAD
            let mut buf = repo.path().to_path_buf();
            buf.push("HEAD");
            let repo_path = buf.as_path();
            let head_file = File::open(repo_path)?;
            let mut reader = BufReader::new(head_file);
            let mut head_line = String::new();
            reader.read_line(&mut head_line)?;
            head_line = head_line.replace("ref: refs/heads/", "");
            Ok(head_line)
        }
    }
}

fn get_git_status(repo: &git2::Repository) -> Result<String> {
    let modified_files = repo
        .statuses(Some(git2::StatusOptions::new().include_untracked(true)))?
        .iter()
        .map(|s| !s.status().is_ignored())
        .count();
    if modified_files == 0 {
        Ok(String::new())
    } else {
        Ok(format!(" {}", modified_files.to_string(),))
    }
}

fn print_fish_source() {
    println!(
        r#"
function fish_prompt
    set P_LAST_CMD_STATUS $status
    p
end
    "#
    );
}
