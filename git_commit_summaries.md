# Git Commit Summaries

-----------------------------------------------------------------------
-----------------------------------------------------------------------
 
PRESS CMD+SHIFT+V TO VIEW IN MARKDOWN
 
_______________________________________________________________________
-----------------------------------------------------------------------
Total number of commits: 48

<details>
<summary>Summary for commit 1 (951615d57466ebff7f10f919da1540c4132cd5d9)</summary>

This commit reflects a merge of a pull request into the main codebase. The pull request (#337) was aimed at updating the Homebrew formula specifically for the aarch64-apple-darwin architecture. The merge was authored by Louis Beaumont and was completed on September 16, 2024.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 2 (72935a0ac66a7b6f0a4a3f80e2846b77fa9b1d77)</summary>

The git changes involve a merge commit by author Louis Beaumont, which integrates the 'main' branch into a branch named 'update-formula-aarch64-apple-darwin-1074df3b89bf80e88e044304ade7c68da2485cd1'. 

The specific file affected is `Formula/screenpipe.rb`. In this file, the sha256 checksums for both the ARM64 and x86_64 macOS releases have been updated:

- For the ARM64 version (`aarch64-apple-darwin`), the sha256 checksum has changed from `fbfb6f20dc2a0ea9cab8dddc37261f6803fa55b013c4378cfe6c3975edddb3bb` to `d41406d867674b21103ec089bf34d77e0476de6ac22e22a6da72adb42dc5fec8`.
- For the x86_64 version (`x86_64-apple-darwin`), the sha256 checksum has changed from `c1071dd2ba6f7e26e318d75ae2c97ef8f2959d2640a836d09242e5abd88aac9c` to `ef2ca50886b92c2ff995a9f3d0f7e6391d832940bd68f130c85fedd2680712a1`.

These changes ensure that the correct checksums are used for verifying the integrity of the downloaded files.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 3 (9d2eb5dbfee1f260958569034808dba79012880d)</summary>

This commit represents a merge operation where Louis Beaumont incorporated changes from a pull request (#338) into the main branch. The pull request was focused on updating the Homebrew formula specifically for the x86_64-apple-darwin architecture. The merge combined changes from two commits: `1074df3` and `4be1ed6`.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 4 (4be1ed694bf5bee6954fdc218302c38933d83ddb)</summary>

This commit, authored by the GitHub Actions Bot, updates the Homebrew formula for the `screenpipe` library. The primary changes in `screenpipe.rb` include:

1. **Version Update**: The version number is bumped from `0.1.84` to `0.1.85`.
2. **SHA256 Checksum Update**: For the x86_64-apple-darwin architecture, the SHA256 checksum for the tarball has been updated from `c1071dd2ba6f7e26e318d75ae2c97ef8f2959d2640a836d09242e5abd88aac9c` to `ef2ca50886b92c2ff995a9f3d0f7e6391d832940bd68f130c85fedd2680712a1`.

These updates ensure that the Homebrew formula reflects the latest version and the corresponding correct file integrity verification checksum.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 5 (3bd38ac9c47b8b28b747bef555dd95b49f0bd732)</summary>

The git changes involve an update to the `screenpipe` formula file in a Homebrew repository. The primary change is that the version of `screenpipe` has been upgraded from 0.1.84 to 0.1.85 for the `aarch64-apple-darwin` platform. This update includes changes to the URL for downloading the new version and the SHA256 checksum for verification:

- **Version updated:** 0.1.84 -> 0.1.85
- **URL updated:** `screenpipe-0.1.84-aarch64-apple-darwin.tar.gz` -> `screenpipe-0.1.85-aarch64-apple-darwin.tar.gz`
- **SHA256 checksum for arm64:** `fbfb6f20dc2a0ea9cab8dddc37261f6803fa55b013c4378cfe6c3975edddb3bb` -> `d41406d867674b21103ec089bf34d77e0476de6ac22e22a6da72adb42dc5fec8`

These changes ensure that users downloading the package on macOS with ARM processors will get the latest version, and the integrity of the downloaded file can be verified.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 6 (1074df3b89bf80e88e044304ade7c68da2485cd1)</summary>

The commit includes the following notable changes:

1. **Version Updates**:
   - `Cargo.toml` updated from version `0.1.83` to `0.1.84`.
   - `screenpipe-app-tauri/src-tauri/Cargo.toml` updated from version `0.2.43` to `0.2.44`.

2. **Frontend Enhancements** in `search-chat.tsx`:
   - Import of new components (`X`, `Square`, `DialogDescription`, etc.).
   - New features and states added, such as `isCurlDialogOpen` and `isStreaming`.
   - Generation of cURL commands for exporting searches.
   - Improved error handling and abort functionality for streaming AI responses.
   - Dialogue and tooltip components for better UX.
   - UI enhancements like tooltips for interpreting timestamps and conditional rendering improvements.

3. **Backend Changes** in `db.rs`:
   - Logic updated to ensure OCR content is always searched for when `app_name` or `window_name` is specified.
   - Refactored the search functionality to ensure `Audio` content is only searched if `app_name` and `window_name` are not specified.
   - Improved sorting and limiting of combined search results.

Overall, these changes improve search functionalities, user interaction, and error handling in both the frontend and backend components.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 7 (85127c9f8188355a1a589fa36f39dc595b7ce7ff)</summary>

The commit includes the following changes:

1. **Meeting History Component Improvements**:
   - Replaced `localStorage` with `localforage` for storing and retrieving meetings, providing better error handling.
   - Added support for streaming summaries from OpenAI's API and handling storage limits by cleaning up older meetings.
   - Introduced a copy-to-clipboard feature for the summary and full transcription, including relevant toast notifications.

2. **Settings Component Enhancements**:
   - Updated input fields for the AI API key and model to disable auto-correct, auto-capitalize, and auto-complete.

3. **Dependency Additions**:
   - Added `localforage` to the project dependencies in `package.json`.

4. **Minor Updates**:
   - Updated package version from `0.2.42` to `0.2.43` in `Cargo.toml`.

These changes collectively improve data handling, user interaction, and overall functionality of the app.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 8 (04ba4347a5a8bba324c915caf10f96d3d898b5a8)</summary>

This commit introduces a new feature for adjusting the frames per second (FPS) in the recording settings of the application. Here's a summary of the changes:

1. **Import Changes:**
   - Added `Slider` component to the `recording-settings.tsx` file.

2. **Interface Updates:**
   - Updated local settings interface to include `fps`.

3. **Event Handling:**
   - Implemented `handleFpsChange` function to update the local settings with the new FPS value.

4. **UI Adjustments:**
   - Added a new section in the UI for adjusting the recording FPS with a slider component and a tooltip explaining its purpose.

5. **Default Settings:**
   - Added `fps` with a default value of 0.5 to the default settings in `use-settings.tsx`.

6. **Settings Retrieval:**
   - Included `fps` in the settings load function, with platform-specific default values (0.2 for macOS and 1 for others).

7. **Sidecar Process:**
   - Modified the sidecar process initialization to include `fps` as an argument if it deviates from its default value of 0.5.

These changes allow users to customize the recording frame rate, providing flexible options for resource management and recording quality.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 9 (da479b07f2e809bb3c4be13e85af7ba3be43291b)</summary>

The recent commit `da479b07f2e809bb3c4be13e85af7ba3be43291b` introduces several changes aimed at making the AI context more user-friendly in the Screenpipe application.

### Summary:
1. **Context Usage Indicator** (`context-usage-indicator.tsx`):
   - Replaced static `<svg>` graphs with animated progress indicators using the `framer-motion` library.
   - Enhanced rendering of context usage with improved styling and animation.

2. **Recording Settings** (`recording-settings.tsx`):
   - Display of the development mode notification is now conditional on the `settings.devMode` being true or specific props, ensuring usability.
   - Minor logic improvements for rendering.

3. **Search Chat** (`search-chat.tsx`):
   - Replaced the fixed `MAX_CONTENT_LENGTH` with a dynamic value from settings.
   - Introduced more interactive elements like checkboxes for selecting search results.
   - Added logic to compute selected content length and updated the validation and toast messages accordingly.
   - Enhanced user interactions with hover effects and transitions using `framer-motion`.
   - Improved overall UI/UX for handling search results, including refined tooltips and content selection indicators.

4. **Settings** (`settings.tsx`):
   - Added a new slider component for configuring the maximum AI context characters (`aiMaxContextChars`).
   - Included clear descriptions and tooltips to explain the purpose and limits of the new setting.

5. **Settings Hooks** (`use-settings.tsx`):
   - Updated the default settings to include a new property `aiMaxContextChars`.
   - Adjusted the logic to handle the new settings attribute, ensuring proper loading and updating from the store.

6. **Application Version** (`Cargo.toml`):
   - Incremented the application version from `0.2.41` to `0.2.42`.

These changes collectively aim to provide a more intuitive and responsive user experience, particularly around displaying and managing AI context usage, and enhance configurability through the updated settings interface.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 10 (c73c5ff259576463f233f8b4e8be10ab12de9546)</summary>

The commit updates the version number of the `screenpipe-app` package in the `Cargo.toml` file from "0.2.40" to "0.2.41". The commit message is simply "bump," indicating this is likely a version bump.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 11 (88dfcbe0088257ace5bb1f368b6b1befe58e393e)</summary>

The commit 88dfcbe0088257ace5bb1f368b6b1befe58e393e by Louis Beaumont introduces several improvements to the `screenpipe-app-tauri` and `screenpipe-server` projects, focusing on enhancing the CLI rendering, tooltips, user experience (UX) feedback, and explanations.

### Key Changes in `screenpipe-app-tauri/components/meeting-history.tsx`:

1. **Imports and Dependencies:**
   - Added `useMemo` import.
   - Formatted import statements consistently using double quotes.
   - Added import for `Badge` component.

2. **Local Storage Functions:**
   - Unified the quotation marks.

3. **Fetch Meetings Logic:**
   - Improved logging messages for consistency.
   - Refactored async function error messages to be lowercase.

4. **Notification Logic:**
   - Enhanced notification messages.
   - Removed some `debouncedCapture` blocks related to analytics.

5. **Meeting Processing:**
   - Refactored meeting merging logic for clarity and consistency.

6. **Rendering Improvements:**
   - Added memoization for expensive computations to optimize re-rendering.
   - Improved error handling and user alerts within the `DialogContent`.
   - Enhanced loading indicator and placeholder rendering.
   - Displayed badges and additional explanations, especially focusing on transcriptions and summaries.

7. **Overall UI/UX:**
   - Provided more detailed and user-friendly descriptions within tooltips and UI elements.
   - Enhanced the dialog layout for better readability and added memoization to optimize performance.

### Key Changes in `screenpipe-app-tauri/components/search-chat.tsx`:

1. **Scroll Handling Logic:**
   - Added state and ref for detecting if the user is scrolling.
   - Adjusted logic to handle scrolling more gracefully, especially around AI loading times.

2. **Form Handling:**
   - Improved input handling to trigger scroll to bottom before processing input.

3. **Tooltip and Help Text:**
   - Added more descriptive help text for tooltips.

4. **Progress Indicators:**
   - Adjusted the positioning of progress indicators for better visual feedback during loading.

### Key Changes in `screenpipe-server/src/bin/screenpipe-server.rs`:

1. **Monitor and Pipe Display Logic:**
   - Updated monitor and pipe display logic for better clarity.
   - Refactored monitor loop to reflect correct IDs.
   - Improved display formatting for pipes, ensuring clearer status messages.

These changes collectively improve the overall usability, readability, and maintainability of the `screenpipe-app-tauri` and `screenpipe-server` projects, providing a more consistent look and feel while enhancing functionality and user experience.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 12 (33604941b571dd24f120e47125cb96d175cfc1e3)</summary>

The commit titled "fix: better context indicator" introduces a new component and improves the user experience in the context of search-related functionalities. Here are the key changes:

1. **New Component**:
   - A new file `context-usage-indicator.tsx` is added that defines a `ContextUsageIndicator` component. This component shows a progress indicator and an alert icon when the usage exceeds 90%.

2. **Integration in Existing Functionality**:
   - In `search-chat.tsx`, the `ContextUsageIndicator` is imported and integrated where appropriate.
   - Updates are made to handle scenarios where the content length exceeds the maximum allowed length (`MAX_CONTENT_LENGTH` set to 30,000).

3. **UI Modifications**:
   - The search button is modified to be part of a flex container that ensures proper spacing and alignment.
   - The indicator and tooltip logic are updated to show the context usage status dynamically and provide appropriate messages when limits are exceeded.

Overall, this commit refines the user interface by offering a context-aware usage indicator, aiding users in understanding their usage limits and guiding them to refine their actions accordingly.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 13 (de558a47c68ba1d6c5217b3e7a701a01789bc2d3)</summary>

This commit introduces several new features and improvements, including:

1. **New Features:**
   - An "Open Data Folder" button with a tooltip in the health status dialog.
   - A tooltip icon next to frames in the search chat results explaining that this is the frame where the text appeared.

2. **Bug Fixes and Adjustments:**
   - Adjustment in `screenpipe-status.tsx`:
     - The `data-dir` example in the debugging commands has been corrected.
     - Added new imports: `Folder` from `lucide-react`, `open` from `@tauri-apps/plugin-shell`, and `homeDir` from `@tauri-apps/api/path`.
     - Adjusted some text formatting for better readability.
     - A method to handle opening the data directory was introduced.
   - In `search-chat.tsx`:
     - Improved structure with a div wrapping the frame image and tooltip for better alignment.
     - Enhanced tooltip explanations for frames in search results.

3. **Backend Enhancements:**
   - Added `include_frames` as a query parameter in the `queryScreenpipe` function.

Overall, the commit enhances the user interface by adding new interactive elements and improving existing ones, while also making backend adjustments to support these features.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 14 (6cacc641c26f9b2a9aa98d06d5c14e7f2a145c8a)</summary>

This commit, authored by Louis Beaumont, addresses a build issue by removing the `SearchPanel` component from the `screenpipe-app-tauri` project. Here are the key changes:

1. **Modification in `app/page.tsx`:**
   - The `SearchPanel` import statement has been removed from this file.
   - No other changes were made to this file.

2. **Deletion of `components/search-panel.tsx`:**
   - The entire file containing the implementation of the `SearchPanel` component was deleted.
   - This file included 396 lines of code related to the `SearchPanel` component's functionality, including UI elements, state management, search logic, and rendering of search results.

Summary: The `SearchPanel` component has been completely removed from the project, and its import in `page.tsx` has been deleted to fix the build issue.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 15 (61e80670594b992f223d229c1d3c06e9cd53935e)</summary>

The commit titled "move min length to api aisle" includes several changes aimed at moving the minimum length functionality for filtering content items from the front-end to the back-end API. Here are the key changes:

1. **Functional Changes in `search-chat.tsx`:**
   - The `minLength` state initializes with a value of 50 instead of 100.
   - Removed local filtering of results based on content length; instead, this filtering is now handled by the API.
   - As part of the initial query, the API request now includes `min_length` and `max_length` parameters.

2. **Changes in `screenpipe.ts` (API handling):**
   - Added `min_length` and `max_length` to API query validation schema, each having default values.
   - These parameters are now passed as strings in the API request.

3. **Dependency Adjustments in `Cargo.toml`:**
   - Updated the package version from `0.2.39` to `0.2.40`.
   - Adjusted the versions of several dependencies including `tauri`.

4. **Permission Schema Update in `acl-manifests.json`:**
   - This file has been modified to add a newline to the end of the file.

Overall, the main objective of this commit is to offload the filtering of content items based on their length from the client-side to the server-side, improving efficiency and simplifying the front-end code.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 16 (14717c43bc6127ab9b4d6ac242f934ea4f3d2eed)</summary>

The commit `14717c43bc6127ab9b4d6ac242f934ea4f3d2eed` by Louis Beaumont merges a pull request (#325) into the main codebase. This merge includes changes that introduce a minimum size search feature in the API, aiming to reduce noise in the search results. The commit occurred on September 16, 2024.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 17 (697434202e7a9d0a94632a1ac8fd59d874353b17)</summary>

The commit involves merging updates from the `main` branch into the `min-max-size-in-api` branch and includes modifications to two files:

1. **`screenpipe-server/src/db.rs`:**
   - Enhanced SQL query construction to include filters for `min_length` and `max_length` of OCR text, allowing searches based on text length.
   - Introduced `param_count` to dynamically handle the SQL parameter indices.

2. **`screenpipe-server/src/server.rs`:**
   - Added several new cURL command examples to demonstrate the usage of the new `min_length` and `max_length` parameters in search queries for OCR, audio content, and specific window/app names.
   - Demonstrated a process to search for recent video content, aggregate file paths from multiple searches, merge those video paths, and generate a merged video by sending a POST request to the merge endpoint.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 18 (20b5b936ef418e3db75018fcf5d49eb32bf26f7e)</summary>

**Summary of Git Changes**

**Commit Overview:**
- **Commit Hash:** 20b5b936ef418e3db75018fcf5d49eb32bf26f7e
- **Author:** Louis Beaumont <louis.beaumont@gmail.com>
- **Date:** Mon Sep 16 12:01:59 2024 -0700
- **Commit Message:** fix: total, impl caching, other stuff

**Detailed Changes:**

1. **File:** `screenpipe-server/src/db.rs`
    - **Caching Improvements:**
        - Enabled SQLite's query result caching by setting cache size to 2MB and storing temporary tables and indices in memory.
    - **Parameterization Enhancements:**
        - Improved parameter handling in SQL queries to dynamically include/exclude conditions based on the presence of related parameters (`min_length`, `max_length`, `app_name`, `window_name`).
        - Utilized `param_count` to dynamically manage the SQL parameters.

2. **File:** `screenpipe-server/src/server.rs`
    - **Code Refactoring:**
        - Changed from using `try_join_all` to `try_join` for concurrently executing the `search` and `count_search_results` operations.
    - **Error Handling:**
        - Improved error handling for concurrent search operations to ensure both operations complete successfully or fail together.
    - **Comment Updates:**
        - Modified and cleaned up comments related to CURL commands for searching and listing operations for better clarity and reference.

These changes enhance code readability, maintainability, and performance by enabling query caching and refining the management of dynamic SQL parameters. Additionally, the improved error handling and concurrent execution logic in the server module contribute to more robust search operations.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 19 (ad01105f04dda401c94b6409fb14a372cd5b9ce0)</summary>

The commit `ad01105f04dda401c94b6409fb14a372cd5b9ce0` is a merge commit authored by Louis Beaumont. It merges changes from the pull request #335, which was created by jandremarais. The purpose of this merge is to fix issues related to broken Linux application builds caused by conflicts in C bindings. The merge combines the parent commits `176a05b` and `aa39af0`.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 20 (176a05ba44aca8d2b47c3b5816c2fce18f05d1fc)</summary>

The commit by Louis Beaumont includes a version bump in the `screenpipe-app-tauri/src-tauri/Cargo.toml` file. Specifically:

1. The `tauri-build` package version is updated from `2.0.0-rc.9` to `2.0.0-rc.11`.
2. The `tauri` package version is updated from `2.0.0-rc.10` to `2.0.0-rc.14`.

These changes update the versions of the Tauri-related dependencies to newer release candidates.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 21 (6945373b1cccba71b34a8b7c1b025e93bb4b3369)</summary>

The git changes involve a merge commit where a pull request (#333) was merged into the main branch. The pull request was focused on updating the Homebrew formula specifically for the aarch64-apple-darwin platform. The commit was made by Louis Beaumont on September 16, 2024.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 22 (b437b45394080ab109c1990fcb67ebad1ec08a4a)</summary>

The git commit b437b45394080ab109c1990fcb67ebad1ec08a4a, authored by Louis Beaumont on September 16, 2024, is a merge commit that combines the changes from branch 'main' into 'update-formula-aarch64-apple-darwin-edce2aea7091c97c185045938ba4b2794aed245a.'

The update specifically modifies the `Formula/screenpipe.rb` file to change the SHA256 checksums for the download URLs:
- The checksum for the `aarch64-apple-darwin.tar.gz` file is updated from `90e9b699b54c3b38840acb5f6c90b12f9923df575099d3e9a3574a7ed46f0db3` to `fbfb6f20dc2a0ea9cab8dddc37261f6803fa55b013c4378cfe6c3975edddb3bb`.
- The checksum for the `x86_64-apple-darwin.tar.gz` file is updated from `36bca4157bc6655d3e4f0f186f6da272ecae2128f0c128ec3f7dc3d174fdef16` to `c1071dd2ba6f7e26e318d75ae2c97ef8f2959d2640a836d09242e5abd88aac9c`.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 23 (e95a7a2ba3f604a827594b8c3f77527138a8dac1)</summary>

The commit merges a pull request (#334) from the `mediar-ai` repository that updates the Homebrew formula for the x86_64-apple-darwin architecture.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 24 (aa39af06891071c070292428de0764c51e698629)</summary>

The commit made by Jan Marais on September 16, 2024, introduces changes to the `.cargo/config.toml` file. The modifications include:

1. **Formatting Changes:**
   - Reformatted the existing rustflags for the target `x86_64-pc-windows-msvc` to ensure each flag and its corresponding argument are on separate lines.

2. **New Configuration Added:**
   - Added a rustflags section for the `x86_64-unknown-linux-gnu` target with a new rustflag `-C link-arg=-Wl,--allow-multiple-definition`.

These changes likely aim at improving readability and adding support for allowing multiple definitions when linking for the Linux GNU target.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 25 (6958721e818c6e2dfbaafd7af6df8843f44f6f6f)</summary>

The given git commit updates the `screenpipe` formula in Homebrew from version 0.1.82 to 0.1.84 specifically for the `x86_64-apple-darwin` architecture. The key changes are:

1. The version number is updated from `0.1.82` to `0.1.84`.
2. The URL for downloading the `screenpipe` tarball is updated to reflect the new version number.
3. The SHA-256 checksum for the `x86_64` architecture is updated to match the new tarball.

This update ensures that users installing `screenpipe` on `x86_64-apple-darwin` get the latest version 0.1.84 with the correct checksum for verification.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 26 (6f5bb495cdeaa4a8093ecfc495f10d3439a754d4)</summary>

The commit updates the Homebrew formula for the `screenpipe` library to version 0.1.84 for the `aarch64-apple-darwin` architecture. The specific changes include:

1. Updating the URL to point to the new version release `0.1.84`.
2. Changing the version number from `0.1.82` to `0.1.84`.
3. Updating the SHA-256 checksum for the `arm64` architecture to ensure the integrity of the downloaded file.

The commit is a chore task performed by the GitHub Actions Bot for managing dependencies.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 27 (6b782c93d678028f80c9ecd9d4b2d6f2aa696c5e)</summary>

The git commit with hash `6b782c93d678028f80c9ecd9d4b2d6f2aa696c5e` by Louis Beaumont makes modifications to the GitHub Actions workflow file `release-app.yml`. Specifically, it enables the release job to trigger automatically on any push that includes a tag starting with "v". The previously commented-out push event configuration for tags has been uncommented and activated.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 28 (edce2aea7091c97c185045938ba4b2794aed245a)</summary>

The commit `edce2aea7091c97c185045938ba4b2794aed245a` by Louis Beaumont updates multiple files with the primary aim of fixing issues related to Deepgram transcription. Here's a summary:

1. **Cargo.toml files:**
   - The version numbers have been incremented:
     - In the root `Cargo.toml`, the version is updated from `0.1.82` to `0.1.83`.
     - In `screenpipe-app-tauri/src-tauri/Cargo.toml`, the version is updated from `0.2.38` to `0.2.39`.

2. **`screenpipe-audio/src/stt.rs`:**
   - The function `transcribe_with_deepgram` has been modified to accept an additional parameter `sample_rate`.
   - The `sample_rate` in the WAV specification is now dynamically adjusted based on the provided `sample_rate` divided by 3.
   - In `stt` function, the call to `transcribe_with_deepgram` now includes the `audio_input.sample_rate`.

3. **`screenpipe-server/src/core.rs`:**
   - Enhanced logging:
     - When receiving a transcription, now logs the device and the transcription content.
     - When inserting an audio chunk, now logs the device and the path of the chunk.

These changes aim to fix potential issues with the Deepgram integration, improve error handling, and enhance logging for better debugging and analysis.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 29 (ec408a0b8e1b9c96a5d84f2bac7307e3ebfb4877)</summary>

The commit `ec408a0b8e1b9c96a5d84f2bac7307e3ebfb4877` merges changes from a pull request (#329) submitted by the user "kerosina." The main intent of the merge is to handle cases where the host system has no default input device by ensuring the application does not panic in such scenarios. The merge was authored by Louis Beaumont and took place on September 14, 2024.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 30 (479604c2d16167c0f2c11055db78f4119b8d1e3a)</summary>

### Summary of Git Commit `479604c2d16167c0f2c11055db78f4119b8d1e3a`
**Author:** Louis Beaumont  
**Date:** Sat Sep 14 19:16:19 2024 -0700  

#### Changes:
1. **Refactoring Videos Merge Endpoint:**
   - The functionality to merge video frames has been consolidated and renamed from merging frames within a single video to merging multiple videos.
   - Functions and endpoint handlers previously handling frame merging (`merge_frames_from_video`, `MergeFramesRequest`, `MergeFramesResponse`) have been replaced with video merging counterparts (`merge_videos`, `MergeVideosRequest`, `MergeVideosResponse`).

2. **Server Code Modifications:**
   - `screenpipe-server/src/server.rs`
     - Refactored the `merge_frames_handler` to use `MergeVideosRequest` and `MergeVideosResponse`.
     - Modified the search and extraction script to search multiple time windows and concatenate the search results into a JSON array for merging videos.
     - Added steps to print and confirm response paths for the merged videos.

3. **Video Utilities Code Modifications:**
   - `screenpipe-server/src/video_utils.rs`
     - Removed code handling frame extraction and merging.
     - Replaced with code to concatenate video files listed in a temporary text file and merge them using FFmpeg.

4. **Logging Changes:**
   - Logging levels for ffmpeg and video processing changed:
     - `info` logs for events changed to `debug`.
     - Improved log messages for frame extraction and video merging actions.

#### Key Outcomes:
- Enhanced the merging functionality to work with multiple videos instead of frames.
- Improved search, payload preparation, and merging process by simplifying the extraction and merging protocol.
- Updated function names, types, and logic to reflect the new scope and operations.
- Improved logging to facilitate easier debugging and traceability of errors.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 31 (7ece166d4d635234de786e7886435d9961809c3f)</summary>

The commit `7ece166d4d635234de786e7886435d9961809c3f` authored by `kerosina` on Sun Sep 15, 2024, updates the `core.rs` file in the `screenpipe-audio` project. 

### Summary of Changes:
- Modified the function `default_input_device` to handle the case where no default input device is detected.
- Previously, the code used `unwrap()` which would panic if there was no default input device.
- The update replaces `unwrap()` with proper error handling using `ok_or(anyhow!("No default input device detected"))?`, which returns an error message if no default input device is found.

This change improves the robustness of the function by preventing unexpected panics and providing a meaningful error message instead.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 32 (4b8fd2416096e1db8f0c79ced59e4c2d62165b1a)</summary>

The commit introduces experimental code to generate video clips from given recordings. Below is a summary of the changes made:

1. **Cargo.toml**:
   - Added the `uuid` crate (version 1.5.0) to the dependencies.

2. **CLI Modifications**:
   - Adjusted the default FPS value for macOS to 0.5.
   - Modified the default video chunk duration from 30 seconds to 60 seconds.

3. **Library Updates**:
   - Added a new module `video_db`.
   - Integrated `video_utils::{merge_frames_from_video, MergeFramesRequest, MergeFramesResponse}` into the server.

4. **Server Handlers**:
   - Added `merge_frames_handler` to handle requests for merging video frames.
   - New endpoint `/experimental/frames/merge` was created for the merging functionality.
   
5. **Video Processing Logic**:
   - Changed the calculation for `frames_per_video` from using `round` to `ceil`.
   
6. **New File**: 
   - Introduced `video_db.rs` which contains methods to interact with the database for retrieving video frame information.

7. **Video Utilities**:
   - Enhanced `video_utils.rs` with new functionality to extract frames from video and merge them back into a new video file.
   - The function `merge_frames_from_video` handles the merging logic including frame extraction, frame merging, and handling edge cases where frames might be missing.

8. **Usage Script**:
   - Added a script snippet at the end of `server.rs` demonstrating how to use the new merging feature by performing a search and then using the `/experimental/frames/merge` endpoint.

Overall, the commit introduces a substantial feature that supports extracting and merging video frames to create clips, which is accessible through a new experimental endpoint.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 33 (e210a37312505b34a58d508fb5b545dc36c82c93)</summary>

The commit e210a37312505b34a58d508fb5b545dc36c82c93, authored by Louis Beaumont, improves audio logging. Key changes include:

1. **README.md**:
   - Removed instructions related to experimental apple native OCR and unstructured.io cloud OCR.
   - Simplified the instructions for disabling audio recording.

2. **screenpipe-audio/src/core.rs**:
   - Corrected the usage of `use crate::AudioInput` to follow convention.
   - Enhanced logging by making debug and info logging messages more consistent and granular, adding device information in several log messages.
   - Updated the way audio data is cast and handled, improving type safety and clarity.

3. **screenpipe-audio/src/stt.rs**:
   - Enhanced logging by including the device name in various log messages related to transcription, language detection, and speech-frame processing.
   - Improved the `transcribe_with_deepgram` function to accept a device parameter for more detailed logging.
   - Ensured the Deepgram API response handling logs more detailed and consistent messages.
   - Incorporated better logging for processes that use the Whisper transcription engine as a fallback.

These changes aim to improve the clarity and detail of audio-related logs, enhancing the ability to debug and monitor audio processing activities within the application.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 34 (e43fe936bb3f5e516ccb00d145615c9a0926e664)</summary>

A new commit (e43fe936bb3f5e516ccb00d145615c9a0926e664) by Louis Beaumont merged a pull request (#326) from the user "joegoldin" into the main branch. The merge combines changes from the commit (29dc3ed) with another commit (c827c65). This merge was performed on September 14, 2024.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 35 (c827c65fdd989a658b5272f182032e26a76e980c)</summary>

The commit `c827c65fdd989a658b5272f182032e26a76e980c` is a merge commit authored by Joe Goldin on September 13, 2024. It combines changes from the 'main' branch of the repository 'mediar-ai' into the current main branch.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 36 (6dc64a3670a1e4eba435e097e5af161cfb19f812)</summary>

This git commit, made by Joe Goldin, updates the file `screenpipe-audio/src/stt.rs`. The change specifically modifies the `transcribe_with_deepgram` function to correct the WAV audio sample rate. The sample rate was previously set to 16000 Hz and has now been changed to 32000 Hz. The commit also mentions that this change was cherry-picked from another commit (0aa78116cb2343f72de288746e778073b0641094).
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 38 (29dc3ed9f45be44aaffe1e236a37c9a3b729ae3c)</summary>

The commit by Louis Beaumont introduces a new feature that enables keyword search for application names and window names. Here are the key changes made:

1. **Version Update**: 
   - The version of the `screenpipe-app` in `Cargo.toml` is incremented from 0.2.37 to 0.2.38.

2. **Database Query Modifications**:
   - The SQL queries in `screenpipe-server/src/db.rs` now use the `LIKE` operator with wildcard characters (`%`) to perform partial matches instead of exact matches.
   - This is applied to both `app_name` and `window_name` fields in several places to facilitate keyword-based searches rather than exact name matches.
   - The specific lines modified involve changing the condition from `AND ocr_text.app_name = ?` to `AND ocr_text.app_name LIKE '%' || ? || '%'`, and similarly for `window_name`.

These changes collectively enhance the search functionality within the application by allowing more flexible search options for app and window names.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 39 (b8aee0907213578b9f46023a9ac3a9e0f6ce6105)</summary>

The commit made by Louis Beaumont on September 13, 2024, introduced a small UI change to the `screenpipe-status.tsx` component in the `screenpipe-app-tauri` project. Specifically, the update added a clarification to the `DevModeSettings` section's description. The new lines inform users that in developer mode, the backend will not automatically start when the app is launched.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 40 (4d3226fdbc5e80915046e93d22365994ecdd0f99)</summary>

### Summary of Git Changes

**Commit:** `4d3226fdbc5e80915046e93d22365994ecdd0f99`  
**Author:** Louis Beaumont \<louis.beaumont@gmail.com\>  
**Date:** Fri Sep 13 18:43:48 2024 -0700  

#### Changes Overview
- Implemented enhancements related to search capabilities with new filters.

#### Details:
1. **Modified `screenpipe-server/src/db.rs`**
    - **Function Enhancements:**
        - Updated various search functions (`search_ocr`, `search_audio`, `count_ocr_results`, `count_audio_results`) to include new optional parameters: `min_length` and `max_length`.
        - These parameters allow filtering of results based on the length of OCR text and audio transcriptions.
    - **SQL Adjustments:**
        - Modified SQL queries to handle the new length filters by adding conditions for `LENGTH` in the `WHERE` clause.
        - Updated parameter binding logic to accommodate the new parameters.

2. **Modified `screenpipe-server/src/server.rs`**
    - **Struct Updates:**
        - `SearchQuery`: Added new fields `min_length` and `max_length` for search constraints on text length.
    - **Search Function Logging and Execution:**
        - Updated the `search` function to log and process the new length constraints.
    - **Example Usage:**
        - Extended example `curl` commands to demonstrate the usage of `min_length` and `max_length` parameters in API calls.

#### Summary:
This commit introduces new filtering capabilities for search functionality by allowing users to specify minimum and maximum length constraints for OCR text and audio transcriptions. The changes enhance the flexibility of search queries and improve the precision of search results.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 41 (e5b452f2b38e567a789aa778d1392c9c0c66c1f2)</summary>

The commit `e5b452f2b38e567a789aa778d1392c9c0c66c1f2` authored by Louis Beaumont introduces a new feature for better logging during development by utilizing an environment variable. Below are the key changes:

1. **Imports**: Added `env` to the list of imports in `screenpipe-server/src/bin/screenpipe-server.rs`.

2. **Logging Configuration**:
    - The existing logging directives were modified to allow dynamic configuration based on an environment variable named `SCREENPIPE_LOG`.
    - Environment variable `SCREENPIPE_LOG` can now be used to set custom log levels for specific modules. The log levels are parsed and added to the existing `env_filter`.
    - Example usage provided in the comment:
        - `SCREENPIPE_LOG=screenpipe_audio=debug ./screenpipe`
        - `SCREENPIPE_LOG=screenpipe_audio=debug,screenpipe_vision=trace ./screenpipe`

These modifications improve the flexibility of setting log levels for different modules without changing the code, enhancing the development experience.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 42 (743b0bbea7d266786f5e3307e9ff308592eb5cbc)</summary>

The commit `743b0bbea7d266786f5e3307e9ff308592eb5cbc` by Louis Beaumont makes several changes aimed at improving consistency and functionality across different components of the project. Here’s a summary of the main changes:

1. **General Conventions**:
   - Lowercase usage: All logging statements and UI text in the project are now consistently written in lowercase.

2. **Documentation**:
   - A guideline was added in the `CONTRIBUTING.md` to always use lowercase for logging and UI messages.

3. **New TypeScript Files**:
   - Added a TypeScript declaration file `screenpipe.d.ts` to define global variables and interfaces for the project.
   - Added a TypeScript configuration file `tsconfig.json` to set compiler options and include new TypeScript files.

4. **Rust Updates**:
   - In `Cargo.toml`, added a dependency on the `dirs` crate version 5.0.0.
   - In `pipes.rs`:
     - Removed an unused asynchronous function `op_is_enabled`.
     - Introduced additional environment variables (`HOME`, `CURRENT_DIR`, `TEMP_DIR`, `PIPE_ID`, and `PIPE_FILE`) to the JavaScript runtime environment.
     - Improved error handling and logging messages for various operations.
     - Reformatted several sections to improve readability.

5. **Server Enhancements**:
   - In `screenpipe-server.rs`:
     - Consolidated `use` statements to reduce redundancy.
     - Enhanced logs and error messages with consistent lowercase usage.
     - Added a new command `PipeCommand::Purge` to the CLI for purging all pipes.
     - Improved formatting and readability of the terminal output.

6. **Pipe Management**:
   - Enhanced error messages in `pipe_manager.rs` to use consistent lowercase.
   - Added a new method `purge_pipes` to delete all pipes.

These changes aim to improve usability, readability, and functionality while ensuring consistent messaging throughout the code and user interfaces.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 43 (9407d62ce64e706f48ad1a228b9068846a63f50e)</summary>

The commit with hash `9407d62ce64e706f48ad1a228b9068846a63f50e` by Louis Beaumont made several changes primarily focused on fixing tests. Below is a summary of those changes:

### `.github/workflows/benchmark.yml`
1. The `Analyze STT benchmarks` job was commented out, indicating it is currently broken and needs to be fixed or updated.

### `screenpipe-audio/tests/core_tests.rs`
1. **Removed Imports:**
   - Removed imports related to `WhisperModel`, `WebRtcVad`, and `stt`.

2. **Deleted Test Function:**
   - `test_speech_to_text` function, which tested speech-to-text functionality, was completely removed, suggesting it was either failing or no longer relevant.

3. **Modified `test_record_and_transcribe`:**
   - Function calls were simplified by removing the `output_path` argument.
   - Adjusted the assertions within the test to check the length of the audio input data rather than the path.

4. **Additional Adjustments:**
   - Adjusted several function calls to align with the updated signatures.

### `screenpipe-core/tests/pipes_test.rs`
1. **Deleted Test Function:**
   - `test_download_pipe_raw_file`, which tested downloading a raw file from a GitHub URL and verifying its content, was completely removed.

### Summary
- STT benchmarks job in the GitHub workflow has been temporarily disabled.
- Tests related to speech-to-text functionality were removed.
- Simplified and corrected some test function arguments and assertions.
- Removed a redundant test from the `screenpipe-core` module.

These changes indicate a refocus on maintaining working tests and disabling or removing those that are currently problematic or redundant.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 44 (836f559ff804f81195a8c4f1debe61129755e658)</summary>

This commit, authored by Louis Beaumont on September 13, 2024, includes the following changes:

1. A binary file located at `screenpipe-app-tauri/bun.lockb` was modified, though specific details of the change are not shown due to the binary nature of the file.

2. A change in the file `screenpipe-app-tauri/components/meeting-history.tsx`:
   - The text color for a live meeting indicator was changed from green (`text-green-500`) to black (`text-black`).

3. The version number in the `Cargo.toml` file for the project `screenpipe-app` was incremented from `0.2.36` to `0.2.37`.

Overall, this commit mainly focuses on updating the text color for a specific part of the app's UI and bumping the project's version.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 45 (f8484b767ea0e0991c6864857718e3436b54d522)</summary>

This Git commit represents a merge operation performed by Louis Beaumont. The merge combines the changes from the branch associated with pull request #323, contributed by user "m13v," into the main branch. The feature merged is referred to as "Meetings feature 3."
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 46 (5dd01699620b438e86963479b87fda2d0cb61013)</summary>

### Summary of Git Changes

**Commit ID:** 5dd01699620b438e86963479b87fda2d0cb61013  
**Author:** Louis Beaumont <louis.beaumont@gmail.com>  
**Date:** Fri Sep 13 14:45:42 2024 -0700  
**Message:** Make pipe store technical preview only for now.

#### Changes in `screenpipe-app-tauri/components/pipe-store.tsx`:
1. **Commented Out Elements:**
    - Commented out the `FeatureRequestLink` component, which was previously displayed when no pipe was selected.
    - Commented out console logging of installed pipes.
    - Commented out the reset button for pipes and related descriptive text.
    - Removed links to GitHub examples and feature request link related to the selected pipe.
    - Added a centralized message indicating the current need to enable pipes via `screenpipe pipe` commands or `/pipes` API and mentioned a future update for non-technical access.
    - Commented out several parts related to listing, adding, and displaying individual pipes, including the main content rendering logic.

#### Changes in `screenpipe-core/src/pipes.rs`:
1. **Sanitization Function:**
    - Added a new function `sanitize_pipe_name` using regex to clean up pipe names.
2. **Updating Pipe Handling Logic:**
    - Incorporated the `sanitize_pipe_name` function to ensure valid pipe names when downloading GitHub folders or single files and handling local directories.
    - Simplified directory creation and file handling logic using the sanitized names.

#### Changes in `screenpipe-server/src/bin/screenpipe-server.rs`:
1. **Console Output Changes:**
    - Removed printing of monitor IDs.
    - Added a new section to display monitor information, indicating whether vision is disabled or if no monitors are available. Lists up to 5 monitors with IDs.
2. **Audio Devices Section:**
    - Refactored to use a common `MAX_ITEMS_TO_DISPLAY` constant.
    - Adjusted logic to potentially list more devices with an indication of the total number available.
3. **Pipes Section:**
    - Added a new section to display available pipes, indicating their IDs and enabled status. Lists up to 5 pipes and handles displaying additional pipes count if exceeding the limit.

### Summary:
This commit marks the pipe store feature as a technical preview by commenting out various UI elements related to pipes in the user interface. Additionally, it adds sanitization for pipe names in the core logic to ensure they are valid and updates the server to better display monitors, audio devices, and pipes, with enhanced formatting and information.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 47 (0caa32169fa69cb0e1da4f6f745a6d199a3e76a6)</summary>

This commit, authored by Louis Beaumont, introduces some small UI fixes. Here are the main changes:

1. **`page.tsx` Modifications**:
    - Added a heading (`<h1>where pixels become magic</h1>`) above the `SearchChat` component when certain settings are enabled (either `useOllama` or `openaiApiKey`).

2. **`search-chat.tsx` Adjustments**:
    - Removed the heading within the `SearchChat` component that was previously displayed (`<h1>where pixels become magic</h1>`).
    - Updated the placeholder text in an input field from "Ask a question about the results..." to "ask a question about the results...".
    - Changed message text in a tooltip to be all lowercase: "Content exceeds 30k tokens. Refine your search for better results." is now "content exceeds 30k tokens. refine your search for better results."

These updates aim to optimize the UI by removing redundant elements and ensuring consistent styling and text casing.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 48 (ab409281f82fb014f46d06c4530c822c0fc231a1)</summary>

### Summary of Git Changes

**Commit:** ab409281f82fb014f46d06c4530c822c0fc231a1  
**Author:** Louis Beaumont <louis.beaumont@gmail.com>  
**Date:** Fri Sep 13 14:20:32 2024 -0700  
**Message:** try to fix windows video embedding

### Changes in `screenpipe-app-tauri/components/video.tsx`
1. **Removed Imports:**
   - `Button` from `./ui/button`
   - `Link` from `lucide-react`

2. **Sanitize File Path Function (`sanitizeFilePath`):**
   - Improved logic to replace backslashes with forward slashes.

3. **New Function: `getMimeType`**
   - Determines the MIME type based on file extension. Supports file types like mp4, webm, ogg, mp3, and wav. Falls back to default MIME types based on the `isAudio` flag.

4. **`loadMedia` Function Changes:**
   - Enhanced the creation of `Blob` by using MIME type obtained from the new `getMimeType` function.

### Changes in `screenpipe-server/src/db.rs`
1. **Cleanup Debug Statements:**
   - Removed `println!` statements used for debugging in the `DatabaseManager` for audio search.
   
These changes aim to improve the handling of video embeddings on Windows by sanitizing file paths correctly and determining the MIME type more accurately. Additionally, the database manager's audio search functionality has been cleaned up by removing unnecessary debug print statements.
</details>

------------------------------------------------------------------------

# Overall Summary of Changes

### Summary of Recent Git Changes

**Commit Overview:**
- **Primary Author:** Louis Beaumont
- **Primary Date:** September 16, 2024

**Highlights:**
1. **All-Inclusive Merge Commit:**
   - A significant merge consolidates various updates and contributions into the main branch.

2. **Homebrew Formula Updates:**
   - The pull request (#337) updated the Homebrew formula for the `aarch64-apple-darwin` architecture.
   - Changes to `Formula/screenpipe.rb`:
     - ARM64 SHA256: `fbfb6f20dc2a0ea9cab8dddc37261f6803fa55b013c4378cfe6c3975edddb3bb` -> `d41406d867674b21103ec089bf34d77e0476de6ac22e22a6da72adb42dc5fec8`.
     - x86_64 SHA256: `c1071dd2ba6f7e26e318d75ae2c97ef8f2959d2640a836d09242e5abd88aac9c` -> `ef2ca50886b92c2ff995a9f3d0f7e6391d832940bd68f130c85fedd2680712a1`.

3. **Version Updates & Dependency Bumps:**
   - `screenpipe` library's Homebrew formula updated from 0.1.84 to 0.1.85, along with new SHA256 checksums for ARM64 and x86_64.
   - The version number in `Cargo.toml` files was incremented to ensure compatibility and improved functionality.
   - Updates aligned with newer Tauri-related dependencies in `screenpipe-app-tauri`.

4. **UI & Frontend Enhancements:**
   - Introduced new controls such as sliders, tooltips, and various user experience improvements in `search-chat.tsx` and `settings.tsx`.
   - Improved error messages and interactive elements for better user interactions in search functionalities.
   - Updated visual components like the `ContextUsageIndicator` for intuitive context usage feedback.

5. **Backend Improvements:**
   - Enhanced `db.rs` with robust SQL dynamic query construction to handle parameters efficiently.
   - Refactored search functionalities to incorporate text length filters (`min_length`, `max_length`) for more fine-grained search results.
   - Added comprehensive logging and error handling mechanisms across server components (`screenpipe-server.rs`).

6. **Merge Pull Requests and Conflict Resolutions:**
   - Multiple pull requests (#325, #326, #329, #333, #335) merged, resolving conflicts and incorporating contributions from various authors.
   - Addressed build issues, especially for Linux application builds affected by C binding conflicts.

7. **New Experimental Features:**
   - Introduced experimental video frame merging through new endpoints and associated handlers.
   - Added backend processing capabilities such as video frame extraction and merging using FFmpeg.

8. **Adjustments for Technical Preview:**
   - Marked the pipe store feature as a technical preview by modifying the `pipe-store.tsx` file to comment out certain UI elements.
   - Updated pipe name sanitization and internal console output formatting to improve clarity and debugging.

9. **Build and CI/CD Enhancements:**
   - Enabled GitHub Actions workflows to automatically trigger on new release tags.
   - Commented out the broken "Analyze STT benchmarks" job in the benchmarks workflow.

**Summary:**
These recent commits represent a broad set of changes across the Screenpipe project, focusing on maintaining compatibility, enhancing functionality, refining user interfaces, and ensuring robust backend operations. The incorporation of contributions via merge requests, updates to the Homebrew formula, frontend and backend enhancements, and various refactorings collectively aim to improve the project's performance, usability, and development workflow.
