# Git Commit Summaries

-----------------------------------------------------------------------
-----------------------------------------------------------------------
 
PRESS CMD+SHIFT+V TO VIEW IN MARKDOWN
 
_______________________________________________________________________
-----------------------------------------------------------------------
Total number of commits: 22

<details>
<summary>Summary for commit 1 (2ba96f7a6a638ad008ba4dd719d81843e5eca007)</summary>

The recent commit by Louis Beaumont involves a change to the `onboarding.tsx` component in the `screenpipe-app-tauri` project. Specifically, an entire block of code dealing with the `experimentalFeatures` slide has been removed. This block previously rendered an `OnboardingExperimentalFeatures` component when `currentSlide` equaled `"experimentalFeatures"`. The removed code handled transitions and navigation between slides using the `handleEnd` and `handlePrevSlide` functions and adjusted opacity based on the `isVisible` state. The commit is tagged as a build fix, indicating that the removal may have been necessary to resolve build issues.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 2 (5326da8a2c082fbb600af1a8513f72f907c4896e)</summary>

The commit, identified by hash `5326da8a2c082fbb600af1a8513f72f907c4896e`, made by Louis Beaumont on October 13, 2024, involves the following changes to the `plugins.mdx` file in the documentation:

1. **Removed a Section**: A section detailing steps for "running a pipe" using `screenpipe` was removed. This section included instructions to download, enable, and run a pipe that streams text to local markdown files.

2. **Updated Examples Table**:
   - Added a new example entry: "pipe post questions on reddit," which posts questions on Reddit based on screen activity.
   - Added another new example entry: "pipe llama32 comment linear while you work," which comments on linear tasks while working using llama32.
   - Removed several existing example entries including:
     - "pipe screen to crm"
     - "pipe stream ocr text"
     - "pipe activity topic tracker"
     - "pipe focus notification buddy"
     - "pipe security check"
     - "pipe sync meetings to notion"
     - "pipe tagging activity"

3. **Minor Edit**: Ensured a newline at the end of the file.

Overall, the commit simplifies the content by removing sections and examples, and it adds new examples related to Reddit and linear task comments.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 3 (d0d0ba3ebb99731ae0677009ba745a6b8bd97e11)</summary>

The git changes in commit `d0d0ba3ebb99731ae0677009ba745a6b8bd97e11` represent a variety of updates and enhancements. Key changes include:

1. **Documentation Updates**: 
   - Minor language adjustments and indentation fixes in `plugins.mdx` for better readability.
   - Changed example pipe in the documentation from `pipe-email-daily-log` to `pipe-post-questions-on-reddit`.
   - Added a detailed example of a `pipe.json` configuration file, illustrating how to specify configuration options for a Screenpipe plugin.

2. **New Features and Configuration**:
   - Introduced configuration handling using `pipe.json`, allowing for dynamic input fields for plugins.
   - Integrated additional environment variable requirements for running scripts with Deno.
   - Emphasized the use of Deno for running pipes within Rust, providing guidance on its setup.

3. **Onboarding and UI Improvements**:
   - Removed the onboarding slide for experimental features and adjusted slide flow.
   - Simplified onboarding text, improving the focus on ease of use and essential features.
   - Added new UI elements like `RainbowButton` for a more modern visual experience.
   - Enhanced styles in `globals.css` with new utility CSS variables for a rainbow effect in buttons.

4. **Code and Style Refinements**:
   - Streamlined and refactored onboarding components for consistency and maintainability.
   - Removed imports and components associated with experimental features.
   - Tweaked the styling in `dev-or-non-dev.tsx`, `api-setup.tsx`, and other onboarding components for a cleaner look.
   - Updated `tailwind.config.ts` to include animations for a rainbow effect and other style enhancements.

5. **Package Version Bump**:
   - The `Cargo.toml` version was incremented from `0.3.8` to `0.3.9`, indicating a minor update with new features or improvements.

Overall, the changes improve documentation clarity, enhance user experience with refined onboarding, and integrate new features for customization and extensibility.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 4 (e05a5dc938ceac106f31d053f6ff27c3b8be9f1b)</summary>

This commit is a merge made by Louis Beaumont. It merges changes from a pull request #491, which was submitted by the user Neptune650. The purpose of the pull request was to fix issues with the continuous integration (CI) setup, labeled as "Fix CI (again)".
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 5 (bc07c7b2a4c2abd7a83b80c8862d25185751c87b)</summary>

The commit with hash `bc07c7b2a4c2abd7a83b80c8862d25185751c87b` by the author Neptune eliminates unused imports from a test file within the `screenpipe-core` project. Specifically, in the `pipes_test.rs` file, the imports `File` and `AsyncWriteExt` from the `tokio::fs` and `tokio::io` modules, respectively, were removed. The import of `create_dir_all` from `tokio::fs` is retained.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 6 (46b37047892f3adcc6d67098a11ec47afc7df7a0)</summary>

The commit `46b3704` by Louis Beaumont updates the `README.md` file for the project located at `examples/typescript/pipe-post-questions-on-reddit/`. The specific changes include:

1. The addition of an image showing a screenshot relevant to the project. This image is included with an HTML `<img>` tag, demonstrating how the feature looks or operates.

2. A minor content update to the description of the project, where a new line is added to highlight the benefits of using the tool: "Easily grow your followers, market your product, or be useful."

Overall, these changes aim to improve the README by providing a visual aid and emphasizing the potential advantages of the project.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 7 (c55f3237ad0ea6934549d9016d8d28a07fea39fc)</summary>

The commit `c55f3237ad0ea6934549d9016d8d28a07fea39fc` by the author Neptune includes modifications to the `pipes_test.rs` file located in the `screenpipe-core/tests` directory. The following changes were made:

1. **Change in Directory Path Construction**:
   - The `pipe_dir` variable now joins the `pipe_name` with a subdirectory named `"pipes"`, thus changing from:
     ```rust
     let pipe_dir = temp_dir.path().join(pipe_name);
     ```
     to:
     ```rust
     let pipe_dir = temp_dir.path().join("pipes").join(pipe_name);
     ```

2. **Modification in Pipe Execution Call**:
   - The call to `run_pipe` has been altered to use a static string `"config_pipe"` as the first argument, changing from:
     ```rust
     let result = run_pipe(&pipe_dir.to_string_lossy().to_string(), screenpipe_dir).await;
     ```
     to:
     ```rust
     let result = run_pipe("config_pipe", screenpipe_dir).await;
     ```

These changes seem to be aimed at fixing the test related to pipe execution, perhaps by ensuring the tests are using the correct directory structure and configuration.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 8 (dc76c9144357a7e78d24a5e86757865e6c12c7f2)</summary>

The commit made by Louis Beaumont includes two main changes aimed at improving the formatting of Reddit posts and handling link elements in a TypeScript and React-based application.

1. **Formatting Reddit Links in Markdown:**
   - In the TypeScript file located at `examples/typescript/pipe-post-questions-on-reddit/pipe.ts`, the format for generating Reddit links has been updated to adhere more closely to Markdown syntax.
   - Titles of Reddit posts are now formatted as Markdown headings (`###`), and subreddit links use Markdown link syntax (`[SEND](url)`) instead of HTML `<a>` elements.

2. **Enhancements in External Link Handling:**
   - In the React component `inbox-messages.tsx` located in `screenpipe-app-tauri/components`, there are enhancements in how anchor elements are rendered.
   - The component now conditionally sets `target="_blank"` and `rel="noopener noreferrer"` attributes for external links (URLs starting with `http`/`https`) to ensure safe opening of links in new tabs.
   - Additionally, all links have been styled with a `text-blue-500 hover:underline` class for better visual consistency and user interaction feedback.

These changes improve the application's adherence to Markdown standards and enhance the handling of external links for better security and user experience.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 9 (56fdebc063e1e744e27f4677da34e80a261bd891)</summary>

The commit identified by `56fdebc063e1e744e27f4677da34e80a261bd891` introduces a new feature to the TypeScript example script `pipe.ts`, which is part of the project for posting questions on Reddit. A new functionality has been added to send a desktop notification with the title "reddit questions" and the message "just sent you some reddit questions" immediately after sending an email with Reddit questions. This enhancement aims to provide users with a direct and timely notification on their desktops whenever Reddit questions are sent out.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 10 (470fe2f39b555eeedaf2219101dbb1b4be94ba06)</summary>

This commit introduces a new feature that adds the ability to send Reddit questions to an inbox. The changes are made within a TypeScript file (`pipe.ts`) in a specific example directory (`examples/typescript/pipe-post-questions-on-reddit`). Major changes include:

1. **Imports Adjustment**: The import statement is modified to import `pipe` from the `screenpipe` package. This change consolidates functionality accessed from `screenpipe`.

2. **Configuration Loading**: The method to load pipe configurations switches from `loadPipeConfig()` to `pipe.loadPipeConfig()`.

3. **Screenpipe Queries**: There are adjustments to how `queryScreenpipe` is called, integrating it as `pipe.queryScreenpipe()`.

4. **Email Sending**: Introduces a new call to `pipe.inbox.send()` to send Reddit questions. This additional feature sends the collected Reddit questions to an inbox with a title and body containing the questions.

5. **Code Clean-Up**: Removal of unnecessary `await` in some scenarios, such as `saveDailyLog(logEntry)`, suggesting that this operation does not need to be awaited.

Overall, the update enhances the functionality of the script by integrating inbox sending capabilities for Reddit-related content, and makes better use of the `pipe` module for configuration and data querying.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 11 (07d054a60b6275c08a269869201214464f45c89c)</summary>

The commit contains two main changes:

1. **Documentation Update:** A new GitHub issue template for "bounty" has been added. This template includes sections for defining the requirements for a bounty, specifying the budget, explaining the need for the bounty, and any additional context. The purpose is to outline precise tasks that require funding and to ensure clarity for awarding the bounty.

2. **Code Deletion:** The `log-viewer.tsx` component used in a Tauri application (`screenpipe-app-tauri`) has been deleted. This component was responsible for displaying application logs in a dialog interface, with functionalities like loading log files based on the current date, handling errors, and updating the logs at regular intervals.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 12 (91577d7a304333f468b2996aab1ebb93d0012f33)</summary>

The commit is a merge from pull request #488 by user "ologbonowiwi". The merge was performed by Louis Beaumont. The changes introduce modifications to the continuous integration (CI) pipeline: they include tests for `loadPipeConfig` and add steps to run Deno tests as part of the pipeline.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 13 (d1eef3ea0bdf2cfa00161dfe56190710156ffe9c)</summary>

The git commit `d1eef3ea0bdf2cfa00161dfe56190710156ffe9c` introduces changes to the GitHub Actions workflow configuration (`.github/workflows/ci.yml`) for a CI (continuous integration) setup. The main update is the addition of Deno tests across different operating systems (Linux, Windows, and macOS) in the CI process.

Key changes include:

1. **Setup Deno**: The workflow now uses the `denoland/setup-deno@v2` action to install Deno (version 2.x) on all platforms (Linux, Windows, macOS) as part of the CI setup steps.

2. **Run Deno Tests**: After setting up the environment and before running the existing Cargo tests, new steps have been added to execute Deno tests. These steps:
   - Run the command `deno test --allow-env --allow-write --allow-read`
   - Are executed within the `screenpipe-js` directory.

3. **Naming Adjustments**: Existing test job names have been slightly revised to clarify which tests are being run (e.g., added "cargo" to differentiate from new Deno tests).

Overall, the changes aim to incorporate Deno testing into the existing CI workflow to ensure broader test coverage and integration with Deno-based components.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 14 (f2c4c188824ab7f57231e379abd40715ad14985c)</summary>

The commit `f2c4c188824ab7f57231e379abd40715ad14985c` authored by Wesley Matos adds a new test file named `loadPipeConfig.test.ts` to the `screenpipe-js/tests` directory. This file consists of unit tests for the `loadPipeConfig` function, which is likely responsible for loading configuration data for a hypothetical "pipe" functionality. 

In summary, the tests include:

1. A test to ensure that `loadPipeConfig` returns an empty object when the configuration file `pipe.json` is not found. It does this by setting environment variables `SCREENPIPE_DIR` and `PIPE_ID` to random UUIDs, ensuring they point to non-existent paths.

2. Another test to verify that the configuration can be successfully loaded from a JSON file located at the path specified by the environment variables `SCREENPIPE_DIR` and `PIPE_ID`. The setup involves creating a temporary directory and file structure, populating it with a JSON representation of the configuration.

3. A test to check that `loadPipeConfig` returns default values when they are not provided in the configuration file. This involves similar setup steps but with some fields having default values instead of explicitly provided ones.

The tests use `Deno.test` for structuring test cases and include utility functions to set up the environment and generate random configuration data for testing.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 15 (3e05266fb51f38a7f373f576bbfa13f7029d69b7)</summary>

The git commit titled "refactor: include `ParsedConfig` type" involves changes in a TypeScript file (`main.ts`) of the `screenpipe-js` project. The key changes include:

1. A new TypeScript interface called `ParsedConfig` has been introduced. This interface is generic (with a default type of `unknown`) and defines a structure for configurations with fields that have names, and optional values and default values.

2. The function `loadPipeConfig` is updated to use the new `ParsedConfig` type. Previously, the `parsedConfig` variable was inferred as any type, but now it is explicitly typed as `ParsedConfig`. This adds clarity and type safety to the code.

3. The loop within `loadPipeConfig` that processes fields from the parsed configuration no longer specifies `any` for the field type, taking advantage of the type definition provided by `ParsedConfig`.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 16 (43357b355dc72d49ce425dbe5bae5a045eb7a580)</summary>

This commit represents a merge operation conducted by Louis Beaumont. The merge incorporates changes from a pull request (#482) that updates a Homebrew formula specifically for the `x86_64-apple-darwin` architecture. The update is identified by the specific commit hash `315f2daa35161e288b4caed73379cda2e4ad823e` in the source repository.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 17 (ba3dd72eeea214b919e1b9b4ee7050ca35c32266)</summary>

The git commit represents a merge into the branch `update-formula-x86_64-apple-darwin-315f2daa35161e288b4caed73379cda2e4ad823e` from the `main` branch. It involves changes to the file `Formula/screenpipe.rb`, specifically altering the SHA256 hash values for downloaded tar.gz files.

The SHA256 hash for the ARM64 (Apple Silicon) version of the `screenpipe` package has been updated from `3b4a1012341b60806eb403d8c5c36365540dc80b53ef595226f2d73689debf88` to `ea228ce5a7c6402ea8ca93cb96b3682c73bb68eb0eec5aa866b9c8c1a35ca4e6`.

Similarly, the SHA256 hash for the x86_64 (Intel) version has been updated from `23add0dbfd536f0904ed6973d03b585a8bc1cf540fe78c1055900587c5c7e213` to `045fbeeb985efa9b64ca938734d766b20a84c0cc34e9b6b55b19d6c168d7b07e`. 

These changes likely reflect updates or re-releases of the binaries for these architectures.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 18 (9cf65c75d8ac10ea7938740ed81e79b6755f0ad5)</summary>

The git changes indicate a merge commit with hash `9cf65c7`, authored by Louis Beaumont on October 11, 2024. This commit merges pull request #481 into the main branch, which updates the Homebrew formula specifically for the `aarch64-apple-darwin` architecture. The pull request was originally identified by its commit hash `315f2daa35161e288b4caed73379cda2e4ad823e`.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 19 (e6a89ab04b8cd647eb9cc89069718fc9eb2d31e7)</summary>

The commit updates the homebrew formula `screenpipe.rb` for the `screenpipe` library. Specifically, the version is updated from 0.1.90 to 0.1.95 for the x86_64 architecture on macOS. Consequently, the SHA256 checksum for the x86_64 binary has been updated to reflect the new version. This change ensures that the homebrew formula correctly fetches and verifies the new version of the `screenpipe` library.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 20 (f5df7935d7b254fa46a8a9fad0e572b2f4ff9a1f)</summary>

The commit updates the Homebrew formula for the `screenpipe` library to version 0.1.95, specifically for the `aarch64-apple-darwin` platform. The URL in the formula is updated to point to the new version's tarball, and the `sha256` checksum is also updated to correspond to the new version file. This change ensures that users will download the correct version and verify its integrity using the updated checksum.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 21 (8a2ad82d76ee8e3d541f60ca3c290f6f4d3f3fa9)</summary>

The commit with hash `8a2ad82d76ee8e3d541f60ca3c290f6f4d3f3fa9` by Louis Beaumont on October 11, 2024, adds a new dependency to the Homebrew formula for `screenpipe`. Specifically, the formula located in `Formula/screenpipe.rb` has been modified to include `deno` as a dependency, in addition to the existing `ffmpeg` dependency.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 22 (315f2daa35161e288b4caed73379cda2e4ad823e)</summary>

This commit includes a variety of changes primarily focused on improving the logging and message handling features in the project. Here’s a summary of the key updates:

1. **GitHub Workflow Changes**: 
   - The GitHub Actions workflow file `.github/workflows/perf-long-running-end-to-end.yml` is updated to log specific log files based on the current date, improving log management by segregating logs by days.

2. **Rust Project Changes**: 
   - The version of the Cargo.toml in the main Rust project (`Cargo.toml`) and the Tauri application (`screenpipe-app-tauri/src-tauri/Cargo.toml`) has been incremented, indicating a new release.
   - Added `tracing-appender` dependency to enable logging to rotating file appenders, which is further configured for daily rotation and a maximum of 5 log files retained.

3. **Logging**: 
   - In `screenpipe-server/src/bin/screenpipe-server.rs`, logging setup is refactored to use `tracing_appender` for rotating log files. Old custom logging module (`logs.rs`) and related code have been entirely removed.
   - Logs are now written to daily rotating files, thus ensuring better log management and reducing manual interventions for log file maintenance.

4. **Frontend Changes**: 
   - In the `screenpipe-app-tauri` components, message handling and storage logic are improved using the `localforage` library in JavaScript/TypeScript. Messages are saved to and retrieved from local storage, ensuring persistence.
   - Updated message management logic to mark messages as read and delete them asynchronously, allowing for smoother user interactions and data consistency with local storage.

5. **Log File Naming**: 
   - Changes in file paths to include current date in log file names across multiple components, enhancing organization and retrieval of log files.

6. **Miscellaneous**: 
   - Improved the `LogViewer` component in Tauri app by tweaking how log files are read and displayed using platform-specific paths, and handling errors more gracefully.

These changes not only fix the broken logger but also introduce enhancements across both the client-side and server-side components for better reliability and maintainability.
</details>

------------------------------------------------------------------------

# Overall Summary of Changes

The recent git changes encompass a variety of updates focused on enhancing documentation, refactoring code, improving the user experience, and maintaining project infrastructure. Here's a streamlined summary of the key modifications:

1. **Code and UI Enhancements**:
   - Removed the `experimentalFeatures` slide from the `onboarding.tsx` component to resolve build issues.
   - Introduced new UI elements like `RainbowButton` for a modern visual experience and updated global styles to support these elements.

2. **Documentation Updates**:
   - Simplified and updated the content of `plugins.mdx` by removing certain sections and examples while adding new ones to highlight current capabilities, such as posting on Reddit.
   - Updated `README.md` files with screenshots and minor content adjustments, focusing on usability and effectiveness.

3. **Testing and CI Pipeline Improvements**:
   - Enhanced continuous integration (CI) by incorporating Deno tests to cover more ground and ensure the stability of Deno-based components.
   - Modified GitHub Actions workflows to log files efficiently and added more comprehensive test steps.

4. **Homebrew Formula Updates**:
   - Updated the Homebrew formula for the `screenpipe` library, reflecting changes in version and corresponding checksums for different architectures to ensure users download the correct versions.

5. **Log Management and Messaging**:
   - Improved logging capabilities using `tracing-appender` for rotating log files based on the current date, making log management more efficient.
   - Enhanced message handling in the Tauri app, using `localforage` for storage, ensuring messages are managed asynchronously and persistently.

6. **Package Updates**:
   - Version bump in `Cargo.toml` files to indicate the incorporation of new features and improvements, aligning with the project’s progress.

In summary, these commits reflect a comprehensive approach toward refining functionality, enhancing usability, and maintaining the project infrastructure effectively.
