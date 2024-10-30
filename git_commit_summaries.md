# Git Commit Summaries

-----------------------------------------------------------------------
-----------------------------------------------------------------------
 
PRESS CMD+SHIFT+V TO VIEW IN MARKDOWN
 
_______________________________________________________________________
-----------------------------------------------------------------------
Total number of commits: 24

<details>
<summary>summary for commit 1 (de025c2ad11c41ccee4b27d444cf063445d2f2c1)</summary>

This commit represents a merge of a pull request into the main branch. The pull request #596, titled "Update Homebrew formula for aarch64-apple-darwin," likely includes changes related to updating the Homebrew formula for the specified architecture (aarch64-apple-darwin). The merge was performed by Louis Beaumont on October 29, 2024.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 2 (ba4ed58e0b074210bf6053d0806d5107451e7d9d)</summary>

This git commit represents a merge where updates were made to the `screenpipe.rb` formula file for the Homebrew package manager. Specifically, the SHA256 checksums for the `screenpipe` binary downloads on macOS have been updated for both arm64 (Apple Silicon) and x86_64 (Intel) architectures.

Changes:
- For the arm64 architecture, the SHA256 checksum was updated from `8fff1b31922a383e2e097dce46eb09e69d7358b4e6b0bd64c029b2dc1ee4f516` to `564f0927b392e6375b3964246948c20b641cdf34af5844b703287b865d5ad2f1`.
- For the x86_64 architecture, the SHA256 checksum was updated from `276cff5d1787881fc6cd08200a9548c342b2968b3d3a788ef5e0bc2234e0f450` to `a6fa5c46025f35eeb0290f050f95d37614d2cd5c54f1b8ed2c381705744938be`.

These changes generally suggest a new release or update of the binaries was made available, necessitating checksum updates to ensure file integrity during downloads.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 3 (218cfbcecfb5371f21a7ae4756c577bc5b64de66)</summary>

This Git commit represents a merge into the codebase. Louis Beaumont merged pull request #597 into the main branch on October 29, 2024. The merge pertains to an update of a Homebrew formula specifically for the x86_64-apple-darwin platform. The reference `4633a6ac6c0badabf8ba3f30caf1d5df62eadfec` likely indicates a specific commit or change ID within the pull request that was merged.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 4 (c1cdd6facdc6c11ec4c2e7ffdcb1471f50be3b86)</summary>

The commit by Louis Beaumont removes references to the "Entelligence" service from the codebase. Specifically:

1. In `next.config.mjs`, entries for "https://entelligence.ai" and "https://d345f39z3arwqc.cloudfront.net" have been removed from the `script-src` part of the Content Security Policy.

2. In `_document.tsx`, the import and use of `next/script` for loading the `entelligence-chat.js` script have been entirely removed from the `<Head>` tag. Additionally, any initialization code for the `EntelligenceChat` has also been deleted.

Overall, the changes focus on eliminating the integration with Entelligence from the project.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 5 (e55211215c1de8a3cd0058242cc40af2b334851a)</summary>

The commit updates the Homebrew formula for the `screenpipe` package. The version is updated from 0.1.96 to 0.1.97 for the `x86_64-apple-darwin` architecture. The relevant URL for downloading the package reflects this version change, and the SHA-256 checksum for the `x86_64` version is updated to match the new package. The update is performed by a GitHub Actions Bot.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 6 (b41aee52929c9aded45f758aecc44b75691fd417)</summary>

The commit updates the Homebrew formula for the `screenpipe` library. The version is incremented from `0.1.96` to `0.1.97`, and the corresponding download URLs and SHA256 checksums are updated accordingly for the `aarch64-apple-darwin` architecture.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 7 (4633a6ac6c0badabf8ba3f30caf1d5df62eadfec)</summary>

The commit `4633a6ac6c0badabf8ba3f30caf1d5df62eadfec` made by Louis Beaumont updates the `Cargo.toml` file. The specific change is a version bump of the package from `0.1.96` to `0.1.97`. Other metadata like the author, description, and repository remain unchanged. The commit is titled "bump brew."
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 8 (e215f9a2d6d9251536a9b74a951eb52b332fa2aa)</summary>

The commit updates the version of the "screenpipe-app" package in the `Cargo.toml` file from "0.7.6" to "0.7.7".
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 9 (12e2460f4e21416996e0efdeb119e38194c07f1f)</summary>

The git commit introduces support for Intel's Math Kernel Library (MKL) to improve performance:

1. **Installation Script Changes**: 
   - The `install_dependencies.sh` script now includes the installation of `intel-mkl-full`, which is required for MKL acceleration.

2. **GitHub Actions Workflow Modifications**:
   - In the `release-app.yml` workflow, the `publish-tauri` job no longer depends on the `draft` job.
   - For the `ubuntu-22.04` build platform, the `args` for Tauri have been updated to include `--features mkl`.
   - Similarly, for the `windows-latest` build platform, the `args` have been updated to include `--features mkl`.
   - Additional package installations for `ubuntu-22.04` now include `intel-mkl-full`.
   - A new step to install MKL-related Python packages (`mkl`, `mkl-devel`, `mkl-static`) is added for both Windows and Ubuntu platforms.

3. **Cargo Configuration Update**:
   - The `Cargo.toml` file in the `screenpipe-server` directory adds a new feature set named `mkl`, which includes several MKL-related dependencies.

Overall, the changes aim to enhance the application's performance by incorporating MKL support across various platforms.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 10 (04feb51fc54fa050ed7b5df0f24a7859f81d746e)</summary>

The commit made by Louis Beaumont on October 28, 2024, updates the `README.md` file. The change involves modifying a heading in the document. Specifically, the phrase "24/7 open source 24/7 screen & voice recording for the age of superintelligence" was simplified to "open source 24/7 screen & voice recording for the age of superintelligence." The title now removes the redundant "24/7" at the beginning.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 11 (76b1b44021f1321c515ee24028ae56c3454a6743)</summary>

The commit updates the `README.md` with several changes:

1. **Title Change**: The title has been replaced from "24/7 local AI screen & audio capture" to "24/7 open source 24/7 screen & voice recording for the age of superintelligence."

2. **Removed Text**: A sentence describing the library's purpose and its association with Ollama and Rewind.ai has been removed. This sentence emphasized the library's security, open nature, and user data ownership.

3. **Updated Messaging**: The text beneath the "why?" section has been changed. Previously, it emphasized the rapid progress of AI and the need to prepare for future technological capabilities. The new text is more concise, focusing on having the necessary data ready by 2025 for a future where context will not be an issue.

Overall, the changes simplify and update the description and rationale for the project to align with forward-looking insights on data readiness and AI advancements.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 12 (59ebe00013276a2f0cbeebb3636cf83c34f95e9b)</summary>

The commit, authored by Louis Beaumont, updates the pull request (PR) template in a GitHub repository. The changes include:

1. **Testing Steps**: 
   - Instructions were added to include steps to test the PR in the most time-efficient way.
   - A suggestion to add screenshots or screen captures if relevant to demonstrate the PR functionality.

2. **Checklist Updates**:
   - A new emphasis was added, stating that the PR should be quick to review, merge, and release without causing issues for users.
   - Removed the checklist item about adding a custom cursor AI prompt.
   - Removed the checklist item about following the project's style guidelines.
   - Removed the checklist item about self-review of the code.
   - Removed the checklist item about running all tests locally with the code changes.

3. **Formatting and Clarity**:
   - Minor formatting changes for clarity and instructions in the template.

Overall, these updates aim to streamline the review process and ensure efficient testing and clarity in the pull request submission.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 13 (2227d96c8f11512571d041854fdca0c33aadddec)</summary>

The commit made by Louis Beaumont adds a new documentation file called `integrations.mdx` to the project. This file outlines various integrations, including "ollama," "openwebui," "Omi AI Friend wearable," "iPhone screen mirroring," and "iPhone microphone recording." Each section provides a brief statement or a placeholder, indicating that more information will be added later ("doc in progress"). There is also a link provided for instructions related to the "openwebui" integration.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 14 (f263db3a14c37ad14dfa99a56e746a88958896e7)</summary>

The given Git changes are primarily focused on enhancing the functionality and usability of a Twitter autopilot example, improving the handling of configurations in the `screenpipe` project, and introducing some frontend analytics.

1. **Twitter Autopilot Example Enhancements**:
   - The `pipe.json` configuration file for the Twitter autopilot example now includes additional fields, such as AI provider selection (`ollama` or `openai`), OpenAI API configurations, and keyword/username settings for more targeted searches.
   - In `pipe.ts`, the code has been refactored and expanded:
     - An AI provider creation function is introduced to handle different AI services based on the configuration.
     - Enhanced logic for keyword extraction and tweet filtering using the AI model is included.
     - Improved user interaction with AI for generating Twitter comments, with rules grounded in Twitter engagement metrics.
     - Dynamic retry and filtering strategies are applied to fetch relevant tweets when the initial attempts do not yield sufficient results.
   - A new test function (`testTwitterSuggestionsOnce`) has been introduced for testing suggestions with screen data from the last hour.

2. **Inbox Frontend Changes**:
   - Integration of `posthog-js` to capture an event when the inbox is opened, allowing user behavior analytics.
   - Adjustments in rendering links to handle both internal and external links appropriately.

3. **Screenpipe Core Adjustments**:
   - Revised logic to handle the merging of `pipe.json` configurations, preserving user-configured values while incorporating schema updates from new configurations.
   - The download and unpacking flow is improved by using a temporary directory to ensure atomicity and integrity.
   - Improvements in logic where existing pipe configurations are preserved during updates, with consideration to user-customized fields.
   - Enhancements include handling the identification of Next.js projects in the download process.

These changes aim to enhance developer experience by allowing greater flexibility in configurations, ensuring robustness in handling code/config updates, and adding analytics for more insightful frontend usage tracking.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 15 (311267b64620885c31267a122c1381faab5b0db3)</summary>

The commit with hash `311267b64620885c31267a122c1381faab5b0db3` made by Louis Beaumont on October 28, 2024, involves cleaning up the documentation. Specifically, it removes a line from the "getting-started.mdx" documentation file that provided an additional step about "building the desktop app (with pre_build script)", thus eliminating unnecessary noise from the document.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 16 (a82ba220e79c81b2e4c6bf68a43955ba714fcf72)</summary>

The commit with hash `a82ba220e79c81b2e4c6bf68a43955ba714fcf72`, authored by Louis Beaumont, removes a detailed step (6b) involving a manual build process for a desktop app on M-Series devices from the `getting-started.mdx` documentation. The removed section included setting up environment variables, copying necessary binaries (deno, screenpipe-vision, ffmpeg), downloading and initializing AVBUILD ffmpeg and ollama binaries, modifying dynamic library paths with `install_name_tool`, and finally executing a build with Tauri. The change is aimed at reducing noise in the documentation.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 17 (64567ed4ade9744c4b62cb76081ba3877fc96709)</summary>

The commit with hash `64567ed4ade9744c4b62cb76081ba3877fc96709` by Louis Beaumont involves changes in the `lib/opentelemetry.ts` file to improve error tracking. The main update is a change in variable usage:

- The `provider` variable has been renamed to `tracerProvider` throughout the `initOpenTelemetry` function.

Specifically, these changes include:
- Replacing `provider` with `tracerProvider` when creating a new instance of `WebTracerProvider`.
- Changing the function call `provider.addSpanProcessor` to `tracerProvider.addSpanProcessor`.
- Changing the function call `provider.register` to `tracerProvider.register`.
- In the global error handler, changing `provider.getTracer` to `tracerProvider!.getTracer`.

These modifications aim to improve clarity and potentially adhere to a more descriptive naming convention in the codebase.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 18 (d944a3e90e02c6e7711878073e17a49a0d4f7ae4)</summary>

The commit improves error tracking within the application by making the following changes:

1. **In `status.tsx`**:
   - Imported the `trackError` function from `opentelemetry`.
   - Enhanced the error handling logic in the `OnboardingStatus` component to include a call to `trackError` when an error is encountered. This function logs the error with additional context, including the operation type and whether the Chinese mirror is being used.

2. **In `opentelemetry.ts`**:
   - Introduced a new `trackError` function that logs errors using OpenTelemetry. This function starts a tracing span for the error, sets the status to indicate an error, attaches error type and message attributes, and optionally includes additional attributes.
   - Added `getTracerProvider` to manage an instance of `WebTracerProvider`, ensuring it's initialized only once.

3. **In `package.json`**:
   - Added a `prebuild` script entry to the `scripts` section to run `bun scripts/pre_build.js` before building the application.

These changes aim to improve the application's ability to track and log errors in a structured manner using OpenTelemetry, potentially aiding in better debugging and monitoring.
</details>

------------------------------------------------------------------------

# Overall Summary of Changes

The summary of the git changes presents a variety of updates across different components of a project, primarily focusing on version bumps, architecture-specific changes, documentation cleanup, and the introduction of error tracking enhancements.

1. **Homebrew Formula Updates**:
   - The `screenpipe` package Homebrew formula was updated for both `aarch64-apple-darwin` and `x86_64-apple-darwin` architectures. The version was incremented from 0.1.96 to 0.1.97, with corresponding SHA256 checksum updates for both architectures to ensure file integrity during downloads.

2. **Removal of Entelligence Integration**:
   - The project codebase was cleaned of Entelligence-related components, including script imports and Content Security Policy entries in various files.

3. **Performance Improvements**:
   - The commit introduces support for Intel's Math Kernel Library (MKL) to enhance performance. This includes script modifications, GitHub Actions workflows changes, and updating the `Cargo.toml` file to incorporate a new feature set for MKL dependencies.

4. **Documentation Changes**:
   - Several adjustments were made to the `README.md` file and other documentation files, focusing on simplifying titles and removing redundant sections.
   - Steps related to the build process in documentation were removed to streamline content.

5. **GitHub Repository Management**:
   - Updates were made to the pull request template to streamline submission and review processes, emphasizing efficient testing and integrating user feedback.

6. **New Feature Addition**:
   - A file `integrations.mdx` was added to outline existing and future integration plans, although it's marked as a work in progress.

7. **Enhancements in Configurations and Frontend Analytics**:
   - Various changes to enhance the functionality of a Twitter autopilot example and improve configuration handling for the `screenpipe` project.
   - Introduction of frontend analytics for tracking user behavior using `posthog-js`.

8. **Error Tracking Enhancements**:
   - Significant improvements in error tracking were made by utilizing OpenTelemetry, fixing naming conventions, and ensuring better error logging and monitoring within the application code.

Overall, these changes enhance the efficiency, performance, and usability of the project while removing deprecated components and clarifying documentation.
