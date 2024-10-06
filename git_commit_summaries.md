# Git Commit Summaries

-----------------------------------------------------------------------
-----------------------------------------------------------------------
 
PRESS CMD+SHIFT+V TO VIEW IN MARKDOWN
 
_______________________________________________________________________
-----------------------------------------------------------------------
Total number of commits: 11

<details>
<summary>Summary for commit 1 (b027372dc1b92f831494bcc64b5419214b3cbe41)</summary>

The commit attempts to fix issues on Windows by modifying the `screenpipe-app-tauri/scripts/pre_build.js` script. Previously, the script set certain environment variables (`SystemDrive`, `SystemRoot`, `windir`) globally when the platform was detected as Windows. These lines have been removed, and instead, these environment variables are now set inline when the `vcpkg` command is executed. This change aims to resolve problems related to downloading or installing packages on Windows by ensuring the necessary environment variables are explicitly set during the `vcpkg` package installation process.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 2 (131957629ed7873df544329c5e2febf55bcf88aa)</summary>

The commit "131957629ed7873df544329c5e2febf55bcf88aa" by Louis Beaumont on October 4, 2024, contains changes to the GitHub Actions workflow for building a project. The commit modifies the `.github/workflows/release-app.yml` file primarily to simplify the conditional platform-specific build steps:

1. Previously, the workflow had separate steps to run `pre_build.js` for non-Windows and Windows platforms.
2. The non-Windows step used a bash shell and the Windows step used a PowerShell shell. The differentiation was based on the operating system (`runner.os`).
3. In this commit, these steps are consolidated into a single step to run `pre_build.js`, removing the platform-specific conditions. Now, the script is run using bash without the operating system check.
4. Environment variable `SKIP_SCREENPIPE_SETUP` is set to true in both cases to avoid copying screenpipe binaries that have not yet been built.
5. The Windows-specific commands using PowerShell have been removed.

This change simplifies the workflow by unifying the script execution across platforms.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 3 (5bce6f4da65dadaca307663539ca0480d038167b)</summary>

The commit with hash `5bce6f4da65dadaca307663539ca0480d038167b` by author Louis Beaumont, aims to address issues on the Windows platform in a GitHub workflow file. Specifically, the `.github/workflows/release-app.yml` file was modified. The change involves updating the `vcpkgGitCommitId` for the `lukka/run-vcpkg@v11` action from `"5b1214315250939257ef5d62ecdcbca18cf4fb1c"` to `"7adc2e4d49e8d0efc07a369079faa6bc3dbb90f3"`. This update suggests a change to the version or state of vcpkg used for the Windows build process.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 4 (c488d307e17979517c6cede49270dc385435d581)</summary>

The Git commit with hash `c488d307e17979517c6cede49270dc385435d581` authored by Louis Beaumont introduces changes to the GitHub Actions workflow specified in the `release-app.yml` file. The modification targets the pre-build step in the automated release workflow by splitting it into two separate tasks based on the operating system.

Here are the key changes:
- The pre-build process has been separated into two distinct tasks: one for non-Windows platforms and one specifically for Windows.
- For non-Windows systems, the pre-build step now runs a Bash shell script with a conditional check (`if: runner.os != 'Windows'`) and executes the `pre_build.js` script using Bun. It sets an environment variable `SKIP_SCREENPIPE_SETUP` to `true` and lists directory contents using recursive listing.
- For Windows systems, the pre-build step uses PowerShell with a corresponding conditional (`if: runner.os == 'Windows'`). It similarly sets the `SKIP_SCREENPIPE_SETUP` to `true`, runs the `pre_build.js` script using Bun with environment parameters, and lists files recursively using `Get-ChildItem`.

This restructuring is likely intended to address issue #432, ensuring that the pre-build script executes correctly across different operating system environments.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 5 (dfeab2f271c73aa679519a03105cb00b70a2d9f2)</summary>

The commit made by Louis Beaumont, intended to address issue #432, includes a minor change in the GitHub Actions workflow file `.github/workflows/release-app.yml`. Specifically, the change pertains to the `shell` statement in a script run step. The syntax for the conditional operator within the `shell` configuration remains the same but is reformatted slightly, although it might appear identical due to unchanged characters. This suggests the edit may have been aimed at resolving a subtle issue (such as formatting or an overlooked mistake) affecting the workflow's execution.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 6 (1cd05d5acab433fb52288d5c5d8e14aa5d610ac6)</summary>

The commit by Louis Beaumont attempts to address issue #432 and introduces the following changes:

1. **GitHub Actions Workflow (`release-app.yml`):**
   - The shell command in the `Run pre_build.js` step is modified to dynamically choose between `bash` and `pwsh` based on the platform. It uses `pwsh` for Windows (`windows-latest`) and `bash` for other platforms.
   - The commands within the `run` block are similarly adjusted to accommodate the platform-specific syntax for executing scripts and listing directory contents:
     - On non-Windows platforms, it runs the `pre_build.js` script and lists the directory contents using Unix-style commands.
     - On Windows, it uses Windows PowerShell syntax to run the script and list directory contents.

2. **Version Update in `Cargo.toml`:**
   - The version of the `screenpipe-app` package is incremented from `0.2.94` to `0.2.95`.

These changes aim to improve cross-platform compatibility in the build workflow by adapting command execution based on the operating system.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 7 (d79644a7c59e7674f87e61b8a5501902b2075a1f)</summary>

The commit `d79644a` by Louis Beaumont addresses issue #433 and involves modifications to `server.rs` in the `screenpipe-server` project. The changes made include:

1. **Removal of Code Block:**
   - A section of code responsible for returning a "loading" response during the application's initialization phase was removed. This block checked if the time since the application start was less than a specified loading threshold (120 seconds) and returned a "loading" status if the application was still starting up.

2. **Adjustment in Search Script:**
   - In the search example using `curl`, the limit parameter was adjusted from 50 to 5 for a search query that retrieves content between 2 hours ago and 1 hour ago.

3. **Minor Cleanup:**
   - A trailing newline at the end of a script was removed. 

Overall, the commit simplifies the health check logic by removing the initial loading state check and slightly adjusts a search query example to use a smaller limit.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 8 (765d12dbb71140f26c25ef253a1162cc153252a2)</summary>

The commit with ID `765d12dbb71140f26c25ef253a1162cc153252a2` adds a notification feature to a Rust application's `main.rs` file within the `screenpipe-app-tauri` project. Specifically, when the `"update_now"` event is triggered, a notification is created using the `tauri_plugin_notification` library. The notification displays the title "screenpipe" and the body text "installing latest version" to inform users that the latest version is being installed. This addition is encapsulated within an existing asynchronous function.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 9 (1b4cac33e8d36ba2c28a1781aab1e5653926f44e)</summary>

The git commit updates the interval for checking application updates from hours to minutes in a Tauri application. Specifically:

1. In the `main.rs` file, the update check function is now called with a parameter of `5` (assumed to be minutes) instead of `1` (previously hours).
2. The `UpdatesManager` structure in `updates.rs` is modified to accept an interval in minutes instead of hours. Consequently, the calculation of the interval in seconds now multiplies the minutes by 60 instead of the previous hours by 3600.
3. Text strings in the update menu items have been converted to lowercase to maintain consistency (e.g., "Screenpipe is up to date" is changed to "screenpipe is up to date").
Overall, the changes focus on configuring the update interval to be specified in minutes, increasing the frequency of checks, and adjusting some display strings.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 10 (dcbde84686ed5255f0fe588575565bc812ac8e2b)</summary>

The commit with hash `dcbde84686ed5255f0fe588575565bc812ac8e2b` made the following changes:

1. **GitHub Actions Workflow**:
   - Modified the `release-app.yml` workflow to add a specific step for the `windows-latest` platform under the `run` command.
   - The new step exports three environment variables (`SystemDrive`, `SystemRoot`, and `windir`) from existing system environment variables (`$SYSTEMDRIVE`, `$SYSTEMROOT`, and `$WINDIR`) on Windows systems.

2. **JavaScript Script**:
   - Removed the TODO comment in the `pre_build.js` file, which questioned a potential issue with Windows lacking MP3 support. The comment was directly above the configuration related to the Windows platform.

3. **Rust Project Configuration**:
   - Updated the version of the `screenpipe-app` package in the `Cargo.toml` file, changing it from `0.2.93` to `0.2.94`.

Overall, the changes focus on improving Windows-specific configurations and bumping the application version.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 11 (f7c63aa6ab0fd09a7adb8dbbeccab3058caf2721)</summary>

This commit includes several modifications across different files in the project. Here’s a summarized view of the changes:

1. **GitHub Workflow**: 
   - Removed commented lines related to the build process in `release-app.yml`.

2. **Tauri Components (React/TypeScript)**:
   - Introduced telemetry capabilities using `posthog-js` and `@opentelemetry/api` in `recording-settings.tsx`.
   - Added UI elements allowing users to enable or disable telemetry and updated related console logs.
   - The telemetry toggle state updates settings accordingly. 

3. **Settings Adjustment**:
   - Made changes in `settings.tsx` related to telemetry management, consolidating this functionality into `recording-settings.tsx`.
   - Deleted code that handled telemetry settings from `settings.tsx`, suggesting these were moved or redesigned.

4. **Hooks and Build Scripts**:
   - Added console logging to help track loading settings in `use-settings.tsx`.
   - Added a platform-specific fix for Windows in `pre_build.js`.

5. **Cargo.toml and Schemas**:
   - Updated the version in `Cargo.toml` and added dependencies for telemetry (`highlightio`).
   - Added a new JSON schema file `acl-manifests.json` for handling permissions and command configurations. This file seems extensive and details various command permissions and defaults.

6. **Sidecar Implementation**:
   - Integrated telemetry-related arguments in `sidecar.rs`, allowing runtime checks and toggling of telemetry (`--disable_telemetry`).

7. **Server Configuration**:
   - Added telemetry support in `screenpipe-server` using the `highlightio` package.
   - Modified CLI to include a `--disable-telemetry` option, which affects telemetry behavior during runtime.
   - Included checks to activate or disable telemetry and log relevant feedback or warnings based on the telemetry status.

These changes enhance the application by incorporating telemetry features with user-level control, improving telemetry data handling, and ensuring platform-specific issues (on Windows) are addressed with a script fix.
</details>

------------------------------------------------------------------------

# Overall Summary of Changes

Here is a summary of the git changes across multiple commits, examining both improvements to cross-platform compatibility and the introduction of new telemetry features:

1. **Cross-Platform Workflow Improvements**:
   - **Environment Variables in Scripts**: The `pre_build.js` script was modified to set Windows-specific environment variables (`SystemDrive`, `SystemRoot`, and `windir`) directly inline with the `vcpkg` command execution instead of globally. This resolves issues related to Windows package installations.
   - **Simplification of GitHub Actions**: Changes in the `.github/workflows/release-app.yml` file simplify the build process for different operating systems, moving away from separate steps for Windows and non-Windows platforms. The workflow now uses a unified Bash shell for running pre-build scripts across platforms, with a specific focus on using Bash instead of PowerShell for Windows. This unifies the steps and removes unnecessary conditions, improving maintainability.
   - **Vcpkg Update**: The vcpkg version was updated in the GitHub workflow to ensure compatibility with the latest package management features on Windows.

2. **Telemetry Introduction and Enhancements**:
   - **Tauri Frontend Updates**: In `recording-settings.tsx`, telemetry capabilities were introduced using `posthog-js` and `@opentelemetry/api`, allowing users to toggle telemetry settings through the UI.
   - **Settings Management Consolidation**: Telemetry settings were moved from `settings.tsx` to `recording-settings.tsx`, consolidating related functionalities.
   - **Addition of New JSON Schema**: A new schema file `acl-manifests.json` was introduced for managing command permissions and configurations.
   - **Sidecar and Server Adjustments**: Added telemetry arguments in `sidecar.rs` and support using the `highlightio` package in `screenpipe-server`. A CLI option `--disable-telemetry` was introduced to provide runtime control over telemetry features.
   - **Update Notifications**: A new feature in the `main.rs` file triggers notifications for update events, informing users about the installation of the latest version.

3. **Version Updates and Code Cleanup**:
   - Versions in the `Cargo.toml` file were incremented, reflecting updates like introducing telemetry and smoothing out Windows-specific configurations.
   - Code cleanup involved removing unnecessary comments and adjusting code blocks for improved logic flow, particularly simplifying the health check logic on initialization.

Overall, these changes improve the application's build workflow, especially on Windows, and introduce comprehensive telemetry capabilities, enhancing the user's control over data settings and improving operational transparency.
