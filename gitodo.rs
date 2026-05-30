// Copyright 2026 sawsent
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::collections::HashMap;
use std::path::PathBuf;
use std::process::{Command, exit};

const USAGE: &str = r#"gitodo - branch-scoped todo list for git repositories

USAGE:
    gitodo                List all todos for the current branch
    gitodo add <task>     Add a new todo task to the current branch
    gitodo done <n>       Mark todo number <n> as done (removes it)
    gitodo done all       Remove all todos for the current branch
    gitodo check          Exit with a message if any todos remain; succeed if none
    "#;

type TDL = HashMap<String, Vec<String>>;

enum Result {
    Usage,
    Display(String, bool),
    Save(TDL),
    NoOp,
}
fn main() {
    if !is_in_git_worktree() {
        eprintln!("gitodo: Not in git repository");
        exit(1);
    }

    let args: Vec<String> = std::env::args().skip(1).collect();
    let cmd = args.first();
    let save_file = data_fp();

    let result = match cmd {
        None => handle_ls(&load_tdl(&save_file).get(&current_branch())),
        Some(cmd) if (cmd == "add") => {
            let task = args.iter().skip(1).map(|s| s.to_owned()).collect::<Vec<_>>().join(" ");
            handle_add(&current_branch(), &load_tdl(&save_file), task)
        }
        Some(cmd) if (cmd == "done") => handle_done(&current_branch(), &load_tdl(&save_file), args.get(1)),
        Some(cmd) if (cmd == "check") => handle_check(&load_tdl(&save_file).get(&current_branch())),
        _ => Result::Usage,
    };

    match result {
        Result::Save(tdl) => save_tdl(&tdl, &save_file),
        Result::Display(msg, false) => println!("{}", msg),
        Result::Display(msg, true) => {
            eprintln!("{}", msg);
            exit(1);
        }
        Result::Usage => eprintln!("{}", USAGE),
        Result::NoOp => (),
    }
}

// HANDLERS
fn handle_add(branch: &str, tdl: &TDL, task: String) -> Result {
    if task.is_empty() {
        return Result::Usage;
    }
    let mut new_tdl = tdl.clone();
    let mut new_tasks = tdl.get(branch).map(|t| t.clone()).unwrap_or(Vec::new());
    new_tasks.push(task);
    new_tdl.insert(branch.into(), new_tasks);
    Result::Save(new_tdl)
}
fn handle_done(branch: &str, tdl: &TDL, idx_str_opt: Option<&String>) -> Result {
    match idx_str_opt {
        Some(all) if all == "all" => {
            let mut new_tdl = tdl.clone();
            new_tdl.remove(branch);
            return Result::Save(new_tdl);
        }
        _ => ()
    }
    let idx_opt = idx_str_opt.map(|i| i.parse::<usize>().ok()).flatten();
    if idx_opt.is_none() || idx_opt.unwrap() < 1 {
        return Result::Usage;
    }

    let idx = idx_opt.unwrap() - 1;
    match tdl.get(branch) {
        Some(tasks) if idx < tasks.len() => {
            let mut new_tdl = tdl.clone();
            let mut new_tasks = tasks.clone();
            new_tasks.remove(idx);
            if new_tasks.is_empty() {
                new_tdl.remove(branch);
            } else {
                new_tdl.insert(branch.into(), new_tasks);
            }
            Result::Save(new_tdl)
        }
        _ => Result::Display(format!("gitodo: Task {} does not exist", idx + 1), true),
    }
}
fn handle_ls(tasks: &Option<&Vec<String>>) -> Result {
    if tasks.is_none() {
        return Result::NoOp;
    }

    let mut idx = 1;
    let mut builder = String::new();
    for task in tasks.unwrap() {
        builder.push_str(&format!("{}: {}\n", idx, task));
        idx = idx + 1;
    }
    let out = builder.strip_suffix("\n").unwrap_or("").to_string();
    Result::Display(out, false)
}
fn handle_check(tasks: &Option<&Vec<String>>) -> Result {
    match tasks {
        Some(tasks) if !tasks.is_empty() => {
            let amount = tasks.len();
            Result::Display(format!("gitodo: Check failed. There are {} gitodos to complete.", amount), true)
        }
        _ => Result::NoOp,
    }
}

// HELPERS
fn execute(args: &[&str]) -> String {
    let output = Command::new("git")
        .args(args)
        .output()
        .expect("Failed to run git command");

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        panic!("git command failed: git {}\n{}", args.join(" "), stderr);
    }

    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

fn is_in_git_worktree() -> bool {
    Command::new("git")
        .args(["rev-parse", "--is-inside-work-tree"])
        .output()
        .map(|o| o.status.success() && String::from_utf8_lossy(&o.stdout).trim() == "true")
        .unwrap_or(false)
}

fn data_fp() -> PathBuf {
    PathBuf::from(execute(&["rev-parse", "--show-toplevel"])).join(".git").join(".gitodo")
}

fn current_branch() -> String {
    execute(&["rev-parse", "--abbrev-ref", "HEAD"])
}

// SERDE
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

// STORAGE
fn save_tdl(tdl: &TDL, file: &PathBuf) {
    std::fs::write(file, tdl_to_str(tdl)).expect("Unable to save gitodo list");
}

fn load_tdl(file: &PathBuf) -> TDL {
    if !file.exists() {
        HashMap::new()
    } else {
        str_to_tdl(&std::fs::read_to_string(file).expect("Unable to load gitodo list"))
    }
}
