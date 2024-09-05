# Git Commit Summaries

-----------------------------------------------------------------------
-----------------------------------------------------------------------
 
PRESS CMD+SHIFT+V TO VIEW IN MARKDOWN
 
_______________________________________________________________________
-----------------------------------------------------------------------
Total number of commits: 59

<details>
<summary>Summary for commit 1 (7da72ae86e6e65ad589b5d63329f1b658bd18a25)</summary>

This commit involves signing an Apple app. Here are the main changes:

1. **Workflow Update**:
    - Modified the GitHub Actions workflow file (`release-app.yml`).
    - Changed the environment variables from Tauri-specific signing keys to Apple-specific signing credentials.
    - Updated to use `APPLE_CERTIFICATE`, `APPLE_CERTIFICATE_PASSWORD`, `APPLE_SIGNING_IDENTITY`, `APPLE_ID`, `APPLE_PASSWORD`, and `APPLE_TEAM_ID` for signing.
    - Added `CI: true` to denote a continuous integration environment.

2. **Version Bump**:
    - Updated the `Cargo.toml` file for the `screenpipe-app` to increment the package version from `0.1.83` to `0.1.84`.

Overall, these changes sign the app for macOS using Apple certificates and increment its version.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 2 (a42fc3e2cfb47217051982552578abf9b086c699)</summary>

The commit with the hash `a42fc3e2cfb47217051982552578abf9b086c699`, authored by Louis Beaumont on September 3, 2024, updates the `README.md` file located in the `examples/apps/screenpipe-app-tauri` directory. Specifically, the update involves removing a commented-out note that advised adding environment variables to the integrated terminal configuration in VS Code. The rest of the configurations remain unchanged.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 3 (8e21969d0ea4d47c604ed3f08a24b420a0398058)</summary>

The commit `8e21969d0ea4d47c604ed3f08a24b420a0398058` by Louis Beaumont updates the `README.md` file in the `screenpipe-app-tauri` subdirectory of the `examples/apps` directory. The update adds a new section titled "macos specific," which provides instructions for modifying the `.vscode/settings.json` file to include environment variables necessary for linking the Apple native OCR compiled library to the binaries. Specifically, it instructs users to set the `DYLD_LIBRARY_PATH` environment variable for `rust-analyzer` and the integrated terminal in Visual Studio Code.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 4 (585b4de655d4df8e25aae74e44c70a8187b13da2)</summary>

The commit `585b4de655d4df8e25aae74e44c70a8187b13da2` by Louis Beaumont on Tue Sep 3, 2024, updated the `README.md` file in the `examples/apps/screenpipe-app-tauri/` directory. The changes simplify and modify the instructions within the file:

1. Removed placeholder text "README TODO."
2. Simplified the "Getting started locally" section to just the command list prefixed with "getting started locally:".
3. Provided a new sequence of commands for releasing the app:
   - Replaced setting up environment variables and dependency installations (`pnpm` commands) with `cargo` and `bun` commands.
   - Included steps to build the project using Cargo, change the directory, install dependencies using `bun`, and run pre-build and build scripts with `bun`.
4. Added a cautionary note that the process can be more complex on Windows.

Overall, the update clarifies and possibly streamlines the setup and build process.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 5 (83419b45560ae4cf619fc78e0136844c2bbf73d5)</summary>

The commit with hash `83419b45560ae4cf619fc78e0136844c2bbf73d5` is a merge commit authored by Louis Beaumont on September 3, 2024. This commit merges changes from a pull request (#271) into the main branch. The pull request updated the Homebrew formula specifically for the `x86_64-apple-darwin` platform.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 6 (4ceb875b211373bec1fc4158cf77ddc84e38711c)</summary>

The commit `4ceb875b211373bec1fc4158cf77ddc84e38711c` is a merge commit authored by Louis Beaumont. It merges the branch `main` into another branch named `update-formula-x86_64-apple-darwin-30f56ec8db0a9a10159f34fe4e1ddcc113bff646`. 

The changes made in this commit are within the `Formula/screenpipe.rb` file:

- The SHA-256 checksum for the `aarch64-apple-darwin` build has been updated from `924b079f24939be270dc74c2040e117ff7229ed285fb7f246ed52c6dbf6d160d` to `676312b29923fd80a4b5ba6210364a899d56e17ddcd0a0924ccb11b592ae1bbb`.
- The SHA-256 checksum for the `x86_64-apple-darwin` build has been updated from `98e91de2eb3f6823ca5974ab48fb1d28b09ae5c382893f510838f9213dfe618a` to `3ba1c0421bff3c3b2aec8f3ea364f086bdca0574eb3057151253aa59c4a5a1dc`.

These checksum updates likely reflect new or rebuilt binaries for different macOS architectures.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 7 (063357d08815eca35e070bb7283676d1d178d8db)</summary>

This git changes summary consists of a merge commit with the following details:

- **Commit ID:** 063357d08815eca35e070bb7283676d1d178d8db
- **Merged Branches:**
  - Source: `30f56ec`
  - Target: `533de65`
- **Author:** Louis Beaumont (<louis.beaumont@gmail.com>)
- **Date:** Tue Sep 3 18:16:18 2024 -0700
- **Title:** Merge pull request #272 from mediar-ai/update-formula-aarch64-apple-darwin-30f56ec8db0a9a10159f34fe4e1ddcc113bff646
- **Description:** Update Homebrew formula for aarch64-apple-darwin. 

In summary, this merge commit integrates changes from the specified pull request aimed at updating the Homebrew formula to support the `aarch64-apple-darwin` architecture.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 8 (533de652b023de1f180cce06d1a7e43f156c99ec)</summary>

In the given git commit, there was an update to the `screenpipe` formula in a Homebrew formula file. The changes include:

1. **Version Update**: The screenpipe version was updated from `0.1.74` to `0.1.75`.
2. **Download URL Update**: The URL corresponding to the updated version reflects the new version number in its path.
3. **SHA256 Checksum Update**: The SHA256 checksum for the `arm64` architecture was updated to a new value, corresponding to the new version. 

These updates ensure that when users install the screenpipe formula, they will get the latest version (0.1.75) of the software with the correct integrity checks.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 9 (a8725adbbbf2bea738de92f73971141661055763)</summary>

The commit `a8725adbbbf2bea738de92f73971141661055763` updated the Homebrew formula for the `screenpipe` package. Specifically, the following changes were made:

- The version of `screenpipe` was updated from `0.1.74` to `0.1.75`.
- The SHA256 checksum for the `x86_64-apple-darwin` download was updated to reflect the new version.

The changes ensure that users will download and install the updated version (`0.1.75`) of the `screenpipe` package on macOS systems, with the correct integrity verification via a new SHA256 checksum value.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 10 (30f56ec8db0a9a10159f34fe4e1ddcc113bff646)</summary>

The commit `30f56ec8db0a9a10159f34fe4e1ddcc113bff646` by Louis Beaumont on September 3, 2024, includes changes to several files:

1. **General Update:**
   - **Cargo.toml**: Updated the version of the package from `0.1.74` to `0.1.75`.
   - **examples/apps/screenpipe-app-tauri/src-tauri/Cargo.toml**:
     - Updated the version of the screenpipe-app package from `0.1.82` to `0.1.83`.

2. **Specific Fix:**
   - **screenpipe-server/src/cli.rs**:
     - Reverted the default value for the `audio_chunk_duration` argument from 120 seconds back to 30 seconds for consistency across platforms. The change addresses an issue detailed in the GitHub issue [#173](https://github.com/mediar-ai/screenpipe/issues/173).

The commit message indicates that the primary purpose was to fix the default audio duration setting.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 11 (33b20dddc8a6cedab6b86979b045617cbdd57357)</summary>

The commit made by Louis Beaumont updates the `README.md` file in the `typescript/pipe-sync-meetings-to-notion` example directory. This update includes adding a URL (https://github.com/user-attachments/assets/795dfd91-393a-4eef-a20b-5b2c35d594f9) at the beginning of the file. The rest of the content in the file remains unchanged.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 12 (4452d651fb59a1bbc1c0e6b79a6fe21fde0d809c)</summary>

This git change involves a merge commit with the ID `4452d651fb59a1bbc1c0e6b79a6fe21fde0d809c` by author Louis Beaumont on September 3, 2024. The commit merges changes from pull request #269, which updates the Homebrew formula for the `aarch64-apple-darwin` system.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 13 (276c8058c1edd343ea694d691d0fcb7bfd7bd355)</summary>

The git changes summary:

- **Commit Information**: 
  - **Commit Hash**: `276c8058c1edd343ea694d691d0fcb7bfd7bd355`
  - **Author**: Louis Beaumont
  - **Date**: Tue Sep 3, 2024

- **Action**: Merged the branch `main` into `update-formula-aarch64-apple-darwin-0e24c5caa49839487e3c18fcd69931d1a61feccb`.

- **Affected File**: `Formula/screenpipe.rb`.
  - **Changes**:
    - Updated the SHA-256 checksum for the arm64 (AArch64) build to `"924b079f24939be270dc74c2040e117ff7229ed285fb7f246ed52c6dbf6d160d"`.
    - Updated the SHA-256 checksum for the x86_64 build to `"98e91de2eb3f6823ca5974ab48fb1d28b09ae5c382893f510838f9213dfe618a"`.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 14 (c47c920407e242df0d08066d79593e38d9df53d8)</summary>

This git commit represents a merge of pull request #270 into the main branch. The pull request was focused on updating the Homebrew formula for the `x86_64-apple-darwin` architecture. The commit was authored by Louis Beaumont on September 3, 2024.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 15 (92a4d9b30ee991fb7aaaf1a0eca7c632b01f3edd)</summary>

The commit adds new functionality and improvements for syncing audio transcriptions from Screenpipe to Notion. Here are the key changes summarized:

1. **New Example Added:**
   - Created a new folder `examples/typescript/pipe-sync-meetings-to-notion/` containing example code (`main.js`) and a README for syncing Screenpipe meetings to Notion.

2. **README.md:**
   - Provides detailed instructions on setting up the integration and database in Notion.
   - Explains how to run the synchronization script.
   - Mentions current limitations and makes suggestions for code modifications.

3. **main.js:**
   - Implemented an interval-based synchronization script that queries audio data from Screenpipe and sends transcriptions to Notion.
   - Uses the Notion API to create entries in a specified Notion database.
   - Splits long transcriptions into smaller chunks to fit within character limits.
   - Contains error handling and logs for debugging purposes.

4. **Core Changes to `screenpipe-core`:**
   - **runtime.js:**
     - Added a custom `fetch` function to handle HTTP requests within the Screenpipe runtime.
     - Augmented timeout functionality and console accessibility.
   - **pipes.rs:**
     - Added `op_fetch` operation to support `fetch` functionality from the code.
     - New operations to interact with environment variables, improving customization via global scope settings.
     - Modified the environment variable initialization to populate `process.env` with vars starting with `SCREENPIPE_`.
   
These changes enhance the capabilities of Screenpipe by enabling users to automatically sync meeting transcriptions to their Notion workspace, with improved script execution and a documented setup process.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 16 (ec36f7f464b7a9bb06b04b9e187e9eb10913f157)</summary>

The commit `ec36f7f464b7a9bb06b04b9e187e9eb10913f157` authored by the GitHub Actions Bot updates the Homebrew formula for the `screenpipe` library. The update involves the following changes:

1. **Version Update**: The version of `screenpipe` has been updated from `0.1.73` to `0.1.74`.

2. **Checksum Update**:
   - For the `x86_64` architecture on macOS, the SHA-256 checksum has been updated from `8acabe46b935ac8ed6607a43a5d47e81b5647ee2c52193bcb3b646628a4ced9b` to `98e91de2eb3f6823ca5974ab48fb1d28b09ae5c382893f510838f9213dfe618a`.

This ensures that the formula now references the correct version and corresponding checksum for this updated release.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 17 (9134e3a947fce84ed8880b62caf3554bac2cd30b)</summary>

This commit updates the Homebrew formula for the `screenpipe` library to version 0.1.74. The changes include:

- The `url` has been updated to point to the new release archive for version 0.1.74 for the ARM architecture on macOS (aarch64-apple-darwin).
- The `version` has been updated from 0.1.73 to 0.1.74.
- The `sha256` checksum for the ARM architecture on macOS has been updated to match the new release. 

Essentially, this commit ensures that users who install `screenpipe` via Homebrew get the latest version, 0.1.74.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 18 (bb7c6928be1b41397bb0edc93d7f14f5988afe37)</summary>

The commit bb7c6928be1b41397bb0edc93d7f14f5988afe37 is a merge commit performed by Louis Beaumont on September 3, 2024. It merges the changes from pull request #267, which was submitted by user `mediar-ai`. The primary update in this merge is to the Homebrew formula for the `aarch64-apple-darwin` platform.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 19 (0ae2f241498ea849ed6a8cf9089927fc7e5069e9)</summary>

This git commit is a merge of the 'main' branch into a feature branch named 'update-formula-aarch64-apple-darwin-aebb9adbd09d50e29f009a6b56f7e45011e3d0c9', authored by Louis Beaumont. The changes in the `Formula/screenpipe.rb` file involve updating the `sha256` checksums for different architecture-specific binaries of the `screenpipe` software:

- For `aarch64-apple-darwin` (ARM64) architecture, the `sha256` checksum is changed from `b334841820ff3d9fc4dc47c1b33a2f62cd961a02b999834a675dc8e4fae4b89b` to `6b6b540d9565a13bbf52f8c14224dc7876f40861044313aa8c992b866e015973`.
- For `x86_64-apple-darwin` architecture, the `sha256` checksum is changed from `538fdeb9024a041d29b6647a47381ef701c0fe46841b5bced98038fffa816fd4` to `8acabe46b935ac8ed6607a43a5d47e81b5647ee2c52193bcb3b646628a4ced9b`.

Additionally, the dependency on `tesseract` was removed.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 20 (cba68cf6b0ac775b5bb41de9b15d8669116fbdfe)</summary>

The commit with hash `cba68cf6b0ac775b5bb41de9b15d8669116fbdfe` was created by Louis Beaumont on September 3, 2024. This commit is a merge of two previous commits (with hashes `0e24c5c` and `d43ab8e`). The merge is related to pull request #268 from the `mediar-ai` repository. The changes update the Homebrew formula specifically for the `x86_64-apple-darwin` architecture.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 21 (0e24c5caa49839487e3c18fcd69931d1a61feccb)</summary>

In the commit identified by `0e24c5caa49839487e3c18fcd69931d1a61feccb`, authored by Louis Beaumont on September 3, 2024, log messages related to debugging the available tracks in the `pcm_decode` function within the `pcm_decode.rs` file of the `screenpipe-audio` module were removed. Specifically, the code that iterated over the tracks and printed debug information about each track's codec and codec parameters was deleted.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 22 (12c6898aebff8ac3c6320b47ce00032311f9116a)</summary>

The commit `12c6898aebff8ac3c6320b47ce00032311f9116a` by Louis Beaumont addresses encoding issues in the file `pcm_decode.rs` within the `screenpipe-audio` project. The changes involve the following adjustments:

1. **Hint Initialization**: A mutable `Hint` object (`hint`) which initialized with the file extension "mp4" has been replaced by an immutable `Hint` object.
2. **Commented Code**: The call to `hint.with_extension("mp4")` has been commented out.

These changes likely improve or fix encoding-related problems by adjusting how the hint object is created or used.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 23 (ac622532331a43a5ca3078c60f8dfc8e70310970)</summary>

The commit ac622532331a43a5ca3078c60f8dfc8e70310970 by Louis Beaumont fixes encoding issues in the `pcm_decode` function of the `screenpipe-audio/src/pcm_decode.rs` file. The specific change involves modifying the creation of probe hints for media files. Initially, a default hint was created, but it has now been updated to include the file extension "mp4", by using the `with_extension` method. This refinement aims to improve how media source streams handle file extensions during decoding.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 24 (fcf3612cf926474aed89166d5ac3c7d0b5e74dee)</summary>

The commit with ID `fcf3612cf926474aed89166d5ac3c7d0b5e74dee`, authored by Louis Beaumont, addresses encoding issues in a Rust file (`screenpipe-audio/src/pcm_decode.rs`). Specifically, the changes add debug logging to the `pcm_decode` function to log detected format details and information on all available tracks. These additions occur after the format reader has been instantiated and before selecting the first audio track with a decodable codec.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 25 (25e1bb5c6a5e7baaad6a5c76a2ca0411ddcd42b6)</summary>

The commit with hash `25e1bb5c6a5e7baaad6a5c76a2ca0411ddcd42b6`, authored by Louis Beaumont, addresses encoding issues in the `screenpipe-audio` project. 

Changes made in the `screenpipe-audio/src/core.rs` file include:

1. **Reduction of Audio Bitrate**: The bitrate for audio encoding is reduced from 128 kbps to 64 kbps, aiming for higher compression.
2. **Profile Specification**: Adds the `aac_low` profile to use the AAC-LC (Low Complexity) profile for better compatibility.
3. **Fast Start Optimization**: Adds the `+faststart` flag to the `-movflags` options to optimize the output for web streaming.

These changes should improve compatibility and streaming performance of the generated audio files.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 26 (75619e6de6b6392db16da30aefa5214b38339df4)</summary>

The commit with hash `75619e6de6b6392db16da30aefa5214b38339df4`, authored by Louis Beaumont, includes the following changes:

1. **Version Updates:**
   - The main `Cargo.toml` file's version is updated from `0.1.73` to `0.1.74`.
   - The `Cargo.toml` file for the example application `screenpipe-app-tauri` is updated from version `0.1.81` to `0.1.82`.

2. **Configuration Changes in `cli.rs`:**
   - The default value for `audio_chunk_duration` is changed from 30 seconds to 120 seconds.
   - A comment was added to clarify that the `restart_interval` feature (which restarts the recording process every X minutes) is marked as "NOT RECOMMENDED".

These changes address the release that fixes Windows microphone issues and include a few other updates.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 27 (cc4e608e28c799ef7ad5394b3f960e40400e6c4a)</summary>

The commit `cc4e608e28c799ef7ad5394b3f960e40400e6c4a` authored by Louis Beaumont on September 3, 2024, addresses an issue with audio devices having more than 2 channels. 

Summary:

- The code modification is in the file `core.rs` within the `screenpipe-audio/src` directory.
- The relevant change adjusts the number of audio channels used by the FFmpeg command. If the number of channels exceeds 2, it is capped at 2.
- Additionally, the audio bitrate parameter has been changed from `384k` to `128k`.
- The use of the `-strict experimental` flag for the AAC codec has been removed.

This update ensures compatibility with audio devices that have more than 2 channels by limiting the channel count to 2 for processing.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 28 (378b861434b8bdc2a53feb205f8bdf947cd6f40e)</summary>

### Summary of Git Changes

**Commit**: `378b861434b8bdc2a53feb205f8bdf947cd6f40e`   
**Author**: Louis Beaumont   
**Date**: Tue Sep 3 14:32:54 2024 -0700

**Description**: 
Fixed an issue where the audio device name on Windows contained slashes, which could cause crashes.

**Code Changes**:
- **File Modified**: `screenpipe-server/src/core.rs`
- **Lines Removed**: 12 lines of code were removed where the output path was manually sanitized to replace slashes with underscores (`/` -> `_`).
- **Modifications**:
  - The creation of the `output_path` no longer involves manual sanitization.
  - Inside the loop, the `audio_device` name is sanitized to replace both forward slashes (`/`) and backslashes (`\`) with underscores (`_`).
  - The sanitized device name is then used to construct the file path for recording audio.

Overall, the commit refactors the code to handle the sanitization of audio device names in a cleaner, more reliable manner.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 29 (2a2e2c23fcceac1e9437f455b5e86688b2d13b66)</summary>

The commit `2a2e2c23fcceac1e9437f455b5e86688b2d13b66` authored by Louis Beaumont addresses an issue on Windows where audio device paths contain slashes. This was causing crashes or incorrect behavior. 

In the `record_audio` function within the `screenpipe-server/src/core.rs` file, Louis introduces a fix that processes the output path for audio devices. The new code segment replaces slashes (`/`) in the device paths with underscores (`_`). For example, an audio device named "Speakers/Headphones (Realtek High Definition Audio)" is converted to "Speakers_Headphones_(Realtek_High_Definition_Audio)" ensuring Windows compatibility. 

Summary of changes:
- Added code to sanitize the output path by replacing slashes with underscores.
- Ensured the modified path is encapsulated within an `Arc<PathBuf>` for thread-safe reference counting.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 30 (60305d3554567efbcd256b52e865123f64e71145)</summary>

The commit by Louis Beaumont fixes an issue when an audio device has more than three channels. The changes are made in the `screenpipe-audio/src/core.rs` file. Specifically:

1. **Code Formatting Change:**
   - A blank line is added after the `debug!("Starting FFmpeg process");` statement.

2. **FFmpeg Command Adjustments:**
   - Added the `-strict` and `experimental` flags to the FFmpeg command to allow experimental features necessary for handling complex audio devices.
   - Updated the audio bitrate from `128k` to `384k` to accommodate higher quality audio.
   - Removed the `-aac_coder fast` option.

These changes ensure that FFmpeg handles audio devices with more than three channels correctly, potentially improving the compatibility and quality of the audio processing pipeline.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 31 (fe33ece1068ed51845aff60699f5280e736390e6)</summary>

The commit `fe33ece1068ed51845aff60699f5280e736390e6` authored by Louis Beaumont on September 3, 2024, aims to address an issue related to "Access is denied" errors on Windows. The change is made in the file `screenpipe-audio/src/core.rs` where two new lines have been added to the function `run_ffmpeg`. Specifically, the following FFmpeg parameters were added:

1. `-aac_coder`
2. `fast`

These parameters may help with the Windows-specific issue by modifying how audio encoding is handled in the pipeline.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 32 (b2ca6c00fc381b1ea9c72ae0de4c50c4ad9c949a)</summary>

The commit `b2ca6c00fc381b1ea9c72ae0de4c50c4ad9c949a`, authored by Louis Beaumont on September 3, 2024, attempts to fix an "Access is denied" error on Windows by modifying two files:

1. **`screenpipe-audio/src/core.rs`**:
   - Commented out the code block responsible for opening and syncing a file at the specified path. The commented-out code is suspected to be causing the "Access is denied" error on Windows.

2. **`screenpipe-vision/src/microsoft.rs`**:
   - Added a newline at the end of the file.

The primary focus of this commit is addressing a potential access issue on Windows by temporarily disabling the file sync operation.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 33 (2ca65c59ed4b1127cbc6f53d02b71c75338109f7)</summary>

The commit `2ca65c59ed4b1127cbc6f53d02b71c75338109f7` by Louis Beaumont updates the Continuous Deployment (CD) version in the GitHub Actions workflow configuration file (`.github/workflows/release-app.yml`). Specifically, it changes the versions of the `crabnebula-dev/cloud-release` action from `v0.2.0` and `v0.2.2` to `v0` at two places in the file. This is likely to ensure the workflow uses the latest minor or patch versions of the `v0` major version series.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 34 (d43ab8e558733ebc79bae0b74d221f3f21e72122)</summary>

The commit updates the Homebrew formula for the `screenpipe` library. Specifically, the version of `screenpipe` is bumped from `0.1.72` to `0.1.73`. In the updated formula:

1. The version number in the URL for both arm64 and x86_64 builds on macOS is updated to `0.1.73`.
2. The SHA-256 checksum for the x86_64 build has been changed from `538fdeb9024a041d29b6647a47381ef701c0fe46841b5bced98038fffa816fd4` to `8acabe46b935ac8ed6607a43a5d47e81b5647ee2c52193bcb3b646628a4ced9b`.

This update ensures that the formula correctly references and verifies the new version of the `screenpipe` library.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 35 (ab7248f6908583f6d2cf1a902f0b88ae21af2bc6)</summary>

The commit `ab7248f` updates the Homebrew formula for the `screenpipe` library:

- The version of `screenpipe` is bumped from `0.1.72` to `0.1.73`.
- The download URL is updated to reflect the new version (`0.1.73`).
- The SHA-256 checksum for the `aarch64-apple-darwin` architecture is updated to ensure the new version's integrity.

Overall, this commit automates the update process to a new version of the `screenpipe` library for Apple Silicon (ARM) builds via a GitHub Actions bot.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 36 (aebb9adbd09d50e29f009a6b56f7e45011e3d0c9)</summary>

In this commit, the author has:

1. **Temporarily Disabled the Restart Interval in the App UI:**
   - In `recording-settings.tsx`, the `restartInterval` property in the settings is commented out.
   - The UI elements related to the restart interval input have been commented out as well, effectively hiding this feature from the user interface.

2. **Changed the Handling of the Restart Interval in the Backend:**
   - In `main.rs`, the `restart_interval` argument is now hardcoded to "0" instead of using the value from `restart_interval_str`, which indicates that the feature has been disabled due to a memory leak issue.

3. **Version Bumps:**
   - Updated the package version in the primary `Cargo.toml` from `0.1.72` to `0.1.73`.
   - Updated the package version in `screenpipe-app-tauri/src-tauri/Cargo.toml` from `0.1.80` to `0.1.81`.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 37 (1152a96a6084bf48d411d9863b050d984ae80f01)</summary>

### Summary of Git Changes in Commit 1152a96a6084bf48d411d9863b050d984ae80f01

**Author:** Louis Beaumont  
**Date:** Tue Sep 3, 2024

**Description:**
Fix related to uneven screen sizes in the video processing using FFmpeg.

**Changes:**

1. **File Modified:** `screenpipe-server/src/video.rs`
   
2. **Code Changes:**
    - **Removal:** 
      - Unused `frame_queue` was removed from the `VideoCapture` struct.
      - Code segments using `frame_queue` were stripped from the initialization and queue management logic.
    - **Addition/Modification:** 
      - The FFmpeg command was adjusted with filter options to ensure screen dimensions are both even (`pad=width=ceil(iw/2)*2:height=ceil(ih/2)*2`).

These changes streamline the code by eliminating an unused frame queue and fix the issue of uneven screen dimensions in video processing by modifying the FFmpeg command line options.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 38 (b1085f26b51d51d592311279b196448492c7f098)</summary>

The commit is a merge that incorporates changes from pull request #252 into the main branch. This pull request updates the `vercel-ai-chatbot` to utilize the `gpt-4o-mini` model. The changes were authored by Louis Beaumont and committed on September 3, 2024.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 39 (84f6154303e34de4b9cf45813a707ab1aef40499)</summary>

### Summary of Git Changes

**Commit**: 84f6154303e34de4b9cf45813a707ab1aef40499  
**Author**: Louis Beaumont  
**Date**: Tue Sep 3 10:34:43 2024 -0700  
**Type**: Chore  
**Description**: Small memory optimization on vision module  

#### Changes Overview

The commit introduces memory optimizations in the `screenpipe-vision` module by replacing some `Arc` usage with `Weak` references to prevent potential memory leaks and reduce unnecessary strong reference counts.

#### Detailed Changes

1. **File `core.rs`**:
   - Replaced `Arc` with `Weak` references for `monitor` and `ocr_engine`:
     - `let weak_monitor = Arc::downgrade(&arc_monitor);`
     - `let weak_ocr_engine = Arc::downgrade(&ocr_engine);`
   - Updated function `continuous_capture` to use `weak_monitor`:
     - `let capture_result = match capture_screenshot(weak_monitor).await { ... }`
   - Updated function `process_ocr_task` to take `Weak<OcrEngine>` as a parameter:
     - `ocr_engine: Weak<OcrEngine>`
   - In `process_ocr_task`, upgraded `Weak` to `Arc` for use when needed:
     - `let (window_text, window_json_output, confidence) = match ocr_engine.upgrade() {`

2. **File `utils.rs`**:
   - Updated function `capture_screenshot` to accept and handle `Weak<Monitor>`:
     - `monitor: Weak<Monitor>`
     - Added logic to upgrade `Weak` to `Arc` within the function:
       - `let monitor = weak_monitor.upgrade().ok_or("Monitor no longer exists").unwrap();`

These changes improve memory management by ensuring that objects are not kept alive unnecessarily, leading to better resource usage.

#### Other Minor Changes

- Code formatting improvements for better readability.
- Fixed a missing newline at the end of `core.rs`.

Overall, these changes focus on enhancing the efficiency and management of memory within the screen capture and OCR process in the vision module.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 40 (ae371460a4e527ab79d2e05b8377ed1c872b6375)</summary>

**Summary of Git Changes:**

- **Commit ID:** ae371460a4e527ab79d2e05b8377ed1c872b6375
- **Author:** Louis Beaumont
- **Date:** Mon Sep 2 13:31:16 2024 -0700
- **Change Type:** Chore (Cleanup)
- **Purpose:** Remove unnecessary log statements.

**File Modified:**
- `examples/apps/screenpipe-app-tauri/components/chat-list-openai-v2.tsx`

**Modifications:**
1. Removed some console logs that split a JSON string and printed it in chunks to the console.
2. Simplified the template string concatenation for displaying user timezone.

**Detailed Changes:**
- Removed code block that converts the first stream message to a JSON string and prints it in chunks of 50 characters to the console.
- The unnecessary concatenation of user timezone has been simplified.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 41 (3dc7f4e8e5d04b6c13189bade090f26b34447e6a)</summary>

The summarized git changes for the given commit are as follows:

**Commit ID:** 3dc7f4e8e5d04b6c13189bade090f26b34447e6a  
**Author:** Louis Beaumont  
**Date:** Mon Sep 2 13:16:21 2024 -0700  

**Change Description:** Updated the `README.md`.

**Details of Changes:**
1. Marked the "Pipe Store" feature as complete.
2. Simplified the audio + STT section to state that Linux, MacOS, and Windows support both input and output devices comprehensively. Also, included and marked iPhone microphone as completed in the list.
3. Removed the sub-item indicating the specific output support for each OS as it is now generalized.
4. Updated the "security" section to indicate that "PII removal" is now complete.
5. Removed the "bug-free & stable" item entirely from the checklist.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 42 (1cf6ffcc7e24f464a15a5a64eb5372d0cf2c6082)</summary>

This commit, authored by Louis Beaumont, merges a pull request (#251) from the user 'ac3xx' into the codebase. The changes are focused on enhancing the application's handling of datetime objects across different timezones. The merge was completed on September 2, 2024.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 43 (17959eda8f1737c2f9d5cb07cadeb330dff77d54)</summary>

The commit by Louis Beaumont on September 2, 2024, with the message "fix bench", introduces changes to the GitHub Actions workflow configuration file for benchmarking (.github/workflows/benchmark.yml). Specifically, it modifies the job that installs dependencies. The updated dependency installation command now includes additional packages:

- tesseract-ocr
- libtesseract-dev
- libavformat-dev
- libavfilter-dev
- libavdevice-dev
- libasound2-dev
- libgtk-3-dev
- libsoup-3.0-dev
- libjavascriptcoregtk-4.1-dev
- libwebkit2gtk-4.1-dev

These added dependencies are likely required for the project's benchmarks to run correctly, suggesting that previous runs were possibly failing or incomplete due to missing these libraries.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 44 (09725097aa29f61bec233c4595f5e4610d2aaa39)</summary>

This commit (09725097aa29f61bec233c4595f5e4610d2aaa39) merges a pull request (#265) from the `mediar-ai` repository. The main purpose of this merge is to update the Homebrew formula specifically for the `aarch64-apple-darwin` architecture. It includes changes from the branch associated with the hash `27c1e92ac3d3a87d028361fbec7b24d41503d55b`. The merge was authored by Louis Beaumont on September 2, 2024.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 45 (1b2c99e1a637cb99d82004b00ddd3ae9fd9beb23)</summary>

In the git commit identified by `1b2c99e1a637cb99d82004b00ddd3ae9fd9beb23`, authored by Louis Beaumont, the main branch was merged into the branch `update-formula-aarch64-apple-darwin-27c1e92ac3d3a87d028361fbec7b24d41503d55b` on September 2, 2024. The primary change occurred in the `Formula/screenpipe.rb` file:

1. For macOS on ARM (arm64) architecture, the SHA256 checksum for the downloaded release tarball was updated from `"bc2605353fc9f080d2f06c7590f66c752a928f5073ee8f18fda0906a4becc49d"` to `"b334841820ff3d9fc4dc47c1b33a2f62cd961a02b999834a675dc8e4fae4b89b"`.
2. For macOS on x86_64 architecture, the SHA256 checksum for the downloaded release tarball was updated from `"a364fdc1d873bde4c95e3d7596a1ee01e473daebca749e468a5a3e4211534d8b"` to `"538fdeb9024a041d29b6647a47381ef701c0fe46841b5bced98038fffa816fd4"`.

These changes ensure the integrity and authenticity of the binaries being downloaded for the respective architectures.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 46 (1921ba7d499c3817f5423a7b55c0b135fab66ea6)</summary>

This commit, authored by Louis Beaumont on September 2, 2024 at 08:48 AM, merges pull request #266 from the mediar-ai repository. The pull request updates the Homebrew formula intended for the x86_64-apple-darwin platform. The merge resulted in commit `1921ba7d499c3817f5423a7b55c0b135fab66ea6`, which integrates changes from commit `812c0b6` and `d39cad9`.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 47 (812c0b694d6c263c7e3511117b682e1d3892f6c1)</summary>

This commit, authored by Louis Beaumont, addresses two main changes:

1. **Benchmark Workflow:**
   - The GitHub Actions workflow file `benchmark.yml` was updated. Specifically, the benchmarking tool used for the "STT Benchmarks" job was changed from "criterion" to "cargo".

2. **Core Tests:**
   - In the file `core_tests.rs` within the `screenpipe-audio/tests` directory, the creation of a "whisper channel" was modified. An additional return value was introduced in the line that creates the channel, changing from two variables to three: `(whisper_sender, mut whisper_receiver)` to `(whisper_sender, mut whisper_receiver, _)`. 

These changes indicate fixes related to benchmarking and testing processes.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 48 (d39cad915046bb8d5d90e971d809436ec335ec01)</summary>

The commit `d39cad915046bb8d5d90e971d809436ec335ec01` by the GitHub Actions Bot, dated September 2, 2024, updates the `screenpipe` formula to version 0.1.72 for `x86_64-apple-darwin`. Specifically:

- The version number in the formula has been updated from `0.1.71` to `0.1.72`.
- The URL for the x86_64-apple-darwin tar.gz file has been updated to reflect the new version.
- The SHA-256 checksum for the x86_64-apple-darwin tar.gz file has been updated accordingly to `538fdeb9024a041d29b6647a47381ef701c0fe46841b5bced98038fffa816fd4`.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 49 (85f959072edba6e5f7ee3d9de293787cf3a27332)</summary>

The commit with hash `85f959072edba6e5f7ee3d9de293787cf3a27332` updates the `screenpipe` formula in Homebrew to version 0.1.72 specifically for the `aarch64-apple-darwin` architecture. The primary changes in this update include:

1. The URL in the formula has been updated to point to the new release archive `screenpipe-0.1.72-aarch64-apple-darwin.tar.gz`.
2. The version has been incremented from `0.1.71` to `0.1.72`.
3. The SHA256 checksum for the `arm64` architecture has been updated to `"b334841820ff3d9fc4dc47c1b33a2f62cd961a02b999834a675dc8e4fae4b89b"`.

This update was made by the GitHub Actions Bot on September 2, 2024.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 50 (27c1e92ac3d3a87d028361fbec7b24d41503d55b)</summary>

The commit primarily addresses memory usage optimization within the `screenpipe-audio` component. Here's a summary of the changes:

1. **Version Increments:**
   - Bumped the version of the main package in `Cargo.toml` from `0.1.71` to `0.1.72`.
   - Bumped the version of the Tauri app in its `Cargo.toml` from `0.1.79` to `0.1.80`.

2. **Core Codebase Changes:**
   - Switched from `Arc` (Atomic Reference Counting) to `Weak` references for `is_running` throughout `screenpipe-audio/src/core.rs`. This prevents memory leaks by allowing the memory to be reclaimed when no `Arc` instances exist.
   - Altered code logic to check if the `Weak` reference can be upgraded (i.e., check if it can still create a strong reference) before loading the `AtomicBool`.
   - Modified various sections where `is_running` was used in loops and conditional checks, ensuring they work with `Weak` references.
   - Added comments for potential improvements or code drops (`// ? ffmpeg.kill().await?` and `// ? drop(tx);`).

3. **Error Handling and Process Management:**
   - Improved the way the FFmpeg process is managed, ensuring the weak references are used to regulate the lifecycle appropriately.
   - Adjusted comments for clarity and future considerations, particularly regarding the audio device stream management and FFmpeg process handling.

Overall, these changes significantly cut down on memory usage by using weak references where appropriate, which ensures better memory management, particularly in long-running processes like audio recording and processing.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 51 (fac2066e46a1acf62fe42dd1e1dd6b489380e01a)</summary>

The changes in commit `fac2066e46a1acf62fe42dd1e1dd6b489380e01a` include:

1. **Cargo.toml Update:**
   - The version number of the package `screenpipe-app` in `examples/apps/screenpipe-app-tauri/src-tauri/Cargo.toml` has been updated from `0.1.78` to `0.1.79`.

2. **Code Changes in `screenpipe-audio/src/stt.rs`:**
   - There are updates related to macOS-specific code. 
   - An `autoreleasepool` block has been added at the import level to manage memory usage efficiently in macOS.
   - Additional `cfg` directives and conditional compilation blocks are used to ensure that the `autoreleasepool` is only utilized when the target OS is macOS.
   - There is a refactor to ensure the `autoreleasepool` is correctly scoped within the macOS-specific sections while also adding handling to flag unreachable sections for non-macOS code.

These updates aim to fix the app build, ensuring better compatibility and memory management on macOS.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 52 (98c88612f4ba0ab2c3448faff4eab8669ecd9fa1)</summary>

The given git changes consist of a commit that merges a pull request into the repository. The commit identifier is 98c88612f4ba0ab2c3448faff4eab8669ecd9fa1, authored by Louis Beaumont on September 1, 2024. This merge combines the changes from branch `58cb166` into the current branch `0097a99`. The pull request (#262) updates the Homebrew formula specifically for the `aarch64-apple-darwin` platform.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 53 (58cb16695fb93a429c67810dd16acca7f1f4fba2)</summary>

The commit `58cb16695fb93a429c67810dd16acca7f1f4fba2` by Louis Beaumont merges the `main` branch into `update-formula-aarch64-apple-darwin-a2fbc5d08aace57ae3bfa546636567c0f9297c8f`. 

The changes occur in the file `Formula/screenpipe.rb`:

1. **For macOS with ARM architecture (`aarch64-apple-darwin`)**:
   - The SHA256 checksum for the URL `screenpipe-<version>-aarch64-apple-darwin.tar.gz` is updated from `8676a000b7bd63d1140d33565c4007e47e1af104f086ba5f4cfaba938e71c65a` to `bc2605353fc9f080d2f06c7590f66c752a928f5073ee8f18fda0906a4becc49d`.

2. **For macOS with x86_64 architecture**:
   - The SHA256 checksum for the URL `screenpipe-<version>-x86_64-apple-darwin.tar.gz` is updated from `ab7058a7c288ee242c16c9c12a7886581303d224532c5676fe38b3a0457ee614` to `a364fdc1d873bde4c95e3d7596a1ee01e473daebca749e468a5a3e4211534d8b`.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 54 (0097a9921f5e99e135db6a7ee9d51b335df3456d)</summary>

The commit `0097a9921f5e99e135db6a7ee9d51b335df3456d` by Louis Beaumont on September 1, 2024, merges a pull request (#263) from the branch `update-formula-x86_64-apple-darwin-a2fbc5d08aace57ae3bfa546636567c0f9297c8f`. The purpose of this merge is to update the Homebrew formula for the `x86_64-apple-darwin` architecture.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 55 (458c8c428a307dfc4b3ef6cbba33e963aa13c220)</summary>

The commit `458c8c428a307dfc4b3ef6cbba33e963aa13c220` by the GitHub Actions Bot updates the `screenpipe` formula in the `Homebrew` repository. The changes include:

- Updating the version of `screenpipe` from `0.1.70` to `0.1.71` in the `screenpipe.rb` formula.
- Updating the `sha256` checksum for the x86_64-apple-darwin version to ensure the integrity of the downloaded file.

This routine maintenance is part of keeping the formula current with the latest released version of `screenpipe` and ensuring compatibility for macOS users.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 56 (d044ac8a30722e62b88208d464f0e35170148afa)</summary>

The commit `d044ac8a30722e62b88208d464f0e35170148afa` involves a chore update conducted by the GitHub Actions Bot. The update pertains to the Homebrew formula for the `screenpipe` library. Specifically, the formula version has been incremented from `0.1.70` to `0.1.71` for `aarch64-apple-darwin` architecture. Additionally, the SHA256 checksum for the arm64 download link has been updated to `"bc2605353fc9f080d2f06c7590f66c752a928f5073ee8f18fda0906a4becc49d"`. This ensures the integrity of the updated package.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 57 (a2fbc5d08aace57ae3bfa546636567c0f9297c8f)</summary>

### Summary of Git Changes

**Commit:** a2fbc5d08aace57ae3bfa546636567c0f9297c8f
**Author:** Louis Beaumont <louis.beaumont@gmail.com>
**Date:** Sun Sep 1 16:25:45 2024 -0700
**Description:** Fix for major performance improvements, specifically addressing a memory leak in the Candle MacOS implementation.

### Key Changes

1. **Cargo.toml Version Updates:**
   - Updated the workspace package version from `0.1.70` to `0.1.71`.
   - Updated the `screenpipe-app-tauri/src-tauri` package version from `0.1.77` to `0.1.78`.
   - Added `objc` (version 0.2.7) dependency for MacOS in `screenpipe-audio/Cargo.toml`.

2. **Performance Improvements in `screenpipe-audio`:**
   - Reduced the channel size for audio data from 1000 to 100.
   - Added proper handling and cleanup for the audio input stream, ensuring the thread is correctly joined and resources are released.
   - Introduced a shutdown flag for the whisper channel to manage its lifecycle more effectively.
   - Added conditional checks and memory management improvements specific to MacOS (via `objc`).

3. **Functional Improvements in `create_whisper_channel`:**
   - Added a new argument `shutdown_flag` to manage the shutdown process.
   - Ensured the whisper channel breaks its loop and performs cleanup when the shutdown flag is set.

4. **Server and Recording Improvements:**
   - Modified `start_continuous_recording` to handle the new `whisper_shutdown_flag`.
   - Ensured all background tasks are correctly awaited and handled errors gracefully.
   - Implemented the shutdown process for the whisper channel at the end of recording sessions to avoid memory leaks.

5. **Miscellaneous Improvements and Fixes:**
   - Added a note to the `parse_json_output` function in `screenpipe-vision` indicating performance issues and suggestions for binary serialization.
   - Removed redundant info log line in `screenpipe-server/src/server.rs`.

Overall, these changes collectively aim at improving the performance and stability of the codebase, particularly focusing on memory management and resource cleanup in the MacOS context.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 58 (ff5a19117bac730698a760a21c45c67e3bc4440d)</summary>

The git commit updates the `vercel-ai-chatbot` model from `gpt-3.5-turbo` to `gpt-4o-mini` within the TypeScript example project. Specifically, the change occurs in the `actions.tsx` file located in the `lib/chat` directory. The modification replaces the model used in the `streamUI` function from `openai('gpt-3.5-turbo')` to `openai('gpt-4o-mini')`.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 59 (a8ac878226def378c6489cc79ff3fc279a7699cd)</summary>

### Summary of Git Changes

#### Commit Information:
- **Commit**: `a8ac878226def378c6489cc79ff3fc279a7699cd`
- **Author**: James Long
- **Date**: Sun Sep 1 22:25:31 2024 +0100
- **Message**: Enhance datetime handling across timezones.

#### Key Changes:
1. **Date Formatting Change**: Switched from `toLocaleString()` to `toString()` to provide a consistent datetime format that avoids confusion for LLMs (Large Language Models).
    - Previous format: Locale-dependent
    - New format: `Thu Jan 01 1970 00:00:00 GMT+0000 (Coordinated Universal Time)`

2. **Simplified Logic and Prompts**:
    - Improved the system prompt to clearly state the handling of date and time, including timezone and offsets.
    - Instruct AI to convert times to UTC only for tool calls.
    - Added guidelines to always reformat timestamps to a human-readable format in responses.

3. **Example Adjustments and User Queries**:
    - Provided updated example queries to demonstrate correct timezone conversions and handling relative time queries (e.g., "in the last 10 minutes").

#### Specific Code Changes:
- Modified logging of the current date from `new Date().toLocaleString()` to `new Date().toISOString()`.
- Updated system prompts to:
    - Display the current time using `new Date().toString()`.
    - Clearly state rules for handling user's timezone and offsets.
    - Include additional rules for converting to UTC and formatting timestamps.
- Adapted example responses to reflect proper timezone conversions (e.g., converting local times to UTC).

These updates collectively improve the reliability and clarity of datetime handling in the system when interacting with users across different timezones.
</details>

------------------------------------------------------------------------

# Overall Summary of Changes

### Summary of Key Git Changes

 

**Commit:**

 

1. **Commit ID:**

```
a42fc3e2cfb47217051982552578abf9b086c699
2. **Affects**: `README.md`
3. **Details**: Removed a commented-out recommendation to add environment variables to the integrated terminal configuration in VS Code. 
4. **Date**: September 3, 2024
5. **Author**: Louis Beaumont
```

6. **Commit ID:**

```
8e21969d0ea4d47c604ed3f08a24b420a0398058
7. **Affects**: `README.md`
8. **Details**: Added "macOS specific" instructions for modifying `.vscode/settings.json` to include environment variables required for linking the Apple native OCR compiled library to the binaries.
9. **Date**: September 3, 2024
10. **Author**: Louis Beaumont
```

11. **Commit ID:**

```
585b4de655d4df8e25aae74e44c70a8187b13da2
12. **Affects**: `README.md`
13. **Details**: Simplified setup instructions, provided new command sequence for releasing the app, and added a caution for Windows users.
14. **Date**: September 3, 2024
15. **Author**: Louis Beaumont
```

16. **Commit ID:**

```
83419b45560ae4cf619fc78e0136844c2bbf73d5
17. **Details**: Merged changes from pull request #271 which updated Homebrew formula for `x86_64-apple-darwin`.
18. **Date**: September 3, 2024
19. **Author**: Louis Beaumont
```

20. **Commit ID:**

```
4ceb875b211373bec1fc4158cf77ddc84e38711c
21. **Details**: Merged main into branch `update-formula-x86_64-apple-darwin-30f56ec8db0a9a10159f34fe4e1ddcc113bff646`, updating SHA-256 checksums for different macOS architectures.
22. **Date**: September 3, 2024
23. **Author**: Louis Beaumont
```

24. **Commit ID:**

```
063357d08815eca35e070bb7283676d1d178d8db
25. **Details**: Merged pull request #272 updating Homebrew formula for `aarch64-apple-darwin`.
26. **Date**: September 3, 2024
27. **Author**: Louis Beaumont
```

28. **Commit ID:**

```
30f56ec8db0a9a10159f34fe4e1ddcc113bff646
29. **Details**: Updated `Cargo.toml` to bump versions from `0.1.74` to `0.1.75` and fixed issues with default audio duration.
30. **Date**: September 3, 2024
31. **Author**: Louis Beaumont
```

32. **Commit ID:**

```
bb7c6928be1b41397bb0edc93d7f14f5988afe37
33. **Details**: Merge pull request #267 updating Homebrew formula for `aarch64-apple-darwin`.
34. **Date**: September 3, 2024
35. **Author**: Louis Beaumont
```

36. **Commit ID:**

```
fcf3612cf926474aed89166d5ac3c7d0b5e74dee
37. **Affects**: `screenpipe-audio/src/pcm_decode.rs`
38. **Details**: Added debug logging for detected format details and information on available tracks in `pcm_decode` function.
39. **Date**: September 3, 2024
40. **Author**: Louis Beaumont
```

These summarized changes span various updates including version bumps, README modifications, workflow and formula updates, debug enhancements, and improvements related to supporting macOS-specific functionalities.
