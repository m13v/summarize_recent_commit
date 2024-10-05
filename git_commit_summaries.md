# Git Commit Summaries

-----------------------------------------------------------------------
-----------------------------------------------------------------------
 
PRESS CMD+SHIFT+V TO VIEW IN MARKDOWN
 
_______________________________________________________________________
-----------------------------------------------------------------------
Total number of commits: 11

<details>
<summary>Summary for commit 1 (b027372dc1b92f831494bcc64b5419214b3cbe41)</summary>

The commit simplifies and modifies the script `pre_build.js` to address an issue specific to Windows. Previously, the script set certain environment variables at the start of the file if the platform was detected as Windows. These variables were necessary for a vcpkg download fix. In this update, the redundant block of code defining the variables globally has been removed. Instead, the necessary environment variables (`SystemDrive`, `SystemRoot`, `windir`) are set inline when the vcpkg command is executed. This change aims to resolve Windows-related issues more concisely without cluttering the beginning of the script.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 2 (131957629ed7873df544329c5e2febf55bcf88aa)</summary>

The commit with ID `131957629ed7873df544329c5e2febf55bcf88aa`, authored by Louis Beaumont, attempts to fix issues related to Windows in the `release-app.yml` GitHub Actions workflow. The changes remove separate steps for running `pre_build.js` on Windows and non-Windows platforms, consolidating them into a single step. Previously, `pre_build.js` was run with a condition that checked if the runner was not Windows for one step and was Windows for another step, using different shells (bash for non-Windows and PowerShell for Windows). Now, the script is run unconditionally without distinguishing between operating systems and uses a bash shell in a single unified step, while maintaining the essential environment setup.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 3 (5bce6f4da65dadaca307663539ca0480d038167b)</summary>

The git commit made by Louis Beaumont, identified as `5bce6f4da65dadaca307663539ca0480d038167b`, involves changes to the `.github/workflows/release-app.yml` file. Specifically, it updates the `vcpkgGitCommitId` for the `lukka/run-vcpkg` action from `"5b1214315250939257ef5d62ecdcbca18cf4fb1c"` to `"7adc2e4d49e8d0efc07a369079faa6bc3dbb90f3"`. This change is part of a job that runs when the matrix platform is set to 'windows-latest', suggesting an attempt to fix issues related to Windows builds by using a different version of vcpkg.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 4 (c488d307e17979517c6cede49270dc385435d581)</summary>

The commit c488d307e17979517c6cede49270dc385435d581, authored by Louis Beaumont on October 4, 2024, includes modifications to the GitHub Actions workflow file `release-app.yml` in an attempt to fix issue #432. The changes refine how the `pre_build.js` script is run on different platforms:

1. **Separation of Platform-Specific Steps**:
   - The script execution has been separated for non-Windows and Windows platforms.
   
2. **For Non-Windows Platforms**:
   - The script `pre_build.js` is run only when the runner OS is not Windows.
   - The `bash` shell is used for execution.
   - An environment variable `SKIP_SCREENPIPE_SETUP` is set to `true`.
   - The `working-directory` is set to `./screenpipe-app-tauri`.

3. **For Windows Platforms**:
   - The script `pre_build.js` is run only when the runner OS is Windows.
   - The `pwsh` (PowerShell) shell is used for execution.
   - The same environment variable `SKIP_SCREENPIPE_SETUP` is set to `true`.
   - The script output uses `Get-ChildItem -Recurse` to list directory contents.
   - The `working-directory` is maintained as `./screenpipe-app-tauri`.

These changes appear aimed at improving the build process by ensuring the script runs correctly on both Windows and non-Windows environments.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 5 (dfeab2f271c73aa679519a03105cb00b70a2d9f2)</summary>

The commit made by Louis Beaumont attempts to fix issue #432. It modifies the GitHub Actions workflow file `.github/workflows/release-app.yml`. Specifically, it slightly adjusts the syntax of a conditional assignment within the "Run pre_build.js" step. The change is simply a correction in spacing or formatting for the condition that selects the shell to use, but the logic itself remains unchanged (`bash` for non-Windows platforms, `pwsh` for Windows).
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 6 (1cd05d5acab433fb52288d5c5d8e14aa5d610ac6)</summary>

The recent commit, authored by Louis Beaumont, includes updates aimed at addressing issue #432. The changes involve:

1. **GitHub Actions Workflow Update:**
   - The `release-app.yml` file was modified to improve compatibility across different platforms (`windows-latest` and others).
   - The shell used in the "Run pre_build.js" step now dynamically switches between `bash` for non-Windows platforms and `pwsh` (PowerShell) for Windows.
   - Pre-existing Windows-specific environment variable exports were removed and replaced with platform-appropriate commands for running scripts (`bun ./scripts/pre_build.js`) and listing directory contents (`ls -R .` or `Get-ChildItem -Recurse`).

2. **Version Bump in Cargo.toml:**
   - The version of the `screenpipe-app` package was incremented from "0.2.94" to "0.2.95", indicating a minor update likely related to the adjustments made in the workflow. 

These changes aim to enhance build reliability and platform compatibility as part of the release process.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 7 (d79644a7c59e7674f87e61b8a5501902b2075a1f)</summary>

The commit `d79644a7c59e7674f87e61b8a5501902b2075a1f` by Louis Beaumont includes the following changes:

1. **Code Simplification**: In the `health_check` function of `server.rs`, code related to application initialization status has been removed. Specifically, a block checking if the application was still initializing (within the first 120 seconds after start) and returning a "loading" status has been deleted. This change removes the initial "loading" status check from the health check response.

2. **Search Params Change**: In a line involving a `curl` command for a search request, the `limit` parameter has been changed from 50 to 5. This reduces the number of search results returned by the search query example.

3. **Minor Cleanup**: A redundant empty line has been removed at the end of the file, indicating a minor cleanup.

These updates address issue #433 as indicated by the commit message.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 8 (765d12dbb71140f26c25ef253a1162cc153252a2)</summary>

The commit made by Louis Beaumont on October 4, 2024, introduces a change to the `main.rs` file in the `screenpipe-app-tauri` project. The modification adds a notification feature when the "update_now" event is triggered. 

Key changes include:
- Importing the `NotificationExt` from the `tauri_plugin_notification` library.
- Adding a notification that pops up with the title "screenpipe" and the body message "installing latest version" before proceeding with updating tasks.

This change enhances user experience by informing them about the ongoing installation of the latest version of the application.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 9 (1b4cac33e8d36ba2c28a1781aab1e5653926f44e)</summary>

The commit authored by Louis Beaumont makes the following changes to the update functionality in the application:

1. **Update Check Interval Changed**: The update check interval is modified from being specified in hours to being specified in minutes. Specifically, the call to `start_update_check` is changed to pass `5` instead of `1`, indicating a change from every hour to every 5 minutes.

2. **Constructor Parameter Updated**: The `UpdatesManager::new` function now takes an interval in minutes instead of hours. The conversion for the duration is updated from `hours * 3600` to `minutes * 60`.

3. **Text Modifications**: Several text strings in the update manager are changed from title case to lowercase:
   - `"Screenpipe is up to date"` becomes `"screenpipe is up to date"`.
   - `"Downloading Latest Version of Screenpipe"` becomes `"downloading latest version of screenpipe"`.
   - `"Update Now"` becomes `"update now"`.

These changes collectively adjust the update checking mechanism to run more frequently and modify some text strings for consistency in formatting.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 10 (dcbde84686ed5255f0fe588575565bc812ac8e2b)</summary>

The commit `dcbde84686ed5255f0fe588575565bc812ac8e2b` by Louis Beaumont includes the following changes:

1. **GitHub Actions Workflow Update**:
   - In the `release-app.yml` workflow file, a condition specific to the `windows-latest` platform was added. If the platform is Windows, environment variables `SystemDrive`, `SystemRoot`, and `windir` are exported to match environment values (`$SYSTEMDRIVE`, `$SYSTEMROOT`, `$WINDIR`). This is likely intended to address issues related to the Windows environment when running scripts.

2. **JavaScript Script Modification**:
   - In `scripts/pre_build.js`, a minor change was made: a `TODO` comment regarding possible lack of mp3 on Windows was removed from the `windows` object configuration.

3. **Version Update in Cargo.toml**:
   - The version of the `screenpipe-app` package in `Cargo.toml` was incremented from `0.2.93` to `0.2.94`.

These changes suggest improvements to the build process for Windows and a version bump for the `screenpipe-app`.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 11 (f7c63aa6ab0fd09a7adb8dbbeccab3058caf2721)</summary>

The commit introduces several changes primarily related to telemetry and user settings management in the `screenpipe` application. 

- **GitHub Actions**: Removed some commented code related to the cargo build process in the workflow file.

- **Frontend (React components)**:
  - Integrated `posthog` for analytics in the `recording-settings.tsx` component.
  - Introduced functionality to toggle telemetry with the ability to enable or disable it based on user preference.
  - Added a switch component to the UI for users to toggle telemetry data collection.
  - Moved the telemetry handling logic from `settings.tsx` to `recording-settings.tsx`, suggesting a restructuring of how settings are managed.

- **Backend (Rust)**:
  - Updated the `Cargo.toml` files:
    - The app version was bumped from `0.2.91` to `0.2.93`.
    - Added a `highlightio` dependency for telemetry purposes.
  - In the sidecar and server code:
    - Introduced logic to check and potentially disable telemetry based on configuration settings.
    - Added support for a command-line argument to disable telemetry in the server application.
  - Added a JSON schema for access control to manage permissions related to the app's storage and behavior at different levels of access.

- **Scripts and Pre-Build Modifications**:
  - Added a fix specific to Windows for downloading necessary components using `vcpkg`.

Overall, this commit enhances the app's telemetry capabilities, allowing users to toggle telemetry settings and introducing infrastructure to handle telemetry data more effectively. It also includes structural changes and updates to manage settings more efficiently across different components.
</details>

------------------------------------------------------------------------

# Overall Summary of Changes

This summary captures several Git-related changes made by Louis Beaumont aimed at improving the functionality and fix issues in the `screenpipe` application. Here's a breakdown of the key modifications:

1. **GitHub Actions Workflow**:
   - A significant rewrite of the `release-app.yml` file consolidates platform-specific steps for running `pre_build.js`. The script is run unconditionally using a bash shell to simplify platform handling.
   - Updates include modifying environment variable exports for Windows to facilitate script execution without extra steps.

2. **Code and Script Simplifications**:
   - The `pre_build.js` script was refined on Windows by removing global environment variable declarations and setting them inline with the vcpkg command to combat platform-specific issues.
   - A minor adjustment in `main.rs` introduces user notifications when updates are being installed, enhancing user interaction.
   
3. **Version Control and Dependencies**:
   - Updates in `Cargo.toml` reflect version increments and additional dependencies, like `highlightio`, to enhance telemetry functionalities.

4. **Frontend and Backend Enhancements**:
   - Telemetry features were integrated into the `recording-settings.tsx` React component using `posthog` to collect analytics.
   - Backend updates in Rust allow toggling telemetry settings and incorporate control at different access levels.

5. **Configuration Management**:
   - Changes involved tweaking the update check frequency from hourly to every 5 minutes and refining script execution logic.
   - The addition of new configuration management capabilities ensures that the app's storage and operational behavior are aligned with user preferences.

Overall, these commits concentrate on streamlining build processes for different operating systems, enhancing user experience during updates, managing application telemetry better, and preparing the framework for future improvements in feature settings and data handling within the `screenpipe` application.
