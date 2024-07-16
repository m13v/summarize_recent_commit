# Git Commit Summaries

-----------------------------------------------------------------------
-----------------------------------------------------------------------
 
PRESS CMD+SHIFT+V TO VIEW IN MARKDOWN
 
_______________________________________________________________________
-----------------------------------------------------------------------
Total number of commits: 22

<details>
<summary>Summary for commit 1 (ee266580b2b703b64cff62885f7ea2df90048962)</summary>

### Summary of Git Commit

**Commit ID:** ee266580b2b703b64cff62885f7ea2df90048962  
**Author:** Louis Beaumont <louis.beaumont@gmail.com>  
**Date:** Mon Jul 15 15:53:34 2024 +0200

#### Commit Message:
- **chore:** add simple bench for vision

#### Changes Made:

1. **`Cargo.toml` Updates:**
   - Added a new benchmark target named `vision_benchmark` with `harness = false` under the `[bench]` section.

2. **`vision_benchmark.rs` Updates:**
   - Refactored the benchmark for `continuous_capture` in the `
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 2 (63f86d6c0f94c7adc54357a8b95dd064f354a4e1)</summary>

### Summary of Git Changes
#### Commit Information
- **Commit Hash:** 63f86d6c0f94c7adc54357a8b95dd064f354a4e1
- **Author:** Louis Beaumont <louis.beaumont@gmail.com>
- **Date:** Mon Jul 15 15:14:08 2024 +0200
- **Summary:** Implemented the new indexing feature `#37`.

### Detailed Changes
#### `CONTRIBUTING.md`
- Added sections for Benchmarks and Migration Creation under "Other Hacks".
- Added instructions for running `cargo bench` and creating new migrations with `sqlx-cli`.
- Additional instructions for performance optimization using Xcode Instruments.

#### `
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 3 (8043f71ec8762ba45207f0157ccb6535f30a2147)</summary>

The commit 8043f71ec8762ba45207f0157ccb6535f30a2147, authored by Louis Beaumont on July 15, 2024, merges pull request #44 addressing issue #32. The changes include:

- Integration of Tokio in the screenpipe-vision module.
- Utilizing multiple parallel tasks to maintain previous behavior.
- Addition of a feature to skip frames during OCR if the CPU is overloaded (configurable).
- Enabling the ability to turn devices on or off via the API in screenpipe-audio.
- Enhancing the vision module to be controlled via the API for turning on/off.
- Ensuring that disconnecting an audio device in use stops the listening service, with a requirement for
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 4 (0a6b5479cfee4a6b50d809a4581c703999a15f6c)</summary>

The commit made by Louis Beaumont on July 15, 2024, includes the following change:

- **Bug Fix**: Ensure that the software listens to all audio devices.
- **Code Change**: In the `screenpipe-server/src/bin/screenpipe-server.rs` file:
  - Import the `warn` function from the `log` module within a conditional compilation block for Linux and Windows operating systems.
  - Add a warning log that alerts Linux and Windows users that Screenpipe has not been extensively tested on their operating systems and encourages feedback.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 5 (a41384fb3397f043693e59f18ba9380747226c1a)</summary>

The commit `a41384fb3397f043693e59f18ba9380747226c1a` introduces changes to ensure the system listens to all audio devices properly. Authored by Louis Beaumont, the key modifications include:

1. **Dependency Cleanup:**
   - The `DeviceControl` import is removed from `screenpipe-audio/src/bin/screenpipe-audio.rs`.

2. **Enhanced Device Handling:**
   - The `record_audio` function in `screenpipe-server/src/core.rs` is modified to:
     - Use a non-blocking check (`try_recv`) to receive new device controls.
     - Abort device-related threads when the device control signals to stop.
     - Consistently process recording and
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 6 (1f3f19a5b513a7f780c49acbbdbcf408b83c226d)</summary>

### Commit Summary

**Commit ID**: 1f3f19a5b513a7f780c49acbbdbcf408b83c226d  
**Author**: Louis Beaumont  
**Date**: Mon Jul 15 13:29:33 2024 +0200  

**Summary**: This commit introduces the enhancement to stop listening to an audio device upon its disconnection. Additionally, refactoring and clean-up are performed across various files.

### Detailed Changes

1. **Core Functionality Updates**:
    - **Enhanced Error Handling**:
        - Added functionality to stop listening to an audio device if it disconnects or becomes invalid.
        - Introduced a more specific error handling mechanism for `
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 7 (7affe52e8332404b58b088c086414665b8420feb)</summary>

The commit `7affe52e8332404b58b088c086414665b8420feb` by Louis Beaumont fixes issues with handling audio devices in the project. Major changes include:

1. **File `screenpipe-audio/src/core.rs`:**
   - Removed `tokio` dependencies from async functions and switched to using synchronous `std::thread` operations.
   - Modified `record_and_transcribe` and `create_whisper_channel` functions to be synchronous.
   - Adjusted logic to start recording in a separate thread and switched waiting mechanisms from `tokio` to `std::thread`.

2. **File `screenpipe-audio/src/stt.rs`:**
   - Replaced `tokio::sync::
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 8 (b9fc5e0afa96aad6711d14fa43570115e429b879)</summary>

Here’s a summary of the changes introduced in commit `b9fc5e0afa96aad6711d14fa43570115e429b879`:

### General Changes:
- Introduced async support using Tokio across the project to replace synchronous operations with asynchronous ones for better efficiency.
- Updated various Cargo.toml files to include `tokio` and dependencies related to async operations.
- Added configuration files and dependencies to assist with tracing and monitoring the async runtime, like `tracing-subscriber` and `console-subscriber`.

### Detailed Changes:

#### Configuration and Dependencies:
- **Added** a `.cargo/config.toml` file for enabling `tokio_unstable` features.
- **Updated** dependencies in `Cargo.toml`
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 9 (43efd4fecdd827fe91ad85c007d7112beb435fd0)</summary>

The commit made the following changes to the `screenpipe-audio` and `screenpipe-server` components:

### `screenpipe-audio` Changes:
1. **Main Entry Point (`screenpipe-audio.rs`):**
   - Changed import `parse_device_spec` to `parse_audio_device`.
   - Added a TODO comment about the CLI.
   - Updated the logic to parse the audio device using `parse_audio_device`.

2. **Core Module (`core.rs`):**
   - Introduced a new `DeviceType` enum and updated the `AudioDevice` struct to use it.
   - Refactored `DeviceSpec` into `AudioDevice` and provided relevant methods and trait implementations.
   - Updated functions to use `
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 10 (050aed5111cb0c6d284acf0252d03a908d7bdb86)</summary>

### Summary

This commit involves multiple changes and additions across various files, primarily aimed at enhancing the `screenpipe-audio` and `screenpipe-server` functionalities. Below are the key highlights:

1. **Dependency Updates**:
   - Added `tokio` as a development dependency in `screenpipe-audio/Cargo.toml`.
   - Added `tower` dependency in `screenpipe-server/Cargo.toml`.

2. **Audio Recording Enhancements**:
   - Introduced `DeviceControl` structure to manage the state of audio devices, including fields for running and paused status.
   - Modified `record_and_transcribe` function to integrate `DeviceControl` for more granular control of recording operations.
   - Adjusted the duration for
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 11 (6dac6cd988fcf8526cace4fc77442f4d21b058dc)</summary>

The commit with hash `6dac6cd988fcf8526cace4fc77442f4d21b058dc` is a merge commit authored by Louis Beaumont. This merge incorporates changes from the `main` branch of the contributor `wangshifeng`, as indicated by pull request #46. The primary purpose of the merge is to address and fix the issue related to `InvalidFilename` on Windows. The commit was made on Monday, July 15, 2024, at 12:54 PM (GMT+2).
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 12 (77ff3fd6c0af2f5991e1425fa04fc0d0f87d9e2d)</summary>

This commit carries out a change in the way timestamps are handled within the codebase, specifically switching from local time (using `chrono::Local`) to Coordinated Universal Time (UTC) (using `chrono::Utc`). The files affected by this change are:

1. `screenpipe-server/src/bin/screenpipe-video.rs`
   - Changed from `chrono::Local` to `chrono::Utc` in the import statement.
   - Altered the variable assignment from `Local::now()` to `Utc::now()`.

2. `screenpipe-server/src/video.rs`
   - Similarly, modified the import statement from `chrono::Local` to `chrono::Utc`.
   - Updated the code to replace `Local::now()` with `
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 13 (fa88c1d4f197cb22e0b78f354a36304d1f562c83)</summary>

The commit `fa88c1d4f197cb22e0b78f354a36304d1f562c83` authored by Wang Shifeng on July 13, 2024, indicates the renaming of a demo file. Specifically, a new binary file named `2024-07-12_01-14-14.mp4` was added to the `data` directory. There is no indication of what the previous name of the file was before renaming.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 14 (5472087f29fee50d1b86ff384a1de86074c60704)</summary>

The commit with hash `5472087f29fee50d1b86ff384a1de86074c60704` was authored by Wang Shifeng on July 13, 2024. It merges changes from the branch `louis030195-main` into the current branch. The parent commits are `6ef5c6f` and `877d11c`.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 15 (877d11c22597c1b91430ba83427c804732bdea6c)</summary>

The commit is a merge of branch 'main' from the repository 'https://github.com/louis030195/screen-pipe' into another branch named 'louis030195-main'. The specific change involves modifying the `README.md` file to correct the file path format in a code snippet demonstrating how to play a sample frame recording from the database. The corrected path format is now consistent with the existing examples.

Key points:
- Fixed path format in a sample command.
- Merged two branches into one.

</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 16 (6ef5c6f5402e8a751dc02762c663de6ebd10697b)</summary>

The commit addresses filename issues on Windows by modifying timestamp handling and file paths to use a format that is compatible with the Windows filesystem. 

Here are the key changes:

1. **README.md**:
   - The sample `ffplay` command was updated to use a filename format that replaces spaces and colons with underscores and dashes.
     ```diff
     -ffplay "screen-pipe/data/2024-07-12 01:14:14.078958 UTC.mp4"
     +ffplay "screen-pipe/data/2024-07-12_01-14-14.mp4"
     ```

2. **File Deletion**:
   - The file named `2024-07-12 
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 17 (37aea8649ff092b1abf6e156962d3e3e48064085)</summary>

The commit `37aea8649ff092b1abf6e156962d3e3e48064085` is a merge commit by Matthew Diakonov. It merges changes from the branch associated with pull request #45, which was contributed by louis030195. The specific update involved is labeled "Readme 3," indicating changes or additions were made to the README file. The merge took place on July 12, 2024.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 18 (c5523c1c41c6d0f5753de1090cba2fce657e4f1d)</summary>

The commit made by matthew-heartful on July 12, 2024, simplifies the instructions and structure of the README.md file for the screen-pipe project. Here are the key changes:

1. **Added Sections with Expanded Details**:
   - Introduced sections for `BACKEND` and `FRONTEND` each with collapsible `<details>` tags.
   - The `BACKEND` section includes instructions for installation using both a pre-built binary (`Option I: Library`) and from the source (`Option II: Install from the source`).

2. **Installation Steps Reorganized**:
   - Installation commands for MacOS dependencies have been restructured for clarity.
   - Detailed steps to install dependencies, clone the repository, build
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 19 (b41414de3e1e38d0e38adb7b0a121975401f7e42)</summary>

The commit with the hash `b41414de3e1e38d0e38adb7b0a121975401f7e42` is a merge commit. The author of the commit is Wang Shifeng, who merged changes from another branch (specifically the branch `main` from the repository of the user `louis030195`) into their current branch. The purpose of this merge was to fetch updates from the upstream repository. The parent commits involved in this merge are `9e0476a` and `0a1b860`. The commit was made on July 13, 2024.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 20 (a8533745bbb8ad0979b02551dc1ca5c568d76e94)</summary>

The commit with ID `a8533745bbb8ad0979b02551dc1ca5c568d76e94` authored by "matthew-heartful" on July 12, 2024, adds new "dislike" and "like" buttons to the `README.md` file.

### Changes in `README.md`:
- Added two new badge-style buttons:
  1. A button labeled "😭_It didn't work_😭" that likely directs users to an error-report link.
  2. A button labeled "Like" that points to a serverless function URL to register a 'like'.
  
The `README.md` now includes:
```markdown
[![It didn't work](https
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 21 (7bae4af1bd44f8708450867cdff81e5d19596cf4)</summary>

The given commit, authored by Matthew Heartful on July 12, 2024, includes a small modification in the `README.md` file. Specifically, it changes the URL in a link that users can click if they encounter issues setting up their OPENAI API KEY. The original link, which directed users to a page to report a new dislike issue with the title "New dislike," has been updated to direct users to report a new dislike issue with the title "vercel app didn't work."

Details:
- **File Modified**: `README.md`
- **Line Changed**: The link under the section for setting up the OPENAI API KEY.
- **Change Description**: The link title in the URL changed from "New+dis
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 22 (cd02b4fff9bffa1dc3da97e801afca34d2a4ad48)</summary>

### Commit Summary
- **Commit ID:** cd02b4fff9bffa1dc3da97e801afca34d2a4ad48
- **Author:** Matthew Heartful
- **Date:** Fri, Jul 12, 2024 at 12:25:02 -0700
- **Message:** trying out dislike button

### Changes Made
- **File Modified:** `README.md`
- **Modifications:**
  - Added a line encouraging the submission of issues if something didn't work, with a new "dislike" button:
    ```markdown
    [😭 It didn't work 😭](https://github.com/louis030195/screen-pipe/issues/new?assignees=&
</details>

------------------------------------------------------------------------

