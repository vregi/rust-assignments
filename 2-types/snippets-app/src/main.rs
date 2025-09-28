use std::env;
use std::fs;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::process;

const STORE_DIR: &str = ".snippets";

enum Action {
    Create(String), 
    Read(String),   
    Delete(String),
    List,
    Help,
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
        process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let action = parse_args().unwrap_or(Action::Help);

    match action {
        Action::Create(name) => create_snippet(&name),
        Action::Read(name) => read_snippet(&name),
        Action::Delete(name) => delete_snippet(&name),
        Action::List => list_snippets(),
        Action::Help => {
            print_help();
            Ok(())
        }
    }
}

fn parse_args() -> Option<Action> {
    let mut args = env::args().skip(1);
    let mut action: Option<Action> = None;

    while let Some(arg) = args.next() {
        let mut next_is_name = || -> Result<String, String> {
            args.next().ok_or_else(|| format!("Missing value after {}", arg))
        };

        let new_action = match arg.as_str() {
            "--name" => Action::Create(next_is_name().ok()?),
            "--read" => Action::Read(next_is_name().ok()?),
            "--delete" => Action::Delete(next_is_name().ok()?),
            "--list" => Action::List,
            "--help" | "-h" => Action::Help,
            other => {
                eprintln!("Unknown flag: {}", other);
                return Some(Action::Help);
            }
        };

        if action.is_some() && !matches!(new_action, Action::Help) {
            eprintln!("Please use exactly one of --name/--read/--delete/--list.");
            return Some(Action::Help);
        }
        action = Some(new_action);
    }

    action
}

fn create_snippet(name: &str) -> Result<(), String> {
    ensure_store_dir().map_err(to_s)?;
    let path = path_for(name);


    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).map_err(to_s)?;

    if buf.is_empty() {
        return Err("No input on stdin. Pipe or type your snippet, e.g.:\n  echo \"code\" | ./snippets-app --name \"My Snippet\"".into());
    }

    fs::write(&path, buf).map_err(to_s)?;

    eprintln!("Saved snippet: {}", display_name_from_path(&path));
    Ok(())
}

fn read_snippet(name: &str) -> Result<(), String> {
    let path = path_for(name);
    let content = fs::read_to_string(&path).map_err(|_| format!("Snippet not found: {}", name))?;
   
    print!("{}", content);
    io::stdout().flush().map_err(to_s)?;
    Ok(())
}

fn delete_snippet(name: &str) -> Result<(), String> {
    let path = path_for(name);
    if path.exists() {
        fs::remove_file(&path).map_err(to_s)?;
        eprintln!("Deleted snippet: {}", display_name_from_path(&path));
        Ok(())
    } else {
        Err(format!("Snippet not found: {}", name))
    }
}

fn list_snippets() -> Result<(), String> {
    let dir = Path::new(STORE_DIR);
    if !dir.exists() {
        eprintln!("No snippets yet. Create one with:\n  echo \"code\" | ./snippets-app --name \"My Snippet\"");
        return Ok(());
    }

    let mut names: Vec<String> = fs::read_dir(dir)
        .map_err(to_s)?
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().map(|t| t.is_file()).unwrap_or(false))
        .filter_map(|e| e.file_name().into_string().ok())
        .collect();

    names.sort_unstable();

    for n in names {
        println!("{}", n);
    }
    Ok(())
}

fn ensure_store_dir() -> io::Result<()> {
    fs::create_dir_all(STORE_DIR)
}

fn path_for(name: &str) -> PathBuf {
    let safe = sanitize_name(name);
    Path::new(STORE_DIR).join(safe)
}

fn sanitize_name(name: &str) -> String {
    let mut out = String::with_capacity(name.len());
    for ch in name.trim().chars() {
        match ch {
            '<' | '>' | ':' | '"' | '/' | '\\' | '|' | '?' | '*' => out.push('_'),
            c if c.is_control() => out.push('_'),
            _ => out.push(ch),
        }
    }
    if out.is_empty() {
        "untitled".to_string()
    } else {
        out
    }
}

fn display_name_from_path(p: &Path) -> String {
    p.file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("<unknown>")
        .to_string()
}

fn print_help() {
    let help = r#"snippets-app — tiny CLI to store and reuse code snippets

USAGE:
  Create (content from stdin):
    echo "println!(\"hi\");" | ./snippets-app --name "Hello example"

  Read:
    ./snippets-app --read "Hello example"

  Delete:
    ./snippets-app --delete "Hello example"

  List:
    ./snippets-app --list

NOTES:
  • Snippets are saved as plain text files under ./.snippets/
  • Names are lightly sanitized to be safe as filenames.
"#;
    eprintln!("{}", help);
}

fn to_s<E: std::fmt::Display>(e: E) -> String {
    e.to_string()
}
