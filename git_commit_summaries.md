# Git Commit Summaries

-----------------------------------------------------------------------
-----------------------------------------------------------------------
 
PRESS CMD+SHIFT+V TO VIEW IN MARKDOWN
 
_______________________________________________________________________
-----------------------------------------------------------------------
Total number of commits: 11

<details>
<summary>summary for commit 1 (4dd5a053186e3bad70178c08ce58f7df8b24f82d)</summary>

The commit made by Louis Beaumont on November 11, 2024, updates the `README.md` file. The changes include modifying section headers for better clarity:

1. The "## usage" section header has been changed to "## create plugins". This suggests a refocus from general usage instructions to specifically discussing how to create plugins within the screenpipe system.

2. The "## examples" section header has been changed to "## other examples". This might indicate that additional context or different types of examples will now be provided, broadening the scope from initial examples.

These changes likely aim to improve the organization and readability of the documentation.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 2 (b943112d6990e7c46226b11c201f5664539015e8)</summary>

The commit made by Louis Beaumont includes two changes:

1. **Spacing adjustment in a React component:** In the `search-chat.tsx` file, within a `div` element, the class attribute was modified to increase the horizontal spacing between child items by changing from `space-x-1` to `space-x-2`.

2. **Version update in Cargo.lock:** In the `Cargo.lock` file, the version of the `screenpipe-app` package was updated from `0.10.1` to `0.10.7`.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 3 (afc2e083f7e93a7eadf28fb4b4958603e135aa47)</summary>

The commit made by Louis Beaumont on November 11, 2024, involves several changes to a project related to macOS support:

1. **Script Changes**: In the `pre_build.js` script, the setup process for FFMPEG on macOS has been commented out. This includes downloading, extracting, moving, and renaming FFMPEG binaries for different Mac architectures (x86_64 and arm64). The entire block of code related to FFMPEG setup is now commented out.

2. **Version Update**: The project's version number in the `Cargo.toml` file has been incremented from 0.10.6 to 0.10.7, indicating a new release or update.

3. **Configuration Changes**: In the `tauri.macos.conf.json` file:
   - Several FFMPEG libraries have been removed from the `"frameworks"` section, which specified the dynamic libraries to be bundled with the application.
   - The `ffmpeg` entry has been removed from the `"externalBin"` section, which lists external binaries to be included with the app package.

Overall, this commit seems focused on modifying or disabling the integration of FFMPEG with the macOS version of the project, possibly for testing, troubleshooting, or simplifying the build configuration.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 4 (67eabf637e330ffaaa9e046d0003e04ffbada5f3)</summary>

The git changes in this commit (67eabf637e330ffaaa9e046d0003e04ffbada5f3) involve two main updates:

1. **Package Version Update**: 
   - The version of the `screenpipe-app` package in the `Cargo.toml` file for the Tauri application was incremented from `0.10.5` to `0.10.6`.

2. **Code Modification**:
   - In the `screenpipe-server/src/bin/screenpipe-server.rs` file, the code handling a potential error during a PATH check was altered.
   - A previous error return statement (`return Err(e.into());`) was commented out to ensure the application does not crash when it encounters a path-related error. This change suggests a decision to handle the error more gracefully without terminating the program.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 5 (e70445b15f3973563dc3997489553a6ac6a3e9f3)</summary>

The commit with hash `e70445b15f3973563dc3997489553a6ac6a3e9f3` by Louis Beaumont, dated November 11, 2024, introduced a new feature related to an "ffmpeg sidecar" in the project. The following changes were made:

1. **Version Update**: The version number of the `screenpipe-app` package in the `Cargo.toml` file was incremented from "0.10.4" to "0.10.5".

2. **Code Changes**:
   - In the module `video.rs` within the `screenpipe-server` directory:
     - The `std::env` import statement was removed as it was unused.
     - A conditional compilation block was added to handle the ffmpeg `-vcodec` argument differently based on the target operating system:
       - For macOS (`#[cfg(target_os = "macos")]`), the arguments `"-vcodec", "libx264", "-preset", "ultrafast", "-crf", "23"` are used.
       - For operating systems other than macOS (`#[cfg(not(target_os = "macos"))]`), the arguments `"-vcodec", "libx265", "-preset", "ultrafast", "-crf", "23"` are used.
     - The codec-related code changes are accompanied by a comment hinting at a specific issue on macOS (linked to a GitHub pull request), indicating a potential workaround or fix for a known problem.

These changes suggest a targeted optimization or compatibility adjustment concerning video encoding on different platforms.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 6 (93a4f6b7c259b34510bd17ec0a5d3517782ff467)</summary>

The commit by Neptune introduces several changes related to adding "ffmpeg-sidecar" support:

1. **Script Enhancements (`run_screenpipe.sh`, `test_audio_capture.sh`, `test_ocr.sh`):**
   - Resource monitoring was added to these scripts, which now include loops that check for CPU and memory usage every 10 seconds for their respective durations, replacing fixed sleep intervals.

2. **GitHub Workflow (`linux-integration-test.yml`):**
   - A step was added to check the final storage usage of a certain directory (`~/.screenpipe/data`) after running tests.

3. **Dependency Updates (`screenpipe-audio/Cargo.toml`, `screenpipe-core/Cargo.toml`):**
   - Added `tar` version 0.4.42 to `screenpipe-audio`.
   - Integrated `ffmpeg-sidecar` from a GitHub repository into `screenpipe-core`.

4. **FFmpeg Handling (`ffmpeg.rs`):**
   - Added logic to ensure FFmpeg is installed using `ffmpeg-sidecar`. This includes downloading, checking the version, and handling the installation if it's missing.
   - Modified the function to handle paths and installation checks.

5. **Server Enhancements (`screenpipe-server.rs`):**
   - Added a check to verify if FFmpeg is installed and log the path or log an error if the installation fails. This acts as part of the setup process.

6. **Video Processing Changes (`video.rs`):**
   - Altered FFmpeg video encoding settings to always use `libx265` with specified flags (`-preset ultrafast`, `-crf 23`) for video processing, simplifying previous platform-specific logic.

Overall, these changes add support for the `ffmpeg-sidecar`, enhance resource monitoring in scripts, and make the video encoding more consistent across platforms.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 7 (804e948826b3179b8312646f9182669eefbcf137)</summary>

The commit by Louis Beaumont updates the `README.md` file. The changes include:

1. An additional line is added under the subtitle to include more information: "24/7 screen & voice recording for the age of super intelligence. get your data ready or be left behind."

2. The section titled "open source 24/7 screen & voice recording for the age of superintelligence" is replaced with "how it works?"

These changes enhance the description of the project's functionalities and rephrase a section title for clarity or emphasis.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 8 (c80facc05a909e7e660eff13687a75efb93f38f8)</summary>

The commit titled "fix: remove misleading error message" includes the following changes:

1. **`page.tsx` in `screenpipe-app-tauri/app/timeline/`**:
   - Removed the line `retryCount.current = 0;` within the `onopen` event handler of `eventSource`. This change likely addresses an unnecessary or misleading reset of the retry count, potentially related to error handling.

2. **`Cargo.toml` in `screenpipe-app-tauri/src-tauri/`**:
   - Updated the package version from `0.10.3` to `0.10.4`. This version bump indicates a minor change or a bug fix.

3. **`server.rs` in `screenpipe-server/src/`**:
   - Commented out a log error message `error!("frame cache not initialized");`. The error message is now part of the data response sent when the frame cache is not initialized, which might aim to reduce noise in the logs while still conveying the error state to relevant parts of the application.

These changes collectively aim to clean up error handling by reducing potentially misleading log messages and making minor adjustments to improve maintainability.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 9 (0efbcd83b190f6a7b9c69b4fb6535ac090db5cc5)</summary>

The commit with hash `0efbcd83b190f6a7b9c69b4fb6535ac090db5cc5`, authored by Louis Beaumont, involved removing a large number of entries from the `.gitignore` file. Specifically, it removed paths related to work-in-progress (WIP) versions, certain node_modules subdirectories, and specific files across a variety of libraries and packages. The overarching change aimed to "remove trash" or clean up unnecessary or unwanted entries from `.gitignore`. The only entry that remained is ".env", which indicates that environment variable files are still ignored.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 10 (6d797014aa6a4feeab73f53e86f8525868c4eab7)</summary>

The commit with hash `6d797014aa6a4feeab73f53e86f8525868c4eab7` involves a few changes in a JavaScript build script and a Rust project file. Here are the details:

1. **JavaScript Build Script (`pre_build.js`)**:
   - The script changes the FFMPEG version used for macOS from `ffmpeg-7.1` to `ffmpeg-7.0-macOS-default`.
   - The download URL for FFMPEG is updated accordingly.
   - The script now uses `tar.xz` instead of `7z` for extracting FFMPEG files, which involves replacing the `7z e` command with `tar xf` and references to `.7z` files with `.tar.xz`.

2. **Rust Project File (`Cargo.toml`)**:
   - The version of the `screenpipe-app` package is incremented from `0.10.2` to `0.10.3`.

The overall purpose of the commit is to revert back to an older version of FFMPEG for macOS and update the version of the application.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 11 (e664119016e1f8a5faaecadf47556e5e668aaac0)</summary>

The commit identified by `e664119016e1f8a5faaecadf47556e5e668aaac0`, authored by Louis Beaumont on November 11, 2024, includes a documentation fix in the `getting-started.mdx` file. The change adds a missing `git clone https://github.com/mediar-ai/screenpipe` command to the instructions, ensuring that users know how to clone the necessary repository before accessing the example file.
</details>

------------------------------------------------------------------------

# Overall Summary of Changes

The git changes described span multiple commits, primarily authored by Louis Beaumont and one by Neptune, focusing on documentation, code updates, build scripts, and project configurations. Here's a consolidated summary:

1. **Documentation Updates:**
   - **README.md:** Improved section headers for better clarity, adjusted descriptions, and emphasized project functionalities.
   - **Guide Correction:** Added missing steps to the `getting-started.mdx` file for cloning the repository.

2. **Code Modifications:**
   - **React Component (search-chat.tsx):** Enhanced spacing with a minor CSS change in class attributes.
   - **Error Handling (screenpipe-server):** Modified error handling to prevent crashes and cleaned up misleading error messages.
   - **JavaScript Build Script (pre_build.js):** Adjusted handling of FFMPEG versions and extraction methods for macOS.

3. **Feature Enhancements:**
   - Introduced "ffmpeg-sidecar" support for better resource handling and setup across scripts and server components.
   - Standardized video encoding settings in `video.rs` for more consistent performance across platforms.

4. **Build and Configuration Changes:**
   - Updated versions in `Cargo.toml` and `Cargo.lock`, indicating minor and patch-level updates.
   - Commented out the FFMPEG setup in the build process for macOS, potentially to simplify or troubleshoot configurations.

5. **Cleanups:**
   - `gitignore`: Removed unnecessary entries while retaining essential ignores like `.env`.

Overall, the commits collectively improve project documentation, refine build processes, enhance feature support, and maintain code quality and error handling.
