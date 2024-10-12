# Git Commit Summaries

-----------------------------------------------------------------------
-----------------------------------------------------------------------
 
PRESS CMD+SHIFT+V TO VIEW IN MARKDOWN
 
_______________________________________________________________________
-----------------------------------------------------------------------
Total number of commits: 14

<details>
<summary>Summary for commit 1 (56fdebc063e1e744e27f4677da34e80a261bd891)</summary>

The recent commit, authored by Louis Beaumont, introduces a new feature to the TypeScript file located at `examples/typescript/pipe-post-questions-on-reddit/pipe.ts`. Specifically, the code now sends a desktop notification when reddit questions are sent. This addition involves calling the `pipe.sendDesktopNotification` function with a title and body indicating that reddit questions have been sent. The change is located around line 328 in the file.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 2 (470fe2f39b555eeedaf2219101dbb1b4be94ba06)</summary>

The commit introduces new functionality to integrate the "inbox" feature with Reddit in the TypeScript example for posting questions on Reddit. The key changes in the code include:

1. **Module Import Updates**: 
   - Changed the way modules are imported from "screenpipe" by importing `ContentItem` and `pipe` instead of separately importing `queryScreenpipe` and `loadPipeConfig`.

2. **Function Updates**: 
   - Replaced calls to `loadPipeConfig()` and `queryScreenpipe()` with `pipe.loadPipeConfig()` and `pipe.queryScreenpipe()`, respectively, indicating a possible change in the module API to encapsulate functions under the `pipe` namespace.
   - Modified the `saveDailyLog` function to be called without `await`, possibly indicating a transition from an asynchronous to a synchronous execution for this specific function, or it is now handled differently.

3. **New Feature**:
   - Added a new operation to send emails with Reddit questions by utilizing `pipe.inbox.send()` to dispatch an email with a "reddit questions" title and body, which is the core enhancement introduced in this commit. 

These updates augment the daily log pipeline with a mechanism to send summarized Reddit questions through an inbox service.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 3 (07d054a60b6275c08a269869201214464f45c89c)</summary>

In this commit, two main changes were made by Louis Beaumont:

1. **Documentation Update:**
   - Added a new issue template for creating bounties within the project's GitHub repository. This template is intended to guide users in specifying the details of a task or feature they want to see implemented, including:
     - The definition of done with a checklist for activities.
     - A proposed investment amount for completing the task.
     - The necessity and use case of the requested feature.
     - Any additional context or screenshots.

2. **Code Removal:**
   - The `log-viewer.tsx` component file was removed from the path `screenpipe-app-tauri/components/`. This file included a React component for viewing application logs. It used Tauri APIs to read and display log files and had logic for periodically refreshing the displayed logs. The component utilized a modal dialog to render the logs in a user interface.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 4 (91577d7a304333f468b2996aab1ebb93d0012f33)</summary>

This git commit represents a merge of a pull request (#488) into the main codebase. The commit was authored by Louis Beaumont on October 12, 2024. The description indicates that the pull request includes changes to the continuous integration (CI) pipeline. Specifically, it incorporates tests for the `loadPipeConfig` function and ensures that Deno tests are run as part of the pipeline.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 5 (d1eef3ea0bdf2cfa00161dfe56190710156ffe9c)</summary>

The recent commit by Wesley Matos focuses on enhancing the continuous integration (CI) workflow. Specifically, the changes involve enabling Deno tests across different operating systems (Linux, Windows, and macOS) in the GitHub Actions workflow file (`.github/workflows/ci.yml`).

Key changes include:
- Addition of the `denoland/setup-deno@v2` action to set up Deno with version `v2.x` on all platforms.
- Inclusion of a new job step in each OS-specific test job (`test-linux`, `test-windows`, `test-macos`) to run Deno tests located in the `./screenpipe-js` directory.
- Existing test steps have been renamed for better clarity, specifying when they are related to running cargo tests.

Overall, the commit augments the CI process by integrating Deno testing alongside existing cargo tests, ensuring comprehensive testing coverage for both Rust and Deno environments.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 6 (f2c4c188824ab7f57231e379abd40715ad14985c)</summary>

The commit adds a new test file `loadPipeConfig.test.ts` in the `screenpipe-js/tests` directory to test the functionality of the `loadPipeConfig` function. The tests verify several scenarios:

1. **Test for Missing `pipe.json`**:
   - A test checks that if the `pipe.json` file is not found, `loadPipeConfig` returns an empty object. It ensures this by setting the `SCREENPIPE_DIR` and `PIPE_ID` environment variables to random UUIDs, which should not point to a valid `pipe.json`.

2. **Setup Function for Test Environment**:
   - A helper function `setupPipeDir` is created to assist in setting up a temporary environment where a `pipe.json` can be written to a specified path, derived from environment variables. It uses the `process.env` to set environment variables and writes a JSON configuration file at the constructed path.

3. **Test for Loading Configuration**:
   - A test confirms that `loadPipeConfig` correctly loads the configuration from the `pipe.json` stored in the directory specified by `SCREENPIPE_DIR` and `PIPE_ID`. The test uses random values for `interval`, `summaryFrequency`, and `emailAddress`.

4. **Test for Default Values**:
   - Another test verifies that if certain fields are not provided in `pipe.json`, `loadPipeConfig` uses default values. In this test, `summaryFrequency` is given a default of 5 if not explicitly specified.

Each test uses the `assertEquals` function from a standard testing library to ensure the loaded configuration matches the expected values.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 7 (3e05266fb51f38a7f373f576bbfa13f7029d69b7)</summary>

This commit refactors the `main.ts` file in the `screenpipe-js` directory by introducing a new TypeScript interface called `ParsedConfig`. The `ParsedConfig` interface is used to type the structure of parsed configuration data, where each field has a `name`, a potential `value`, and a `default` value. The `loadPipeConfig` function is updated to use this new `ParsedConfig` type, improving type safety by replacing the use of `any` with the more specific `ParsedConfig` type for the `parsedConfig` variable.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 8 (43357b355dc72d49ce425dbe5bae5a045eb7a580)</summary>

The commit `43357b355dc72d49ce425dbe5bae5a045eb7a580` is a merge commit by Louis Beaumont that incorporates changes from a pull request (#482) into the main branch. The main focus of this pull request was to update a Homebrew formula specifically for the x86_64-apple-darwin architecture.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 9 (ba3dd72eeea214b919e1b9b4ee7050ca35c32266)</summary>

The git changes involve a merge commit by the author Louis Beaumont. This commit merges changes from the 'main' branch into a feature branch related to updating the formula for `screenpipe` on the `x86_64-apple-darwin` platform. 

In the file `Formula/screenpipe.rb`, the SHA256 checksums for the downloadable URLs for different macOS architectures have been updated:

- For ARM64 architecture, the SHA256 checksum has been changed from `3b4a1012341b60806eb403d8c5c36365540dc80b53ef595226f2d73689debf88` to `ea228ce5a7c6402ea8ca93cb96b3682c73bb68eb0eec5aa866b9c8c1a35ca4e6`.
- For x86_64 architecture, the SHA256 checksum has been changed from `23add0dbfd536f0904ed6973d03b585a8bc1cf540fe78c1055900587c5c7e213` to `045fbeeb985efa9b64ca938734d766b20a84c0cc34e9b6b55b19d6c168d7b07e`.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 10 (9cf65c75d8ac10ea7938740ed81e79b6755f0ad5)</summary>

This Git change represents a merge commit by Louis Beaumont, combining changes from pull request #481 into the main branch. The purpose of this merge is to update a Homebrew formula specifically for the `aarch64-apple-darwin` architecture.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 11 (e6a89ab04b8cd647eb9cc89069718fc9eb2d31e7)</summary>

This commit updates the `screenpipe` Homebrew formula to version 0.1.95 for the `x86_64-apple-darwin` architecture. The change involves updating the version number from 0.1.90 to 0.1.95, along with the associated `sha256` checksum for the x86_64 release. This ensures that users who install this formula will get the correct updated version and verifiable checksum for the x86_64 architecture. The update is managed by the GitHub Actions Bot.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 12 (f5df7935d7b254fa46a8a9fad0e572b2f4ff9a1f)</summary>

This commit updates the version of the `screenpipe` formula in a Homebrew package. Specifically, it changes the version from 0.1.90 to 0.1.95 for the `aarch64-apple-darwin` architecture. The URL for downloading the tarball has been updated to reflect this new version, and the SHA-256 checksum for the `arm64` version is also updated to match the new release. These changes ensure that users downloading the `screenpipe` package via Homebrew will receive the updated version.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 13 (8a2ad82d76ee8e3d541f60ca3c290f6f4d3f3fa9)</summary>

The git commit with ID `8a2ad82` by Louis Beaumont on October 11, 2024, introduces a fix. Specifically, it adds a new dependency to the `screenpipe` Homebrew formula. The `deno` package is now listed as a dependency alongside the existing `ffmpeg` dependency.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 14 (315f2daa35161e288b4caed73379cda2e4ad823e)</summary>

The Git commit introduces several notable changes across different files and directories of the project:

1. **Logger Update and File Modification**:
    - The logging mechanism has been revamped to handle daily log files, enhancing how log files are named using the current date to avoid overwrites and ensure clarity.
    - Deprecated a custom log module and instead utilized the `tracing-appender` crate for appending log messages to files with a rolling file appender mechanism.

2. **Version Bumps**:
    - The version numbers in `Cargo.toml` for both the main workspace and the `screenpipe-app` have been incremented, indicating updates and new features.

3. **Workflow File Update**:
    - The workflow configuration in `.github/workflows/perf-long-running-end-to-end.yml` was modified to point to the new log file format which includes the date in its filename.

4. **Inbox Message Handling Enhancements**:
    - Refactored the handling of inbox messages in the Tauri components:
        - Introduced asynchronous handling of message retrieval and manipulation using `localForage` to store and persist messages locally.
        - Changed how messages are read, deleted, and displayed, aiming for a more efficient use of React hooks and asynchronous operations.

5. **Log Viewer and Platform Specific Paths**:
    - The `LogViewer` component was updated to compute platform-specific paths and dynamically resolve them to handle logs correctly across different operating systems.
    - Removed the deprecated use of the `watch` API in favor of periodic polling to check for log updates.

6. **Removal of Deprecated Code**:
    - Removed obsolete log management code (`MultiWriter`, `SingleFileRollingWriter`) from the `screenpipe-server`'s source, simplifying the codebase and adopting `tracing-appender` for these functionalities.
    - Cleaned up unnecessary dependencies and unused imports to enhance efficiency and readability.

These changes collectively aim to enhance logging, update versions to reflect changes, improve message handling, ensure platform compatibility, and remove outdated code for better maintenance and performance.
</details>

------------------------------------------------------------------------

# Overall Summary of Changes

The provided summary captures several recent git changes authored by Louis Beaumont and other contributors. Here's a concise breakdown of the key updates:

1. **TypeScript Example Enhancements**:
   - A new feature in the TypeScript example for Reddit integrations includes sending desktop notifications and emails when Reddit questions are dispatched. This involves a new `sendDesktopNotification` function and integration with an inbox service using `pipe.inbox.send()`.

2. **Module and Function Adjustments**:
   - The code structure for importing modules was revised to use a unified `pipe` namespace instead of separate function calls, reflecting a potential API update for better encapsulation.

3. **Documentation and Code Removal**:
   - Added an issue template for GitHub to help users specify tasks for bounty creation. Additionally, the `log-viewer.tsx` component was removed, previously used to display logs in a Tauri application.

4. **CI/CD Pipeline Enhancements**:
   - Enhanced the CI workflow by incorporating Deno tests across multiple operating systems. This includes using `denoland/setup-deno@v2` in the GitHub Actions workflow and adding Deno tests alongside existing cargo tests. Updates to test files to ensure Deno testing coverage.

5. **Comprehensive Testing and Refactoring**:
   - Added tests for the `loadPipeConfig` function in a new test file and refactored the `main.ts` with a `ParsedConfig` interface to improve type safety.

6. **Homebrew Formula Updates**:
   - Several merge commits focused on updating the Homebrew formula's version and checksums for different macOS architectures (`aarch64` and `x86_64`), ensuring users receive the latest version and secure downloads.

7. **Dependency Management**:
   - Introduced the `deno` package as a new dependency in the `screenpipe` Homebrew formula alongside `ffmpeg` to address build or runtime requirements.

8. **Logging and Infrastructure Improvements**:
   - Revamped the logging system by adopting the `tracing-appender` crate for better log file management, including daily log rotations and date-specific naming to prevent overwriting. This overhaul also included cleaning deprecated code and dependencies to streamline the project.

Overall, these changes focus on improving functionality, streamline testing and development workflows, and ensure that the project maintains updated and secure external dependencies.
