# Git Commit Summaries

-----------------------------------------------------------------------
-----------------------------------------------------------------------
 
PRESS CMD+SHIFT+V TO VIEW IN MARKDOWN
 
_______________________________________________________________________
-----------------------------------------------------------------------
Total number of commits: 90

<details>
<summary>Summary for commit 1 (c3326e2738c8607b744068593af57f9cc7344beb)</summary>

The commit with hash `c3326e2738c8607b744068593af57f9cc7344beb` contains two main changes:

1. **Workflow Changes**: In the GitHub Actions workflow file `.github/workflows/release-app.yml`, there are modifications related to the build process:
   - The `V8_FLAGS` environment variable configuration, which previously set `--max-old-space-size=4096`, has been commented out.
   - The `cargo build --release` command has been commented out and replaced with a non-release `cargo build`, indicating a shift to debug builds potentially related to an existing issue with Deno Core (`denoland/deno_core/issues/916`).

2. **Version Update**: In the `Cargo.toml` file located at `screenpipe-app-tauri/src-tauri/`, the package version has been changed from `0.2.90` to `0.2.90-beta`, suggesting a move to a beta release for testing or development purposes.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 2 (dda7147cbb867f03077125383b24ad5058be7fd9)</summary>

The recent commit by Louis Beaumont, made on October 3, 2024, updates the `README.md` file. The change involves updating an image that depicts the GitHub star history. Specifically, the image has changed from an asset with an ID of `91cdc7a6-19ea-4156-a7bf-a2bef4cc946f` to a new asset with an ID of `5d5c9672-d2d3-4e4c-8734-a7e0c2fee246`.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 3 (1762d31c55730df36abb94c9fba82d6c2c1dd8d9)</summary>

The commit made by Louis Beaumont on October 3, 2024, updates the `README.md` file. The specific change updates one of the latest news entries by correcting the number of users running "screenpipe" 24/7 from 70 to 150.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 4 (46a2c1f8737bd6f85ad0d5b9bf79a0bca08edc96)</summary>

The commit by Louis Beaumont includes two main changes:

1. **GitHub Workflow Update**: In the `.github/workflows/release-app.yml` file, the `RUSTFLAGS` environment variable for MacOS builds was updated. The `-C target-cpu=native` flag was removed. The line now focuses on setting linker arguments related to runtime paths for dynamic libraries without specifying the target CPU as native.

2. **Pre-build Script Enhancement**: In the `screenpipe-app-tauri/scripts/pre_build.js` file, error handling has been added around the setup process for ONNX Runtime libraries on Windows. The original code for downloading and extracting the libraries was wrapped in a `try` block with a corresponding `catch`. In the event of an error during download or extraction, an error message is logged, and a placeholder comment suggests that an alternative download method should be added. This enhances the robustness of the script by providing a mechanism to handle failures gracefully.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 5 (518d7cf4dc30f8b41dc1138322af78376873a57c)</summary>

The commit with the hash `518d7cf4dc30f8b41dc1138322af78376873a57c` made documentation updates in the `getting-started.mdx` file. The author, Louis Beaumont, added comments to clarify the importance of running the `bun scripts/pre_build.js` command. Specifically, the comments indicate that this step is critical because it copies the CLI into the app before building with Tauri. This additional information has been inserted in three separate instances within the file, helping users understand the necessity of this step in the process.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 6 (95ee02098f1c9279acb7bd9c4df5d774c65bf33b)</summary>

The commit `95ee02098f1c9279acb7bd9c4df5d774c65bf33b` by Louis Beaumont includes the following changes:

1. In the GitHub Actions workflow file `.github/workflows/release-app.yml`, a typo in an environment variable assignment was fixed. The `V8_FLAGS` assignment originally had a colon (`:`) after `V8_FLAGS` which was corrected to an equals sign (`=`).

2. In the `Cargo.toml` file located at `screenpipe-app-tauri/src-tauri/`, the version of the dependency `tauri-plugin-updater` was updated from `2.0.0` to `2.0.1`.

These changes collectively address issue #375.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 7 (e07b438ed87f08b97daa4982a467d2a05ed153d5)</summary>

The commit `e07b438ed87f08b97daa4982a467d2a05ed153d5` by Louis Beaumont updates the `screenpipe-app` version from `0.2.89` to `0.2.90` in the `Cargo.toml` file of the Tauri application. The commit is associated with fixing issue #375.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 8 (98899f0d3045423ac9c0de00d26aa279bb73fd0b)</summary>

The commit made by Louis Beaumont addresses issue #375. It includes the following changes:

1. **Workflow File `.github/workflows/release-app.yml`:**
   - The `RUSTFLAGS` environment variable was updated. The compiler flag `-C target-cpu=native` was added to optimize Rust builds to use native CPU optimizations.
   - The `V8_FLAGS` setting now uses `export` to ensure it's applied as an environment variable.

2. **TypeScript React Component `screenpipe-app-tauri/components/pipe-store.tsx`:**
   - Removed a `disabled` property from a `Button` component that was disabling it based on the condition where `health?.status` equaled "error".

These changes likely aim to optimize performance and possibly fix or adjust the behavior of a UI component.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 9 (d61e005f8be84eac99703d4baaad333abfb36c29)</summary>

The commit `d61e005f8be84eac99703d4baaad333abfb36c29` by Louis Beaumont addresses issue #375. The changes include:

1. Modifying the GitHub Actions workflow file (`release-app.yml`):
   - A new environment variable `V8_FLAGS` is set to `"--max-old-space-size=4096"`, referencing a linked issue (denoland/deno_core#916) for further context.

2. Updating the `Cargo.toml` file in the `screenpipe-app-tauri` directory:
   - The version of the package `screenpipe-app` is incremented from `0.2.88` to `0.2.89`. 

These changes aim to fix the identified issue while also updating the package version to reflect the modifications.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 10 (75c421f15bfb845af3614afe6a7d9183f46a6746)</summary>

The commit with ID `75c421f15bfb845af3614afe6a7d9183f46a6746` by author Louis Beaumont on October 2, 2024, contains a fix related to "tauri 2.0". The change involves deleting the file `function-call-message.tsx` from the `screenpipe-app-tauri/components` directory. This file was previously used to define a React component named `FunctionCallMessage`. The component, along with its supporting functions (such as `MarkdownContent` and `generateCurlCommand`) and imports, has been entirely removed. This file involved UI elements like badges, cards, icons, accordions, dialogs, and buttons, which were used to render and manage messages involving tool calls and their results. The deletion suggests a removal or refactoring of this component as part of the update or fix for "tauri 2.0".
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 11 (b8a48a93adc74e61101470ce1ca389f2fa34c26e)</summary>

The git changes in this commit involve a modification to the "screenpipe-app-tauri/app/page.tsx" file. The import statement for the `ChatList` component from "@/components/chat-list-openai-v2" has been removed. This change is part of a fix related to "tauri 2.0."
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 12 (7781e6e0aff6a307755808f079193e638d2fa9bc)</summary>

The commit contains several significant changes associated with Tauri 2.0 updates:

1. **Workspace Configuration:**
   - The workspace version in `Cargo.toml` file has been updated from "0.1.93" to "0.1.94".
   - Removed "screenpipe-app-tauri/src-tauri" from the workspace's member list and included it in the exclude list.

2. **Component Deletions:**
   - Removed three components: `chat-list-openai-v2.tsx`, `chat-list-web-llm.tsx`, and `meeting-summarized.tsx`. These were large files associated with chat-related functionalities.

3. **Settings Updates:**
   - In the `settings.tsx` file, added a hack to reload the window if the dialog is closed to ensure settings are updated.
   - Migrated the storage approach using `createStore` instead of directly using `Store`. This aligns with the update in the store plugin (possibly part of Tauri 2.0 updates).
   - Removed `ollama` related configurations suggesting possible shifts in AI configurations or reliance.

4. **Package Updates:**
   - Updated Tauri-related plugins and CLI dependencies from beta versions to stable versions "2.0.0".
   - Added "@opentelemetry/resources" to the project's dependencies.
   - Updated `@tauri-apps/api` from a beta version to "2.0.0".

5. **Configuration and Permissions:**
   - Updated various configuration files like `main.rs`, `sidecar.rs`, and schema files presumably tied to Tauri 2.0 updates.
   - Ensured permissions in ACL manifests reflect changes demanded by Tauri 2.0.

6. **Binary Files and Minor Changes:**
   - `bun.lockb` binary file is updated which might involve dependency-lock updates.
   - Minor refactoring and restructuring in the main source files, including module imports and comments for improved clarity.

These changes likely reflect upgrading the underlying technology or dependencies (from Tauri 1.x to Tauri 2.0) and code cleanup/deletions for improving the app performance or fixing issues related to the new version upgrade.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 13 (3ca0b91ae7e95aae4ce44d206c2f18724ca4a7f4)</summary>

The commit by Louis Beaumont adds GitHub templates to the repository. Specifically, it introduces two new files: 

1. **Bug Report Template (`.github/ISSUE_TEMPLATE/bug_report.md`)**: This file is a template for creating bug reports. It includes sections for describing the bug, steps to reproduce it, expected behavior, screenshots, and system information like OS, OS version, and application version. It also allows for adding additional context such as hardware specifications, relevant software versions, and error logs.

2. **Pull Request Template (`.github/pull_request_template.md`)**: This file is a template for submitting pull requests. It includes sections for providing a brief description of changes, linking related issues, specifying the type of change (bug fix, new feature, breaking change, or documentation update), and giving instructions on how to test the changes. It also includes a checklist for best practices and a section for additional notes.

These templates aim to standardize and streamline the processes for reporting issues and submitting pull requests, ensuring consistency and thoroughness.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 15 (01670086470873f63320878cc045bc8306b2a0ac)</summary>

The commit identified by `01670086470873f63320878cc045bc8306b2a0ac`, authored by Louis Beaumont, updates the version number of a package. Specifically, it modifies the `Cargo.toml` file for the "screenpipe-app" package, changing the version from "0.2.86" to "0.2.87". The commit message "bump" suggests this change is a version bump, likely indicating a minor update or patch.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 16 (7ebb47883ecb9f9b5018d639e1979d2bd08ceb1e)</summary>

The commit made by Louis Beaumont updates the documentation for setting up and using Screenpipe, particularly in the "getting-started.mdx" file. Key changes include:

1. **Reorganization of Installation Options:** 
   - The order of installation instructions has been switched; "Build from Source" is now Option 1 and "Homebrew Installation" is now Option 2.
   - Clarifications have been added for building the desktop app, including specific configurations for VSCode settings for Rust development.

2. **Addition of Detailed Instructions for Building the Desktop App:**
   - Provided a step-by-step guide, including setting up environment variables and using Bun to handle dependencies and build the app.

3. **Updates for Windows Installation:**
   - Simplified Windows setup instructions for dependencies, now recommending the use of "winget" for installing UnZip.
   - Additional commands for building the desktop app using Bun have been added.

4. **Enhancements for Nix-based Systems:**
   - The heading now specifies it's for installing the CLI using Nix.

5. **New Section for Business Use:**
   - Introduced options for using Screenpipe in business environments and included a link to book a demo.

6. **README Update in `screenpipe-app-tauri`:**
   - The README was simplified to direct users to the official online documentation for getting started instead of including detailed instructions.

These changes overall aim to streamline the setup process, provide clearer guidance for different installation methods, and support broader use cases, especially for business settings.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 17 (c8b150901512d53043f53d2e826f8fb8031c93c2)</summary>

This commit represents a merge operation performed by Louis Beaumont. It merges changes from pull request #392 into the main branch. The pull request was initiated by EzraEllette and addresses issue #386, which involves adding an update feature to a tray component.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 18 (eb156baa826a34e8c114c26132b7aef5323e4e5b)</summary>

The commit `eb156baa826a34e8c114c26132b7aef5323e4e5b` by Ezra Ellette adds a new entry to the `.gitignore` file to exclude `.env` files from being tracked in the Git repository. This change is likely made to ensure that environment variable files, which may contain sensitive information or configuration specific to local development or testing workflows, are not included in version control. Additionally, there is no newline at the end of the file after this change.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 19 (845dd0c3ae03f9e2bab6676d55845f344f700b6c)</summary>

This commit is a revert operation made by the author, Ezra Ellette. It undoes a previous commit (with ID `83111e28ec966b07044e56f514d077ce163a1f47`) that added the `.env` file to the `.gitignore` list. The change involves modifying the `.gitignore` file by removing the line that ignored `.env`, thus allowing `.env` files to be tracked by Git again.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 21 (e36b459595d3ca3ca8576ccbd3a5eb40ab9deca3)</summary>

The commit by Ezra Ellette adds the `.env` file to the `.gitignore` list, indicating that this file should be ignored by Git and not tracked in the repository. This is likely done to prevent sensitive information or environment-specific configurations from being shared.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 22 (5f3db8fa4b1681bb42f606adf76c5fade0dc631d)</summary>

The commit, authored by Louis Beaumont, contains a small user interface fix in the `pipe-store.tsx` file. The changes adjust nullish checks for the `selectedPipe.source` property when determining if certain UI elements should be displayed. Specifically, it adds optional chaining (`?.`) to safely check if `selectedPipe.source` starts with "https://" or "http", preventing potential runtime errors if `source` is null or undefined.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 23 (3052aadbd2c866696016e097a1ec5492c3d6bc8b)</summary>

The commit updates the README.md file with a minor change in the "Latest News" section. Specifically, it modifies the description of a news item from saying "screenpipe is top github repo & on hackernews!" to "screenpipe is number 1 github trending repo & on hackernews!".
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 24 (f03f4b784dc8b7979ba2271419c6995cfb34aa51)</summary>

The git changes made in this commit by Louis Beaumont include modifications to the file `settings.tsx` within the `screenpipe-app-tauri/components` directory. The changes involve adding additional information to a tooltip section. Specifically, a note has been added for Windows users indicating that they may need to run the `ollama` command with specific parameters. The following lines were added to the tooltip content:

- A line break (`<br />`).
- A note explaining that users on Windows might need to run `ollama` with a particular command.
- A code block showing the command: `OLLAMA_ORIGINS=* ollama run llama3.2`.

These additions are meant to assist users with specific instructions for configuration when running the software on Windows.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 25 (f92872eb145b2ec6ee0cb7c7de4bfbd600ccaa8a)</summary>

The commit updates the version number of the `screenpipe-app` package in the `Cargo.toml` file within the `screenpipe-app-tauri/src-tauri` directory. The version is bumped from `0.2.85` to `0.2.86`.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 26 (af041fb40c88d53b38075095c08bc89123d91b60)</summary>

The commit titled "fix build" involves several changes:

1. **Binary File Update**:
   - The `bun.lockb` file for the `screenpipe-app-tauri` project was updated. This file is a binary, so the exact changes are unspecified.

2. **Code Changes**:
   - In `next.config.mjs`, an export default statement for `nextConfig` was added, ensuring proper module export.

3. **Dependency Updates**:
   - The `package-lock.json` was updated, with new dependencies and updates mainly related to OpenTelemetry libraries. Notable additions include:
     - `@opentelemetry/instrumentation-fetch` and `@opentelemetry/instrumentation-xml-http-request` at version `0.53.0`.
     - Several sub-dependencies related to OpenTelemetry and additional packages such as `@types/shimmer`, `acorn-import-attributes`, `cjs-module-lexer`, `import-in-the-middle`, `module-details-from-path`, and `require-in-the-middle` were introduced.

4. **Minor Text Fixes**:
   - In both `desktop-schema.json` and `macOS-schema.json` within the `screenpipe-app-tauri/src-tauri/gen/schemas` directory, a correction was made to change the word "programmatic" to "programatic" for consistency in the description fields.

These changes seem to collectively address issues impacting the build process, likely ensuring that all necessary packages are correctly configured and exported within the project.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 27 (d1bbe7ddad2ab21d53894455ee3c84ad382a5db3)</summary>

The commit made by tribhuwan-kumar on October 1, 2024, addresses a video rendering issue specific to Windows in a project, as shown in the `screenpipe-server/src/video.rs` file. The fix involves changing the video encoder used when the application is running on a Windows system. Specifically, the encoder has been switched from "mpeg4" to "h264_mf" to presumably address compatibility or performance issues with the former encoder. An associated comment was also updated to reflect this change and a TODO note indicates an intention to switch back to `libx264` when FFmpeg is updated in a script named `pre_build.js`.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 28 (c3b62d7cd6e36da887ae74aefa53d54668e9736b)</summary>

The recent Git commit by Louis Beaumont contains the following changes:

1. **Removal of Sentry Integration:**
   - The `Sentry` imports have been removed from `chat-list-openai-v2.tsx` and `settings.tsx`.
   - A call to `Sentry.captureException(error)` has been removed from the `chat-list-openai-v2.tsx` file.

2. **Deleted Sentry Configuration File:**
   - The `sentry.config.ts` file has been deleted. This file previously contained the initialization setup for Sentry with a DSN (Data Source Name) for error tracking.

3. **Version Bump:**
   - The version of the package in `Cargo.toml` has been updated from `0.2.84` to `0.2.85`.

These changes primarily focus on removing the Sentry error tracking integration from the codebase and increasing the application version.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 29 (bf14dbbc1073b96eef5564a513a51b0e728b3e78)</summary>

The commit with hash `bf14dbbc1073b96eef5564a513a51b0e728b3e78` by Louis Beaumont on October 1, 2024, updates the `README.md` file. The change involves updating an image link in the "star history" section. The old link contained the asset ID `fd8968fb-e59c-4d81-a42d-059c94ce9c71`, which has been replaced with a new asset ID `91cdc7a6-19ea-4156-a7bf-a2bef4cc946f`.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 30 (e14b2ceccedb9b75c9556bf48d669ed509ce835d)</summary>

The recent commit by Glavin Wiechert adds a new step to the "Getting Started" guide for a project. The modification involves two main changes: 

1. Updating the installation command to include `cmake` and `wget` as additional dependencies. The updated command is:
   ```bash
   brew install pkg-config ffmpeg jq tesseract cmake wget
   ```

2. Adding a new step in the guide to instruct users on building the open-source desktop app. Users are directed to follow specific instructions in the README file for `screenpipe-app-tauri` on GitHub. 

These changes help users prepare their environment and provide guidance on how to proceed if they intend to build the desktop application.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 31 (c42cf79d053fa36dbd89e35e7d6b961e3dcc1710)</summary>

The commit `c42cf79d053fa36dbd89e35e7d6b961e3dcc1710` authored by Louis Beaumont on September 30, 2024, replaces the use of Sentry with OpenTelemetry across the Screenpipe application. Here are the key changes introduced:

1. **Error Handling:**
   - **JavaScript:** In `global-error.jsx`, Sentry's error capturing functionality is replaced with OpenTelemetry's APIs. Errors are captured using OpenTelemetry tracers and spans.
   - **React Components:** Removed Sentry initialization and error tracking in several components, such as `settings.tsx`, replacing them with OpenTelemetry logic or comments marking  telemetry states.

2. **Providers:**
   - In `layout.tsx`, the provider component `PHProvider` is renamed to `Providers`, integrating OpenTelemetry initialization.

3. **New Module:**
   - A new OpenTelemetry initialization module `opentelemetry.ts` is introduced to setup tracing and error tracking using OpenTelemetry within the client application.

4. **Package and Configuration Files:**
   - The `package-lock.json` and `package.json` files are modified to include new OpenTelemetry dependencies and remove Sentry-related packages.
   - In `next.config.mjs`, Sentry configuration is removed.

5. **Rust Backend:**
   - The Rust backend in the `Cargo.toml` file shows `sentry` dependency commented out and mentions of a new potential OpenTelemetry tracing setup prepared but commented out.
   - In `main.rs`, Sentry initialization for capturing errors and logging is removed, and traces show attempts to integrate OpenTelemetry, but it's also commented out.

6. **Version Update:**
   - The application version is updated from `0.2.83` to `0.2.84` in the Rust Cargo file.

These changes mainly center around replacing Sentry error tracking and tracing with OpenTelemetry, affecting client-side JavaScript/React components as well as Rust server components in a unified logging and monitoring strategy.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 32 (5c057c1c8581b4041efc7d0c1465b262f983868c)</summary>

The Git commit by BrianTeeman on September 30, 2024, addressed a series of spelling and typographical errors across multiple files in the project. Here is a summary of the key changes:

1. **Whitespace Adjustments**: Removed unnecessary whitespace and improved formatting in markdown documentation (`NOTES.md`).

2. **Text Corrections**:
   - Corrected "sould" to "should" in `NOTES.md`.
   - Fixed a misspelling of "asnwer" to "answer" in a JavaScript file related to Apple shortcuts.
   - Corrected "intructions" to "instructions" in a TypeScript file dealing with meeting summaries by email.
   - Fixed "implmenetation" to "implementation" in a Vercel AI Chatbot component file.
   - Changed "browing" to "browsing" in a Tauri component file for chat window lists.
   - Corrected "notifcation" to "notification" in another Tauri component dealing with notifications.
   - Corrected several instances of "programatic" to "programmatic" in JSON schema files for Tauri apps.
   - Fixed "suppoed" to "supposed" in a Rust file related to audio devices.
   - Corrected "unconsistent" to "inconsistent" in a Rust file for server CLI.
   - Changed "data strcture" to "data structure" in a Rust file related to vision/monitor functionality.

These fixes improve the clarity and correctness of the documentation, comments, and code throughout the project.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 33 (bb8e1d4d98ccc755c0b2bc40265416d761d0bd55)</summary>

The commit by Louis Beaumont updates the `README.md` file. Specifically, it adds a new line in the "Latest News" section. The added line highlights that "screenpipe is a top GitHub repo and featured on Hacker News," along with a link to a relevant post on X (formerly Twitter).
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 34 (f7cb31eb2dad570a9e9fc144efef4955131e7c3d)</summary>

The git commit made by Louis Beaumont on September 30, 2024, updates the `README.md` file. The change involves modifying the YouTube subscriber badge image. The previous badge source URL that used an endpoint has been replaced with a direct URL to the YouTube channel’s subscriber badge image. The updated image now uses a URL directly from "img.shields.io" to represent the YouTube channel subscribers for the specified channel.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 35 (43186243161613a58eb7b802b8b52cb0830dbb52)</summary>

The commit made by Louis Beaumont on September 30, 2024, updates the `README.md` file. Specifically, it changes the URL for the GitHub star history image, updating it to reference a new image file with a different identifier. The image identifier in the URL was changed from `6bb3f559-5016-4e1c-a91e-563ce89cd855` to `fd8968fb-e59c-4d81-a42d-059c94ce9c71`.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 36 (91b0b4200b6062ad8d9980b1a4b58ca29091365b)</summary>

The `README.md` file was updated in this commit. The changes include:

1. Indentation correction in the HTML elements formatting for a badge image related to "Rewarded Bounties".
2. Addition of a new section with a link and badge for "Open Bounties" in the initial badges section.
3. At the end of the file, two badges were added: one for "Rewarded Bounties" and another for "Open Bounties", providing visual links to the bounty statuses on algora.io.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 37 (ff6a1f69ba5f8cbd84a528b6a57c1716b1dd7b87)</summary>

In the git commit `ff6a1f69ba5f8cbd84a528b6a57c1716b1dd7b87`, the author, Louis Beaumont, updated the `README.md` file. The changes include formatting adjustments and the addition of a new link with a badge. Specifically, the existing Twitter badge was realigned, and a new link was added that points to a webpage showing completed bounties on an Algora page. This addition includes a badge displaying "Rewarded Bounties".
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 38 (8ab30024e2cbd1a7a1cea91e6546ce4ae525c19b)</summary>

The commit introduces a new feature in the screenpipe application, allowing users to select the duration of audio chunks via the user interface. Here are the key highlights of the changes:

1. **UI Update**: A new user interface element has been added to `recording-settings.tsx` to let users adjust `audioChunkDuration`. It includes a slider that allows values from 5 to 3000 seconds, and a tooltip explaining the impact of different durations on resource usage and transcription quality.

2. **Default Settings**: In `use-settings.tsx`, a new field `audioChunkDuration` has been added to the settings schema. The default value is set to 30 seconds.

3. **Settings Persistence**: Logic has been added to save and retrieve the `audioChunkDuration` from persistent storage, ensuring the user's choice is stored and reused.

4. **Version Update**: The version number in `Cargo.toml` is incremented from `0.2.82` to `0.2.83` to reflect the addition of a new feature.

5. **Command-Line Argument Handling**: In `sidecar.rs`, the `audioChunkDuration` is retrieved from the settings and logged. If it's not the default value, `--audio-chunk-duration` is appended to the command-line arguments passed to sidecar processes.

6. **Server Code**: In `screenpipe-server.rs`, the code is modified to use the new `audioChunkDuration` setting.

These changes collectively enable users to configure the duration of audio chunks processed by the application, potentially allowing for performance tuning and improved transcription quality.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 39 (2c04a8e6104a6a9b04b71a66fd3161a107b96d61)</summary>

The commit by Ezra Ellette includes changes in two files within the `screenpipe-app-tauri` project:

1. **`main.rs` Modifications**: 
   - The function call `sidecar::kill_sidecar` has been replaced with `sidecar::kill_all_sreenpipes`. This change occurs within an asynchronous block where an error log is emitted if killing the sidecar fails.

2. **`sidecar.rs` Modifications**:
   - The function `kill_sidecar` has been completely removed. This function previously handled the process of stopping a "sidecar" by canceling any ongoing restart tasks and then calling `kill_all_sreenpipes`.

Overall, the commit streamlines the process of terminating sidecar processes by eliminating the `kill_sidecar` function and directly using `kill_all_sreenpipes`.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 40 (674be266dfbc1b9fbbdfd9378855088c9d40d36c)</summary>

The commit identified by `674be266dfbc1b9fbbdfd9378855088c9d40d36c` introduces a minor user interface fix in the `pipe-store.tsx` file within the `screenpipe-app-tauri` project. The changeset involves two modifications:

1. In two locations, the code now uses optional chaining (`?.`) when accessing the `startsWith` method on the `selectedPipe.source` property. This ensures that the code does not throw an error if `selectedPipe.source` is `undefined` or `null`.
   
These changes are likely intended to enhance the reliability of the UI when dealing with possibly undefined URL sources.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 41 (416c56d0332e083c9c8bcb57465cbbcd16427c8a)</summary>

The Git commit made by Ezra Ellette updates tray icons in a Tauri application. Here is a summary of the changes:

1. **Assets Update:**
   - Added a new icon file `update-logo-black.png`.
   - Deleted two existing icon files: `updates-black.png` and `updates-white.png`.
   - Added a new icon file `screenpipe-logo-tray-black.png`.

2. **Code Changes in Rust Files:**
   - **`main.rs`:** Removed a commented line to enable `update_manager.update_screenpipe()`.
   - **`sidecar.rs`:** Removed an unused import `use tracing_subscriber::field::debug;`.
   - **`updates.rs`:** Modified the path for the tray icon from `assets/updates-white.png` to `assets/update-logo-black.png` and set the icon as a template.

3. **Configuration File Modification:**
   - **`tauri.conf.json`:** Updated the tray icon path to `icons/screenpipe-logo-tray-black.png` and added the property `"iconAsTemplate": true`.

Overall, these changes update the tray icons and related configurations while cleaning up the code by removing unused imports and comments.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 42 (14d58f8211a145158148c3c09eac01fe0d1b128b)</summary>

The recent git commit introduces a series of updates aimed at enhancing the functionality and stability of the `screenpipe-app-tauri` project. Here's a summary of the changes:

1. **Main Updates:**
   - Addition of logic to terminate a "sidecar" process when an update is initiated. This is handled within the `main.rs` file.
   - Integration of Tokio's task management to safely execute async operations in a synchronous code block using `tokio::task::block_in_place`.

2. **Functionality:**
   - New command `kill_sidecar` is introduced in `sidecar.rs`. This command is responsible for terminating the sidecar process by halting any ongoing tasks and executing the `kill_all_sreenpipes` command to ensure all processes are stopped.
   - On triggering the "update_now" menu action, the application will now invoke the `kill_sidecar` process to clean up the sidecar before proceeding with updates.

3. **Syntax and Code Formatting:**
   - Formatting improvements are included for better readability, including spacing and line breaks.
   - Adjustments to the method for retrieving array configurations, ensuring consistency and clarity in `spawn_sidecar` function.

Overall, the commit aims to enhance the application's update mechanism by ensuring processes related to the sidecar are cleanly terminated during updates, improving the application's robustness and reliability.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 43 (c5806a91d8f79fcecf40ea83e3c65e9f03aaf59e)</summary>

The git commit c5806a91d8f79fcecf40ea83e3c65e9f03aaf59e, authored by Louis Beaumont, updates the `README.md` file. The specific change modifies a line in the "Latest News" section by refining the description of a September 2024 update. The phrase "[screenpipe is top github repo & on hackernews!]" was updated to "[screenpipe is number 1 github trending repo & on hackernews!]".
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 44 (3fb613d717e2056282ddf904a320705b91c92751)</summary>

The commit introduces a modification to the `settings.tsx` file in the `screenpipe-app-tauri/components` directory. The update adds a note to the tooltip section of the settings component. This note provides a specific instruction for Windows users. It suggests that they might need to run the `ollama` command with a specified environment variable and command to execute, notably `OLLAMA_ORIGINS=* ollama run llama3.2`, to ensure proper operation. The added code snippet occurs after an existing URL display, and it is formatted similarly with HTML preformatted text (`<pre>`) and a line break (`<br />`) for better visual clarity.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 45 (0acdcaca34b28c017ef7ede6868591fc44ddf168)</summary>

The git changes reflect a version update for the package "screenpipe-app" in the `Cargo.toml` file. The version number was incremented from "0.2.85" to "0.2.86". No other changes were made to the file.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 46 (a856c17aee0779c60218bf3eba45ce7cbd7abdf9)</summary>

The commit made by Louis Beaumont on October 1, 2024, fixes the build by making several changes:

1. **Binary File Update**: The `bun.lockb` file in the `screenpipe-app-tauri` directory was modified, but details of binary file changes are not shown.

2. **JavaScript Configuration File Update**: The `next.config.mjs` file was updated to properly export the configuration object by adding `export default nextConfig;` at the end.

3. **Dependencies Update**: In `package-lock.json`, several dependencies were added or updated, primarily related to OpenTelemetry packages, instrumentation tools, and several utilities used in the project. Notable additions include `@opentelemetry/instrumentation-fetch` and `@opentelemetry/instrumentation-xml-http-request`, both version `0.53.0`, among other dependencies.

4. **Schema Files Update**: Two JSON schema files, `desktop-schema.json` and `macOS-schema.json`, located in the `src-tauri/gen/schemas` directory, had a minor textual change. The word "programmatic" was corrected to "programatic" in these files.

Overall, this commit addresses build issues by adjusting configurations, updating dependencies, and making minor documentation/text fixes.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 47 (a0e22d5735968cfb242f22341c2be40f80ff42f2)</summary>

This summary represents a Git merge commit made by Louis Beaumont on October 1, 2024. The commit indicates the merging of changes from pull request #391, originally contributed by Tribhuwan Kumar from the `main` branch. The primary purpose of the changes in this merge was to fix a video rendering issue specific to Windows.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 48 (1725b294712cd80f65c7766bb18c977ec6586ee5)</summary>

The commit by Louis Beaumont with ID `1725b294712cd80f65c7766bb18c977ec6586ee5` focuses on fixing a build issue. The changes consist of:

1. **Removal of Sentry Integration:**
   - In `chat-list-openai-v2.tsx`, the import statement for Sentry and the use of `Sentry.captureException(error);` have been removed.
   - In `settings.tsx`, the Sentry import statement is removed.
   - The `sentry.config.ts` file has been deleted, which included the Sentry initialization with its DSN configuration.

2. **Version Update:**
   - The version number in `Cargo.toml` has been updated from `0.2.84` to `0.2.85`.

These changes suggest a removal of Sentry error tracking from the `screenpipe-app-tauri` project and a minor version increment.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 49 (f1f9ec1ad7c939eb053825c846c85f66414774bf)</summary>

The git commit made by Louis Beaumont on October 1, 2024, updates the `README.md` file. Specifically, the change involves updating the GitHub Star History image. The link to the image has been altered, changing from one URL to another, updating the image reference in the documentation.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 50 (33d7313825563629c57ed04d8eedb79437d3d360)</summary>

This git commit indicates that a pull request (#390) was merged by Louis Beaumont. The purpose of the pull request was to update documentation by adding a step to the "getting started" guide for building an open-source software (OSS) desktop application. The commit is a merge between the branches identified by the commit hashes `a722a1c` and `99576f0`.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 51 (a722a1ce3afcc42b3114339bdc9fb02e9c977928)</summary>

The changes made in the commit involve replacing Sentry with OpenTelemetry for error tracking and telemetry throughout the `screenpipe-app-tauri` project. Here's a summary of the key changes:

1. **Global Error Handling**:
   - Replaced Sentry's error tracking with OpenTelemetry in `global-error.jsx`. This involves using `@opentelemetry/api` for tracing and recording exceptions.

2. **Providers Update**:
   - The `PHProvider` was renamed to `Providers` in `layout.tsx`.
   - Introduced OpenTelemetry initialization using `initOpenTelemetry` in `providers.tsx`.

3. **Configuration and Component Adjustments**:
   - Updated error and telemetry handling logic in `settings.tsx` by replacing Sentry dependencies and initialization/commenting-out with pseudo OpenTelemetry handling.

4. **Removed Sentry References**:
   - Removed Sentry integration from `next.config.mjs`.
   - Updated package dependencies to remove Sentry and include OpenTelemetry packages.

5. **New OpenTelemetry Configuration**:
   - Added a new module `opentelemetry.ts` to configure OpenTelemetry, including setting up a span processor to handle errors and integrating resource information.

6. **Cargo and Rust Changes**:
   - In `Cargo.toml`, Sentry dependencies in Rust were commented out, and OpenTelemetry was set up (albeit commented out in this commit).
   - The version number in `Cargo.toml` was updated from "0.2.83" to "0.2.84".
   - Changes in `main.rs` include setup stubs for OpenTelemetry integration in the Rust backend (again, this is commented out), and Sentry initialization code was removed.

7. **Package Changes**:
   - In `package.json`, removed Sentry packages and added OpenTelemetry packages.
   - Made necessary updates to lock files and project versions for package management.

The focus of the changes is migrating telemetry and error-tracking capabilities from Sentry to OpenTelemetry, affecting both the React frontend and the Rust backend of the application.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 52 (e55fb84f6fe5922e96789075e8abca4af1bb24d9)</summary>

The recent commit, authored by Ezra Ellette, makes several changes to the codebase, primarily focusing on fixing how icons are displayed:

1. **File Renaming:**
   - Two image files, `updates-black.png` and `updates-white.png`, were moved from the `icons` directory to the `assets` directory within the `src-tauri` structure. This suggests a reorganization of file locations to better align assets with their usage.

2. **Code Modification:**
   - In `updates.rs`, an additional import, `Manager`, has been added. This change appears to be part of the refactoring or enhancement process.
   - The code logic for setting a tray icon in the `UpdatesManager` structure has been revised. The image path is now resolved using `self.app.path()` and points to the new location under `assets`. If the image is retrieved successfully, it sets the icon using the `set_icon` method.

3. **Configuration File Update:**
   - The `tauri.conf.json` file shows an update to the `bundle/resources` array. The moved icons (`updates-white.png` and `updates-black.png`) have been removed from this array, indicating they are no longer bundled as separate icon files and are instead located in the `assets` directory, which is included as a wildcard.

Overall, these changes likely improve asset management and icon display functionality within the application, possibly making it more efficient and organized.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 53 (d6abeae0787c7cc4fef9898f439de4b8205e074d)</summary>

The commit with hash `d6abeae0787c7cc4fef9898f439de4b8205e074d` by author Ezra Ellette, made on October 1, 2024, includes a minor formatting change in the file `commands.rs` located in the `screenpipe-app-tauri/src-tauri/src/` directory. The change involved adding a newline at the end of the file to ensure proper formatting.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 54 (dc4c5dce40f85a78237f47f38dec5bae857decb1)</summary>

The git changes summarize the following updates:

1. **Code Changes:**
   - The `checkForAppUpdates` function call within a `useEffect` hook in `app/page.tsx` was commented out.
   
2. **Dependency Updates:**
   - Added minor formatting adjustments in `Cargo.toml` with no change in versions.
   - "tauri-plugin-updater" dependency was uncommented and set to version "2.0.0-alpha.2".

3. **Updater Functionality:**
   - Introduced `updates.rs` which implements an `UpdatesManager` for checking application updates. It performs periodic update checks and manages update-related menu items and notifications.
   - Updated references and imports in `main.rs`, integrating `UpdatesManager` for enabling automatic update checks and setting up the system tray update menu.
   - Added logic to handle tray menu events for application updates.
   
4. **Icon Additions:**
   - Added two new icon files: `updates-black.png` and `updates-white.png` meant for display changes when updates are available.

5. **Configuration Updates:**
   - Updated `tauri.conf.json` to include the new update icons in the package.
   - Reformatted JSON arrays and objects for improved readability.

Overall, the changes focus on implementing and integrating functionality for automated updates within the application, alongside the related UI and configuration adjustments.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 55 (bb8f2fd761a1e0709ea6a056692613751807ce56)</summary>

The commit `bb8f2fd761a1e0709ea6a056692613751807ce56`, authored by tribhuwan-kumar, addresses a video rendering issue specifically on Windows. In the `screenpipe-server/src/video.rs` file, changes were made to the video encoding process for Windows. The encoder was switched from "mpeg4" to "h264_mf". Additionally, a TODO comment was updated to reflect this change, indicating the switch to "H264_mf" until an update is made to `ffmpeg` in `pre_build.js`.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 56 (99576f04bdb7a4dff503f193d544a1a363a9ffd5)</summary>

The git commit with hash `99576f04bdb7a4dff503f193d544a1a363a9ffd5` by author Glavin Wiechert, made on October 1, 2024, includes the following changes to the `getting-started.mdx` documentation file:

1. **Dependencies Installation Update:** 
   - The command for installing dependencies on macOS has been updated to include `cmake` and `wget`. The updated command now reads:
     ```bash
     brew install pkg-config ffmpeg jq tesseract cmake wget
     ```

2. **New Step Added for Desktop App:**
   - A new step (step 5) has been introduced for building the OSS desktop app.
   - This step instructs users to continue following the README for `screenpipe-app-tauri`, which is accessible via the provided GitHub link:
     ```
     https://github.com/mediar-ai/screenpipe/tree/main/screenpipe-app-tauri/README.md
     ```

These changes offer additional guidance for users looking to build the open-source desktop application, enriching the "Getting Started" documentation.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 57 (7634a84f1b6669fa14652a84062181202085ab7a)</summary>

The commit with hash `7634a84f1b6669fa14652a84062181202085ab7a` is a merge commit authored by Louis Beaumont on September 30, 2024. It merges changes from pull request #389, which was submitted by the user "brianteeman". The purpose of the pull request was to fix assorted typos and spelling errors in the codebase.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 58 (6ff3f9f7989ae1b983fa6ad8e4ccb8a317f3872a)</summary>

The commit made by Louis Beaumont updates the `README.md` file. The change consists of adding a new bullet point to the "Latest News" section. The addition notes that in September 2024, screenpipe became a top GitHub repository and was featured on Hacker News, with a link to a relevant status update on X (formerly Twitter).
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 59 (daaf2c88706c9d48dc3283c2404387dcffbca224)</summary>

The commit updates the `README.md` file. Specifically, it modifies an HTML section that displays a badge for YouTube channel subscribers. The update changes the image source URL for the subscriber badge to a different API endpoint, which directly references the YouTube channel ID for subscribers, potentially simplifying the previous setup.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 60 (f1e6bb5d7a785fc25223ed98b453443feba6b9c4)</summary>

The commit by Louis Beaumont on September 30, 2024, updates the `README.md` file. The change specifically alters the GitHub Star History image link, modifying the URL from one associated with ID `6bb3f559-5016-4e1c-a91e-563ce89cd855` to a new URL with ID `fd8968fb-e59c-4d81-a42d-059c94ce9c71`.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 62 (5307b7c4212b8c2b00054081f0c8eb7cd53e7d19)</summary>

The changes made in this Git commit update the `README.md` file. Specifically, the update adds a new badge to the existing list. This new badge links to a page on Algora that displays completed bounties for the organization `mediar-ai`. Additionally, there is a minor formatting adjustment where existing HTML is indented for better alignment.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 63 (b65f0abedbf2c1e856f903c85ad81e09acd2f5a9)</summary>

The commit `b65f0abedbf2c1e856f903c85ad81e09acd2f5a9` by Brian Teeman focuses on fixing assorted typos and spelling errors across multiple files in a Git repository. Key changes include:

1. **Whitespace Adjustments**: Several instances where there were unnecessary trailing spaces in `content/docs/NOTES.md` were removed for cleaner formatting.
   
2. **Specific Typo Corrections**:
   - In `examples/typescript/apple-shortcut/main.js`: Corrected "asnwer" to "answer".
   - In `examples/typescript/pipe-meeting-summary-by-email/pipe.ts`: Corrected "intructions" to "instructions".
   - In `examples/typescript/vercel-ai-chatbot/components/empty-screen.tsx`: Corrected "implmenetation" to "implementation".
   - In `screenpipe-app-tauri/components/chat-list-web-llm.tsx`: Corrected "browing" to "browsing".
   - In `screenpipe-app-tauri/components/notification-handler.tsx`: Corrected "notifcation" to "notification".
   - In multiple schema files (`screenpipe-app-tauri/src-tauri/gen/schemas/desktop-schema.json` and `macOS-schema.json`): Corrected "programatic" to "programmatic".
   - In `screenpipe-audio/src/core.rs`: Corrected "suppoed" to "supposed".
   - In `screenpipe-server/src/cli.rs`: Corrected "unconsistent" to "inconsistent".
   - In `screenpipe-vision/src/monitor.rs`: Corrected "strcture" to "structure".

These corrections improve the overall readability and professionalism of the codebase by addressing minor spelling errors and cleaning up formatting issues.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 64 (cbe254b1aea234738ad0dc72cec6931eadc25e84)</summary>

The commit by Louis Beaumont adds functionality to select the audio chunk duration in the UI of the `screenpipe-app`. Here's a summary of the changes:

1. **User Interface Update:**
   - Added an interface element in `recording-settings.tsx` where users can adjust the audio chunk duration through a slider.
   - A tooltip is provided to guide users about the benefits of adjusting the audio chunk duration.

2. **Settings Management:**
   - Introduced a new field `audioChunkDuration` in the `Settings` interface in `use-settings.tsx`.
   - A default value of 30 seconds is set for `audioChunkDuration`.
   - Logic is added to load and save this setting using the existing settings management hooks.

3. **Backend Logic:**
   - In the `sidecar.rs`, the newly introduced `audioChunkDuration` setting is read and applied, ensuring that the sidecar component configures the audio chunk duration accordingly.
   - The change ensures that `screenpipe-server.rs` utilizes this setting by passing the duration to the server.

4. **Package Update:**
   - The version in `Cargo.toml` is incremented from `0.2.82` to `0.2.83` to reflect the new feature addition.

Overall, this commit enhances the application's flexibility by allowing users to set the audio chunk duration, potentially optimizing performance and transcription quality.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 65 (9ab0899c14ce3a20204516e082d60d8544a8b2bb)</summary>

The recent commit with hash `9ab0899c14ce3a20204516e082d60d8544a8b2bb` introduces a new feature allowing users to enable or disable telemetry in the Screenpipe app. The notable changes include:

1. **Settings Component (`settings.tsx`):**
   - A toggle switch was added to the settings UI for enabling or disabling telemetry.
   - New imports were added for `Switch` component, `Sentry`, and `posthog`.
   - A function `handleAnalyticsToggle` is implemented to manage the state of telemetry, capturing the enabling/disabling events with `posthog` and configuring `Sentry` accordingly.

2. **Default Settings (`use-settings.tsx`):**
   - A new `analyticsEnabled` setting is added with a default value set to `true`.
   - The code for retrieving and storing this setting in the local storage is implemented.

3. **Version Update (`Cargo.toml`):**
   - The application version is incremented from `0.2.81` to `0.2.82`.

This feature aims to enhance user privacy choices by allowing them to opt-in or out of data collection for improving application functionality.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 66 (a9c027018d8e7441afeb96a2659b3bbb773f3b29)</summary>

The commit with ID `a9c027018d8e7441afeb96a2659b3bbb773f3b29` by Louis Beaumont updated the `README.md` file. The specific change made was to clarify the way users can obtain a license for the desktop app. Previously, it stated that users could get the free forever desktop app by sending a pull request or sharing about the app online. This was revised to mention that users can receive a one-year license for the desktop app by sending a pull request or sharing about the app online.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 67 (0b30664443a0034078c26e383b5162fbc1b1b734)</summary>

The commit `0b30664443a0034078c26e383b5162fbc1b1b734` by Louis Beaumont contains several changes and enhancements across the `screenpipe-app-tauri` project. Here is a summary of the changes:

1. **README.md**: The build instructions were updated to suggest including additional features like `metal` or `cuda` in the Rust backend build command. There was also a clarification in handling macOS specific settings.

2. **VSCode Settings**: MacOS specific instructions in the `.vscode/settings.json` were updated to include additional cargo features like "metal" and "pipes", and added environment variable `SCREENPIPE_APP_DEV`.

3. **search-chat.tsx**: The default similarity threshold in the `SearchChat` component was increased from `0.9` to `1`. Also, the `handleFilterDuplicates` function now ensures that when the threshold is `1`, all results get selected automatically.

4. **updater.tsx**: The conditional operation to kill all screenpipe processes on Windows was uncommented, making it effective as intended when updates occur.

5. **Dependency Updates**: Added new dependencies in the `package-lock.json`, including `@types/js-levenshtein` and `js-levenshtein`.

6. **Cargo.toml**: The project version was incremented from `0.2.80` to `0.2.81`.

7. **tauri.conf.json**: Adjusted the `media-src` policy to allow blobs from `youtube` and `github` domains to comply with content security policy requirements.

These changes fix the identified issue #368 and include a small UI change to the application, enhancing both functionality and security configurations.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 68 (c76b72ee8f59911d10ea7d304497d71146067b29)</summary>

The commit with hash `c76b72ee8f59911d10ea7d304497d71146067b29` by Louis Beaumont, dated September 30, 2024, contains changes aimed at fixing the build. Specifically, the changes involve:

1. **Removal of an Import**: The import statement for `DevSettings` has been removed from `screenpipe-app-tauri/app/page.tsx`.

2. **Commented Out Code**: A reference to `<DevSettings />` in the `screenpipe-status.tsx` component has been commented out.

These changes suggest a cleanup or temporary removal of development settings to resolve issues with building the application.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 69 (40dc63313857182782618f2eba82bfd94dc66bbf)</summary>

The recent commit, identified by hash `40dc63313857182782618f2eba82bfd94dc66bbf`, includes several changes aimed at fixing issues in the `screenpipe-app-tauri` project. Here is a summary of the changes:

1. **Removal of `dev-dialog.tsx`:** The file `dev-dialog.tsx` was completely removed from the `components` directory. This component previously handled developer settings, including UI elements and logic for updating settings and handling changes.

2. **Changes in `pipe-store.tsx`:** The `useSettings` hook and `invoke` import were removed, suggesting a refactoring or simplification of the module.

3. **Updates in `recording-settings.tsx`:**
   - Introduction of `platform` from `@tauri-apps/plugin-os`.
   - Removed `currentPlatform` prop from the `RecordingSettings` function.
   - Redundant `console.log` statements were removed.
   - An extended section of code handling OCR engine options now distinguishes between different platforms (Linux, Windows, macOS) with a new function `renderOcrEngineOptions()`.
   - Adjusted the dependencies for useEffect from `localSettings, setLocalSettings` to `settings`.

4. **Changes in `screenpipe-status.tsx`:** Removed import of `DevSettings`, indicating its previous responsibilities may have been removed or transferred.

5. **Modifications in `settings.tsx`:**
   - Removed the import of `platform`.
   - Simplified state updates by directly setting new values in `setLocalSettings`.
   - Commented out a hacky page reload mechanism in `Dialog`'s `onOpenChange` event.

6. **Version bump in `Cargo.toml`:** The version of the package was incremented from `0.2.79` to `0.2.80`.

These changes collectively indicate a clean-up and reorganization of the codebase, simplifying logic, and likely integrating platform-specific logic more dynamically.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 70 (4f2391c6dfd8775919a5f59f72313ea0d5d353eb)</summary>

The commit with hash `4f2391c6dfd8775919a5f59f72313ea0d5d353eb` made by Louis Beaumont on September 29, 2024, updates the `README.md` file. The specific change made is the update of an image link under the "star history" section. The URL for the image has been changed from one asset identifier to another.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 71 (cba866ff343324ff37b8b419c270136c4c4ca576)</summary>

This git commit represents a merge of a pull request (#379) into the codebase. The pull request was made by a contributor with the username "eltociear," and it involves an update to the `screenpipe.js` file. The commit was authored by Louis Beaumont on September 29, 2024. The purpose of the changes is categorized as a "chore," indicating routine maintenance or updates that do not add functionality or fix bugs.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 72 (6d3dc33f97bd2ff639cdfac53115f532cb8aa08c)</summary>

The commit with hash `6d3dc33f97bd2ff639cdfac53115f532cb8aa08c` involves a minor textual change in a JavaScript file located at `examples/typescript/rag-over-your-life-in-obsidian/screenpipe.js`. Specifically, it corrects a typo in a comment by changing the word "infomation" to the correct spelling, "information". This update is classified as a "chore," indicating it's a non-functional change aimed at maintaining code quality.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 73 (874716bf6822c4e62f5d09e2d4c5c2344d13aa5b)</summary>

The Git commit `874716bf6822c4e62f5d09e2d4c5c2344d13aa5b` by Louis Beaumont updates the version number of the `screenpipe-app` package in the `screenpipe-app-tauri/src-tauri/Cargo.toml` file from `0.2.78` to `0.2.79`. The commit message is "release," indicating this is likely a version bump for a new release. Other details, such as description, authors, and license, remain unchanged.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 74 (8bc7812968878b091e47e45c7ce77e75552da980)</summary>

The recent commit updates the README.md file by adding a new section titled "star history." This section includes an image displaying the GitHub Star History. The rest of the file remains unchanged.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 75 (b4f7ff3e02724473f4ce36599d59e7f58ef5d019)</summary>

This commit, authored by Louis Beaumont, includes several changes aimed at fixing tests and reverting some Windows-specific encoding adjustments. Here's a summary of the changes:

1. **GitHub Actions Workflow**:
   - Commented out the environment variable `V8_FROM_SOURCE=1` in `.github/workflows/release-app.yml`.
   - The tmate setup step is commented out, indicating it's temporarily disabled.

2. **Cargo.toml in `screenpipe-app-tauri`**:
   - Added a `package.metadata.cargo-machete` section, which indicates ignoring the `tauri-utils`.

3. **Cargo.toml and Benchmarks in `screenpipe-audio`**:
   - Removed a benchmark entry for `stt_benchmark`.
   - Deleted the `stt_benchmark.rs` file, which contained benchmarking code for audio transcription and memory usage.

4. **Examples in `screenpipe-core`**:
   - Wrapped module imports and code related to text streaming with `#[cfg(feature = "llm")]`, making them conditional on the `llm` feature.
   - For example, in `phi.rs`, added a fallback `main` function to handle cases where the `llm` feature is not enabled.

5. **Tests in `screenpipe-core`**:
   - Deleted `llm_test.rs`, which contained a test for generating text streaming.
   - Marked `test_directory_functions` in `pipes_test.rs` as ignored due to inconsistent behavior.

6. **Cargo.toml in `screenpipe-server`**:
   - Removed some dependencies and rearranged features.
   - Added a `package.metadata.cargo-machete` section, which specifies ignoring certain dependencies.

7. **Benchmark code in `screenpipe-server`**:
   - Updated database benchmark code to include `AudioDevice`.
   - Minor changes in configuration for running asynchronous benchmarks.

8. **Video Processing in `screenpipe-server`**:
   - Updated the encoder for Windows in `video.rs` from `h264_mf` to `mpeg4`.

9. **WebSocket example in `screenpipe-vision`**:
   - Temporarily commented out a portion of the code dealing with base64 encoding of images.

Collectively, these changes address the handling of certain features and configurations, likely in response to ensuring compatibility and functionality across multiple systems and conditions, such as conditional compilation and feature flags.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 76 (bc57c8ae829f918ffd01dfb104e46a03a039b7a4)</summary>

The commit `bc57c8ae829f918ffd01dfb104e46a03a039b7a4` authored by Louis Beaumont includes the following changes:

1. **GitHub Actions Workflow**: 
   - In the `release-app.yml` file, an environment variable `V8_FROM_SOURCE=1` is added to a conditional block, which appears to be part of configuring the build environment for a macOS platform.
   - The `tmate` session setup section, which was previously commented out, is now enabled, indicating that a remote tmate session will be established during the workflow.

2. **Cargo.toml Update**:
   - The version of the `screenpipe-app` in `Cargo.toml` has been incremented from `0.2.77` to `0.2.78`. This suggests a minor patch or bug fix, which is consistent with the commit message `fix: try #375`.

Overall, these changes involve enabling certain build configurations and incrementing the application version, likely in response to addressing an issue identified in context `#375`.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 77 (9e4a119706afc52bd6d2709f199b1ce8b81e7cd4)</summary>

The commit with hash `9e4a119706afc52bd6d2709f199b1ce8b81e7cd4` authored by Louis Beaumont is a fix for a script in the `screenpipe-app-tauri` project. The changes primarily focus on simplifying and ensuring the presence and usability of the `wget` command in a pre-build script for a Windows platform.

Key changes include:
1. **Refactoring `wget` path detection**: A new asynchronous function `findWget` is introduced to identify available locations of `wget.exe` on the system by iterating over possible paths. This replaces the previous method that used `where.exe` to locate the executable.

2. **Removal of `where.exe` logic**: The existing logic using `where.exe` to locate executables is removed and replaced by direct calls to `findWget`.

3. **Hardcoding the 7-Zip path**: Instead of dynamically finding the path for the `7z` executable, it is now hardcoded to `"C:\\Program Files\\7-Zip\\7z.exe"`.

4. **Streamlined package setup**: The setup for various packages like FFMPEG, Tesseract, ONNX Runtime, OpenBlas, and CLBlast is updated to use `findWget` for downloading resources. The usage of `7z` for extracting files is standardized.

5. **Setup of vcpkg packages**: The setup process for vcpkg packages is simplified by removing the try-catch block for running the `vcpkg.exe install` command, and now the command is run quietly.

6. **Minor cleanup and consistency improvements**: Redundant code and error handling are removed to streamline the script.

The overall change consolidates and simplifies dependency download and installation procedures on Windows, focusing on ensuring necessary tools are available and correctly installed.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 78 (0ac2f87d0483b8bffbff6f895d9781d1031010c7)</summary>

The commit with hash `0ac2f87` authored by Louis Beaumont updates the `README.md` file located in `examples/typescript/pipe-post-questions-on-reddit/`. The changes include:

- The addition of a URL pointing to a GitHub user attachment. This URL is added above the "quick setup" section in the README file. 
- Some extra blank lines have been introduced to make space for the new content.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 79 (599f2fd5f9763d34743d9b868cb976ab4312605c)</summary>

The commit with hash `599f2fd5f9763d34743d9b868cb976ab4312605c` is a merge commit performed by Louis Beaumont on September 28, 2024. It merges changes from the pull request #373 authored by the user `m13v` into the codebase. The pull request is titled "Reddit pipe," suggesting the changes involved relate to a feature or update associated with a Reddit integration or component named "pipe." The merge integrates these changes, as reflected by the merging of two parent commits: `bee5102` and `ec3ef2b`.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 80 (bee5102a89137dc71946f33dc34a9bfe6e9e5213)</summary>

The commit by Louis Beaumont updates the version number of the "screenpipe-app" package from "0.2.76" to "0.2.77" in the Cargo.toml file located in the `screenpipe-app-tauri/src-tauri/` directory. This change is associated with fixing issue #319, as indicated by the commit message "fix #319 release".
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 81 (21ebe20a550c24acc8fd493d179c4ae131194746)</summary>

The commit is a merge of a pull request (#372) from user "tribhuwan-kumar" into the main branch. The purpose of the merge is to fix a video rendering issue on Windows. The commit was authored by Louis Beaumont on September 28, 2024.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 82 (d1ccd55404971d5295743935b5a3e7ba66075b89)</summary>

The commit adds a new feature to the `screenpipe-app-tauri` project, which includes several updates to the `meeting-history.tsx` component. Notable changes are:

1. **Merge Button Functionality**: A new button has been added to allow users to merge consecutive meetings. The function `mergeMeetings` facilitates this merging by combining the data from two meetings and updating their timestamps and transcriptions, while removing the second meeting from the list.

2. **UI Enhancements**: 
   - The meeting card now displays if it has been merged with other meetings using badges.
   - Timestamps are now formatted using a new `formatDate` function.
   - Additional components like `ChevronDown` and `ChevronUp` from `lucide-react` are introduced for UI purposes.
   - Tooltip descriptions and button labels have been refined for clarity.

3. **Data Handling**: 
   - The `Meeting` interface includes a new optional property `mergedWith` to track merged meetings.
   - There are improvements in the handling and display of meeting transcriptions and summaries.

4. **Version Update**: The application's version is incremented from `0.2.75` to `0.2.76` in the `Cargo.toml` file to reflect the updated functionality.

These updates collectively introduce a merge feature for meeting entries and make UI improvements to enhance the user experience.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 83 (71cd8974b072decc240cceb90181bcbc07693ddb)</summary>

This commit removes the `findWget()` function from the `pre_build.js` script in the `screenpipe-app-tauri/scripts` directory. The `findWget()` function was responsible for searching for the `wget` executable in various system paths and reporting if it was found. If `wget` was not found, it would log an error message and terminate the process. The function has been entirely commented out, simplifying the script.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 84 (d6e1ffc7d0e81892f8efd5e7cec68b693c2f82da)</summary>

This commit is a merge of the main branch from the 'mediar-ai' repository into the current main branch of the repository. This action was performed by a user named Tribhuwan on September 28, 2024. The merge incorporates changes from a remote upstream repository, identified by the SHA values 'e8f21b4' and '4ea365a' pertaining to the merging branches.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 85 (e8f21b4c6e8bbd1abe32eba5fd6294f688a7aac1)</summary>

The commit updates the `screenpipe-server/src/video.rs` file to change how video encoding is handled based on the operating system. Previously, the `libx264` codec was used across all platforms for encoding. However, the change introduces conditional logic to use `h264_mf` codec specifically for Windows platforms due to its availability in the `avbuild` FFmpeg. For non-Windows platforms, `libx264` continues to be used. Additionally, a reminder is added as a comment to switch back to `libx264` once FFmpeg is updated in `pre_build.js`. The `-q:v 5` option is added for the `h264_mf` encoder on Windows as part of this change.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 86 (b2f20b845496862fddeadf7cf20c4c5448907047)</summary>

The commit involves modifications to the `pre_build.js` script within the `screenpipe-app-tauri` repository. The changes focus on improving the Windows-specific build process:

1. **Dynamic Executable Path Retrieval:**
   - The function `findWget()` originally attempted to locate `wget.exe` by checking multiple hard-coded paths. This function has now been commented out.
   - A new function, `exePath(command)`, has been introduced to dynamically locate the executable paths for given commands using `where.exe`, making it more reliable and flexible.

2. **Dependency Management:**
   - The script now uses `exePath` to find the paths for required executables (`wget`, `7z`, and `unzip`) dynamically instead of assuming their presence in predefined locations.
   
3. **Command Improvements:**
   - The `wget` command uses the `--continue` option, which allows the download to resume from where it left off if interrupted. This option is added throughout the script for downloading various resources.
   - Hard-coded paths for `7-Zip` and other operations have been replaced with dynamically retrieved paths, improving portability and reducing dependency on specific file paths.

Overall, these changes enhance the script's flexibility in identifying executable paths and ensure better handling of downloads on Windows systems.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 87 (4ea365ad2655343c350e6dd2159d287c820e0699)</summary>

The commit made by Louis Beaumont updates the version numbers in two `Cargo.toml` files:

1. **`screenpipe-app-tauri/src-tauri/Cargo.toml`:**
   - The package version is updated from `0.2.74` to `0.2.75`.
   
2. **`screenpipe-core/Cargo.toml`:**
   - The `deno_core` build dependency version is updated from `0.307.0` to `0.311.0` and remains optional.

This commit is intended to fix the Deno version used in the project.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 89 (3fff6836691b977fb7c6bc24b8f19a04687f0ca9)</summary>

This commit represents a merge operation. The author "tribhuwan" merged changes from the 'main' branch of the 'mediar-ai' repository into the 'main' branch of their current repository. The commit is a combination of changes from two previous commits: `f6d9132` and `89f8c6c`. This update likely integrates updates or features that were added to the 'mediar-ai' repository into their own project.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 90 (f6d91327d91a8441651cf937de3b1cc9c1b29b11)</summary>

The changes in this commit involve updating the `pre_build.js` script used for the Windows installation of a software application. Key modifications include:

1. **Installation Check Function:**
   - Added a new `isInstalled` asynchronous function to check if a command is available on the system using PowerShell. This function helps ensure necessary tools are installed before attempting operations.

2. **7zip and unzip Installation Checks:**
   - Implemented checks to determine if `7z` and `unzip` are installed before performing compression-related tasks. This prevents operations from failing due to missing dependencies.

3. **wget Command Usage:**
   - Adjusted `wget` command invocations to remove duplicate `-O` flags and corrected the syntax for downloading files.

4. **Handling vcpkg Installation:**
   - Updated the logic for installing `vcpkg` packages. The script now tries to run `vcpkg` using both a direct path and from the system path, accommodating different user configurations.

5. **Miscellaneous Adjustments:**
   - Improved log messaging for missing commands to instruct users to install them.
   - Modified errors and processes around checking and moving files to better align with Windows environments.

Overall, these changes enhance compatibility and robustness of the script on Windows systems by ensuring dependencies and tools are properly managed.
</details>

------------------------------------------------------------------------

# Overall Summary of Changes

The collection of Git changes can be summarized as a series of updates, bug fixes, improvements, and feature additions across various components of a project. Here's a concise summary of the key modifications:

1. **Version and Configuration Updates**:
   - Several `Cargo.toml` files were updated to increment version numbers, add new dependencies, or configure existing ones, reflecting new releases or minor fixes.
   - GitHub Actions workflows were modified to adjust build configurations, such as environment variables for compatibility or performance.

2. **Feature and Functionality Enhancements**:
   - New UI features were introduced, such as sliders for audio chunk duration and telemetry toggles for user privacy.
   - A merge feature for meetings in the UI was added to enhance user interaction.
   - OpenTelemetry replaced Sentry for error tracking, indicating a shift in monitoring strategy.

3. **Documentation and Getting Started Guides**:
   - Documentation updates included reorganizing installation instructions, clarifying setup processes, and adding new sections and dependencies.
   - Typos and formatting in both code comments and documentation were corrected for clarity and consistency.

4. **Platform-Specific Adjustments**:
   - Improvements were made to Windows-specific build scripts, ensuring dependencies like `wget` and `7z` are managed correctly.
   - Adjustments in video encoding processes specifically address Windows compatibility, temporarily using `h264_mf`.

5. **Bug Fixes and Optimizations**:
   - Several commits focused on fixing typos, build issues, benchmarking code, and dependency management.
   - Unused imports and code were removed to streamline applications and reduce potential errors.

6. **Code Refactor and Cleanup**:
   - Cleanups included removing deprecated functions, refactoring scripts, and reorganizing assets such as icons for consistency and efficiency.

7. **Project Management Enhancements**:
   - GitHub workflow improvements included adding standard issue and pull request templates to enhance collaboration consistency.
   - Merged pull requests from contributors indicating ongoing collaborative development and integration efforts.

These changes indicate ongoing development aimed at enhancing functionality, improving user experience, ensuring stability, and maintaining code quality across the project.
