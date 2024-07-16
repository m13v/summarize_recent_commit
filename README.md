[Demo:](https://github.com/user-attachments/assets/7a6fa4d5-f77f-4493-a652-6f3e464ddf95)

# Summarize commits of your team members with a single command

I hate reading thousands of commits of my team. It takes a lot of time, the commit messages are often incomplete, the files diffs don't give immediate understanding of the commits logic. Also, copying commit manually to chat gpt is frustrating. 

Solved! Use your normal git command to summarize all commits of your team in a single markdown text file

## Next steps

If you like it, tell me, I'll package it into a npm/brew library to make it easier to use.
Need a feature? Tell me
[Didn't work?](https://github.com/m13v/summarize_recent_commit/issues/new?assignees=&labels=dislike&template=dislike.yml&title=installation+didnt+work)

## Give it a Star!

If you find this project useful, please give it a star! It helps us to grow and improve.
[![GitHub stars](https://img.shields.io/github/stars/m13v/summarize_recent_commit.svg?style=social&label=Star)](https://github.com/m13v/summarize_recent_commit/stargazers)


## Reach out: 

i@m13v.com. Discord: matthew.ddy

## Getting started

Install [Rust](https://www.rust-lang.org/tools/install).
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Clone the repo:
```bash
git clone https://github.com/m13v/summarize_recent_commit.git
```

Set up you OPENAI API KEY in .env
```bash
echo "OPENAI_API_KEY=XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX" > .env
```

Build it
```bash
cargo build --release   # enter cloned repo
```

Run it # enter path of your git project repo (# under the hood it runs a separate command to get details of each commit)
```bash
cargo run --release /Users/matthewdi/Desktop/screenpipe/screen-pipe git log HEAD..origin/main  
```

## Overview

Summarize Recent Commit is a tool that helps you quickly understand the changes made in the most recent commit of your project. It provides a concise summary of the commit message, files changed, and the impact of those changes.

## Features

- **Commit Summary**: Get a brief overview of the latest commit.
- **File Changes**: See which files were added, modified, or deleted.
- **Impact Analysis**: Understand the potential impact of the changes.


Written in Rust

Matthew Diakonov