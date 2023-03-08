<h1 align="center">gelp.rs 🛠️</h1>

This tool was written to help me with writing commands like:

```bash
$ git reset HEAD~2
```

They would either strain my fingers because it would require pressing some tilde or they would just get repetitive.

# Installation

Clone this repo and the command below while being in the gelper directory:

```bash
$ cargo install --path ./
```

# Commands

```bash
$ gelp go-back <n>                 # => git reset HEAD~n
$ gelp overwrite-last <change_msg> # => git commit -a --amend --no-edit
$ gelp clean                       # => git clean -fdx
```
