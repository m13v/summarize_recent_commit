# Git Commit Summaries

-----------------------------------------------------------------------
-----------------------------------------------------------------------
 
PRESS CMD+SHIFT+V TO VIEW IN MARKDOWN
 
_______________________________________________________________________
-----------------------------------------------------------------------
Total number of commits: 44

<details>
<summary>summary for commit 1 (9377333a0e6501fe9c8cfc478b69fb52747c6fe9)</summary>

The commit `9377333a0e6501fe9c8cfc478b69fb52747c6fe9` authored by the GitHub Actions Bot updates documentation by adding a changelog for version 0.14.2. The changes include:

1. **Improvements**: 
   - The build process has been refactored to improve dependency handling by installing `bun` in `build.rs`, instead of `pre_build`, aiding in an improved build process. This addresses issues #772 and #779.
   - Management of dynamic libraries (dylibs) has been improved by implementing build management in `build.rs`, related to issue #784.

2. The changelog update removes previous features, fixes, and other improvements for version history clarity and consistency. Among these were new features like a customizable data directory and enhanced build instructions for Linux.

3. A new changelog file, `0.14.2.md`, has been added under `content/changelogs/`, highlighting the improvements and providing a link to the full changelog comparison ([fe18f..8a8cd]). 

The overall aim is a better build and dependency management process without introducing new features or fixes in this particular update.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 2 (8a8cdf96f047316c3b216e7bf9152e3fe82535c3)</summary>

This commit refactors the installation process of the Bun Javascript runtime within a project. The main changes include:

1. **Bun Installation Moved to `build.rs`:** 
   - Previously, Bun was being installed via a script (`pre_build.js`), but this responsibility has been shifted to the Rust build script (`build.rs`).
   - The installation logic now checks if Bun is installed using a function (`is_bun_installed`) and installs it if not already present.

2. **Platform-Specific Installation Logic:**
   - For Windows, Bun is installed using npm, while for other Unix-like systems, it's installed using a shell script from the Bun website.
   
3. **Onnxruntime Installation Refactored:**
   - The onnxruntime installation logic is moved to a separate method (`install_onnxruntime`) for cleaner organization, particularly in handling Windows builds.
   
4. **Installation Script Adjustments:**
   - The PowerShell installation script (`install.ps1`) now includes a Bun installation command using a shell invocation to ensure Bun is installed.

5. **Documentation Updates:**
   - Documentation was updated to provide instructions for Windows developers, likely reflecting the new build procedures.

6. **Miscellaneous Fixes:**
   - Several build-related fixes for Windows (addressing issues with PowerShell) and a change to the audio stale threshold.

These changes aim to streamline the Bun installation process during the build phase, improve cross-platform compatibility, and enhance maintainability by separating concerns within the build configuration.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 3 (b0f3d181437a846045f8dca16c36428b44154342)</summary>

The git changes involve refactoring the way dynamic libraries (dylibs) are handled for a macOS build in a Tauri application. Previously, the process was managed within `pre_build.js` using JavaScript, where the script copied binaries and adjusted dylib paths based on architecture (arm64 and x86_64).

The updated approach consolidates these tasks into a `build.rs` file, leveraging Rust to manage these operations. The new method uses Rust's build script capabilities to copy executables and modify dylib paths directly during the build process. 

Key changes include:
- Removal of the JavaScript logic in `pre_build.js` for setting up binaries and modifying dylib paths.
- Addition of `build.rs` that uses Rust to perform equivalent actions by checking the target architecture and system (macOS) to copy the appropriate binary and update its dylib dependencies using the `install_name_tool`.

This refactor likely aims for better integration with Rust's build system, increased maintainability, and a more streamlined build process.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 4 (fe18f32f9e1e24cfb50beb4f5014e902ee56519b)</summary>

This Git commit addresses a bug fix referenced as #773. The modifications involve the following changes:

1. **File: `log-file-button.tsx`**
   - Enhanced the `getLogFilePath` function to properly handle file path creation for macOS and Linux by using forward slashes (`/`) and appending the current date to the log file name. Previously, the function primarily used the Windows-style backslashes (`\\`).

2. **File: `use-settings.tsx`**
   - Improved the conditional logic in the `getDataDir` function by ensuring that the `settings.dataDir` is not "default", empty, or undefined before returning it. This helps in making the application's settings directory handling more robust.

3. **File: `Cargo.lock`**
   - The version for the `screenpipe-app` package in the lock file is updated from `0.12.9` to `0.14.2`.

4. **File: `Cargo.toml`**
   - The version for the `screenpipe-app` in the manifest file is incremented from `0.14.1` to `0.14.2`.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 5 (2af360fb79783451a66048518631338c1dd1f56d)</summary>

The commit made by Louis Beaumont changes the audio stale threshold in the `screenpipe-server` project. Specifically:

1. **Code Adjustment:**
   - The threshold for determining whether audio data is stale was updated from 60 seconds to 3600 seconds (1 hour) in the `server.rs` file, within the `health_check` function.

2. **Search Command Modifications:**
   - Several `curl` commands in comments were updated to include a `q=test` query parameter, enhancing search functionality to include keyword searching.
   - A specific `curl` command using a fixed timestamp for search was modified to dynamically calculate times using `date` commands, enhancing flexibility by searching between 2 hours ago and 1 hour ago.

These changes collectively adjust server behavior concerning audio data's freshness and improve the sample search queries in the code documentation or comments.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 6 (e02f2f7a03c4b544e527abc2b4ba22fb85357e70)</summary>

The commit by Louis Beaumont on November 26, 2024, introduces comprehensive instructions for setting up a development environment for the project on Windows. The changes involve additions to the `CONTRIBUTING.md` file, detailing the following steps:

1. **Core Build Tools**: Instructions for installing essential development tools such as Visual Studio Build Tools, Rust, LLVM, and others using `winget` commands, along with obtaining `pkg-config`.

2. **Vcpkg Package Manager**: Steps to clone and configure Microsoft's `vcpkg` package manager, including the integration and installation of required libraries like FFmpeg.

3. **Environment Variables**: Details on setting necessary environment variables such as `PKG_CONFIG_PATH`, `VCPKG_ROOT`, and `LIBCLANG_PATH` within Windows system settings.

4. **VSCode Settings**: Configuration suggestions for a `.vscode/settings.json` file to enhance the development experience with Rust Analyzer.

5. **Building the Project**: Instructions for cloning the project repository and building both the CLI and desktop application parts of the project using `cargo` and `bun`.

Additional notes advise running PowerShell as an administrator, restarting the computer post-installation, and ensuring environment variables are set correctly. Troubleshooting tips for potential issues such as path-related errors and signing problems are also included.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 7 (48679f4ddad1158652468f24235dc3eda6e72794)</summary>

The commit with ID `48679f4ddad1158652468f24235dc3eda6e72794` by Louis Beaumont updates the documentation in the `getting-started.mdx` file. The changes include:

1. Adjusting the code block for installing and running `screenpipe` using Homebrew by modifying the tab formatting and adding a `copy` label after `bash`.
   
2. Updating the instructions for opening the timeline UI. The previous single set of `bash` commands is replaced with a new set of tabbed instructions that cover macOS, Linux, and Windows separately:
   - For macOS, it uses `open` to launch the file.
   - For Linux, it uses `xdg-open`.
   - For Windows, it uses the `powershell` command `start`.

These changes enhance the documentation by providing platform-specific instructions for better user guidance.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 8 (b3bc1a0882127ad47e40031435e10b03b60caf17)</summary>

The commit by Louis Beaumont updates the documentation in the `getting-started.mdx` file. Specifically, it adds an instruction to run `screenpipe.exe` in a new terminal after the initial setup command, enhancing guidance for users on how to proceed after installing using the provided PowerShell script.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 9 (1fbfda268ee64226f09b333de5969c0d39eb0d5e)</summary>

This commit makes changes to a PowerShell script called `install.ps1`. The following modifications have been made:

1. **PATH Update Fix**:
   - The script originally checked if `$installDir` was in the system's PATH and added it if it wasn't. This has been changed to check for `$installDir\bin` instead. It also adjusts the PATH by adding `\bin` to the directory being appended to ensure the executable path is correctly set.

2. **Installation Verification**:
   - A new section has been added to verify the installation. It constructs the full path to `screenpipe.exe` within the `bin` directory and checks if the file exists. If the executable is missing, the script throws an error indicating the file was not found after the installation.

These changes improve the reliability of the installation process by ensuring the correct directory is added to the PATH and by validating that the `screenpipe.exe` file is properly installed.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 10 (fb7c96b6e2a6d1983deff5488bd176366c414668)</summary>

The commit contains several changes to the `install.ps1` PowerShell script:

1. **Administrator Check Removal**: The code that checks if the script is run as an administrator has been removed.

2. **Version Fetching Update**: The method to fetch the latest version of the `screenpipe` has been updated. Instead of directly accessing the latest release, the script now retrieves all releases, filters out prerelease versions, and selects the first available stable release.

3. **Asset Selection**: The script now specifically searches for a Windows-compatible asset in the release (`*-x86_64-pc-windows-msvc.zip`), and throws an error if no suitable asset is found.

4. **Dynamic URL Assignment**: The download URL for the installation file is now determined from the selected asset's `browser_download_url`.

5. **Improved Output Messages**: Console messages have been updated to include more detailed download information and now use color for clarity. Successful installation messages are shown in green, and errors in red. 

6. **Error Handling**: If an error occurs during installation, the script now exits with a status of 1.

These changes improve the script's reliability in fetching and installing the correct release, as well as provide clearer user feedback.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 11 (e521b47f4ab80a607fd4e32b7c00e92387555a5b)</summary>

The git changes involve a single commit by Louis Beaumont, which fixes an issue related to the Windows build of the project. The specific change made in the codebase is an update to the `Cargo.toml` file for the `screenpipe-app` package, where the version number has been incremented from "0.14.0" to "0.14.1". This likely reflects a minor fix or update that addresses the Windows build without introducing new features.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 12 (1d8e1177caff3fbd1c04a9b19aa0fb4cd4ed6ddb)</summary>

The commit identified by `1d8e1177caff3fbd1c04a9b19aa0fb4cd4ed6ddb` by author Louis Beaumont, dated November 26, 2024, includes a fix related to building on Windows. Specifically, the commit updates the version of the `screenpipe-app` project from "0.13.9" to "0.14.0" in the `Cargo.toml` file located in the `screenpipe-app-tauri/src-tauri` directory.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 13 (b6f863873f4a27d61877318f3604bd7e55a964e1)</summary>

The git commit `b6f863873f4a27d61877318f3604bd7e55a964e1` introduces changes to the GitHub Actions workflow file `release-app.yml`. In this commit, the author, Neptune, added a new step in the workflow to install Python using the `actions/setup-python@v5` GitHub Action. This step is specifically conditioned to run only when the `matrix.tauri-args` is set to `'--target x86_64-pc-windows-msvc'`. The Python version specified for installation is `3.13`. This change helps in setting up the environment needed for MKL (Math Kernel Library) setup, which is part of the larger job configurations in the workflow.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 14 (7d1eebdad227f3fd29d5e00503ecc8f51bc29649)</summary>

The recent Git commit, authored by Louis Beaumont, primarily aims to address issue #297. The changes made in this commit are as follows:

1. **Version Update**:
   - The version number of the "screenpipe-app" package was incremented from "0.13.8" to "0.13.9" in the `Cargo.toml` file located in the `screenpipe-app-tauri/src-tauri` directory.

2. **Dependency Update**:
   - In the `screenpipe-audio/Cargo.toml` file, the dependency for the `cpal` crate, which is used for cross-platform audio capture, has been updated. The previous dependency configuration was a commented-out version and a temporary hack pointing to a specific fork and commit. It is now changed to point to the master branch of a fork hosted at "https://github.com/Kree0/cpal.git". 

These changes likely address some issues or enhancements related to audio capture functionality as indicated by the reference to `cpal`.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 15 (c1e6b21e5d5b5d7725779b2b23041aba4e714c82)</summary>

The git commit made by Louis Beaumont on November 26, 2024, includes modifications to a GitHub Actions workflow file named `release-app.yml`. The primary change involves fixing an issue related to pip on Windows systems. Specifically, the commit adds commands to first download and install pip by using `Invoke-WebRequest` to fetch the `get-pip.py` script and then running it with Python. This change ensures that pip is installed before continuing with the existing steps in the workflow, which include creating a directory and installing the `intel-openmp` package.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 16 (be502d9358d9bb064c7049692da27e7b0b8c5bc7)</summary>

The commit `be502d9358d9bb064c7049692da27e7b0b8c5bc7`, authored by Louis Beaumont, involves changes to the `install.ps1` PowerShell script. The key modifications include commenting out two `Write-Host` statements. The first commented line previously provided a success message after the installation of Screenpipe, instructing the user to restart their terminal and run Screenpipe. The second commented line was used to output an error message in red if the installation failed. There are no changes at the end of the file, other than these lines being commented out.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 17 (06aaf34bbafbac9e4bb3e72ae3f3ad8b1c9dd566)</summary>

### Summary of Git Changes

**Commit:** `06aaf34bbafbac9e4bb3e72ae3f3ad8b1c9dd566`  
**Author:** Louis Beaumont  
**Date:** November 26, 2024

**Change Summary:**

- **File Modified:** `install.ps1`
- **Key Changes:**
  - Removed the script termination when not run as an administrator: The line `exit` was removed from the block checking for administrator privileges, which previously would have terminated the script.
  - Simplified error handling: Removed the line `return 1` from the catch block, which provided a clean exit without explicitly using `Environment` or `exit` commands.
- **Purpose:** These modifications were likely made to improve the script's flow, prevent abrupt termination, and enhance user experience by not enforcing immediate exit conditions.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 18 (e8f774f13f1f9aecd1aec74aef3de67d39d5b3e9)</summary>

The commit with hash `e8f774f13f1f9aecd1aec74aef3de67d39d5b3e9` made by Louis Beaumont on November 26, 2024, includes a fix in the `install.ps1` PowerShell script. 

### Summary of Changes:
- The error handling section of the script was updated.
- Previously, if an installation error occurred, the script printed the error message using `$($_.Exception.Message)` and then exited with an error code using `[Environment]::Exit(1)`.
- The fix involved:
  - Storing the error message in a variable `$errorMessage` for clarity.
  - Modifying the exit strategy by using a `return 1`, which signifies a clean exit from the script without relying on `[Environment]::Exit` or `exit` commands.
  
This update is likely aimed at improving the robustness of the script's error handling process.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 19 (c5fde5e5d0d2318841ff04f36cfb672d319a9753)</summary>

The commit with hash `c5fde5e5d0d2318841ff04f36cfb672d319a9753`, authored by Louis Beaumont, contains a fix for the `install.ps1` PowerShell script. The changes improve error handling during the installation process. Specifically, the script now outputs the error message in red using `Write-Host` with the `-ForegroundColor Red` option, instead of using `Write-Error`. Additionally, it correctly exits with an error code of 1 using `[Environment]::Exit(1)` but prevents the exception from propagating further.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 20 (bc7bb22127ddfb927c19607d9999db12bc88f417)</summary>

The recent commit improves the documentation for installing the `screenpipe` CLI. In the "Getting Started" section of the documentation, the instructions for installing `screenpipe` using `nix` or building it from source have been removed. This suggests a streamlining of the documentation, possibly to make the installation process simpler for users by focusing on using Homebrew, as the remaining instruction is to install `screenpipe` with Homebrew.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 21 (36ab68805380c6ba5fea6f0352dbfe466d57b3d3)</summary>

The commit made by Louis Beaumont on November 26, 2024, updates the documentation for installing the Screenpipe CLI more effectively across different operating systems and introduces a new installation script for Windows.

### Key Changes:

1. **Documentation Enhancements:**
   - Introduced tabs for different OS platforms (macOS, Linux, Windows) in the `getting-started.mdx` documentation file. This helps differentiate the installation instructions for each platform.
   - The macOS section provides installation instructions using Homebrew.
   - The Linux section offers an option to install Screenpipe using Homebrew and Nix, with an alternative to build from source by installing necessary dependencies and compiling using Rust.
   - The Windows section provides a way to run a PowerShell script to automate the installation process.
   - Instructions for opening a simple timeline UI remain unchanged but are now better organized.

2. **New Windows Installation Script (`install.ps1`):**
   - A new PowerShell script was added to the repository to facilitate the installation of Screenpipe on Windows.
   - The script checks for administrative privileges, downloads the latest release from GitHub, extracts the files, and adds the installation directory to the user's PATH.
   - It ensures a smooth installation process and alerts the user if there are any issues during the installation.

Overall, these changes improve the accessibility and clarity of installation instructions for various users, catering to both technical and non-technical users across different operating systems.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 22 (efb6c50247582dcae76147e4fc1fd8eec2a54ca2)</summary>

The git commit with ID `efb6c50247582dcae76147e4fc1fd8eec2a54ca2`, authored by Louis Beaumont on November 26, 2024, includes a feature addition described as "feat: windows mkl". The changes made in this commit include an update to the `Cargo.toml` file of the `screenpipe-app-tauri` project, where the version number is incremented from `0.13.7` to `0.13.8`.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 23 (8bbb6c99334f4ba72b02c538ffab49cab65daee0)</summary>

This git commit involves several changes to improve the support of Intel's Math Kernel Library (MKL) in the project's build process:

1. **Workflow Configuration**: 
   - In the GitHub Actions workflow file (`release-app.yml`), the build arguments for the Windows x86_64 platform have been updated to include the `--features mkl` flag, indicating the use of MKL in the build process.

2. **Dependency Installation**:
   - The section for installing dependencies was slightly changed, although the visible change (`brew install ffmpeg`) appears to be a whitespace adjustment with no functional impact.

3. **MKL Library Handling**:
   - A previously commented-out block for copying MKL libraries has been activated. This involves creating a new directory and using Python's package manager, `pip`, to install `intel-openmp`, which provides the necessary MKL runtime libraries. The `*.dll` files are then copied to the `screenpipe-app-tauri/src-tauri/mkl/` directory.

4. **Windows Configuration**:
   - The `tauri.windows.conf.json` file has been updated to ensure that MKL's dynamic link libraries (`*.dll`) are included in the build output. The JSON configuration maps these libraries to the build directory.

Overall, the changes primarily focus on ensuring that the necessary MKL components are incorporated and properly handled in the Windows build process, allowing for enhanced performance or specific functionality that relies on MKL.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 24 (fbf04ea6f1b6a74975e0b14b9cf637d66db31c07)</summary>

The git commit with hash `fbf04ea6f1b6a74975e0b14b9cf637d66db31c07` by Louis Beaumont makes several changes to the repository related to fixing issues with Continuous Integration (CI). Here is a summary of the changes:

1. **Deleted Workflows:**
   - Two GitHub workflow files were deleted:
     - `perf-long-running-end-to-end.yml`: This workflow was responsible for running Linux memory tests over long durations, building and running the `screenpipe` application, and monitoring its memory usage.
     - `publish-app.yml`: This workflow handled the publishing of app releases upon tagging the repository, especially for Tauri framework applications.

2. **Modifications in `Cargo.toml` files:**
   - In the `screenpipe-audio/Cargo.toml` file:
     - The `tar` dependency version `0.4.42` was removed from the dependencies list. 
     - An entry was added to ignore `ort-sys` in the `cargo-machete` package metadata.
   - In the `screenpipe-vision/Cargo.toml` file:
     - The `once_cell` dependency version `1.19` was removed from the dependencies list.

These changes indicate a removal of specific CI workflows and a cleanup or modification of dependencies and metadata in the project's `Cargo.toml` files.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 25 (f1917a70f77ba2717c18510f893441eca4dce163)</summary>

The commit introduces changes to the GitHub Actions workflow (`release-cli.yml`) to add support for building and releasing a CLI for Linux. Here are the key updates:

1. **New Build Job for Linux**: 
   - A new job `build-linux` is added to the workflow, which runs on `ubuntu-latest`.
   - The job uses a matrix strategy to build for two targets: `x86_64-unknown-linux-gnu` and `aarch64-unknown-linux-gnu`.

2. **Setup and Dependencies**:
   - The job includes steps to check out code, set up Rust using `dtolnay/rust-toolchain@stable`, and install necessary dependencies using `apt-get`.

3. **LLVM and Cache**:
   - Installs LLVM and Clang using the `KyleMayes/install-llvm-action@v1`.
   - Sets `LIBCLANG_PATH` in the environment.
   - Uses caching for cargo to speed up builds.

4. **Building and Packaging**:
   - The CLI is built using `cargo build` with specific features and targets.
   - The version is determined from Git tags or set to `0.0.0` if none exist.
   - A deployment package is created in a tar.gz format, which includes the built CLI binary.

5. **SHA256 and Artifact Upload**:
   - Calculates the SHA256 checksum for the tar.gz packages.
   - Uploads the built artifacts using `actions/upload-artifact@v4` for `screenpipe-linux`.

6. **Releasing**:
   - Updates the `release` job to depend on `build-linux`.
   - New step to upload Linux artifacts to GitHub Releases.

Overall, these changes enable continuous integration and deployment for a Linux version of the CLI tool, similar to the existing macOS and Windows builds.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 26 (f69ae2efaa90a166afb41573eaa657119a9f0f21)</summary>

The commit made by Louis Beaumont addresses a migration issue in the `screenpipe-server` project. Here is a summary of the changes:

1. **Version Update**: The version number in `Cargo.toml` in the `screenpipe-app-tauri` project was updated from `0.13.6` to `0.13.7`. This indicates a minor update, which typically includes backward-compatible bug fixes.

2. **Migration Code Adjustment**: In `screenpipe-server/src/db.rs`, the migration function within the `DatabaseManager` has been modified. The function now includes error handling for database migration errors. If a migration error is encountered, specifically one that involves a known mismatch with migration `20241110041538`, this error will be logged and ignored rather than causing the function to fail. Other errors will continue to be returned as usual.

These changes aim to handle a specific migration issue more gracefully while maintaining the rest of the migration functionality.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 27 (4809085e1b9f23de9af53b36e796dffced77078f)</summary>

The commit 4809085e1b9f23de9af53b36e796dffced77078f by Louis Beaumont includes the following changes:

1. **Header Component Update**: In the `header.tsx` file of the `screenpipe-app-tauri` project, the `isLoading` variable logic was altered. Initially, `isLoading` was set based on the health status. It has been temporarily set to `false`, with a comment indicating it’s for testing purposes due to a prior issue.

2. **Version Bump**: The `Cargo.toml` file of the `screenpipe-app-tauri` project has been modified, incrementing the package version from "0.13.5" to "0.13.6".

3. **Database Migration**: A new SQL migration file named `20241126220600_ensure_speaker_id_column.sql` has been added to the `screenpipe-server` project. This migration checks if the `speaker_id` column exists in the `audio_transcriptions` table. If it doesn't exist, the migration adds the `speaker_id` column with a foreign key reference to the `speakers` table's id column.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 28 (5e5622c703523946eb4197d7c696bb197626bede)</summary>

The git commit authored by Louis Beaumont with the ID `5e5622c703523946eb4197d7c696bb197626bede` includes the following changes:

1. **GitHub Workflow Update**:
   - The GitHub Actions workflow file `.github/workflows/release-app.yml` was modified.
   - Two steps related to Windows binary signing were commented out:
     - The step uploading the unsigned `screenpipe` binary for signing was commented out.
     - The step handling the signing process using `signpath/github-action-submit-signing-request@v1` was also commented out.
   - This change effectively disables the Windows signing process within the workflow.

2. **Version Bump**:
   - The version of the `screenpipe-app` in the `Cargo.toml` file was incremented from `0.13.4` to `0.13.5`.

Overall, the commit disables the Windows signing process in the release workflow and updates the application's version.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 29 (db8b87c05e67cee75cb22214f6b6469efcdca23f)</summary>

The commit introduces a new feature in the `screenpipe-server` project. A message is added to the command-line interface (CLI) indicating where users can find the latest changes. The message prints a link to the project's GitHub releases page, styled in bright blue and italic, to direct users to the changelog. This change occurs in the `screenpipe-server.rs` file at line 667.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 30 (71a5e177e907934a5954de299a32a9b5d57d0193)</summary>

This commit, authored by Louis Beaumont, includes two notable changes:

1. **Version Update in Tauri Application:**
   - The version number in the `Cargo.toml` file of the `screenpipe-app-tauri` project was incremented from `0.13.3` to `0.13.4`.

2. **Migration Fix in Screenpipe Server:**
   - The SQL migration script related to adding a `speaker_id` column to the `audio_transcriptions` table was modified.
   - The original script directly added the column without checking for its existence. The updated script first checks if the `speaker_id` column already exists in the table.
   - If the column does not exist, the script adds it; otherwise, it executes a dummy query (`SELECT 1;`) to avoid altering the table unnecessarily.

These changes address a broken migration and ensure the app version is updated, likely indicating a bug fix or minor improvement reflected by the version bump.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 31 (b7d24ea1b61d26644ba99046f6f49e7aa5e27cae)</summary>

The git change involves a single commit where the version number in the `Cargo.toml` file has been updated from `0.2.3` to `0.2.4`. This change is likely made to bump the version for a new Windows build of a command-line interface (CLI).
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 32 (fdbea15a8d8d32a4bcdca8868c0b38efacf3d2bb)</summary>

In this commit, the author made changes to the GitHub Actions workflows to streamline the build and release process for a CLI application. The summary of changes is as follows:

1. **Removed `CARGO_BUILD_JOBS` Setting:** The `CARGO_BUILD_JOBS` environment variable, previously set to "12", has been removed from the `release-app.yml` workflow file. This setting was found in different sections of the workflow script but has been taken out to improve or modify the build process.

2. **Added Windows CLI Build:** A new job called `build-windows` has been introduced in the `release-cli.yml` workflow file. This job sets up the necessary tools and environment for building the CLI on Windows (targeting `x86_64-pc-windows-msvc`) using Rust. This includes steps for setting up Rust, MSVC, and LLVM, caching dependencies, building the CLI, setting the version, creating a deployment package, calculating the SHA256 checksum, and uploading the resulting artifact.

3. **Updated Release Process:** The release section now includes the `build-windows` job as part of its dependencies, alongside `build-macos` and `update-homebrew`. The artifact download step was modified to include Windows artifacts in addition to macOS artifacts. Finally, during the release upload process, Windows artifacts (ZIP files) are also uploaded to GitHub releases.

These updates enhance the build and deployment process by introducing Windows builds and optimizing existing configurations.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 33 (920e04d51044cb729ba0d38e50142570b1713757)</summary>

The commit made by Louis Beaumont on November 26, 2024, involves two main changes:

1. **GitHub Workflow Change:** The `release-app.yml` GitHub Actions workflow was modified to disable automatic releases on tags. The section in the workflow file that triggers on tag pushes (matching "v*") has been commented out.

2. **Version Update:** The `Cargo.toml` file for a Rust project was updated to increase the workspace package version from "0.2.2" to "0.2.3".

</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 34 (02e3e9fc7de6ca98aacdd5040b1483f5e00de188)</summary>

This git commit by Louis Beaumont updates the GitHub Actions workflow, specifically targeting the macOS release build process. The key changes include:

1. **Clean-up and Removal**:
   - Removed commented-out sections related to Linux and Windows build processes, streamlining the focus to the macOS flow.
   - Removed commented-out code that suggested how to run the workflow for non-macOS platforms.

2. **Workflow Enhancements**:
   - Added a new job step to upload the macOS build artifacts using the `actions/upload-artifact@v4` action.
   - Introduced a new job, `update-homebrew`, which depends on the macOS build (`build-macos`). This job is responsible for updating the Homebrew formula with the version set based on the current tag or the latest tag available.
   - Modified the release process to depend on both `build-macos` and `update-homebrew` jobs, ensuring the build and formula update are completed before proceeding with the release.

These changes appear to be aimed at tightening up the release process for macOS, focusing efforts on uploading artifacts and managing Homebrew formula updates efficiently.
</details>

------------------------------------------------------------------------

# Overall Summary of Changes

### Summary of Git Changes

#### Commit `9377333a0e6501fe9c8cfc478b69fb52747c6fe9`

**Authored by:** GitHub Actions Bot

**Main Changes:**

1. **Documentation Update:**
   - A changelog for version 0.14.2 was added to the documentation.
   - The changelog highlights improvements in the build process and management of dynamic libraries.

2. **Build Process Improvements:**
   - Refactored build process by installing `bun` in `build.rs` instead of `pre_build`.
   - Improved handling of dynamic libraries (dylibs) in `build.rs` for better cross-platform compatibility.

3. **Changelog File Addition:**
   - A new changelog file, `0.14.2.md`, was added to `content/changelogs/`.

4. **Removal of Previous Entries:**
   - Removed some previous features and fixes from the changelog for clarity.

5. **Major Refactoring:**
   - Moved Bun installation to Rust's `build.rs` for a more integrated and streamlined approach.
   - Platform-specific installation logic was implemented for different environments (Windows and Unix-like systems).

6. **Installation Script Adjustments:**
   - Changes to the PowerShell script (`install.ps1`) to include a Bun installation command and ensure proper directory paths in the PATH variable.

7. **Documentation & Misc Updates:**
   - Updated documentation to improve developer instructions, especially for Windows.
   - Helped to address several build-related issues and enhancements.

**Overall Aim:** 
To refine the build and dependency management processes for improved efficiency and clarity in the development workflow. The update did not introduce new features but focused on strengthening the infrastructure and documentation.
