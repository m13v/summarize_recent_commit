# Git Commit Summaries

-----------------------------------------------------------------------
-----------------------------------------------------------------------
 
PRESS CMD+SHIFT+V TO VIEW IN MARKDOWN
 
_______________________________________________________________________
-----------------------------------------------------------------------
Total number of commits: 26

<details>
<summary>Summary for commit 1 (24a12ddbf30e01ab065764f2276582d0a8d96a5d)</summary>

The git commit made by Louis Beaumont on September 10, 2024, includes changes to bump the version numbers in two `Cargo.toml` files. Specifically:

1. The version in the main `Cargo.toml` file was updated from `0.1.80` to `0.1.81`.
2. The version in the `Cargo.toml` file for the `screenpipe-app` within the `src-tauri` directory was updated from `0.1.90` to `0.1.91`.

These changes are aimed at updating the package versions.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 2 (d36e891fe1ddecda833d38e1fa2961d3f7353704)</summary>

The commit introduces several enhancements and bug fixes to the application, primarily focusing on monitor selection and user interface improvements:

1. **CLI and UI Enhancements:**
   - Fixed the CLI default monitor setting issue.
   - Added a monitor selection UI to the application, allowing users to select multiple monitors instead of a single default monitor.

2. **Empty Screen Prompt:**
   - Improved the empty screen prompt with new and clearer suggestions based on user feedback.

3. **Search Improvements:**
   - Enhanced the search functionality within the application.

4. **Component and Hook Updates:**
   - Modified components to support multiple monitor IDs (`monitorIds` instead of `monitorId`).
   - Updated `RecordingSettings` to handle multiple monitor selections with improved UI interactions (e.g., popovers for monitor selection).
   - Added and utilized a new `ScrollArea` component for better UI scrolling experience.

5. **Server and Core Changes:**
   - Server-side changes to handle multiple monitor IDs.
   - Updated video recording logic to support multiple monitor recordings.
   - Added retries for writing frames to FFmpeg to handle potential write failures more robustly.

6. **Dependency Updates:**
   - Added `@radix-ui/react-scroll-area` to the list of dependencies.

Overall, the changes improve user experience by allowing more flexible monitor selections and providing enhanced search and display functionalities in the application.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 3 (8ec44018e5ce8a666bf5f9bcddac162173e0aa85)</summary>

The commit `8ec44018e5ce8a666bf5f9bcddac162173e0aa85` by Louis Beaumont on September 9, 2024, at 19:36 includes a minor change in the test file `pipes_test.rs` in the `screenpipe-core` directory. Specifically, it adds an `#[ignore]` attribute to the asynchronous test function `test_js_execution`. This change indicates that the `test_js_execution` will be ignored during test runs.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 4 (d1b0cd68f99d1987487006ec2eede48fc66c67f4)</summary>

The commit `d1b0cd68f99d1987487006ec2eede48fc66c67f4` by Louis Beaumont updates the `README.md` file. Here are the key changes:

1. **Description Adjustments**:
   - Changed the description of using "screenpipe" from "after a meeting for example (automatically with our webhooks)" to "to summarize meetings".

2. **Code Section**:
   - Removed extraneous empty lines before and after the code section.

3. **Additional Information**:
   - Added a hyperlink to examples for further usage, enhancing the description about writing plugins in JavaScript which can run in the Rust code through the Deno engine.

4. **Removed Content**:
   - Removed a URL referencing an attachment (`https://github.com/user-attachments/assets/edb503d4-6531-4527-9b05-0397fd8b5976`).

5. **Minor Edits**:
   - Other minor textual adjustments for clarity and format.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 5 (aa897e09ffe672e3ec3c3fe9953cb9632cdf5e2d)</summary>

The commit identified by aa897e09ffe672e3ec3c3fe9953cb9632cdf5e2d was made by Louis Beaumont on Mon Sep 9, 2024. It includes updates to the `README.md` file. The specific changes are as follows:

1. A new example command was added to display the first frame from a specific timeframe (from 30 minutes to 25 minutes ago) for macOS:
    ```sh
    curl "http://localhost:3030/search?limit=1&offset=0&content_type=ocr&include_frames=true&start_time=$(date -u -v-220M +%Y-%m-%dT%H:%M:%SZ)&end_time=$(date -u -v-120M +%Y-%m-%dT%H:%M:%SZ)" | jq -r '.data[0].content.frame' | base64 --decode > /tmp/frame.png && open /tmp/frame.png
    ```

2. Updated system usage statistics were added: the application uses 600 MB of RAM and 10% CPU.

Additionally, an extra line with system resource usage specifications has been included in the list outlining the `Alpha` performance statistics.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 6 (2bd6c73d47c875c4b3558ab12b59cb4198441b8c)</summary>

The commit by Louis Beaumont introduces several changes to the `screenpipe-server` and `screenpipe-vision` codebase to address feature #295. Here's a summary of the changes:

1. **Multi-Monitor Support:**
   - The CLI option for specifying a monitor (`monitor_id`) has been updated to support multiple monitors (`monitor_ids`), changing from a single `Option<u32>` to a `Vec<u32>`.

2. **Code Adjustments for Multi-Monitor Handling:**
   - The code logic throughout the `screenpipe-server` has been updated to correctly handle multiple monitors.
   - The selection and validation of monitors have been adjusted to iterate through multiple monitor IDs.
   - The video tasks that were previously designed to handle a single monitor now handle multiple monitors by spawning tasks for each monitor.

3. **Refactoring and Cleanup:**
   - Removed unused imports and redundant code.
   - Simplified logic for focused window detection and image capture for efficiency.

4. **Updates in `core.rs` and `capture_screenshot_by_window.rs`:**
   - Introduced the ability to handle multiple monitor IDs during the continuous recording process.
   - Adjusted functions to ensure captured images and recorded data are correctly managed for each monitor.
   - Improved error handling to differentiate between errors that occur on different monitors.
   - Removed the redundant `get_focused_window` function and moved its logic inline to make code more straightforward and efficient.

5. **CLI Struct Update in `cli.rs`:**
   - Changed the type of `monitor_id` in the `Cli` struct from `Option<u32>` to `Vec<u32>`.

These changes collectively enhance the application’s ability to handle multiple monitors, thus improving its flexibility and functionality.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 7 (1fe8a01be05bfef048937bd6ba11906bf27d2e9b)</summary>

This git commit merges a pull request (#298) that updates the Homebrew formula for the aarch64-apple-darwin platform. The commit was authored by Louis Beaumont on September 9, 2024.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 8 (9c2840510681ce33fd7167864100bdae2d504e75)</summary>

The commit `9c2840510681ce33fd7167864100bdae2d504e75` involves merging changes from the `main` branch into the `update-formula-aarch64-apple-darwin-8359d1b40aece31a887f763f7b0317224a8fc1be` branch. The updates involve modifications to the `Formula/screenpipe.rb` file:

1. **For MacOS on ARM architecture**:
   - The SHA-256 checksum was updated from `0adf737c627524d3a5f7c1f726103ef143e7ba0c65fe610ac8f9ba8442ed7937` to `48a2b60b2ac44fd23d7c223abdb083c1d0d879e0971c3e9770b0a97ddf572b40`.

2. **For MacOS on x86_64 architecture**:
   - The SHA-256 checksum was updated from `68db5c35ebf77a62bdb75cad79029141be948f56cdbe36abb79e92351b36d622` to `7d8ec607a74110159770787bfaf78b7907e8a371baba67c4439808e95a2634d5`.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 9 (4eeabfdfa14fa7c678561477d1691b5d4238061c)</summary>

This git change merges a pull request (#299) from the `mediar-ai` repository into the current branch. The changes in the pull request include an update to the Homebrew formula specifically for the `x86_64-apple-darwin` architecture. The commit was authored by Louis Beaumont on September 9, 2024.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 10 (3f0306f632ce34ae57bb85f2f2f55c3e47a84458)</summary>

The git commit titled "fix benchmark" by Louis Beaumont involves a small but crucial modification in the OCR (Optical Character Recognition) benchmark code. Specifically, the change is in the `screenpipe-vision/benches/ocr_benchmark.rs` file:

- The code inside the `bench_windows_ocr` function has been updated.
- Previously, the OCR result was obtained with the line:
  ```rust
  let (result, _, _) = perform_ocr_windows(black_box(&image)).await;
  ```
- This line has been changed to unwrap the result:
  ```rust
  let (result, _, _) = perform_ocr_windows(black_box(&image)).await.unwrap();
  ```

The update ensures that the `perform_ocr_windows` function call now explicitly handles the result by unwrapping it, which likely prevents potential errors related to handling `Result` or `Option` types in Rust. This fix aims to improve the stability and accuracy calculations within the benchmark process by ensuring successful OCR operation results.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 11 (ec1848cadebc2fe49b843c066adae11a1ca85f3c)</summary>

The commit `ec1848cadebc2fe49b843c066adae11a1ca85f3c` made by the GitHub Actions Bot on Mon Sep 9, 2024, updates the Homebrew formula for `screenpipe`. Specifically, it:

- Changes the version of `screenpipe` from `0.1.78` to `0.1.79`.
- Updates the SHA256 checksum for the `x86_64-apple-darwin` architecture from `68db5c35ebf77a62bdb75cad79029141be948f56cdbe36abb79e92351b36d622` to `7d8ec607a74110159770787bfaf78b7907e8a371baba67c4439808e95a2634d5`.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 12 (e983e5edc9ced3e39e133c771959e13adfb5de56)</summary>

The commit updates the Homebrew formula for the `screenpipe` library. The changes are as follows:

- Updates the version of `screenpipe` from `0.1.78` to `0.1.79`.
- Changes the URL for the source tarball to point to the new version `0.1.79` instead of `0.1.78`.
- Updates the `sha256` checksum for the `arm64` build to ensure integrity for the new version. 

These updates ensure that the Homebrew formula references the latest version of the software with the correct verification hash.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 13 (8359d1b40aece31a887f763f7b0317224a8fc1be)</summary>

The commit with hash `8359d1b40aece31a887f763f7b0317224a8fc1be` authored by Louis Beaumont on September 9, 2024, includes a modification to the test file `screenpipe-core/tests/pipes_test.rs`. Specifically, it updates the `test_simple_pipe` function by adding the `#[ignore]` attribute to the `#[tokio::test]`, effectively marking this test to be ignored during the test run.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 14 (072052d07cd294882aac3ab4fd60c53244afaa05)</summary>

The commit `072052d07cd294882aac3ab4fd60c53244afaa05` by Louis Beaumont on September 9, 2024, introduces a new feature to the `recording-settings.tsx` component within the `screenpipe-app-tauri` example application. The feature adds explanations to tooltip contents for settings that deal with including or ignoring specific windows during screen recording. The messages now provide examples to clarify how case-insensitive window name matching works:

- For ignored windows: Examples include how "bit" will ignore "Bitwarden" and "bittorrent," and "incognito" will ignore any tabs or windows containing "incognito."
- For included windows: Examples include how "chrome" will match "Google Chrome," and "bitwarden" will match both "Bitwarden" and "bittorrent."
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 15 (836c55ef16591c582f4e265ee0985b11282187f9)</summary>

The commit `836c55ef16591c582f4e265ee0985b11282187f9` introduces new features related to screen recording settings in a Tauri-based application. The changes include:

### Modifications:
1. **`recording-settings.tsx`**:
   - Added two new properties to the local settings: `ignoredWindows` and `includedWindows`.
   - Created functions to handle adding and removing ignored and included windows.
   - Updated the UI to display current ignored and included windows as badges, with the ability to add new windows via input fields.

2. **`use-settings.tsx`**:
   - Extended the default settings with `ignoredWindows` and `includedWindows` arrays.
   - Updated the `Settings` interface to include these new properties.
   - Adjusted the settings loading mechanism to include these new properties.

3. **`Cargo.toml`**:
   - Incremented the application version from `0.1.89` to `0.1.90`.

4. **`sidecar.rs`**:
   - Fetched `ignoredWindows` and `includedWindows` from the settings store.
   - Added these windows as arguments to the sidecar process while spawning it.

### Summary:
This commit enhances the application's configurability by allowing users to specify windows to ignore and include during screen recording. These settings are managed through the UI and respected during the sidecar process execution.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 16 (794f1997a8b294bd66e061ce74f729fa814d32b6)</summary>

This commit introduces the feature to include or ignore specific windows, applications, or tabs in screen recording functionalities for the `screenpipe` project.

**Key Changes:**

1. **Version Update:**
   - Updated the version in `Cargo.toml` from `0.1.79` to `0.1.80`.

2. **CLI Extension:**
   - Added two new command-line arguments in `screenpipe-server/src/cli.rs`:
     - `ignored_windows`: Accepts a list of window titles to ignore.
     - `included_windows`: Accepts a list of window titles to include.

3. **Core Application Modifications:**
   - Updated several functions (e.g., `start_continuous_recording`, `record_video`) in `screenpipe-server/src/core.rs` to handle the new `ignored_windows` and `include_windows` parameters.
   - Passed these parameters to the `VideoCapture` struct and related functions to apply the inclusion/exclusion logic.

4. **Video Capture Adjustments:**
   - Modified `screenpipe-server/src/video.rs` to propagate the ignore and include lists.
   - Adjusted the `VideoCapture` struct to incorporate these lists, influencing which windows to capture.

5. **Utility and Monitoring Adaptations:**
   - Updated the `capture_screenshot` function signature in `screenpipe-vision/src/utils.rs` to include ignore and include lists.
   - Added logic in `capture_all_visible_windows` and `is_valid_window` functions in `screenpipe-vision/src/capture_screenshot_by_window.rs` to respect the ignore and include lists during window capture.

6. **Example and Benchmark Adjustments:**
   - Added a new example (`window-filtering.rs`) demonstrating how to use the new CLI arguments for window filtering in `screenpipe-vision/examples`.
   - Updated the existing examples and benchmarks to handle the new parameters with empty defaults (`[]`).

Overall, these changes help control which windows or applications are included in or ignored from screen recordings based on their window titles, adding significant flexibility for users.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 17 (112b7cf6b60fca6579c126ea2594dbb365975862)</summary>

The commit `112b7cf6b60fca6579c126ea2594dbb365975862` authored by Louis Beaumont on September 9, 2024, updates the `README.md` file located in the `examples/typescript/pipe-security-check` directory. The update includes the addition of several blank lines for formatting purposes and a link to an external resource or asset hosted on GitHub. 

Here's a concise summary of the key changes:

1. Added several blank lines for improved readability.
2. Inserted a URL linking to an asset on GitHub. 

No major content changes besides these additions have been made to the `README.md`.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 18 (751977e2c4996b7c2b8f3fb088e6804c52c80488)</summary>

The commit 751977e2c4996b7c2b8f3fb088e6804c52c80488 by Louis Beaumont encompasses various improvements and updates to the "Screenpipe" project's pipe API and security check functionality.

### Changes in Commit:

#### Documentation:
- **README** updates in `examples/typescript/pipe-security-check/README.md`:
  - Added new command-line examples to run and enable pipes using `ollama` and `screenpipe`.

#### Code Updates:
- **Main Security Check Script (`main.js`):**
  - Changed AI model from `Hermes-llama-3.1:latest` to `mistral-nemo`.
  - Enhancement in filtering browser data from screen data.
  - Refined the prompt for AI to analyze security threats more effectively.
  - Added logic for sending notifications when a security issue is detected.
  - Commented sections to clarify instructions and added conditions to parse AI responses properly.

- **Deno Runtime (`runtime.js`) Adjustments:**
  - Improved the `sendNotification` operation with better error handling and logging sanitation.
  
- **Rust Core (`pipes.rs`):**
  - Enhanced fetch operations to ensure better error handling using `AnyError`.
  - Modified logic to avoid overwriting existing pipes and to accurately fetch and handle files.

- **Pipe Manager (`pipe_manager.rs`):**
  - Added functionality to initialize default pipe configurations.
  - Improved loading of pipe information and incorporated better error handling using `tracing::warn`.
  - Adjusted the API to deliver more structured error messages.

- **Server Handling (`server.rs`):**
  - Updated various endpoints (`add_tags`, `remove_tags`, etc.) to return more structured JSON error responses for better client-side handling.
  - Added new endpoints to handle pipe operations related to the security check.

These changes improve the overall functionality, security, and user experience when using the `screenpipe` system.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 19 (8ba6f2f5707c8d5d2ae52978bf17929a8bcf2d67)</summary>

The recent commit by Louis Beaumont on September 9, 2024, includes the following changes to the `README.md` file:

1. **Addition:** A new line has been added that advises users to open an issue on GitHub if the provided installation steps don't work for them or to get the pre-built desktop app. The added text is:

   ```
   If this does not work for you, please [open an issue](https://github.com/mediar-ai/screenpipe/issues/new?assignees=&labels=dislike&template=dislike.yml&title=windows+install+screenpipe+didnt+work) or get the pre-built [desktop app](https://screenpi.pe)
   ```

2. **Repository URL Update:** The URL for the Git repository to clone has been changed from `https://github.com/louis030195/screen-pipe` to `https://github.com/mediar-ai/screenpipe`.

These changes aim to improve user support and correct the repository's URL.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 20 (f27ee45d54c972973805c5131207ad5ccf797208)</summary>

The commit with hash `f27ee45d54c972973805c5131207ad5ccf797208`, authored by Louis Beaumont and dated Mon Sep 9 2024, merges pull request #292 from joegoldin into the main branch. The primary feature introduced by this pull request is the ability for the Tauri app to run in the background while ensuring it operates as a single instance.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 21 (3ac45469200d62e7cacac006a8f1ed6fca3a000d)</summary>

The git commit titled "fix test" made by Louis Beaumont on September 9, 2024, includes a single modification to the `pipes_test.rs` file in the `screenpipe-core/tests` directory. The specific change involves adding an `#[ignore]` attribute to the `test_pipe_with_http_request` function within the test module. This indicates that the test is not implemented yet and should be ignored during test runs.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 22 (0263bc5917acacc92582b835b039277307a62bc7)</summary>

The commit `0263bc5917acacc92582b835b039277307a62bc7` was authored by Joe Goldin on September 9, 2024. It is a merge commit that combines changes from the 'main' branch into the 'feat-run-in-bg' branch. The commit merges the previous commits identified by hashes `b53d554` and `ad251dc`.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 23 (b53d554275bfceacabff4e29970d3131155ab939)</summary>

The commit with ID **b53d554275bfceacabff4e29970d3131155ab939** by **joegoldin** makes several key updates to a Tauri application for macOS:

1. **Opening the App via Dock Icon**:
   - A new function `show_main_window` was added to handle showing and focusing the main window.
   - This ensures the application can be opened from the dock even when it is hidden.

2. **Right-Click Menu Addition**:
   - The tray icon menu now includes an additional option to show the application window.
   - Previously, it only included the 'Quit' option.

3. **Dependencies Update**:
   - The `tauri` dependency in `Cargo.toml` was modified to include the `"macos-private-api"` feature.

4. **Main Application Changes**:
   - Integration of the new `show_main_window` function within various event handlers.
   - Adjusted tray menu setup to use the new show option.
   - Enhancements to the app's single-instance handling and window focus behavior.
   - Added macOS-specific activation policy setting and event handling for reopening the app.

5. **Configuration Update**:
   - The `tauri.conf.json` was updated to enable `"macOSPrivateApi"`.

These changes collectively enhance the functionality and user experience for macOS users, particularly in how they can interact with the application from the dock and tray menu.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 25 (aa5a755d40d496c4c2c5ee8fdf5b5b5ad329dbe3)</summary>

**Summary of Git Changes:**

- **Author:** Joe Goldin
- **Date:** September 8, 2024
- **Commit ID:** aa5a755d40d496c4c2c5ee8fdf5b5b5ad329dbe3
- **Changes Overview:**
  - **New Feature:** The Tauri app now runs in the background and ensures a single instance is active. Rust version is bumped to 1.75.
  - **Files Modified:**
    - `Cargo.toml`
      - Bumped the app version from `0.1.87` to `0.1.89`.
      - Updated the Rust version from `1.60` to `1.75`.
      - Added the dependency `tauri-plugin-single-instance` for macOS, Windows, and Linux.
    - `main.rs`
      - Updated the Tauri builder to handle the `CloseRequested` event by hiding the window instead of closing it.
      - Integrated the `tauri_plugin_single_instance` to manage single-instance behavior, focusing on already running instances if attempted to relaunch.
      - Updated tray icon event handling to show and focus the main window upon clicking the tray icon.

Overall, these updates aim at improving the app's behavior in running as a single instance, background operation, and user interaction improvements through the system tray.
</details>

------------------------------------------------------------------------

# Overall Summary of Changes

Below is a summarized overview of the provided git changes across multiple commits for the "Screenpipe" project:

### Key Enhancements and Updates:

1. **Version Updates in `Cargo.toml`:**
   - Multiple commits updated version numbers across different files, such as bumping from `0.1.80` to `0.1.81` and `0.1.90` to `0.1.91`. These updates reflect new features, enhancements, and bug fixes.

2. **User Interface Improvements:**
   - Added a monitor selection UI for multiple monitors.
   - Enhanced empty screen prompts and improved search functionality.
   - Implemented a `ScrollArea` component for better UI experiences.

3. **Core and Server Adjustments:**
   - Modified server-side code to handle multiple monitor IDs.
   - Enhanced video recording logic to support multiple monitors.
   - Improved frame writing robustness with retry mechanisms.

4. **Dependency Management:**
   - Updated dependencies such as `@radix-ui/react-scroll-area`.

5. **Testing Adjustments:**
   - Added the `#[ignore]` attribute to certain tests to skip them.
   - Ensured proper unwrapping of results in OCR benchmark tests.

6. **Homebrew Formula Updates:**
   - Merged updates to Homebrew formulas for different macOS architectures with revised SHA-256 checksums to ensure integrity.

7. **README Modifications:**
   - Made various changes for clarity, added usage examples, adjusted formatting, and updated repository URLs.

8. **Feature Implementations:**
   - Introduced the ability to include or ignore specific windows during screen recording.
   - Updated code to handle multiple monitors by iterating monitor IDs.
   - Modified CLI, core logic, and capture functions to support these new features.

9. **Pipe and Security Enhancements:**
   - Enhanced pipe management code for better error handling and introduced new security check functionalities.

10. **MacOS-Specific Adjustments:**
    - Improved support for Tauri app to run in the background and ensure single-instance behavior.
    - Added functionality to display the app from the dock and enhanced tray menu options.

These changes collectively improve flexibility, user experience, system compatibility, and robustness of the "Screenpipe" application, fostering a more efficient and user-oriented tool.
