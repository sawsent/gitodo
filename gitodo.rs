use std::process::{Command, exit};
use std::path::PathBuf;
use std::collections::HashMap;

type TDL = HashMap<String, Vec<String>>;
fn main() {
    if !is_in_git_worktree() {
        eprintln!("gitodo: Not in git repository");
        exit(1);
    }

    let args: Vec<String> = std::env::args().skip(1).collect();
    let cmd = args.first();

    let current_branch = current_branch();

    match cmd {
        Some(cmd) if (cmd == "add") => {
            let task = args.iter().skip(1).map(|s| s.to_owned()).collect::<Vec<_>>().join(" ");
            if !task.is_empty() {
                if let Some(new_tdl) = handle_add(&current_branch, &load_tdl(), task) {
                    save_tdl(&new_tdl);
                }
            } else {
                show_usage();
            }
        }

        Some(cmd) if (cmd == "done") => {
            if let Some(idx) = args.get(1).map(|i| i.parse::<usize>().ok()).flatten() {
                if let Some(new_tdl) = handle_done(&current_branch, &load_tdl(), idx - 1) {
                    save_tdl(&new_tdl);
                }
            } else {
                show_usage();
            }
        }

        Some(cmd) if (cmd == "ls") => {
            handle_ls(&load_tdl().get(&current_branch))
        }

        Some(cmd) if (cmd == "check") => {
            handle_check(&load_tdl().get(&current_branch))
        }
        _ => show_usage(),
    }
}

fn handle_add(branch: &str, tdl: &TDL, task: String) -> Option<TDL> {
    let mut new_tdl = tdl.clone();
    let mut new_tasks = if let Some(tasks) = tdl.get(branch) {
        tasks.clone()
    } else {
        Vec::new()
    };
    new_tasks.push(task);
    new_tdl.insert(branch.into(), new_tasks);
    Some(new_tdl)
}
fn handle_done(branch: &str, tdl: &TDL, idx: usize) -> Option<TDL> {
    let mut new_tdl = tdl.clone();

    if let Some(tasks) = new_tdl.get(branch) {
        if idx >= tasks.len() {
            eprintln!("gitodo: Task {} does not exist", idx);
            None
        } else {
            let mut new_tasks = tasks.clone();
            new_tasks.remove(idx);
            if new_tasks.is_empty() {
                new_tdl.remove(branch);
            } else {
                new_tdl.insert(branch.into(), new_tasks);
            }
            Some(new_tdl)
        }
    } else {
        eprintln!("Task {} does not exist", idx);
        None
    }
}
fn handle_ls(tasks: &Option<&Vec<String>>) {
    if let Some(tasks) = tasks {
        let mut idx = 1;

        for task in *tasks {
            println!("{}: {}", idx, task);
            idx = idx + 1;
        }
    }

}
fn handle_check(tasks: &Option<&Vec<String>>) {
    match tasks {
        Some(tasks) if !tasks.is_empty() => {
            let amount = tasks.len();
            println!("gitodo: Check failed. There are {} gitodos to complete.", amount);
        }
        _ => println!("gitodo: Success"),
    }
}

fn is_in_git_worktree() -> bool {
    execute("git rev-parse --is-inside-work-tree | tr -d '\n'") == "true".to_string()
}

fn data_fp() -> PathBuf {
    PathBuf::from(execute("git rev-parse --show-toplevel | tr -d '\n'")).join(".git").join(".gitodo")
}

fn current_branch() -> String {
    execute("git rev-parse --abbrev-ref HEAD | tr -d '\n'")
}

fn save_tdl(tdl: &TDL) {
    std::fs::write(&data_fp(), tdl_to_str(tdl)).expect("Unable to save gitodo list");
}

fn load_tdl() -> TDL {
    let file = &data_fp();
    if !file.exists() { HashMap::new() }
    else { str_to_tdl(&std::fs::read_to_string(file).expect("Unable to load gitodo list")) }
}

fn tdl_to_str(tdl: &HashMap<String, Vec<String>>) -> String {
    let mut builder = String::new();
    for (branch, tasks) in tdl {
        builder.push_str("[");
        builder.push_str(branch);
        builder.push_str("]\n");
        for task in tasks {
            builder.push_str(task);
            builder.push_str("\n");
        }
    }
    builder
}

fn str_to_tdl(s: &String) -> HashMap<String, Vec<String>> {
    let mut full_map: HashMap<String, Vec<String>> = HashMap::new();

    let split: Vec<&str> = s.split("\n").collect();

    let mut branch: Option<String> = None;
    let mut tasks: Vec<String> = Vec::new();
    for line in split {
        if line.starts_with('[') {
            if let Some(wpx) = line.strip_prefix("[") {
                if let Some(s) = wpx.strip_suffix("]") {
                    if let Some(b) = branch {
                        if !tasks.is_empty() {
                            full_map.insert(b, tasks.clone());
                            tasks = Vec::new();
                        }
                    }
                    branch = Some(s.into());
                }
            }
        } else {
            if !line.is_empty() {
                tasks.push(line.into());
            }
        }
    }
    if let Some(b) = branch {
        if !tasks.is_empty() {
            full_map.insert(b, tasks.clone());
        }
    }
    full_map
}

fn execute(cmd: &str) -> String {
    let output = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .output()
        .expect("Failed to run command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout.to_string()
}

const USAGE: &str = r#"
USAGE:

"#;
fn show_usage() {
    eprintln!("{}", USAGE)
}
