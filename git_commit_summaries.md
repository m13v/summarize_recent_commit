# Git Commit Summaries

-----------------------------------------------------------------------
-----------------------------------------------------------------------
 
PRESS CMD+SHIFT+V TO VIEW IN MARKDOWN
 
_______________________________________________________________________
-----------------------------------------------------------------------
Total number of commits: 68

<details>
<summary>summary for commit 1 (eeaac405ad8d4d03d8fdb7419dad750586e0ffcc)</summary>

The commit with hash `eeaac405ad8d4d03d8fdb7419dad750586e0ffcc` by Louis Beaumont provides several changes to the "screenpipe-app" project:

1. **Components Update**:
   - In `components/meeting-history.tsx`, the behavior and logging for `isOpen` in `useEffect` was modified. The "meeting_history_closed" event capturing was removed.
   - A new `useEffect` was added to log when the dialog state changes, with `console.log`.
   - The handling of button clicks was updated to prevent event propagation by using `e.stopPropagation()`.
   - Code formatting improvements were made for better readability, such as reformatting a multiline sort function.

2. **Version Increment**:
   - The package version was updated from `0.14.4` to `0.14.6` in `Cargo.lock` and from `0.14.6` to `0.14.7` in `Cargo.toml`.

3. **Binary Differences**:
   - The binary files `src-tauri/ui_monitor-aarch64-apple-darwin`, `src-tauri/ui_monitor-x86_64-apple-darwin`, and `lib/libscreenpipe_arm64.dylib` were modified, though specific changes are not visible in the diff output.

These changes primarily improve logging and event handling on the meeting history page and include some version bumps and binary updates.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 2 (fc33fa99155c243445caa533f98d2f9a20019304)</summary>

The commit made the following changes to the project:

1. **Update Pre-Build Script for macOS:**
   - Added a setup for screenpipe binaries targeting macOS, specifically for `arm64` and `x86_64` architectures.
   - The script now checks for the `SKIP_SCREENPIPE_SETUP` environment variable to potentially skip setup.
   - Retrieves the most recent binary path and copies it for setup.
   - Uses `install_name_tool` to adjust dynamic library paths (dylib) for both architectures, with separate handling for development mode.
   - Added logging to track the setup process and error handling for any issues during dylib path update.

2. **Increment Version:**
   - Updated the version of the package from `0.14.5` to `0.14.6` in the `Cargo.toml` file, indicating a minor change likely related to the build script improvements.

3. **Modify Build.rs:**
   - Commented out the macOS-specific build instructions for copying executables and hardcoding dylibs. This section was `#[cfg(target_os = "macos")]` and is now enclosed in a comment block.
   - The script still proceeds with the `tauri_build::build()` invocation, implying the switch to a possibly different mechanism for handling the macOS build process.

Overall, the commit primarily focuses on ensuring proper setup of binaries and linking for macOS, along with a version bump for the application.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 3 (48cc11a8df6a07754dde8c41e0e76b29cf8c2437)</summary>

The Git commit `48cc11a8df6a07754dde8c41e0e76b29cf8c2437` by Louis Beaumont makes the following changes:

1. A fix related to the build, possibly addressing build issues or making necessary updates.
2. In the `Cargo.lock` file within the `screenpipe-app-tauri/src-tauri/` directory:
   - The `lazy_static` crate has been added to the dependencies list.
3. In the `Cargo.toml` file at the same location:
   - The version number of the package `screenpipe-app` has been incremented from `0.14.4` to `0.14.5`. 

These changes suggest a minor version update, likely due to the addition of a dependency and potentially resolving issues related to the project build process.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 4 (5a0510d88939b31c05c60f47f562828016865374)</summary>

The commit made by Neptune focuses on enhancing the build process for a Tauri application targeting macOS. Specifically:

1. The code base within `screenpipe-app-tauri/src-tauri/build.rs` was modified to streamline and ensure handling of specific target triples for different architectures (`x86_64-apple-darwin` and `aarch64-apple-darwin`).

2. Introduced more environment-aware targeting by checking the `TARGET` environment variable, making the build script more robust for the `aarch64-apple-darwin` architecture.

3. Added `env` to the list of imports to facilitate checking environment variables.

4. The code improves clarity by replacing unconditional logic with checks for specific architectures using environment variables, enhancing the script's flexibility and maintainability.

5. Some minor white space changes were made, improving code readability.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 5 (4d29fe46fcfb8aae61d35415826066fe189cc544)</summary>

The commit titled "fix: infinite loop of powershell & use icon cache with localforage and limit powershell process (#802)" introduces multiple changes to address performance and stability issues in the `screenpipe-app-tauri` project. Here's a summary of the changes:

1. **Caching Icons with `localforage`:** 
   - The `timeline-dock-section.tsx` component now imports `localforage` to utilize caching.
   - Icons are cached in the browser's local storage using `localforage`. If an icon is already cached, it's retrieved directly instead of invoking potentially expensive operations.
   - This caching mechanism helps avoid repeated fetching of the same icons, reducing the strain on resources.

2. **PowerShell Invocation Limit:**
   - A count is maintained (`iconInvocationCount`) for the number of times an icon is invoked to prevent excessive PowerShell processes from being executed, capping it at 100 times per appName.

3. **Semaphore for Limiting Concurrent Processes:**
   - A semaphore is introduced in `icons.rs` to limit the number of concurrent PowerShell processes to 5, using the `tokio::sync::Semaphore` and `lazy_static` for ensuring single-time initialization.
   - This helps prevent system overloads due to too many concurrent PowerShell commands being run.

4. **Improved PowerShell Command Execution:**
   - PowerShell command executions for fetching executable paths have been modified for better performance and minimized disruption. The commands now use `-NoProfile` and `-WindowStyle hidden` options along with `creation_flags` set to `CREATE_NO_WINDOW`, reducing the visibility and resource usage of the PowerShell windows.

5. **Dependencies:**
   - The `Cargo.toml` file is updated to include `lazy_static` as a dependency to support the semaphore feature.
   - The `windows-icons` crate is added via a Git dependency, presumably for improved handling of icons on Windows.

These changes collectively aim to enhance the performance, efficiency, and stability of the application by optimizing icon processing and limiting resource-intensive operations.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 6 (59326f5c8c8550a91a31017484bd804a09b73d24)</summary>

The git commit includes the following changes and features:

1. **New Features:**
   - Real-time updates for pipes, with the ability to start and stop them.
   - Full-screen pipes functionality.

2. **TypeScript Config Changes:**
   - In the Next.js configuration file (`next.config.mjs`), a new experimental option was added to control dark mode (`darkMode: false`).

3. **Pipe Execution Updates:**
   - The `run_pipe` function in `pipes.rs` has been updated to return a `tokio::process::Child` instead of an `Option<u16>`, indicating that the process handle is being returned rather than just the port number.
   - Improved logging for dependency installation for Next.js pipes with more consistent casing.
   - Changed the logic to not wait for the child process to finish synchronously, allowing for concurrent execution without blocking.

4. **Server Enhancements:**
   - Removal of unnecessary future polling structures (`FuturesUnordered`) for pipe execution in `screenpipe-server.rs`.
   - Enhanced the `PipeManager` to manage pipe processes asynchronously, using a `HashMap` to track running pipes with `tokio::sync::RwLock` for concurrency control.
   - New method `stop_pipe` for stopping running pipes and refactoring of `start_pipe_task` for task-based execution of pipe processes.
   - Removed `PipeControl` enum and related logic in favor of improved task handling for starting and stopping pipes.
   - Logging adjustments, particularly from `debug!` to `info!`, for more informative output during pipe operations.

5. **Code Cleanup:**
   - Removed redundant future handling code for pipe control and execution.
   - Improved error and info logging in several places to provide clearer output messages to the user.
   - Simplified `PipeManager`'s logic by using more direct task management functions.

Overall, these changes improve the flexibility and control over pipe management, with enhancements to parallel processing and task management within the application.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 7 (de0ab2c3172e9bd3615e78c385e44757bc64c620)</summary>

The commit made by Louis Beaumont on November 28, 2024, updates the `README.md` file. The modification is a small change in the descriptive text for "screenpipe." The change rephrases the features description; it now states "agents powered by 24/7 screen, voice, keyboard, mouse, camera recording. sandboxed. keyboard and mouse control in js" instead of the previous "24/7 screen & voice recording context. sandboxed agents. keyboard and mouse control in js." The updated wording emphasizes the agent's capabilities more explicitly by including "camera recording" in the list and slightly altering the structure for clarity.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 8 (e48b616a2d9656d8f24d5c44f7622bc213066324)</summary>

The commit with hash `e48b616a2d9656d8f24d5c44f7622bc213066324` authored by Louis Beaumont on November 28, 2024, updates the `README.md` file. The change corrects a grammatical error in a section discussing the importance of recording. Specifically, it changes "every seconds you are not recording" to "every second you are not recording."
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 9 (caaa67e157105cf2f5e1aadab2c648b8cdf0fd8b)</summary>

The commit with hash `caaa67e157105cf2f5e1aadab2c648b8cdf0fd8b` made changes to the `README.md` file. Specifically, it modified the "why?" section of the document. The previous text, which mentioned the year 2025 and posed a question about data readiness, was replaced with a statement emphasizing the importance of constant recording to prevent missing context for Artificial General Intelligence (AGI).
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 10 (9daf6fd462f99e6873b804493e2b9986c82b945f)</summary>

The commit made by Louis Beaumont on November 28, 2024, updates the `README.md` file. The changes include:

1. Modification of the project description:
   - Previous description: "rewind.ai x cursor.com = your AI assistant that has all the context. 24/7 screen & voice recording for the age of super intelligence. get your data ready or be left behind."
   - Updated description: "nextjs for desktop agents. 24/7 screen & voice recording context. sandboxed agents. keyboard and mouse control in js."

2. Removal of an embedded image and its associated link, which was displayed as a demo. The image and its link are no longer present in the `README.md`.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 11 (1ed19251f5f5b7a87f51ce7711fdac775a755410)</summary>

The commit with hash `1ed1925` authored by Louis Beaumont on November 28, 2024, removes a binary file named `bun.lockb` from the `examples/typescript/pipe-simple-nextjs` directory. The commit is marked as a fix, indicating that the removal of this file addresses a specific issue.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 12 (f58037afe8114b091c58198c6ba45d413fca2d57)</summary>

The commit made a series of changes across various files, aimed at optimizing performance, adding functionality, and cleaning up the codebase. Here's a concise summary:

1. **Removal of Files**: 
   - `Formula/screenpipe.rb` and `cn` files have been deleted, suggesting a cleanup or refactoring to remove unused files from the repository.

2. **Code Optimization and Cleanup**:
   - Removed unnecessary logging from `pipe.ts` in the Obsidian time logs example to streamline output.
   - Updated `components/keyword-cloud.tsx` to directly process content streaming with SQL-based logic replacing manual data processing, enhancing efficiency.
   - Refactored `log-file-button.tsx` and several other components to remove unused code and improve readability.

3. **Feature Additions and Improvements**:
   - Introduced a new `bun.lockb` file in the `pipe-simple-nextjs` example, likely to manage package dependencies.
   - Added functionality for handling pipes (download, enable, delete) within the server and client application, adding more control over pipe management.
   - Enhanced `pipe-store.tsx` with additional features such as showing a "pipe UI" and a button to delete pipes.

4. **UX Enhancements**:
   - Updated various UI components such as `header.tsx` and `meeting-history.tsx` to improve user interactions, like adding tooltips, buttons, and icons.
   - The `settings.tsx` component now includes a badge displaying cloud credits, providing more extensive UI feedback.

5. **Server Enhancements**:
   - Improved responses for various API endpoints to include structured JSON outputs indicating success or error messages, e.g., for listing, running, stopping, and deleting pipes.
   - Added an API endpoint for deleting pipes, along with logic for confirming deletions interactively.

Overall, the commit focuses on enhancing efficiency, expanding the feature set, improving user interaction, and maintaining clean and organized code.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 13 (11b0a5b53f4e5e878dd6f2c87ca5b25e43744595)</summary>

The commit with ID `11b0a5b53f4e5e878dd6f2c87ca5b25e43744595` addresses fallback handling for Intel Macs by introducing changes to the screenpipe project. The specific changes are as follows:

1. **Version Update**: 
   - The version of the `screenpipe-app` was incremented from `0.14.3` to `0.14.4` in both `Cargo.lock` and `Cargo.toml` files.

2. **Build Script Modification**:
   - In the `build.rs` file, the handling of the build target for x86_64 architecture on macOS has been improved.
   - The script now attempts to copy the `screenpipe` binary from a primary path (`../../target/x86_64-apple-darwin/release/screenpipe`) and falls back to an alternative path (`../../target/release/screenpipe`) if the primary path does not exist.
   - An error message is provided in case the copy operation fails.

These changes ensure that the build process is more robust by providing a fallback mechanism, which is particularly useful if the primary binary path is unavailable.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 14 (d574f8e8d1f7df0698a24f4d5de3a06cdd2de02b)</summary>

The commit made by Louis Beaumont includes the following changes:

1. **GitHub Actions Workflow Update**:
   - In the `release-cli.yml` file within the GitHub workflows directory, the target build for `aarch64-unknown-linux-gnu` was removed. Now, the workflow only targets `x86_64-unknown-linux-gnu` for release builds.

2. **Cargo.toml Update**:
   - The version of the package in the `Cargo.toml` file was incremented from `0.2.6` to `0.2.7`, likely indicating a patch update reflecting minor changes or fixes. 

Overall, this commit seems to focus on simplifying or correcting the build targets for the release process and includes a version bump to indicate a new release.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 15 (eae21c4272ad6cde213bca497b7f8495eac60179)</summary>

The commit with hash `eae21c4272ad6cde213bca497b7f8495eac60179` by Louis Beaumont addresses two main changes to fix issues related to Windows:

1. **GitHub Workflow Update**:
   - The workflow file `.github/workflows/release-cli.yml` was modified to update the dependencies for the `update-homebrew` job. Previously, it depended only on `build-macos`, but now it also depends on `build-linux`.

2. **Source Code Modification**:
   - In the `screenpipe-audio/build.rs` file, a conditional import statement was modified. The `process::Command` import, which was originally included when targeting Windows, has been moved to the unconditional import block. As a result, `Command` is imported regardless of the operating system, which could imply refactoring or fixing a build-related issue on Windows platforms.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 16 (597cb0d266ccac71cfde3c9699b4acf886a1fa85)</summary>

The commit improves the search mechanism for the "bun" executable within the `screenpipe-core/src/pipes.rs` file by introducing several updates:

1. **Introduction of `Lazy` Initialization:**
   - The commit uses `once_cell::sync::Lazy` to initialize a single instance of the "bun" path, caching the result of the search for efficiency.

2. **Modifications in Command Execution:**
   - Replaces hardcoded `"bun"` command strings with a dynamically located `bun_path`, ensuring that commands are executed with the correct bun executable found by the new search logic.

3. **Enhanced `find_bun` Functionality:**
   - Refactors the `find_bun` function to `find_bun_path_internal`, now wrapped lazily in `BUN_PATH`.
   - The search for the executable is improved by checking:
     - The system's `PATH`.
     - The current working directory.
     - The directory of the current executable.
     - Platform-specific directories (e.g., macOS resources).

4. **Error Handling:**
   - Introduces error handling with `anyhow` to provide informative messages when the "bun" executable cannot be found.

These enhancements optimize how the `bun` executable is located and used, reducing potential runtime errors related to command invocation paths.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 17 (4581b73a662e61b0c7308b3c030c26f8f8fc75d3)</summary>

The commit `4581b73a662e61b0c7308b3c030c26f8f8fc75d3` made by Louis Beaumont on November 28, 2024, updates the `README.md` file. The change involves the addition of an image link: `![image](https://github.com/user-attachments/assets/da5b8583-550f-4a1f-b211-058e7869bc91)`, inserted after an existing image diagram in the markdown file.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 18 (2a8b8993fb10f736871ae74dac6d99bf52f423fb)</summary>

The commit `2a8b8993fb10f736871ae74dac6d99bf52f423fb` by Louis Beaumont updates the `README.md` file. The changes simplify the "get started" section, reducing the detailed options for installing `screenpipe` to just two primary methods:

1. A link to get the desktop app.
2. A link to obtain the CLI or instructions on building from source.

This revision removes details about different installation methods, including the paid desktop app, free version for self-build, the option to obtain a license by contributing or sharing, and using `screenpipe` as a Rust or WASM library or for business use.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 19 (0ed79eb898e9c4fb94bda7120a1de263d35b4c3d)</summary>

The commit with hash `0ed79eb898e9c4fb94bda7120a1de263d35b4c3d` by Louis Beaumont updates various parts of a project for a release. Here are the primary changes made:

1. **GitHub Actions Workflow (`release-cli.yml`):**
   - A command to `brew update` was added to the workflow.
   - The `brew bump-formula-pr` command was modified to include the `--force` flag, and the command now uses `|| true` to ensure that any errors during execution do not fail the script.

2. **Cargo Configuration (`Cargo.toml` and `Cargo.lock`):**
   - The version for the `screenpipe` package was updated from `0.2.4` to `0.2.6` in the `Cargo.toml` file.
   - The version of the `screenpipe-app` was updated from `0.14.2` to `0.14.3` in both the `Cargo.lock` and `Cargo.toml` files within the `screenpipe-app-tauri/src-tauri` directory.

These modifications appear to prepare the project for a new release, likely addressing recent updates or bug fixes.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 20 (959f01af68fc581413ff39d870cffbba97d5fd83)</summary>

The commit by Louis Beaumont introduces a fix in the `screenpipe-status.tsx` component of the Tauri screenpipe app, specifically targeting the behavior of the recorder log on Windows:

- Previously, the recorder log section was implemented using a `Collapsible` component that was always rendered, regardless of the operating system.
- The change now ensures that the `Collapsible` component for the recorder logs is only visible on macOS (using the `isMac` condition).
- On non-macOS systems (such as Windows), the recorder log section is replaced with a simple button (`LogFileButton`) without the collapsible functionality.
- This adjustment effectively hides the collapsible recorder log section on Windows systems, addressing the issue specified by the commit message.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 21 (ce54a73bab9db97cc2b6ffae8d44ccece1ed08ea)</summary>

The recent git commit, authored by Louis Beaumont, involves several key changes addressing issue #751:

1. **Timestamp Management for Audio Capture:**
   - The `core.rs` file in `screenpipe-audio` now includes a new static variable `LAST_AUDIO_CAPTURE` of type `AtomicU64`, using `lazy_static!`. It records the timestamp of the last audio chunk captured by setting this value whenever audio data is successfully received.

2. **Integration in `lib.rs`:**
   - The `LAST_AUDIO_CAPTURE` static variable is re-exported in the `screenpipe-audio/src/lib.rs` file, making it accessible to other parts of the codebase that use this module.

3. **Health Check Update in `screenpipe-server`:**
   - In `server.rs`, the health check method has been updated to use the `LAST_AUDIO_CAPTURE` to determine if audio is currently active. This replaces the previous method of checking the last audio timestamp from the database.
   - The audio activity is assessed by checking if the last capture was within the last 5 seconds, simplifying the previous logic that considered timestamps and durations.

4. **Removed Obsolete Logic:**
   - Obsolete checks and timestamp management in the audio status logic have been removed, streamlining the code to rely on the new mechanism using `LAST_AUDIO_CAPTURE`.

These changes improve how the system tracks and verifies live audio activity, enhancing reliability and reducing complexity in determining the health status of audio components.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 22 (50aa3d46cba577b847dee3d4230d699b35755cba)</summary>

The recent commit by Louis Beaumont includes a fix to the `build.rs` file located at `screenpipe-app-tauri/src-tauri`. The change introduces a conditional logic to copy the `screenpipe` binary for the `aarch64-apple-darwin` target architecture. The updated code first checks if the primary path (`../../target/aarch64-apple-darwin/release/screenpipe`) exists using `fs::metadata`. If it does, it uses that; otherwise, it falls back to an alternative path (`../../target/release/screenpipe`). This ensures the build process is more robust by handling cases where the primary path might not be available. Additionally, this operation is wrapped in an `expect` call to provide an error message if the file copy fails.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 23 (b7b047e00b77eac831ce8ddbff2cb9918195dcdb)</summary>

The commit by Louis Beaumont updates the `getting-started.mdx` file within the documentation. The changes made include the addition of instructions for installing Visual Studio with C++ by providing a link to the Visual Studio download page. This information is inserted after the line about installing 7zip.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 24 (c21d825c416a0f45f04e087847a9348a68065083)</summary>

The commit by the author Neptune on November 28, 2024, modifies the `build.rs` file in the `screenpipe-app-tauri` project. The changes focus on unwrapping code statements for filesystem operations within the build script.

Specifically, for both `x86_64` and `aarch64` target architectures, the `fs::copy` functions that had been using the `?` operator to propagate errors are now unwrapped using `.unwrap()`. This change ensures that any potential errors from the file copy operations will cause a panic with an error message rather than being handled by the `?` operator, which propagates errors up the call chain.
</details>

------------------------------------------------------------------------

# Overall Summary of Changes

The series of git commits to the "screenpipe-app" project includes multiple changes across various components, aiming to improve functionality, efficiency, and usability. Here are the highlights grouped by their primary focus:

1. **Component Updates and Logging Enhancements**:
   - Updates in various TypeScript and Rust components to refine logging mechanisms, improve event handling, and adjust UI components for platforms like macOS and Windows. Example: enhancing logging in `components/meeting-history.tsx` and adjusting PowerShell execution for better performance.

2. **Version and Build Process Modifications**:
   - Updates to version numbers in `Cargo.toml` and related files, indicating incremental improvements and bug fixes. Numerous changes to build scripts (`build.rs`) are evident, targeting architecture-specific optimizations (e.g., handling different Mac architectures and ensuring robust copying of binaries).

3. **New Features and Performance Improvements**:
   - Introduction of real-time updates for pipe management, implementing caching mechanisms (e.g., using `localforage` for icons), and employing semaphores to limit concurrent processes, thereby optimizing performance.

4. **Documentation and README Changes**:
   - Revisions to the `README.md` to improve clarity, simplification of installation instructions, and an emphasis on new features, such as camera recording. Removal of outdated descriptions and images replaced by current project capabilities.

5. **Workflow and Dependency Updates**:
   - Adjustments in GitHub workflow files to streamline release processes and improve build targets. Updates in dependencies and the introduction of helper constructs, such as `Lazy` initialization for efficient resource handling.

6. **Code Cleanup and Error Handling Adjustments**:
   - Removal of redundant or unnecessary code, such as logs and obsolete future handling mechanisms. Improved error handling to provide informative messages during executable searches and file operations.

7. **Specific Issue Fixes**:
   - Direct interventions, such as ensuring conditional logic for binary operations, handling Windows-specific issues, and managing timestamps for audio activity checks.

These changes collectively ensure a more reliable, efficient, and user-friendly application, addressing identified issues and implementing new features aligned with project goals.
