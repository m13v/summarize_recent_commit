[Demo:](https://github.com/user-attachments/assets/7a6fa4d5-f77f-4493-a652-6f3e464ddf95)

# Summarize commits of your team members with a single command

I hate reading through thousands of commits from my team. It takes a lot of time, the commit messages are often incomplete, and the file diffs don't provide an immediate understanding of the commit logic. Additionally, copying commits manually to ChatGPT is frustrating.

Solved! Use your normal git commands to summarize all commits from your team into a single markdown text file.

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
brew install rust # or curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
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
cargo build --release
```

Run it # enter path of your git project repo (# under the hood it runs a separate command to get details of each commit)
one-line command:
```bash
cargo run --release -- /Users/matthewdi/Desktop/screenpipe/screen-pipe "git log HEAD..origin/main"
```
or:
```bash
cargo run --release 
/Users/matthewdi/Desktop/screenpipe/screen-pipe
git log HEAD..origin/main
```

## Overview

Summarize Recent Commit is a tool that helps you quickly understand the changes made in the most recent commit of your project which are ahead of your local directory. It provides a concise summary of the commit message, files changed, and the impact of those changes.

## Features

- **Commit Summary**: Get a brief overview of the latest commit.
- **File Changes**: See which files were added, modified, or deleted.
- **Impact Analysis**: Understand the potential impact of the changes.

## TO-DO

- [x] Overall summary for the commits ahead
- [x] Parallel execution
- [x] Ask user for missing arguments in a CLI dialogue
- [x] Debug: appears to be cut off sometimes, expand token limit
- [x] which command is running atm?
- [x] to fix hash issue, handle invalid hashes
- [x] fatal error with git command
- [x] maintain state of commits that were already processed, and last fetch date
- [ ] ask to refetch when running to avoid rerunning commits
- [ ] scroll to the top when opening summary file by default
- [ ] return cursor when finishing the summary
- [ ] (number 1 priority) Auto-trigger summaries daily
- [ ] Error summarizing changes for aefc2769dc98dc9aa6028fe8c3ead977f71cbc65: Failed to send request: error sending request for url (https://api.openai.com/v1/chat/completions): error trying to connect: connection closed via error
- [ ] Error summarizing changes for d6fc795c624988fa3244d5737488ec7610f7a215: Failed to extract content
- [ ] CLI GUI
- [ ] Loading button after git command executed successfully
- [ ] Debug: improve prompt to avoid repetitive text
- [ ] Save summaries to a folder by dates
- [ ] Send summaries over e-mail daily (g-mail app password)
- [ ] Simplify run commands
- [ ] Package into brew
- [ ] Allow for multi-turn chat with commits to ask follow-up questions
- [ ] Standard metadata structure: files changed, lines of code changed, etc.
- [ ] Python implementation
- [ ] Package distribution
- [ ] Other languages
- [ ] kv cache for history of commit summaries


## Example output

# Overall Summary of Changes

### Summary of Git Changes

This series of commits primarily involve improvements to documentation, refactoring for better application performance, enhancements to UI/UX, and codebase changes. Here's a detailed summary:

1. **Documentation Enhancements:**
   - **New `README.md` (b16149887161cee...):**
     - **Project Documentation:** Added to describe a TypeScript project that automates logging of engineering work to Notion via Screenpipe and Phi 3.5 AI.
     - **Setup Instructions:** Detailed installation and setup steps.
     - **Customization Options:** Instructions to adjust logging frequency and AI output via configuration.

   - **Overall Documentation Updates:**
     - Expanded content for various `mdx` (Markdown with JSX) files.
     - Added FAQs, integration details, architecture overview, and detailed "getting started" guides for different platforms.
     - Standardized titles and organized content for improved readability and consistency.
     - Removed experimental feature notices.

2. **Codebase Changes:**
   - **Interface and State Updates:**
     - Renamed interface fields to camelCase and introduced new fields (`tags`, `deviceName`, `deviceType`) for better data handling.
     - Added new UI states (`isRefreshing`, `isClearing`) and methods (`handleRefresh`, `handleClearMeetings`) for enhanced functionality.
     - Incorporated new icon and tooltip imports to improve the UI.

   - **Async Function Conversion:**
     - Replaced blocking calls with asynchronous requests (`reqwest`) to enhance performance and prevent runtime errors, particularly handling asynchronous function calls properly in Rust using `.await`.

   - **Voice Activity Detection (VAD):**
     - Introduced VAD sensitivity settings in both CLI and application UI, improving customization in audio processing.
     - Added support for configuring VAD in the recording settings and sync functionality for VAD across various components.

3. **UI/UX Improvements:**
   - Redesigned meeting dialogs adding buttons for refreshing and clearing data.
   - Enhanced transcription sections with copy functionality and customizable prompts.
   - Added persistence to log viewer data using `localforage` for better user experience.

4. **Version Updates and Merge Fixes:**
   - Incremented package versions to reflect new features and fixes.
   - Addressed merge conflicts and compilation issues, particularly for MacOS, ensuring alignment with the main codebase.
   - Incorporated metadata configuration in `_app.tsx` for better SEO and social media sharing.

5. **Refactoring for Better Maintenance:**
   - Moved and renamed files for better clarity and maintainability (e.g., `motionDiv.tsx` to `motion-div.tsx`).
   - Simplified and streamlined code structure in various component and function definitions.

These changes aim to improve the documentation's usability, enhance application performance, and provide a more interactive and user-friendly interface for users.




Project written in Rust

Matthew Diakonov