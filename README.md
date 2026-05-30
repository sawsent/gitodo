# gitodo

Branch-scoped todo lists for Git repositories.

* Stores todos inside `.git/.gitodo`
* Keeps them out of commits and the working tree
* Separate todo list per branch
* Single Rust source file
* No dependencies

gitodo is intentionally tiny: one Rust file, roughly 200 lines of code, that you can read, understand, and modify in a few minutes.

Completely local, tasks don't follow your repository.

## Why?

Sometimes you just need a small list of tasks tied to the work you're doing right now.

gitodo stores todos inside `.git/`, making them local to the repository and invisible to Git.

Useful for:
* Remembering cleanup tasks before pushing and queuing pipelines
* Maintaining local notes that should never be committed
* Blocking CI or scripts until all tasks are completed

## Properties

* ~200 lines of Rust
* 1 source file
* 0 dependencies
* Plain-text storage
* Branch-scoped task lists

## Example

Tasks follow the branch they belong to.

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

## Usage

```sh
gitodo                List all todos for the current branch
gitodo add <task>     Add a new todo task to the current branch
gitodo done <n>       Mark todo number <n> as done (removes it)
gitodo done all       Remove all todos for the current branch
gitodo check          Exit with a message if any todos remain; succeed if none
```

## CI Example

Use `gitodo check` to fail a script when unfinished tasks remain:

```sh
$ gitodo check
gitodo: Check failed. There are 1 gitodos to complete.
```

For me, this makes it easy to prevent queuing pipelines multiple times because of tiny forgotten changes.

When your pipeline takes a while to run, that gets frustrating very fast.

## Installation

No Cargo required.

```sh
curl -O https://raw.githubusercontent.com/sawsent/gitodo/refs/tags/v0.1.2/gitodo.rs
rustc gitodo.rs -o gitodo
mv gitodo ~/.local/bin/
```

## How it works

gitodo stores all tasks in a single plain-text file:

```text
.git/.gitodo
```

Tasks are grouped by branch name.

Because the file lives inside `.git/`:

* It is never committed
* It does not appear in `git status`
* It does not affect the working tree
* Switching branches automatically switches todo lists

No database. No configuration. No hidden state outside the repository.

## License

Apache 2.0
