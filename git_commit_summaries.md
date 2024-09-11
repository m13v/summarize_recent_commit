# Git Commit Summaries

-----------------------------------------------------------------------
-----------------------------------------------------------------------
 
PRESS CMD+SHIFT+V TO VIEW IN MARKDOWN
 
_______________________________________________________________________
-----------------------------------------------------------------------
Total number of commits: 50

<details>
<summary>Summary for commit 1 (5356762036d534e2b8895ad5870ec6563b934440)</summary>

The commit identified by `5356762036d534e2b8895ad5870ec6563b934440`, authored by Louis Beaumont on Sep 11, 2024, includes changes aimed at fixing the continuous deployment (CD) for an app.

### Changes Summary:

#### Workflow Configuration (`.github/workflows/release-app.yml`):
- **Updated vcpkg Usage**: The workflow now uses `lukka/run-vcpkg@v11` instead of `v7`. Parameters such as `vcpkgGitCommitId` and `vcpkgTriplet` were removed.

#### TypeScript components (`pipe-store.tsx`, `search-panel.tsx`):
- **Integration of `posthog` for Analytics**:
  - Imported `posthog` in both `pipe-store.tsx` and `search-panel.tsx`.

  In `pipe-store.tsx`:
  - Added `posthog.capture` calls at various points for:
    - Downloading a pipe (`download_pipe` event with `pipe_id`).
    - Enabling/disabling a pipe (`toggle_pipe` event with `pipe_id` and `enabled` status).
    - Adding a new pipe (`add_own_pipe` event with `newRepoUrl`).

  In `search-panel.tsx`:
  - Added a `posthog.capture` call when a search is initiated (`search` event).

### Purpose of Changes:
These changes appear to fix deployment issues by updating dependencies in the CI configuration and to enhance user interaction tracking within the application by integrating `posthog` for capturing key events and actions.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 2 (50fa928e42ccb12627f2613a96728fd051fcbf93)</summary>

The commit 50fa928e42ccb12627f2613a96728fd051fcbf93, authored by Louis Beaumont on September 11, 2024, aims to fix the continuous deployment (CD) of an app. The changes made in the `.github/workflows/release-app.yml` file include:

1. Addition of new steps specifically for the Windows platform within the GitHub Actions workflow:
   - Identifying usage of 'esaxx' by inspecting the `cargo tree` output.
   - Installing `vcpkg` with a specified commit ID and triplet.
   - Installing ONNX Runtime via `vcpkg`.
   - Setting up MSVC (Microsoft Visual C++) using `ilammy/msvc-dev-cmd@v1`.

These steps are added before the existing build process using `tauri-apps/tauri-action`.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 3 (6d4273aa9d51e2f2a99950433d2c8c152672f46e)</summary>

In the git commit identified by hash `6d4273aa9d51e2f2a99950433d2c8c152672f46e`, authored by Louis Beaumont on September 11, 2024, the version of the `screenpipe-app` package was updated in its `Cargo.toml` file. Specifically, the version was changed from `0.1.93` to `0.2.0`.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 4 (f0be869c318ba6bd259e56f2277c6829b00ec674)</summary>

The commit with hash `f0be869c318ba6bd259e56f2277c6829b00ec674` was authored by Louis Beaumont on September 10, 2024. The commit is titled "bump version" and it updates the version number in the `Cargo.toml` file located at `examples/apps/screenpipe-app-tauri/src-tauri/`. Specifically, it changes the package version from "0.1.912" to "0.1.93".
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 5 (5ba0aaff6a7e9dd50379e8a9fc281ab2efaa0d5b)</summary>

This commit merges two branches and incorporates the following changes:

1. Implementation of manual Silero VAD (Voice Activity Detection) under the project identified by "m13v".
2. Fixes concerning Windows compilation issues with the new VAD library, addressing problems referenced by issues #289 and #301.

The changes were merged from the pull request #303, contributed by joegoldin.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 6 (35f391dfd36edfec69468608b450855afc783c6f)</summary>

This commit, authored by Joe Goldin on September 10, 2024, merged changes from the remote-tracking branch `origin/fix-windows-compilation-vad` into the local branch `m13v-manual-silero-vad-implementation`. The merge aimed to integrate fixes related to Windows compilation issues into the ongoing work on a manual implementation of Silero VAD (Voice Activity Detection).
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 7 (7f4279209a089d067b30983162b503148bf0c16f)</summary>

The commit 7f4279209a089d067b30983162b503148bf0c16f, authored by Joe Goldin and dated September 10, 2024, merges the branch 'manual-silero-vad-implementation' into 'm13v-manual-silero-vad-implementation'. There are no further specific changes provided in the commit message beyond the merge itself.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 8 (2afd7915b5560ed03e74fec0309db4b63c52afe6)</summary>

This commit represents a merge from the `main` branch into the `manual-silero-vad-implementation` branch and includes significant changes to the `screenpipe-server`.

Key changes:
1. **Multiple Monitor Handling**:
   - Instead of using a single `monitor_id`, the code now supports handling multiple monitor IDs, managed as a `Vec<u32>`.
   - Changes in `screenpipe-server.rs` reflect the iteration and usage of multiple monitor IDs.

2. **Voice Activity Detection (VAD)**:
   - Addition of `vad_engine` parameter and clone mechanism in multiple places to utilize this new functionality.
   - Update in `cli.rs` adds a new argument for `vad_engine` with a default value of `Silero`.

3. **Window Filters**:
   - Introduced new CLI options for `ignored_windows` and `included_windows` which allow the user to specify lists of window titles for inclusion/exclusion during screen recording.
   - Changes to the `start_continuous_recording` function in `core.rs` reflect new parameters for handling these window filters.

4. **Handling Shutdown**:
   - Updated logic for cloning `friend_wearable_uid` and VAD engine to ensure they are properly cloned for each asynchronous iteration.

These enhancements provide more flexibility and control over monitoring, detection engines, and window management during operation.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 9 (609796084ad377547116ef6e7ff28d339bde99f1)</summary>

The commit made by Joe Goldin on September 10, 2024, primarily involves updating the version of the `actions/upload-artifact` GitHub Action from `v2` to `v3` across multiple workflow files. The specific files and lines changed are:

1. `.github/workflows/ci.yml`:
   - The action version is updated on line 160.

2. `.github/workflows/perf-long-running-end-to-end.yml`:
   - The action version is updated on lines 79, 83, 89, and 101.

3. `.github/workflows/release-cli.yml`:
   - The action version is updated on lines 123 and 172.
   - Additionally, there is an update for `actions/download-artifact` from `v2` to `v3` on line 243.

These changes align to ensure that the workflows utilize the latest version of the `upload-artifact` and `download-artifact` actions.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 10 (5cfa40a73646e82a69beadcf55a1a85363ec11ae)</summary>

In this commit, Joe Goldin made changes to the `windows_output_device_tests.rs` file in the `screenpipe-audio/tests` directory to fix a test related to the CABLE virtual device. Specifically, the test titled `test_virtual_audio_device` was modified to handle situations where the virtual device driver ("CABLE Output") is not installed. Instead of the test failing when the virtual device is not found, it now returns silently with an `Ok(())` value, preventing potential test failures due to the absence of the virtual device.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 11 (c52820f4faba55ca8d4dc8c706129469f1c4a209)</summary>

The commit by Joe Goldin, identified by hash `c52820f4faba55ca8d4dc8c706129469f1c4a209`, made a modification to the `Cargo.toml` file in the `screenpipe-audio` directory. Specifically, the line defining the dependency on `v8` version `0.106.0` was removed. There were no additional changes or additions to the file.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 12 (b8193b9c31fe64c77c309ccada15e6a107ed7d86)</summary>

The commit by Joe Goldin on September 10, 2024, includes changes to some configuration files for improving Windows compilation with a new Voice Activity Detection (VAD) library. Below is a summary of the changes:

1. **`.cargo/config.toml`**:
   - An extraneous diff notation indicating a previously missing new line at the end of the file has been resolved.

2. **`screenpipe-audio/Cargo.toml`**:
   - Added three new dependencies:
     - `v8` (version `0.106.0`)
     - `ort` (version `2.0.0-rc.5` with the `load-dynamic` feature and without default features)
     - `esaxx-rs` (version `0.1.10`)

3. **`screenpipe-vision/Cargo.toml`**:
   - Explicitly added a `[target.'cfg(target_env = "msvc")'.dependencies.vcpkg]` section with `version = "0.2"`, specific to the MSVC target environment.

These changes collectively address updates for dependencies and configuration adjustments necessary for Windows compatibility, particularly focusing on the new VAD library integration.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 13 (90deb7fdcc8b8e3ae7ab7111e1cf046edbee343c)</summary>

This commit introduces several new features and changes to the Screenpipe Tauri app:

1. **Search Panel Integration**:
    - Added a new search UI integrating a search panel.
    - `page.tsx`: Updated with a new **Tabs** component containing "chat" and "search" tabs. Requires `useState` and additional imports for tabs.
    - `search-panel.tsx` is a new file providing the search interface with functionalities such as querying data, filters, content types, date pickers, and searching.

2. **Chat UI Enhancements**:
    - `chat-list-openai-v2.tsx` and `chat-message-v2.tsx`: Modified to include handling and notes for errors involving embedded videos which previously could crash the app.

3. **New Components**:
    - `date-time-picker.tsx`: New component for selecting date and time.
    - `video.tsx`: Provides functionality to play video or audio file and a link to open file location.
    - Various new UI components under `components/ui` include `calendar`, `progress`, and `slider`.

4. **UI Improvements**:
    - Removed some commented-out console logs from different components such as `pipe-store.tsx`.
    - `empty-screen.tsx`: Updated the list of user suggestions for the empty screen view.

5. **Backend and Config Updates**:
    - Enhanced querying functions in `lib/screenpipe.ts` to include types and additional parameters for frames.
    - `Cargo.toml`: Bumped the version from `0.1.91` to `0.1.912`.
    - `package.json`: Introduced several new dependencies for date handling, progress bar, slider, etc.
    - `capabilities/main.json`: Added permission `fs:allow-open` for file operations.

6. **General Code Cleanup**:
    - Removed redundant or commented-out code.
    - Improved error handling and informative error messages particularly in `chat-list-openai-v2.tsx` and `video.tsx`.

These changes improve the app's usability by providing a robust search feature, enhancing the chat UI, and improving the overall user experience with new components and better error handling.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 14 (24a12ddbf30e01ab065764f2276582d0a8d96a5d)</summary>

The commit identified by `24a12ddbf30e01ab065764f2276582d0a8d96a5d` was made by Louis Beaumont on September 10, 2024. The commit message indicates that the version numbers were incremented.

Specifically, the following changes were made:
- In `Cargo.toml`, the version was updated from `0.1.80` to `0.1.81`.
- In `examples/apps/screenpipe-app-tauri/src-tauri/Cargo.toml`, the version was updated from `0.1.90` to `0.1.91`.

No other changes to the files were noted in this commit. The primary purpose was to update the version numbers.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 15 (d36e891fe1ddecda833d38e1fa2961d3f7353704)</summary>

This commit introduces several significant changes and improvements to the project's functionality, particularly around the UI and monitor handling:

1. **CLI and Monitor Handling (`recording-settings.tsx` and related files)**:
   - Fixed default monitor setting in CLI.
   - Added the ability to select multiple monitors, replacing the single monitor selection.
   - Modified the setting storage to handle an array of monitor IDs (`monitorIds`) instead of a single monitor ID.
   - Updated the UI to reflect these changes, providing a more interactive selection mechanism using popovers and checkboxes.
   - Enhanced logging for better debugging and monitoring state changes.

2. **UI Improvements (`empty-screen.tsx`, `scroll-area.tsx`, etc.)**:
   - Improved empty screen prompt suggestions based on user feedback for better usability.
   - Added a `ScrollArea` component using `@radix-ui/react-scroll-area` to handle scrollable areas in UI, enhancing the user experience.
   - Updated package dependencies to include `@radix-ui/react-scroll-area`.

3. **Settings and Data Handling (`use-settings.tsx`, `sidecar.rs`, etc.)**:
   - Updated default settings to use an array for monitor IDs and refactored the settings interface to reflect this change.
   - Ensured backward compatibility and transition from single to multiple monitor handling in stored settings.
   - Adjusted the server code to handle arrays of monitor IDs, and ensure continuous recording on multiple monitors.
   - Added detailed logging for monitor operations in the server-side recording logic.

4. **Core and Video Handling (`core.rs`, `video.rs`)**:
   - Enhanced video recording handling by iterating over multiple monitor IDs and creating separate video files per monitor.
   - Implemented retry logic for writing frames to FFmpeg to improve reliability.

These changes aim to improve the application's usability, robustness, and flexibility in handling multiple monitors and providing a better overall user experience.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 16 (8ec44018e5ce8a666bf5f9bcddac162173e0aa85)</summary>

The commit made by Louis Beaumont on September 9, 2024, includes the following change:

- In the `screenpipe-core/tests/pipes_test.rs` file, the `test_js_execution` test function was modified. Specifically, the `#[ignore]` attribute was added to this function.

This change will cause the `test_js_execution` test to be ignored during test runs.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 17 (d1b0cd68f99d1987487006ec2eede48fc66c67f4)</summary>

The commit `d1b0cd68f99d1987487006ec2eede48fc66c67f4` authored by Louis Beaumont on September 9, 2024, updates the `README.md` file with the following changes:

1. Revised a pseudo code block description to:
   - Change: "Here's a pseudo code to illustrate how to use screenpipe, after a meeting for example (automatically with our webhooks):"
   - To: "Here's a pseudo code to illustrate how to use screenpipe, to summarize meetings:"

2. Removed an extraneous newline within the pseudo code block.

3. Enhanced a usage statement to include a link:
   - Change: "Or thousands of other usages of all your screen & mic data!"
   - To: "Or thousands of other usages of all your screen & mic data! [Check examples](https://github.com/mediar-ai/screenpipe/tree/main/examples/typescript), screenpipes makes it easy to write plugins in JS for example that runs directly in the Rust code through Deno engine."

4. Removed an unused/unclear link:
   - Removed: "https://github.com/user-attachments/assets/edb503d4-6531-4527-9b05-0397fd8b5976"

These changes clarify the usage instructions and clean up the content by removing unnecessary text and incorporating helpful resources.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 18 (aa897e09ffe672e3ec3c3fe9953cb9632cdf5e2d)</summary>

The commit `aa897e09ffe672e3ec3c3fe9953cb9632cdf5e2d` by Louis Beaumont on September 9, 2024, involves updates to the `README.md` file. Specifically, the changes include:

1. Adding a new example command showing how to display the first frame from 30 minutes to 25 minutes ago using curl on macOS.
2. Updating the documentation to include a note that the program uses 600 MB of memory and 10% CPU.

These changes help provide additional usage instructions and system resource information.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 19 (2bd6c73d47c875c4b3558ab12b59cb4198441b8c)</summary>

**Commit Summary:**

**Commit ID:** 2bd6c73d47c875c4b3558ab12b59cb4198441b8c
**Author:** Louis Beaumont
**Date:** Mon Sep 9 18:52:47 2024 -0700
**Description:** Added support for multiple monitors in screenpipe-server.

**Changes Made:**

1. **screenpipe-server/src/bin/screenpipe-server.rs:**
   - Removed the usage of `get_monitor_by_id` to validate a single monitor.
   - Refactored the logic to handle multiple monitor IDs.
   - Updated CLI monitor ID to a vector allowing multiple IDs.
   - Modified printing log to display multiple monitor IDs.
   - Adjusted logic to clone monitor IDs for use across threads and functions.
   - Replaced single monitor video task with multiple tasks for each monitor.

2. **screenpipe-server/src/cli.rs:**
   - Updated `monitor_id` field from `Option<u32>` to `Vec<u32>` to support multiple monitor IDs.

3. **screenpipe-server/src/core.rs:**
   - Modified `start_continuous_recording` function to handle multiple monitor IDs.
   - Refactored video recording tasks to spawn for each monitor ID.
   - Added joining of all video tasks to handle completion and error checks for each monitor separately.

4. **screenpipe-vision/src/capture_screenshot_by_window.rs:**
   - Updated the `capture_all_visible_windows` function to capture windows for a specific monitor instead of iterating over all monitors within the function.
   - Removed the `get_focused_window` internal function and integrated its logic within `capture_all_visible_windows`.

5. **screenpipe-vision/src/utils.rs:**
   - Updated the `capture_screenshot` function to pass the specific monitor to `capture_all_visible_windows`.

**Overall Impact:**
This commit enhances the `screenpipe-server` to support recording from multiple monitors concurrently. The change entails updates to CLI options, internal data structures, and the main logic that handles monitor selection and recording tasks, ensuring that each monitor’s capture process is managed individually and efficiently.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 20 (1fe8a01be05bfef048937bd6ba11906bf27d2e9b)</summary>

This commit, authored by Louis Beaumont, merges changes from a pull request (#298) into the main branch. The pull request updates the Homebrew formula to support the `aarch64-apple-darwin` architecture.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 21 (9c2840510681ce33fd7167864100bdae2d504e75)</summary>

The given git changes reflect a merge commit made by Louis Beaumont, in which the 'main' branch was merged into the 'update-formula-aarch64-apple-darwin-8359d1b40aece31a887f763f7b0317224a8fc1be' branch.

The changes pertain to the file `Formula/screenpipe.rb` and involve updates to the SHA-256 checksums for the downloaded `screenpipe` binaries, specifically:

1. For the `aarch64-apple-darwin` architecture, the SHA-256 checksum was updated from `0adf737c627524d3a5f7c1f726103ef143e7ba0c65fe610ac8f9ba8442ed7937` to `48a2b60b2ac44fd23d7c223abdb083c1d0d879e0971c3e9770b0a97ddf572b40`.
2. For the `x86_64-apple-darwin` architecture, the SHA-256 checksum was updated from `68db5c35ebf77a62bdb75cad79029141be948f56cdbe36abb79e92351b36d622` to `7d8ec607a74110159770787bfaf78b7907e8a371baba67c4439808e95a2634d5`.

These updates ensure the integrity and authenticity of the downloaded binaries for the specified platforms.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 22 (4eeabfdfa14fa7c678561477d1691b5d4238061c)</summary>

The commit `4eeabfdfa14fa7c678561477d1691b5d4238061c` is a merge commit authored by Louis Beaumont, merging pull request #299. The purpose of this merge is to update the Homebrew formula for the `x86_64-apple-darwin` architecture. The merge involves changes between the commits `3f0306f` and `ec1848c`.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 23 (3f0306f632ce34ae57bb85f2f2f55c3e47a84458)</summary>

Commit `3f0306f632ce34ae57bb85f2f2f55c3e47a84458` authored by Louis Beaumont on September 9, 2024, addresses a benchmark fix in the `screenpipe-vision` project.

### Changes:

- File: `screenpipe-vision/benches/ocr_benchmark.rs`
- Location: Inside the `bench_windows_ocr` function.
- Modification:
  - Changed the line that performs OCR on Windows from:
    ```rust
    let (result, _, _) = perform_ocr_windows(black_box(&image)).await;
    ```
  - To:
    ```rust
    let (result, _, _) = perform_ocr_windows(black_box(&image)).await.unwrap();
    ```

### Purpose:
The change ensures that the result of the `perform_ocr_windows` function is properly unwrapped, likely to handle a `Result` type and ensure any errors are immediately surfaced during benchmark execution.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 24 (ec1848cadebc2fe49b843c066adae11a1ca85f3c)</summary>

The commit `ec1848cadebc2fe49b843c066adae11a1ca85f3c` made the following changes to the `Formula/screenpipe.rb` file:

- Updated the version of the `screenpipe` library from `0.1.78` to `0.1.79`.
- Correspondingly changed the download URL from `v0.1.78` to `v0.1.79`.
- Updated the SHA256 checksum for the `x86_64` architecture to reflect the new version `0.1.79`.

These changes ensure that the Homebrew formula for `screenpipe` uses the latest version available, version `0.1.79`.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 25 (e983e5edc9ced3e39e133c771959e13adfb5de56)</summary>

The changes in the commit `e983e5edc9ced3e39e133c771959e13adfb5de56` include updating the `screenpipe` software formula to a newer version for Homebrew. Specifically, the version has been updated from `0.1.78` to `0.1.79`, and the corresponding URL and SHA256 checksum for the `aarch64-apple-darwin` architecture have been updated. The changes ensure the formula points to the correct download link and provides the correct checksum for the new version.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 26 (8359d1b40aece31a887f763f7b0317224a8fc1be)</summary>

The commit with ID `8359d1b40aece31a887f763f7b0317224a8fc1be` was authored by Louis Beaumont and made on Mon, Sep 9, 2024. The purpose of this commit is to fix a test in the `pipes_test.rs` file located in `screenpipe-core/tests/`.

The specific change involves adding the `#[ignore]` attribute to the `test_simple_pipe` function within the test module. This attribute causes the test to be ignored when running tests. Here is the change in detail:

```diff
 @tokio::test
+    #[ignore]
     async fn test_simple_pipe() {
```
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 27 (072052d07cd294882aac3ab4fd60c53244afaa05)</summary>

In the commit `072052d07cd294882aac3ab4fd60c53244afaa05`, Louis Beaumont added features related to ignoring and including windows in the `recording-settings.tsx` file of the Screenpipe app built with Tauri. The changes include updates to tooltips to provide examples of how the window filtering works in a case-insensitive manner:

1. For ignored windows, examples were added to illustrate how specific terms like "bit" or "incognito" would affect filtering.
2. For included windows, examples were provided to show how terms like "chrome" or "bitwarden" would work in the filtering process. 

These enhancements help users understand how to specify windows for inclusion or exclusion during screen recording.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 28 (836c55ef16591c582f4e265ee0985b11282187f9)</summary>

The commit by Louis Beaumont introduces new features related to managing window inclusion and exclusion in the screen recording settings for the screenpipe-app. The changes involve updating several files:

1. **`recording-settings.tsx`**:
   - Added two new settings: `ignoredWindows` and `includedWindows`.
   - Implemented functions to handle adding and removing these windows.
   - Updated the UI to include new sections allowing users to add and manage ignored and included windows.

2. **`use-settings.tsx`**:
   - Updated the default settings to include `ignoredWindows` and `includedWindows` as empty arrays.
   - Modified the `Settings` interface to include these new arrays.
   - Added code to load these settings from the stored settings.

3. **`Cargo.toml`**:
   - Bumped the package version from `0.1.89` to `0.1.90`.

4. **`sidecar.rs`**:
   - Modified the sidecar spawning process to read and pass the `ignoredWindows` and `includedWindows` settings if they are not empty.
   - Updated command arguments to include these settings.

Overall, these changes enhance the screen recording app by allowing users to specify which windows should be ignored or included during a recording session.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 29 (794f1997a8b294bd66e061ce74f729fa814d32b6)</summary>

This commit introduces a feature to selectively include or ignore specific windows, apps, and tabs during screen capture and recording in the "screenpipe" project.

Key changes include:

1. **Cargo.toml**: Version bump from `0.1.79` to `0.1.80`.

2. **CLI Modifications (screenpipe-server/src/cli.rs)**:
   - Added new CLI options:
     - `--ignored-windows`: Accepts a list of window titles to exclude from recording.
     - `--included-windows`: Accepts a list of window titles to include in recording.

3. **Core Functional Changes**:
   - Updated various functions in `screenpipe-server/src/core.rs`, `screenpipe-server/src/bin/screenpipe-server.rs`, and `screenpipe-vision/src/core.rs` to handle the new `ignored_windows` and `included_windows` parameters for selective window handling.

4. **Example and Test Adjustments**:
   - Updated examples and tests to include empty lists for the new parameters, ensuring backward compatibility.
   - Added a new example (`window-filtering.rs`) to demonstrate the window filtering feature based on the new CLI options.

5. **Refactor and Enhance Window Handling (screenpipe-vision/src/capture_screenshot_by_window.rs)**:
   - Improved the function `capture_all_visible_windows` to respect the new `ignored_windows` and `included_windows` lists.
   - Modified the `is_valid_window` and `get_focused_window` functions to incorporate these lists for better control over window capture.

6. **Miscellaneous**:
   - Minor formatting and typo corrections throughout the modified files.
   - Added cloning of argument lists and ensured consistent application of filters for window selection in screen capture logic.

In summary, this commit enhances the "screenpipe" application by allowing users to specify which windows, apps, or tabs to include or ignore during screen recording, offering more granular control over the capture process.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 30 (112b7cf6b60fca6579c126ea2594dbb365975862)</summary>

**Commit Summary:**

- **Commit Hash:** 112b7cf6b60fca6579c126ea2594dbb365975862
- **Author:** Louis Beaumont
- **Date:** September 9, 2024

**File Modified:**
- `examples/typescript/pipe-security-check/README.md`

**Changes Made:**
- Added a hyperlink to the README.md file which points to a URL with user attachments (likely an image or document).
- The URL added is: `https://github.com/user-attachments/assets/a4ab0d24-996c-45f5-bffd-142f9757cca5`
- The hyperlink is surrounded by blank lines for readability.
- The remaining content of the README.md file before and after the addition remains unchanged.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 31 (751977e2c4996b7c2b8f3fb088e6804c52c80488)</summary>

The commit mainly focuses on improvements to the pipe API and enhancements related to security checks. Here are the key changes summarized:

1. **Documentation Updates**:
    - The `README.md` in the `examples/typescript/pipe-security-check` directory was updated to include instructions on running and enabling pipes, both via `ollama` commands and direct command-line execution using `screenpipe`.

2. **JavaScript File Enhancements** (`main.js`):
    - The AI model was updated from `Hermes-llama-3.1:latest` to `mistral-nemo`.
    - Security check logic was refined to focus specifically on browser-related data.
    - Structure and rules of the AI prompt were improved for clarity.
    - Added handling to ensure the JSON response does not contain extraneous text.
    - Notifications changed from `console.error` to `pipe.sendNotification`.

3. **Core Runtime Updates** (`runtime.js`):
    - Improved the notification sending flow, including better error handling and logging.

4. **Internal API Adjustments** (`pipes.rs`):
    - Modified function signatures to include more specific error handling with `AnyError`.

5. **Pipe Manager Refactoring**:
    - Adjustments to config loading and pipe handling, ensuring robustness even if config files are missing.
    - Introduced a helper function for loading pipe information.

6. **Server Enhancements** (`server.rs`):
    - Improved error responses in API handler functions to return structured JSON errors rather than plain strings.

Overall, these updates improve the usability, security focus, and robustness of the system, particularly around handling pipe configurations and security checks.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 32 (8ba6f2f5707c8d5d2ae52978bf17929a8bcf2d67)</summary>

The commit made by Louis Beaumont on September 9, 2024, includes the following changes to the `README.md` file:

1. **Additional Instruction:**
   - Added a suggestion to open an issue if the experimental Windows build support doesn't work: 
     ```
     If this does not work for you, please [open an issue](https://github.com/mediar-ai/screenpipe/issues/new?assignees=&labels=dislike&template=dislike.yml&title=windows+install+screenpipe+didnt+work) or get the pre-built [desktop app](https://screenpi.pe)
     ```

2. **Repository URL Update:**
   - Corrected the clone URL for the `screen-pipe` repository from:
     ```
     git clone https://github.com/louis030195/screen-pipe
     ```
     to:
     ```
     git clone https://github.com/mediar-ai/screenpipe
     ```
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 33 (f27ee45d54c972973805c5131207ad5ccf797208)</summary>

The commit `f27ee45d54c972973805c5131207ad5ccf797208` authored by Louis Beaumont on September 9, 2024, merges a pull request (#292) from a branch `feat-run-in-bg` created by Joe Goldin. The feature added in this merge allows a Tauri application to run in the background and ensures it operates as a single instance.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 34 (3ac45469200d62e7cacac006a8f1ed6fca3a000d)</summary>

The commit with hash `3ac45469200d62e7cacac006a8f1ed6fca3a000d` by Louis Beaumont made on Mon Sep 9, 2024, includes a minor change to the file `screenpipe-core/tests/pipes_test.rs`. In this change, an attribute `#[ignore]` was added to the `test_pipe_with_http_request` function, indicating that the test is to be ignored with a note `TODO: fix this test (not implemented yet)`. This suggests that the test is currently not working or incomplete, and a reminder has been inserted to fix it in the future.

</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 35 (0263bc5917acacc92582b835b039277307a62bc7)</summary>

This commit (0263bc5917acacc92582b835b039277307a62bc7) was authored by Joe Goldin on September 9, 2024. It merges the 'main' branch into the 'feat-run-in-bg' branch. The merge involves combining the changes from the 'main' branch into the feature branch named 'feat-run-in-bg'. The commit itself doesn't introduce new changes but integrates updates from another branch.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 36 (b53d554275bfceacabff4e29970d3131155ab939)</summary>

The commit introduces several improvements and fixes to the macOS version of the Screenpipe Tauri app:

1. **Dependencies Update:**
   - Modified the `Cargo.toml` to include a new feature, `macos-private-api`, in the `tauri` dependency.

2. **Function to Show Main Window:**
   - Added a new function `show_main_window` in `main.rs` to handle the display and focus of the main window.

3. **Tray Menu Updates:**
   - Updated the tray menu to include a "Show Screenpipe" option in addition to the "Quit Screenpipe" option.
   - Modified the event handler to use `app_handle` and handle the "show" action by invoking the `show_main_window` function.

4. **Single Instance Plugin Update:**
   - Modified the single instance plugin initialization to ignore `args` and `cwd`.

5. **Activation Policy for macOS:**
   - Added a conditional activation policy to ensure the app remains active on macOS.

6. **Window Event Handling:**
   - Introduced handling for `WindowEvent::Focused` to make sure the main window is shown and focused when it is meant to be.
   - Added handling for the `Reopen` event on macOS to show the main window when no windows are visible.

7. **Configuration Update:**
   - Updated `tauri.conf.json` to enable `macOSPrivateApi`.

These changes ensure the app correctly handles being focused and reopened on macOS, improves user interaction via the tray icon, and configures the app to use macOS private APIs.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 38 (8f563cab2b28939eeb63f056713f3a0c09d8182b)</summary>

This git commit, authored by Louis Beaumont, focuses on fixing an issue related to cloning objects in the `screenpipe-server.rs` file.

Key changes include:
1. Minor formatting correction (removal of an extra space) in a comment on the same line as the argument `restart_sender`.
2. Relocating the cloning of `vad_engine`:
   - Previously, both `friend_wearable_uid` and `vad_engine` were cloned outside of a loop.
   - The cloning of `vad_engine` has been moved inside the loop, presumably to ensure it is cloned for each iteration.
   
These changes seem to ensure proper handling of these objects, specifically improving how `vad_engine` is managed within the loop.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 39 (ae6f234c030381a3ec187c6b2167190168d0dba4)</summary>

This Git commit, authored by Louis Beaumont on September 9, 2024, merges the 'main' branch into the 'manual-silero-vad-implementation' branch. The changes involve modifications to three files: `screenpipe-server.rs`, `cli.rs`, and `core.rs`.

1. **`screenpipe-server.rs`**:
   - Introduced a new variable `vad_engine` cloned from the command-line interface (CLI) arguments.
   - Enhanced the task restart mechanism with the addition of new Tokio runtime handles (`audio_handle` and `vision_handle`) for managing asynchronous tasks.
   - Replaced the existing mechanism to handle the recording task restart and shutdown, making use of `tokio::select!` to handle multiple asynchronous events like task completion, restart timer ticks, and shutdown signals.
   - Added the `vad_engine` parameter to the `start_continuous_recording` function call to support the VAD (Voice Activity Detection) feature.

2. **`cli.rs`**:
   - Introduced a new CLI argument `vad_engine` to select the VAD engine (either `Silero` or `WebRtc`).
   - Added a new subcommand structure to manage pipes, including actions like listing, downloading, getting info, enabling, disabling, and updating pipes.

3. **`core.rs`**:
   - Modified the `start_continuous_recording` function to accept new parameters: `vad_engine`, `vision_handle`, and `audio_handle`.
   - These changes support the integration of a VAD engine and the management of asynchronous tasks through Tokio runtime handles.

The overall updates primarily focus on integrating a new VAD engine into the recording process and improving the task management structure for better handling of recording task restarts and shutdowns.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 40 (aa5a755d40d496c4c2c5ee8fdf5b5b5ad329dbe3)</summary>

The commit `aa5a755d40d496c4c2c5ee8fdf5b5b5ad329dbe3`, authored by Joe Goldin, introduces the following changes:

- **Feature Additions**:
  - **Run Tauri App in Background**: Implements functionality to hide the application window instead of closing it when the close button is pressed, effectively allowing the app to run in the background.
  - **Single Instance Application**: Adds support to ensure only a single instance of the Tauri app can run at any time by utilizing `tauri-plugin-single-instance`.
  
- **Version Updates**:
  - **App Version**: Updates the version of the `screenpipe-app` from `0.1.87` to `0.1.89`.
  - **Rust Version**: Bumps the Rust version requirement from `1.60` to `1.75`.

- **Dependency Changes**:
  - Adds the `tauri-plugin-single-instance` dependency for macOS, Windows, and Linux platforms.

- **Code Modifications**:
  - **Cargo.toml**: Corresponding changes to update the version numbers and add the single-instance plugin dependency.
  - **main.rs**:
    - Adds event handling to hide the window on close rather than exiting the app.
    - Integrates `tauri-plugin-single-instance` to bring the existing app window into focus if another instance is started.
    - Modifies tray icon click handling to show and focus the main app window when the tray icon is clicked.

This commit was cherry-picked from a prior commit `073364675fc71841611c938a2cdc8dba580051a0`.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 42 (2b5a3852671ef6deafd90c3b7315dc1bcdcc9493)</summary>

The Git commit made by matthew-heartful on Sep 6, 2024, includes modifications aimed at resolving a Windows build issue. The changes are summarized as follows:

1. **.cargo/config.toml**:
   - **Original**: `rustflags = ["-C", "target-feature=-crt-static"]`
   - **Updated**:
     ```toml
     rustflags = [
         "-C", "target-feature=-crt-static",
         "-C", "link-args=/NODEFAULTLIB:libcmt.lib"
     ]
     ```
   - **Purpose**: Added an additional linker argument `-C link-args=/NODEFAULTLIB:libcmt.lib` to the `rustflags`.

2. **.github/workflows/ci.yml**:
   - Added a new step within the `test-vcpkg` job to check out the repository using `actions/checkout@v3`.
   - Specified `working-directory: ./screen-pipe` for the `Build with vcpkg integration` step.

These changes likely aim to fix the issue with building the project on Windows by adjusting the Rust configuration and refining the CI workflow.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 43 (08872ba2a3a05ea9b91bd385243d54177499fd20)</summary>

This commit authored by 'matthew-heartful' focuses on resolving a build issue on Windows. The primary changes include updates to the GitHub Actions CI configuration and some Rust project dependency tweaks. Here’s a summary:

1. **GitHub Actions CI Workflow (`.github/workflows/ci.yml`)**:
   - Added the `override: true` parameter to the Rust toolchain setup.
   - Introduced steps to install `vcpkg` and the ONNX Runtime using `vcpkg`.
   - Added a step to set up MSVC (Microsoft Visual C++).
   - Introduced a new job `test-vcpkg` that:
     - Installs `cargo-vcpkg`.
     - Builds the project with `vcpkg` integration.

2. **Project Dependency Configurations**:
   - Removed the `dependencies = ["onnxruntime"]` line from `[workspace.metadata.vcpkg]` in the `Cargo.toml`.
   - Removed the conditional Windows `vcpkg` dependency from the `screenpipe-audio/Cargo.toml`.

These changes collectively aim to ensure that the Windows build processes successfully by correctly configuring the necessary dependencies and build tools.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 44 (921701bb1ab9895cff197645298a452a9aacf3eb)</summary>

The commit 921701bb1ab9895cff197645298a452a9aacf3eb by matthew-heartful attempts to address a Windows build issue. The changes occur in the `.cargo/config.toml` file, specifically for the target `x86_64-pc-windows-msvc`. The update simplifies the `rustflags` configuration:

- Removed multiple linker arguments and specific target features.
- Retained only the `-C target-feature=-crt-static` rustflag.

This suggests a move towards simplifying or addressing conflicts with the specified linker arguments for building on Windows.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 45 (e509775714cbd026adc82249d0593633fcf7ccb3)</summary>

The git commit made by Matthew Heartful on September 5, 2024, adds several changes to the GitHub Actions CI workflow, specifically in the `.github/workflows/ci.yml` file:

1. **Identification of `esaxx` usage on Windows:**
   - A new job step is added to generate a dependency tree using `cargo tree` and search for the string `esaxx` in the output. This step continues even if it encounters errors (`continue-on-error: true`).

2. **Setting `RUSTFLAGS` for Linux builds:**
   - A conditional step is added for Linux runners to set the `RUSTFLAGS` environment variable to allow multiple definitions during the linking phase.

3. **Running tests with modified environment:**
   - A new step that runs `cargo test` with the modified `RUSTFLAGS` environment variable.

These changes are aimed at improving the Windows build process and enabling specific environment settings for different operating systems during CI testing.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 46 (e83953ea72e198c9640b97522de5bbe4ebe30027)</summary>

The commit `e83953ea72e198c9640b97522de5bbe4ebe30027` by the author `matthew-heartful` introduces changes to the `config.toml` file for Windows builds with Rust and updates the `Cargo.toml` file for the project dependencies. Here is a summary of the changes:

1. In `.cargo/config.toml`:
   - Additional linker arguments have been added to the `rustflags` for the `x86_64-pc-windows-msvc` target:
     - `-C link-arg=/NODEFAULTLIB:libcmt.lib`
     - `-C link-arg=/NODEFAULTLIB:libucrt.lib`
     - `-C link-arg=/NODEFAULTLIB:libvcruntime.lib`
     - `-C link-arg=/DEFAULTLIB:ucrt.lib`
     - `-C link-arg=/DEFAULTLIB:vcruntime.lib`

2. In `Cargo.toml`:
   - The property `dynamic = true` has been added next to the vcpkg dependency configuration.

These changes seem to address the configuration needed for setting up and linking libraries correctly for building the project on Windows using Vcpkg.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 47 (484b6b1542e2080b9d793b442a2dee1243bac8cf)</summary>

The commit with hash `484b6b1542e2080b9d793b442a2dee1243bac8cf` by author `matthew-heartful` includes the following updates for enhancing the Windows build process using vcpkg:

1. **Cargo.toml:**
   - Added a new dependency for `vcpkg` version "0.2".
   - Introduced a new `[workspace.metadata.vcpkg]` section which specifies:
     - Dependencies: `"onnxruntime"`
     - Git repository: `https://github.com/microsoft/vcpkg`
     - Revision: `"2023.04.15"`

2. **screenpipe-audio/Cargo.toml:**
   - For Windows-specific builds (`cfg(target_os = "windows")`), added `vcpkg` as a dependency from the workspace.

No other changes are made to these files as part of this commit. The changes provide a mechanism to manage and build C++ dependencies using vcpkg in a cross-platform Rust project, specifically facilitating the Windows build environment.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 48 (37c19bbea0777e0677bce4bf6ee5889163ef1d33)</summary>

The commit `37c19bbea0777e0677bce4bf6ee5889163ef1d33` by Matthew Heartful on September 5, 2024, introduces a new GitHub Actions workflow to check for cross dependencies in the project. Key changes include:

1. **Modification in `test-ubuntu` Job:**
   - Added `RUSTFLAGS: "-C link-arg=-Wl,--allow-multiple-definition"` to the environment configuration.

2. **New Job `check-dependencies`:**
   - Runs on `ubuntu-latest`.
   - Steps added:
     - Checks out the repository using `actions/checkout@v3`.
     - Sets up the Rust toolchain with minimal profile and stable toolchain using `actions-rs/toolchain@v1`.
     - Runs a shell command to generate the dependency tree using `cargo tree`, outputs to `cargo_tree_output.txt`, and searches for usage of `esaxx` and `abseil-cpp`. Logs a message if no direct usage is found.
     - Uploads the `cargo_tree_output.txt` file as an artifact using `actions/upload-artifact@v2`.

This new workflow aims to analyze and verify cross dependencies within the project to ensure no direct usage of certain libraries (`esaxx` and `abseil-cpp`).
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 49 (5fc0e3bc77c6703f216f761ffe9d7f6711363fef)</summary>

The commit `5fc0e3bc77c6703f216f761ffe9d7f6711363fef` by Matthew Heartful introduces a new configuration file specifically for building on Windows. The changes are made to the `.cargo/config.toml` file, which was newly added to the repository. The configuration targets the `x86_64-pc-windows-msvc` platform and includes two compiler flags: one to enable static linking of the C runtime (`+crt-static`) and another to allow multiple definitions (`/FORCE:MULTIPLE`). These changes were likely implemented to address issues related to the Siler VAD (Voice Activity Detection).
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 50 (71093af074e69ee05b018442a497bc2bc6d35b44)</summary>

The commit `71093af074e69ee05b018442a497bc2bc6d35b44`, authored by `matthew-heartful`, reintroduces changes related to the Silero Voice Activity Detection (VAD) implementation in the `screenpipe` project. Key changes are as follows:

1. **README.md Updates**:
   - Added instructions to use either Silero-VAD or WebRTC for identifying speech segments in audio transcription. The default VAD engine is Silero.

2. **Dependency Additions (Cargo.toml)**:
   - Added `ndarray` and `vad-rs` dependencies to support the new VAD functionality.

3. **Code Changes**:
   - **VAD Engine Enum**: Created an enumeration `VadEngineEnum` to switch between `Silero` and `WebRtc`.
   - **Integration with Channel Creation**: Modified functions `create_whisper_channel` and `stt` in multiple files to incorporate the new VAD engine. VAD engine selection is currently hardcoded to Silero.
   - **New VAD Engine Module**: Added a new module `vad_engine.rs` which provides implementations for both Silero and WebRTC VAD engines, with a factory method `create_vad_engine` to initialize them.
   - **Silero VAD**:
     - Downloads the Silero VAD model.
     - Uses the Silero VAD to filter out non-speech segments based on a predefined threshold.
   - **Tests and CLI**:
     - Updated tests to use the VAD engine.
     - Updated the `screenpipe-server` CLI to include a new option `vad-engine` to specify the VAD engine to use. The default value is set to `Silero`.

These changes aim to improve audio transcription accuracy by integrating advanced VAD capabilities, allowing flexibility between using a local model (Silero) or a WebRTC-based approach.
</details>

------------------------------------------------------------------------

# Overall Summary of Changes

### Git Commit Summary (Sep 11, 2024)

#### Continuous Deployment and Analytics Integration
**Commit:** 5356762036d534e2b8895ad5870ec6563b934440  
**Author:** Louis Beaumont  
**Date:** Sep 11, 2024  

- **CI Configuration (`.github/workflows/release-app.yml`):**
  - Updated `lukka/run-vcpkg` to version `v11`.
  - Removed `vcpkgGitCommitId` and `vcpkgTriplet` parameters.

- **TypeScript (`pipe-store.tsx`, `search-panel.tsx`):**
  - Integrated `posthog` for analytics.
  - Captured events for downloading, toggling, and adding pipes in `pipe-store.tsx`.
  - Added event capture for initiating searches in `search-panel.tsx`.

#### Windows Platform and Continuous Deployment Fixes
**Commit:** 50fa928e42ccb12627f2613a96728fd051fcbf93  
**Author:** Louis Beaumont  
**Date:** Sep 11, 2024  

- **CI (`.github/workflows/release-app.yml`):**
  - Added steps for Windows to inspect dependencies, install `vcpkg`, ONNX, and set up MSVC.

#### Version Updates
**Commit:** 6d4273aa9d51e2f2a99950433d2c8c152672f46e  
**Author:** Louis Beaumont  
**Date:** Sep 11, 2024  

- **Cargo.toml Version Bump:**  
  - Updated `screenpipe-app` version from `0.1.93` to `0.2.0`.

**Commit:** f0be869c318ba6bd259e56f2277c6829b00ec674  
**Author:** Louis Beaumont  
**Date:** Sep 10, 2024  

- **Cargo.toml Version Update:**  
  - Updated version from `0.1.912` to `0.1.93`.

#### Silero VAD and Windows Issue Fixes
**Commit:** 7f4279209a089d067b30983162b503148bf0c16f  
**Author:** Joe Goldin  
**Date:** Sep 10, 2024  

- **Merge Branches:** 
  - Related to manual Silero VAD implementation and fixes for Windows compilation issues.

#### Multiple Monitor and VAD Engine Updates
**Commit:** 2bd6c73d47c875c4b3558ab12b59cb4198441b8c  
**Author:** Louis Beaumont  
**Date:** Sep 9, 2024  

- **screenpipe-server:** 
  - Enhancements for multiple monitor handling.
  - Added `vad_engine` and window filter options.
  - Improved shutdown handling and added CLI options.

#### Workflow and Artifact Updates
**Commit:** `4eeabfdfa14fa7c678561477d1691b5d4238061c`  
**Author:** Joe Goldin  
**Date:** Sep 10, 2024  

- **Workflow (`.github/workflows/ci.yml`, `release-cli.yml`, `perf-long-running-end-to-end.yml`):**
  - Updated `actions/upload-artifact` to `v3`.
  - Updated related artifact actions.

#### Virtual Audio Device Test Fix
**Commit:** `f53f25efba6ea576a4cc2b70dd823942a7da4693`  
**Author:** Joe Goldin  
**Date:** Sep 10, 2024  

- **Test Handling (`windows_output_device_tests.rs`):**  
  - Adjusted virtual audio device test for better error handling.

#### README and Documentation
**Commit:** d1b0cd68f99d1987487006ec2eede48fc66c67f4  
**Author:** Louis Beaumont  
**Date:** Sep 9, 2024  

- **README Enhancements:**
  - Improved clarity, added usage instructions, and linked helpful resources.

---

These changes focus on improving the continuous deployment pipeline, enhancing analytics and user interaction tracking, updating software versions, fixing platform-specific issues, and improving documentation and usability. The updates reflect efforts to ensure seamless deployment and better user insights while addressing technical debt and version management.
