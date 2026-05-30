# gitodo

Branch-scoped todo lists for git repositories. Todos are stored per branch inside `.git/.gitodo`, keeping them out of your working tree and index.

Tiny — a single `.rs` file, no dependencies.

## Usage

```
gitodo                List all todos for the current branch
gitodo add <task>     Add a new todo task to the current branch
gitodo done <n>       Mark todo number <n> as done (removes it)
gitodo done all       Remove all todos for the current branch
gitodo check          Exit with a message if any todos remain; succeed if none
```

## Installation

```sh
curl -O https://raw.githubusercontent.com/sawsent/gitodo/refs/tags/v0.1.1/gitodo.rs
rustc gitodo.rs -o gitodo
mv gitodo ~/.local/bin/
```

## Examples

```sh
# Add tasks while working on a feature branch
$ gitodo add write unit tests
$ gitodo add update documentation

# List open tasks
$ gitodo
1: write unit tests
2: update documentation

# Complete a task
$ gitodo done 1
$ gitodo
1: update documentation

# Gate a CI step on having no open todos
$ gitodo check
gitodo: Check failed. There are 1 gitodos to complete.
```

## How it works

Todos are stored in `.git/.gitodo` as a plain-text file grouped by branch name. Because the file lives inside `.git/`, it is never committed and does not affect your working tree or index.

## Licence
Apache 2.0 License
