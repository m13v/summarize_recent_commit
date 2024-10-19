# Git Commit Summaries

-----------------------------------------------------------------------
-----------------------------------------------------------------------
 
PRESS CMD+SHIFT+V TO VIEW IN MARKDOWN
 
_______________________________________________________________________
-----------------------------------------------------------------------
Total number of commits: 9

<details>
<summary>Summary for commit 1 (78a8db7b21c7421f4f6e767ce692e858c65fa31f)</summary>

The commit `78a8db7b21c7421f4f6e767ce692e858c65fa31f` by Louis Beaumont introduces several UI improvements and refactorings in a project related to a screen recording application. Here is a summary of the changes:

1. **New Component**: Added a new file `cli-command-dialog.tsx`, which contains the `CliCommandDialog` component responsible for generating and displaying CLI commands based on current settings. It includes functionality to copy these commands to the clipboard.

2. **Refactoring**: Removed duplicate code for generating CLI commands from `recording-settings.tsx` and instead, integrated the newly created `CliCommandDialog` component. This reduces redundancy and centralizes the CLI command generation logic.

3. **UI Adjustments**: 
   - Improved button placement and dialog components across various files like `recording-settings.tsx` and `screenpipe-status.tsx`.
   - Included the `CliCommandDialog` component in multiple places to provide consistent access to the CLI command throughout the UI.

4. **Code Cleanup**: Removed commented-out code and unnecessary state management from components to streamline the implementation and improve maintainability.

5. **Visual and UX Enhancements**: 
   - Updated dialogs and buttons for better visual consistency.
   - Updated health status messages and tooltips to improve user guidance and troubleshooting steps.

6. **Sidecar Logic Simplification**: Simplified the logic in `sidecar.rs` related to determining the path for screenpipe execution and cleaned unnecessary environment variable manipulations.

These changes aim to enhance the user experience and maintainability of the code by streamlining the command generation process and enhancing UI elements for clarity and ease of use.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 2 (929ed254447f10c9cd6d674914258eaf67eeee79)</summary>

The recent git commit introduces several changes across different files related to audio processing and speech-to-text functionalities:

1. **Whisper Decoder Enhancements:**
   - Added a repetition penalty feature to the Whisper decoder. This involves tracking recent tokens using `token_history` and applying a penalty to the logits during token generation, reducing the probability of repeating recent tokens.
   - Introduced a new function `apply_repetition_penalty` to adjust logits based on recent tokens.

2. **Audio Processing Adjustment:**
   - Simplified the creation of `padded_audio` by directly converting the input audio slice to a vector and appending zeros, replacing the manual loop used previously.

3. **Code Cleanup and Refinements:**
   - Removed unnecessary code redundancy, such as eliminating the explicit closure in `unwrap_or_else` for `deepgram_api_key`.
   - Consistently formatted comments and code in `vad_engine.rs` to improve readability and maintain consistency.

Overall, these changes enhance the functionality of the Whisper decoder by implementing a mechanism to reduce repeated token generation and streamline some aspects of the audio processing code.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 3 (54e74fa823e35819d5d7850b9e263ea556a99453)</summary>

The commit titled "fix: revert health check" by Louis Beaumont on October 17, 2024, makes several changes to the project's codebase:

1. **GitHub Actions Workflow**:
   - In the `.github/workflows/release-app.yml` file, the build arguments for MacOS platforms have been updated to include the `beta` feature, resulting in the lines:
     - `--target aarch64-apple-darwin --features metal` to `--target aarch64-apple-darwin --features metal,beta`
     - `--target x86_64-apple-darwin --features metal` to `--target x86_64-apple-darwin --features metal,beta`

2. **Screenpipe Status Component**:
   - The `screenpipe-status.tsx` file sees the removal of the `forceRestartHealthCheck` function. This includes:
     - Deleting its import and definition within the file.
     - Removing the function call `forceRestartHealthCheck()` after invoking certain screenpipe commands.

3. **useHealthCheck Hook**:
   - In `use-health-check.tsx`:
     - Removed the `useCallback` for `fetchHealth` and the `forceRestartHealthCheck` function.
     - The `fetchHealth` function is now a simple async function instead of a callback.
     - The interval in `useEffect` no longer depends on `restartKey`.
     - The `restartKey` state and references to `forceRestartHealthCheck` have been eliminated.

4. **Cargo.toml Update**:
   - The version number of the `screenpipe-app` package is incremented from `0.5.96` to `0.5.97`.

Overall, these changes revert a health check mechanism and update the package version, while also updating build features in the release workflow configuration.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 4 (8530cffc12c57ac8f47019b86e9d2dc3416a5cc2)</summary>

The commit made by Louis Beaumont on October 17, 2024, includes changes to the GitHub Actions workflow file `release-app.yml`. The modifications focus on updating the platform names for different build environments:

1. Changed the platform name for the Ubuntu build from `"big-linux"` to `"ubuntu-22.04"`.
2. Changed the platform name for the Windows build from `"big-windows"` to `"windows-latest"`.

These updates likely aim to clarify or update the specifications for the build environments used in the workflow.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 5 (6f90cd3fe8e30ab06e209fed06a394cd279d6005)</summary>

The commit made by Louis Beaumont, with commit hash `6f90cd3`, includes the following changes:

1. **Cargo.toml Update:**
   - The version of the `screenpipe-app` package was updated from `0.5.91` to `0.5.96`.

2. **Code Changes in `sidecar.rs`:**
   - Removed the retrieval of the `dataDir` value from the `store`. The corresponding logic to handle the `dataDir` was also removed, simplifying the code.
   - Specifically, the code no longer fetches the `dataDir` from the store or checks if it is set to "default". The conditional logic that added `--data-dir` and its value to the arguments has been removed.

These changes suggest a refactoring to simplify the code concerning data directory handling in the `spawn_sidecar` function, likely to address a build issue, as per the commit message "fix: build".
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 6 (3939a9e5f8db7625f11217d3682b4d0edac3488f)</summary>

The git commit titled "fix macos version script" by Louis Beaumont involves changes to the `.github/workflows/release-app.yml` file, specifically the configurations for macOS platforms. The `args` for both Arm-based Macs (M1 and above) and Intel-based Macs have been updated. The feature flag `beta` has been removed from the `--features` option. Now, both configurations use `--features metal` instead of `--features metal,beta`.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 7 (a6e9d50e1e8b154904054721a53e1d52efb297d7)</summary>

The commit `a6e9d50e1e8b154904054721a53e1d52efb297d7` by Louis Beaumont updates the GitHub Actions workflow file `release-app.yml` to fix the version update script for macOS. The changes involve modifying the `sed` command used to update the version in `Cargo.toml`. The script now checks for the `OSTYPE` environment variable to determine if it's running on macOS (`darwin`) and adjusts the `sed` syntax accordingly by adding an empty `''` argument required for macOS compatibility. Additionally, it updates some YAML entries to use double quotes instead of single quotes in descriptions.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 8 (aaa881c15682b169e373f5dc509e702eca0c5fd7)</summary>

The commit made by Louis Beaumont updates the GitHub Actions workflow file `release-app.yml`. The following changes have been made:

1. The platform label for the `ubuntu-22.04` runner has been changed to `big-linux`.
2. The platform label for the `windows-latest` runner has been changed to `big-windows`.

These changes suggest an update to use 64-core runners, potentially indicating a shift to more powerful build environments labeled as "big-linux" and "big-windows".
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 9 (1c2047754ee7180105c25950348456468f1d080e)</summary>

The commit titled "vercel like revert version ci" updates a GitHub Actions workflow located in the file `.github/workflows/release-app.yml`. Here are the specific changes:

1. **Inputs for Workflow Dispatch:**
   - New optional inputs are added for the `workflow_dispatch` event:
     - `commit_hash`: Describes the specific commit hash to build from, if provided.
     - `version`: Indicates the version to set in the `Cargo.toml` file, required if a `commit_hash` is provided.
   
2. **Job Modifications:**
   - In the `draft` job and another unnamed job, there are modifications to the `actions/checkout` step:
     - The `ref` parameter is set to `${{ github.event.inputs.commit_hash || github.ref }}`, allowing the workflow to check out a specific commit if `commit_hash` is specified, or the default ref otherwise.
   - A new step for updating the version in the `Cargo.toml` file is added in both jobs:
     - This step uses `sed` to replace the version number in `Cargo.toml` if both `commit_hash` and `version` inputs are specified.

These updates enable a more flexible workflow that can be triggered to build from specific commits and update version numbers dynamically when necessary.
</details>

------------------------------------------------------------------------

# Overall Summary of Changes

Here's a consolidated summary of the recent git changes made by Louis Beaumont:

1. **UI Improvements and Refactoring** (`78a8db7b21c7421f4f6e767ce692e858c65fa31f`):
   - A new `CliCommandDialog` component was added to centralize CLI command generation.
   - Removed redundant code and streamlined UI elements for better user experience and maintainability.
   - Performance and visual improvements across multiple components related to screen recording.

2. **Audio Processing Enhancements**:
   - Introduced a repetition penalty to reduce token repetition in the Whisper decoder.
   - Simplified audio processing by refactoring code for creating padded audio.

3. **Health Check Reversion**:
   - Reverted health check functionalities in the `screenpipe-status.tsx` and related files.
   - Updated the version in `Cargo.toml` and adjusted GitHub Actions workflows to include specific build features.

4. **Version and Build Environment Updates**:
   - Updated platform labels in the GitHub Actions workflow for clarity and updated runner specifications.
   - Fixed macOS compatibility in scripts and adjusted feature flags for build configurations.

5. **Workflow Flexibility Enhancements**:
   - Added options to build from specific commits and update version numbers dynamically in GitHub Actions workflows.
   - These updates aim to increase the flexibility and efficiency of the CI/CD processes.

These collective changes focus on improving the codebase's maintainability, user interface experience, and build processes.
