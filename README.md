# gitodo

Branch-scoped todo lists for Git repositories.

Stores todos in a single file inside `.git/.gitodo`.

- One todo list per branch  
- Single Rust source file (~200 LOC)  
- No dependencies  
- Plain-text storage  

gitodo is a small CLI tool you can read and modify quickly. It keeps lightweight, local reminders tied to the branch you're working on.

---

## What it is

gitodo lets you attach simple todo lists to Git branches.

When you switch branches, your todos switch with you.

It’s useful for short-lived, local reminders like:

- cleanup left after debugging
- TODOs tied to a feature branch
- small follow-ups before merging or pushing

---

## Design goals

- Stay out of the working tree (`.git/.gitodo`)
- No external dependencies
- Human-readable storage format
- Branch-isolated state
- Easy to understand implementation

---

## Example

```sh
$ git checkout feature/login

$ gitodo add remove magic string in file
added: remove magic string in file

$ gitodo
1: remove magic string in file

$ git checkout main

$ gitodo
No todos.

$ git checkout feature/login

$ gitodo
1: remove magic string in file

$ gitodo done 1
done: remove magic string in file

$ gitodo
No todos.
```

---

## Usage

```sh
gitodo                List todos for current branch
gitodo add <task>     Add a todo
gitodo done <n>       Mark todo <n> as done
gitodo done all       Clear all todos for branch
gitodo check          Exit non-zero if any todos exist
```

---

## CI usage

`gitodo check` can be used in scripts to ensure no pending todos remain:

```sh
$ gitodo check
gitodo: check failed (2 todos remaining)
```

Exit code is non-zero when todos are present.

---

## Storage format

Todos are stored in:

```text
.git/.gitodo
```

In editable format:

```text
[feature/branch1]
Task 1
Task 2

[feature/branch2]
Task 1
```

Because it lives under `.git/`:

- it is not tracked by Git
- it does not appear in `git status`
- it does not modify the working tree

---

## Installation

No Cargo required.

```sh
curl -O https://raw.githubusercontent.com/sawsent/gitodo/refs/tags/v0.1.2/gitodo.rs
rustc gitodo.rs -o gitodo
mv gitodo ~/.local/bin/
```

---

## Limitations

- Branch names are read as-is, no parsing
- This tool assumes branch names are stable identifiers
- No synchronization or remote behavior
- No cleanup on branch deletion
- Single line todos only

---

## License

Apache 2.0
