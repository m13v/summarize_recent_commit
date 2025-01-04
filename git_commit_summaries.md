# Git Commit Summaries

-----------------------------------------------------------------------
-----------------------------------------------------------------------
 
PRESS CMD+SHIFT+V TO VIEW IN MARKDOWN
 
_______________________________________________________________________
-----------------------------------------------------------------------
Total number of commits: 67

<details>
<summary>summary for commit 1 (1fa461ac09ff0cc7b5e4701ddce5c2f6225834f7)</summary>

The commit `1fa461ac09ff0cc7b5e4701ddce5c2f6225834f7` by Louis Beaumont includes changes to the GitHub Actions workflow file `release-app.yml`. The changes remove the step that installs Python and pip for the `x86_64-pc-windows-msvc` target. Previously, the workflow installed pip using `actions/setup-python@v5` and a direct download from `get-pip.py`, which is now omitted. The rest of the steps, including copying the library for MKL and installing `intel-openmp` using pip, remain unchanged.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 2 (648c89abeb770ab5a2630fa90d9fcd80e1bbc813)</summary>

The commit 648c89abeb770ab5a2630fa90d9fcd80e1bbc813, authored by Louis Beaumont on Mon Dec 9, 2024, addresses an issue with settings not saving in the "release-app". The update includes several significant changes:

1. **DevModeSettings Component**:
   - The `useSettings` hook now returns `localSettings` and `setLocalSettings`, which replaces the previous local state management for settings.
   - Removed debug `console.log` statements.

2. **RecordingSettings Component**:
   - Removed unused dialog, code block, and copy-to-clipboard imports.
   - Simplified checking platform logic by directly assigning `platform()`.
   - Removed `restartInterval` from settings and all related UI elements and handling code.
   - Cleaned up commented-out code pertaining to select items.

3. **ScreenpipeStatus Component**:
   - Removed unused state for fixing setup and the current step.
   - Removed event listener for `settings-updated`.

4. **Settings Component**:
   - Integrated `localSettings` and its state management directly from `useSettings`.
   - Removed platform state and instead rely on direct calls.
   - Eliminated a check for custom URL settings.
   - Removed redundant `useEffect` that set localSettings whenever `settings` changed and cleaned up event listeners.

5. **useSettings Hook**:
   - Added `localSettings` and `setLocalSettings` to manage settings locally.
   - Enhanced `updateSettings` to update both local and persistent store settings.
   - Refined the creation of default settings by encapsulating them in a function based on the platform.

6. **Cargo Files**:
   - Bumped the version number of `screenpipe-app` from `0.15.2` to `0.15.5` in `Cargo.lock` and from `0.15.4` to `0.15.5` in `Cargo.toml`.

7. **Analytics**:
   - Removed the info log line "Analytics started" in `analytics.rs`.

These changes focus on enhancing the settings management by removing unnecessary complexity, unused code, and accurate state synchronization. Additionally, the version has been updated to reflect these changes.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 3 (500c8fc3e658fd8045e19f35b814dc310b5c65a7)</summary>

The commit contains several changes across multiple files:

1. **Cargo.toml (screenpipe-app-tauri/src-tauri)**
   - The package version is incremented from `0.15.3` to `0.15.4`.

2. **db_benchmarks.rs (screenpipe-server/benches)**
   - An import statement is updated to reflect a change in the module path: `ContentType` is now imported from `screenpipe_server::db_types` instead of `screenpipe_server`.

3. **screenpipe-server.rs (screenpipe-server/src/bin)**
   - The logging setup logic is modified to ensure the logging guard variable `_log_guard` persists for the entire duration of the `main` function by changing it to an `Option`.
   - Corrects a formatting issue in a `println!` statement that prints a text-based UI element (note: the line seems like it was corrected but there's a placeholder character issue in the diff snippet provided).

4. **ocr.swift (screenpipe-vision/src)**
   - Redundant image preprocessing code is removed. Specifically, the application of certain filtering steps like `CIColorControls` and `CIUnsharpMask` is commented out, and the `ciImage` is directly used subsequently.

Overall, the commit primarily addresses a logging issue, removes unnecessary code, and makes minor code improvements and version updates.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 4 (72537dd3ec1542e499ebf7d2d4934233b1fa34e4)</summary>

The git commit adds a new changelog file for version 0.15.3. It highlights an improvement in the software: the log button's visibility has been enhanced for easier access and usage. Additionally, the change updates the existing changelog in the `screenpipe-app-tauri/public/CHANGELOG.md` file by removing previous entries for new features and fixes, and focusing on the improved visibility of the log button as the primary update in this version. The full changelog for the changes can be viewed through the provided link comparing two commit hashes.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 5 (1bd7f17f9b91b8e1388b497a029bb5d723de618d)</summary>

This commit updates the GitHub Actions workflow configuration for building the "release-app" on Windows. Key changes include:

1. **Workflow Improvements:**
   - Replaces the usage of `actions/cache` with `Swatinem/rust-cache` for caching Rust dependencies.
   - Adds a new caching step specifically for build artifacts using `actions/cache`.

2. **Cache Configuration:**
   - Customizes cache keys for both Rust dependencies and build artifacts.
   - Ensures shared caching across branches and allows caching even on failed builds.

3. **Python Setup:**
   - Adjusts the Python version to `3.10` for one setup and `3.13` for another, removing the `pip` cache option.

4. **Miscellaneous Updates:**
   - Updates the `Cargo.toml` to bump the version of the `screenpipe-app` package from `0.15.2` to `0.15.3`.
   - Removes unnecessary blank lines for cleaner formatting.

Overall, this commit focuses on optimizing the build process for Windows and keeping dependencies and configurations up to date.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 6 (b94e2ffdfc15136ef3859736c0e2eec21f0c6ed6)</summary>

The commit titled "release-app fix windows build" by Louis Beaumont includes the following changes:

1. **GitHub Workflow File Changes**:
   - **File**: `.github/workflows/release-app.yml`
   - Changes were made to the Python setup for Windows builds:
     - The `python-version` was downgraded from "3.13" to "3.11".
     - A caching strategy for pip was added with `cache: 'pip'`.

2. **Screenpipe App Version Update**:
   - **File**: `screenpipe-app-tauri/src-tauri/Cargo.lock` and `screenpipe-app-tauri/src-tauri/Cargo.toml`
   - The version of the "screenpipe-app" was updated from "0.15.1" to "0.15.2".

These changes primarily focus on fixing the Windows build for the release app and updating the application version.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 7 (4b926cebecfd0a9f92137a6fa560e10a926095c0)</summary>

The git commit with the ID `4b926cebecfd0a9f92137a6fa560e10a926095c0`, authored by Louis Beaumont, introduces a fix related to issue #866. The changes involve two components in the `screenpipe-app-tauri` project.

1. **Header Component (`header.tsx`):**
   - Added a new asynchronous function `handleClearAll`, which clears the messages array and updates local storage by setting `"inboxMessages"` to an empty array.
   - Passed the `handleClearAll` function to the `InboxMessages` component as a prop named `onClearAll`.

2. **InboxMessages Component (`inbox-messages.tsx`):**
   - Modified the `InboxMessagesProps` interface to include a new function prop `onClearAll`.
   - Updated the `InboxMessages` component to accept the `onClearAll` prop.
   - Added a new "Clear All" button in the header section of the Inbox Messages. This button uses the `onClearAll` function to clear all messages when clicked. 
   - Made UI adjustments to accommodate the new button alongside the existing "Mark All as Read" button, arranging them in a horizontal flexbox with space between them.

These changes add a new feature allowing users to clear all messages from the inbox in the application.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 8 (c598531d15424b4fdabb382b1fb9da1829f36cbc)</summary>

The commit by Louis Beaumont makes the log button in the "screenpipe-app-tauri" application more visible. Notable changes include:

1. **Enhancements to `log-file-button.tsx`:**
   - Introduces a `size` prop with options "8", "10", and "12" to adjust the size of the log button.
   - Updates the button's CSS classes to adjust its dimensions (`h-8 w-8`, `h-10 w-10`, `h-12 w-12`) based on the `size` prop.
   - Adjusts the size of the `FileText` icon within the button to match the button size.

2. **Changes in `screenpipe-status.tsx`:**
   - Adds a `LogFileButton` in the `screenpipe-status` dialog with a predefined size of "10".
   - Removes an extra `LogFileButton` that used to appear in a collapsible section, possibly to avoid redundancy.
   - Several minor code formatting changes, such as adjustments to line endings and spacing for consistency.

3. **Version Update in `Cargo.lock`:**
   - The version of `screenpipe-app` in `Cargo.lock` is updated from `0.15.0` to `0.15.1`, indicating a minor version bump likely due to the UI improvement.

These changes collectively aim to improve the visibility and usability of the log button in the application interface.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 9 (4e53262c1ef23a03b0dfd332ff755ac864b26f31)</summary>

The git commit includes several changes across different files:

1. **`.github/workflows/release-app.yml`**:
   - Removed the installation steps for the `unzip` utility when targeting the `x86_64-pc-windows-msvc` platform.

2. **`Cargo.lock` and `Cargo.toml`**:
   - Updated the version of the `screenpipe-app` package from `0.14.37` to `0.15.1`.
   - Added the `which` crate as a build dependency with version `7.0.0` in the `screenpipe-audio` project.

3. **`build.rs`**:
   - Added a new function `find_unzip` to locate the `unzip` executable on Windows systems.
   - The `install_onnxruntime` function now uses the new `find_unzip` function to locate the `unzip` utility.

4. **Code Formatting and Cleanup**:
   - Refactored and formatted various Rust files to improve readability.
   - Grouped imports and adjusted the spacing in `screenpipe-server/src/bin/screenpipe-server.rs`.
   - Added additional logging imports to the file.

5. **`screenpipe-server/src/bin/screenpipe-server.rs`**:
   - Enhanced logging by including more structured logging statements.
   - Added additional command-line options to specify the server port for pipe commands, allowing operations to target a running server instance or fall back to local configurations if the server is unavailable.
   - Extended error handling and logging for more informative feedback during pipe operations.

6. **`cli.rs`**:
   - Extended the `PipeCommand` enum by adding a `port` field to various variants, enabling server communication on a specified port.

7. **`lib.rs`**:
   - Made the `pipe_manager` module public.

Overall, these changes enhance the application's build process, update versioning, augment logging and error handling, refactor for readability, and introduce more flexibility in server command interactions.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 10 (68f302c3483eec2aab11c07cd6a7c7719009f129)</summary>

The commit made by Louis Beaumont includes several changes across different files with the primary intent to fix the build for the "release-app" GitHub workflow. Here are the summarized changes:

1. **GitHub Workflow Update (`.github/workflows/release-app.yml`):**
   - Minor formatting change by removing a trailing space.
   - Added a new step to install `unzip` when targeting `x86_64-pc-windows-msvc`. This involves:
     - Downloading the `unzip` tool.
     - Extracting it to a specified directory.
     - Adding it to the system PATH for subsequent use.
     - Cleaning up the downloaded archive after use.
     - Verifying the installation by checking the version of `unzip`.

2. **Documentation Update (`pipes/pipe-notion-table-logs/README.md`):**
   - The README title was modified from "phi3.5 engineering team logs" to "pipe notion table logs".
   - A clarification in the description to simply "automates logging of work to notion using screenpipe and ai", removing specific references to "engineering work" and "phi3.5 ai".

3. **JavaScript Script Update (`screenpipe-app-tauri/scripts/pre_build.js`):**
   - Commented out a section of code related to using `vcpkg.exe` to install packages on Windows, with a comment questioning its necessity due to potentially using a `build.rs` file for that purpose. This suggests an ongoing consideration or cleanup of the build process.

Overall, the commit focuses on enhancing the build setup, updating documentation for clarity, and potentially refining the build script to avoid redundant steps.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 11 (245f8058e2c648e55a618e65235045f609acd3b7)</summary>

The git commit `245f8058e2c648e55a618e65235045f609acd3b7` introduces a changelog for version 0.15.0. This changelog highlights the following updates:

1. **New Features:** 
   - No new features have been added in this release.

2. **Improvements:**
   - An issue with settings not updating correctly has been fixed, enhancing the user experience.

3. **Fixes:**
   - Cursor rules functionality has been updated for better accuracy.
   - Issues with the Windows build have been resolved to improve stability and performance.

Additionally, the commit updates the existing `CHANGELOG.md` file in the `screenpipe-app-tauri` project with the same information.

The full changelog can be found in the repository comparing the changes between commits `98cc4` and `f8d68`.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 12 (f8d68a5ca6e3e89f9fa994a84e4877ba1e0c6c0f)</summary>

The git commit `f8d68a5` by Louis Beaumont on December 8, 2024, introduces several updates and fixes to the `screenpipe-app-tauri` project:

1. **Fixes in `dev-mode-settings.tsx`:**
   - Simplified the construction of file paths (`logPath` and `dbPath`) for different OS environments by improving string concatenations.
   - Enhanced error handling in `handleDevModeToggle` by logging more informative error messages and adding a toast notification on errors.
   - Added debug logs to check `settings` and `localSettings`.

2. **Changes in `screenpipe-status.tsx`:**
   - Removed unused or redundant logic and handlers such as `openScreenPermissions` and `handleFixSetup`.
   - Commented out or removed complex troubleshooting instructions and handled UI enhancements for concise and cleaner code.
   - Improved formatting and consistency in code, especially within the JSX elements.

3. **Updates in `use-settings.tsx`:**
   - Modified the `updateSettings` function to ensure it updates settings for all fields, even those that might be `falsy`, by targeting keys explicitly from `newSettings`.
   - Removed unnecessary `store!.save()` call due to `autoSave` feature.

4. **Version Bump in `Cargo.lock` and `Cargo.toml`:**
   - Updated the `screenpipe-app` version from `0.14.35` to `0.14.37` in `Cargo.lock`.
   - Changed the version in `Cargo.toml` from `0.14.36` to `0.15.0`, indicating a new minor release with these adjustments.

This update primarily focuses on improving the reliability of settings management, removing non-essential and potentially buggy features, enhancing error visibility, and cleansing the codebase for maintenance simplicity.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 13 (a163e9ba8ea9bf9d00632a92ce85f2c164007de6)</summary>

The commit includes several updates and additions:

1. **Cursorrules Update**: A large addition of a new section called "Code Examples & Context" providing detailed examples and best practices for coding in Rust and NextJS. These include:
   - Logging style recommendations in Rust (use lowercase and `println!` in tests).
   - Error handling patterns in Rust using `anyhow`.
   - Async patterns in Rust, preferring channels over mutexes.
   - UI styling guidelines in NextJS using Tailwind and Shadcn.
   - Usage examples of Screenpipe API for querying and notifications.
   - Key development principles such as cross-platform compatibility and clear error messages.

2. **New Documentation Files**:
   - Addition of a markdown file `2024-12-07.md` detailing various sessions, their descriptions, tags, and time spent. This includes activities like setting up ScreenPipe with Obsidian, screen recording analyses, philosophical explorations, and more varied tasks.
   - Addition of another markdown file `2024-12-08.md` similar to the previous day's log with entries focused on reviewing educational material, social media activity analysis, productivity tasks, and discussions around AI and LinkedIn posts.

3. **CONTRIBUTING.md Change**: Removal of a section describing an AI system prompt. This section included guidelines for coding style and contributions like keeping a minimalist UI style, using certain coding libraries, and principles for code creation and maintenance.

Overall, this commit reflects significant updates to coding standards documentation and daily activity logging in markdown format for better organization and tracking.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 14 (d5e31345d2389ea86fad8fa6ec65a98738069e26)</summary>

The commit with ID `d5e31345d2389ea86fad8fa6ec65a98738069e26`, authored by Louis Beaumont, contains the following changes:

1. A fix to the Windows build process for the "screenpipe-app" project was implemented. This was done by modifying the `pre_build.js` script to use the `VCPKG_ROOT` environment variable when installing Vcpkg packages on Windows. The change ensures the correct path is used for executing the Vcpkg installation command.

2. The version of the "screenpipe-app" was incremented from `0.14.35` to `0.14.36` in the `Cargo.toml` file, indicating a minor change or bug fix in the application.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 15 (98cc49b7c1ac6c5b207e4f26159e460701ed5335)</summary>

The Git commit titled "release-app fix obsidian pipe, ffmpeg windows" includes several changes across different files:

1. **GitHub Action Workflow (`release-app.yml`)**:
   - Update to the PATH modification logic during Windows setup. The change limits the PATH update to the current session only, rather than changing the PATH environment variable for the entire machine.

2. **Obsidian Pipe**:
   - **Dependency Updates**: The `package.json` file for the `pipe-obsidian-time-logs` project has been updated, changing versions for dependencies like `@ai-sdk/openai`, `ollama-ai-provider`, and `ai`. The order of dependencies has also been adjusted.
   - **Configuration Simplification**: Removal of parameters related to OpenAI API keys and GPT model from `pipe.json` and `pipe.ts`. The script now assumes the use of Ollama by default.
   - **Refactored Code**: Simplification of the AI provider initialization in `pipe.ts` by directly using the `ollama` function. Unused code related to choosing between OpenAI and Ollama is removed.
   - **Executable and Configuration Outline**: Additional notes for necessary environment variables and instructions to run the pipe script are provided as comments in `pipe.ts`.

3. **Pre-build Script (`pre_build.js`)**:
   - Addition of another potential location for `wget.exe` on Windows systems, namely `C:\\wget\\wget.exe`, enhancing the script's flexibility in locating the executable.

4. **FFmpeg Code (`ffmpeg.rs`)**:
   - Minor code adjustment in error handling for Windows systems, ensuring error messages provide more detailed context using `anyhow::anyhow!(e)`.

Overall, this commit involves dependency updates, code simplification and refactoring, and adjustments aimed at improving cross-platform compatibility and maintainability for Windows systems.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 16 (467893cf7eeecbe9d80dbb7095646ed5cd553936)</summary>

The recent Git commit by Louis Beaumont makes several important fixes and updates:

1. **Enable/Disable Handling in `pipe-store.tsx`:** The `PipeStore` component in the Tauri app has been updated to immediately reflect changes to the pipe's enabled status in the local state, thus ensuring a responsive UI. The state update logic now directly toggles the pipe's enabled state and updates the details view if applicable. The toast notification message has been modified slightly, and a comment was added to emphasize fetching the latest server state for consistency.

2. **Version Bump:** The version of the `screenpipe-app` has been incremented in both `Cargo.toml` and `Cargo.lock` from `0.14.34` to `0.14.35`, indicating a minor update, likely to incorporate the new fixes.

3. **FFmpeg Path Checking on macOS:** An enhancement to the `screenpipe-core` adds a check for the FFmpeg executable in `$HOME/.local/bin` when running on macOS. This increase in path searching efforts improves ease of use by automatically locating FFmpeg if it's present in this common user directory.

Overall, these changes improve the usability and reliability of the application, particularly for macOS users and in providing a more responsive user interface.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 17 (658f678ade077674349f178be9723d302a5aeeec)</summary>

The commit made by Louis Beaumont on December 7, 2024, addresses an issue with Windows process termination detection in the `screenpipe-server` repository. The changes include:

1. **Version Update**:
   - The version of the `screenpipe-app` in its Cargo.toml file was incremented from `0.14.33` to `0.14.34`.

2. **Code Modification for Windows**:
   - In the `auto_destruct.rs` file, enhancements are made to the process of checking if a Windows application has terminated.
   - The previous approach, which relied on checking the PID using the `tasklist` command, is expanded. Now, both the PID and the process name (`screenpipe-app.exe`) are checked for activity using separate `tasklist` commands.
   - Diagnostic print statements have been added to output the status of the PID and the application process.
   - For non-Windows operating systems, the original check using the `ps` command remains unchanged but has been isolated with the `#[cfg(not(target_os = "windows"))]` attribute for improved clarity and control.

This change aims to improve the reliability of detecting when the `screenpipe-app` on Windows has terminated, potentially addressing issues with automatic self-termination of the app.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 18 (352d98aadc84fdb59f4d5c8f272c86a41ac89167)</summary>

This commit by Louis Beaumont addresses the following changes:

1. **Windows Build Script in GitHub Actions**:
   - Updated the script in the `release-app.yml` workflow to handle the installation of `wget` on Windows more effectively.
   - Removed an alias for `wget` if it exists to avoid conflicts.
   - Changed the download and extraction of `wget` into a specific directory (`C:\wget`) and updated the system `PATH` environment variable to include this directory. 
   - The method for validating the `wget` installation has been slightly modified.

2. **Hide Recorder Log for Non-Mac Users**:
   - Updated the `screenpipe-status.tsx` component to conditionally display the recorder logs based on the operating system, specifically hiding it for non-Mac systems.

3. **Code Formatting**:
   - Improved the formatting of some TypeScript code for better readability, such as the structure for handling permissions checks and requests, using consistent indentation and trailing commas.

4. **Version Update**:
   - Updated the version of the `screenpipe-app` in the `Cargo.lock` file from `0.14.30` to `0.14.33`.

These changes collectively focus on improving the Windows build process and user interface conditional logic regarding operating system-specific features.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 19 (f19fdecec219a334742431232ae098a7d7d9a222)</summary>

The commit with ID `f19fdecec2` introduces documentation changes related to version 0.14.33. It primarily adds a new changelog file, documenting bug fixes. Specifically, it highlights a fix for an issue with `ffmpeg` on macOS Tauri applications, enhancing functionality. The changelog file `/content/changelogs/0.14.33.md` is newly created, and the existing `CHANGELOG.md` file in `screenpipe-app-tauri/public/` is updated to include the same fix. Both the new and updated changelogs reference the same range of changes in the repository detailed in the full changelog link `[19498..fb8e2]`. Previously noted new features and improvements in `CHANGELOG.md` have been removed, focusing solely on the bug fix. Additionally, a new binary file named `cn` was added.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 20 (fb8e2a2a6f10621a0ff9ffd9ce869382ca69d437)</summary>

The commit by Louis Beaumont on December 7, 2024, addresses an issue with FFmpeg on a macOS Tauri app and includes the following changes:

1. **Version Update**: The version of the `screenpipe-app` package has been incremented from `0.14.32` to `0.14.33` in the `Cargo.toml` file, indicating a new release or update.

2. **Code Rearrangement**: The imports in `ffmpeg.rs` have been rearranged, potentially for better organization or readability.

3. **Error Handling Improvements**: The `handle_ffmpeg_installation` function now uses `anyhow::Error` for better error handling, replacing `String`-based errors.

4. **macOS Specific Directory Setup**: A new function, `get_ffmpeg_install_dir`, has been added for macOS. This function ensures that the FFmpeg binary is installed in the user's `~/.local/bin` directory. If the directory does not exist, it creates it and updates the PATH in shell configuration files (`.bashrc`, `.bash_profile` for bash, and `.zshrc` for zsh) to include this directory.

5. **Platform-Specific Logic**: The function `get_ffmpeg_install_dir` uses macOS-specific logic to determine and prepare the FFmpeg installation directory, with a default implementation for other operating systems using existing logic. 

These changes ensure better handling of FFmpeg installation on macOS, potentially solving installation or path issues specific to that platform.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 21 (19498cf44d343548f94881752d881c6e1aa04550)</summary>

The commit titled "release-app" made modifications to the `.github/workflows/release-app.yml` file. Specifically, the changes involve how the `wget` utility is handled when it is not available through Chocolatey:

1. **New Directory Creation**: A new directory for `wget` is created at the path specified by the environment variable `$env:USERPROFILE\wget`.

2. **Download and Extraction Changes**:
   - Previously, the downloaded `wget` file was extracted directly to `C:\Windows\System32`.
   - Now, the file is extracted to the newly created `wget` directory under the user's profile.

3. **Environment Path Update**: The `PATH` environment variable is updated for the current session to include the newly created `wget` directory, ensuring that `wget` can be executed.

4. **Clean-up**: The downloaded `wget` zip file is removed after extraction, as per the existing workflow.

These changes improve the flexibility and safety of the script by avoiding extraction to the `System32` directory and ensuring `wget` is accessible in the session.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 22 (92664d2b3c168ac4ecbb06c61871c0e70d15c905)</summary>

The commit by Louis Beaumont modifies the `.github/workflows/release-app.yml` file, specifically updating the section responsible for setting up `wget` on Windows. Previously, `wget` was installed using Chocolatey. The updated script now first checks if Chocolatey is available and uses it to install `wget`. If Chocolatey is not available, it downloads `wget` directly from a specified URL, extracts it, and places it in the `C:\Windows\System32` directory. Finally, it verifies that `wget` is correctly installed by checking its version.
</details>

------------------------------------------------------------------------

# Overall Summary of Changes

The series of commits by Louis Beaumont cover a range of updates and fixes across the GitHub Actions workflow, application components, version increments, and other functional and structural improvements in the `screenpipe-app-tauri` project. Here's a consolidated summary of the key changes:

1. **GitHub Actions Workflows**:
   - Modifications to `release-app.yml` include switching from `actions/cache` to `Swatinem/rust-cache` for Rust dependencies, and improvements in `wget` and `unzip` setup on Windows. The scripts now dynamically handle path creation and include clean installation strategies.

2. **Version Updates**:
   - Incremental version updates were performed several times, signifying minor tweaks, bug fixes, and improvements from versions `0.14.30` up through `0.15.5` across `Cargo.toml` and `Cargo.lock` files.

3. **Code Refactoring and Features**:
   - Enhanced management of settings in components (`Settings`, `DevModeSettings`, `useSettings`), removal of unused hooks, and improved state synchronization.
   - Addition of a new feature in `InboxMessages` to clear all messages, improving user interactivity.
   - Improved process handling and error reporting, particularly for Windows applications.

4. **Documentation and Changelogs**:
   - Updates and standardization of changelogs, documenting important fixes like FFmpeg issues on macOS versions.
   - Enhanced documentation for coding practices and logging styles in Rust and NextJS, alongside daily activity logging in markdown format relating to project workflows and sessions.

5. **Windows Build and Compatibility Enhancements**:
   - Scripts adjustments to ensure consistent package availability (`wget`, `unzip`) on Windows builds, including path configuration.

6. **UI Improvements**:
   - Enhanced visibility and usability of components such as the log button, conforming to design changes with flexible sizing.

7. **Build Scripts and Dependency Management**:
   - Adjustments to build scripts (`pre_build.js`) for better compatibility and redundancy removal, such as using `vcpkg`.

These updates collectively emphasize improving build processes, enhancing UI features, fine-tuning application settings and state management, and ensuring smooth operation across different operating systems.
