# Git Commit Summaries

-----------------------------------------------------------------------
-----------------------------------------------------------------------
 
PRESS CMD+SHIFT+V TO VIEW IN MARKDOWN
 
_______________________________________________________________________
-----------------------------------------------------------------------
Total number of commits: 8

<details>
<summary>summary for commit 1 (2ff0a4a10e73780742c8206694390219af386daa)</summary>

The git commit with hash `2ff0a4a` by Louis Beaumont contains changes aimed at fixing the UI. The specific changes are:

1. **Version Update:**
   - The version of the `screenpipe-app` package in `Cargo.toml` was updated from `0.11.7` to `0.11.8`.

2. **Code Refactoring in `run_ui_monitoring_macos.rs`:**
   - The `tokio::process::Command`, `tokio::io::{AsyncBufReadExt, BufReader}`, and `std::process::Stdio` imports have been reordered for consistency.
   - The `log` imports were reformatted to consolidate them into one line.
   - Simplified the determination of the binary name by directly setting `binary_name` to `"ui_monitor"` instead of branching with `cfg!(target_arch)`.
   - Ensured the end of the file has a newline character. 

Overall, the commit includes a version increment and some refactoring of the Rust code for clarity and potential performance improvement.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 2 (4a021800c23196b23c4fde1831419778798704f3)</summary>

The commit message indicates that build issues related to `libknf` were fixed. Specifically, the following changes were made:

- In the `Cargo.toml` file for the `screenpipe-audio` project, the source of the `knf-rs` dependency was updated. The Git URL for `knf-rs` was changed from `https://github.com/thewh1teagle/knf-rs.git` to `https://github.com/Neptune650/knf-rs.git`.

This update may have been required to resolve the build errors, possibly because the new repository provides a corrected or updated version of the library.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 3 (c5bc3719daf6b6870bfdf84e931ef0d91aec67f7)</summary>

The commit with hash `c5bc3719daf6b6870bfdf84e931ef0d91aec67f7` authored by Louis Beaumont removes support for building a Linux version of the application in the GitHub Actions workflow. Here's a summary of the changes:

1. **Workflow Updates**:
   - The `publish-tauri` job for Linux has been removed from the `.github/workflows/release-app.yml` file. Previously, there was a section dedicated to running builds on Ubuntu 22.04 which has now been deleted.
   - Sections that defined Linux-specific build arguments and the installation of Linux dependencies have been removed. This includes removing the job steps for installing various packages needed specifically for Linux builds.

2. **Unused Sections Removed**:
   - Sections related to freeing disk space on Ubuntu were entirely removed.
   - Lines with conditional logic specific to Ubuntu in the build environment variables have been simplified to remove references to the Linux platform.

3. **General Streamlining**:
   - The workflow has been aligned to only support macOS and Windows builds moving forward.

These changes suggest a decision to streamline the build process by focusing only on macOS and Windows, citing reasons like high effort and low interest in maintaining Linux support.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 4 (59b325bab4c506131f2db710a00bb8f0e93a3283)</summary>

The Git commit made by Louis Beaumont on November 22, 2024, includes the following changes related to build fixes in two Cargo.toml files:

1. **screenpipe-app-tauri/src-tauri/Cargo.toml**:
   - The version of the package `screenpipe-app` has been incremented from `0.11.6` to `0.11.7`.

2. **screenpipe-audio/Cargo.toml**:
   - Added a new dependency `ort-sys` with the version `2.0.0-rc.8`. 

These changes suggest a version update in one package and the addition of a necessary dependency for another, likely to address issues during the build process.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 5 (f3f5e2757ee33a8425fe996b94936ae90134bbc0)</summary>

The recent commit to the `README.md` file, authored by Louis Beaumont, involves a simplification in the *news* section. It has removed some older news entries from September 2024 and October 2024:

1. Deleted mention of screenpipe's capability to be used in China without a VPN and support for Chinese OCR.
2. Removed the statement about 150 users running screenpipe continuously.
3. Omitted the release of screenpipe's documentation in September 2024. 

This leaves a more concise list of key highlights and developments related to the screenpipe project.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 6 (93f949a6624154594f137f8ea31301241e6fee2b)</summary>

The commit made by Louis Beaumont on November 21, 2024, updates the `README.md` file. Specifically, it changes the date of a news item: "screenpipe is number 1 github trending repo (again)" from September 2024 to November 2024.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 7 (bd338ead412542f8bd98ec54c5a8c3cc4b692a1d)</summary>

The commit with hash `bd338ead412542f8bd98ec54c5a8c3cc4b692a1d` made by Louis Beaumont on November 21, 2024, updates the `README.md` file. A new bullet point was added under the "*news* 🔥" section. The added line notes that in September 2024, "screenpipe" became the number 1 trending repository on GitHub again, with a link to a status update on X (formerly Twitter).
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 8 (d10ca473593b7ec80abc123eadb91b623c9d401d)</summary>

The commit made by Louis Beaumont on November 21, 2024, removes two lines from the `Cargo.toml` file in the `screenpipe-vision` directory. Specifically, it deletes a dependency entry specific to the MSVC (Microsoft Visual C++) target environment, which included the `vcpkg` dependency version "0.2".
</details>

------------------------------------------------------------------------

# Overall Summary of Changes

Here's a summary of the described git changes:

1. **UI Fixes and Version Updates:**
   - A commit with hash `2ff0a4a` involved changes primarily aimed at fixing the UI. The version for the `screenpipe-app` was updated to `0.11.8` and Rust code in `run_ui_monitoring_macos.rs` was refactored for improved clarity and potential performance gains. Build issues related to `libknf` were also addressed by changing the source URL for the `knf-rs` dependency in the `screenpipe-audio` project.

2. **Removal of Linux Build Support:**
   - In a commit with hash `c5bc3719daf6b6870bfdf84e931ef0d91aec67f7`, Linux build support was removed from the GitHub Actions workflow. This included deleting the Linux specific jobs, dependencies, and any related conditional logic, thereby simplifying the workflow to focus on macOS and Windows builds.

3. **Build Fixes and Version Changes:**
   - A separate commit updated the version for `screenpipe-app-tauri` to `0.11.7` and added a new `ort-sys` dependency to `screenpipe-audio`, suggesting efforts to resolve build issues.

4. **README Updates:**
   - Several commits involved updating the `README.md` file. Older news items were removed to make the document more concise, while another update involved changing the date regarding a GitHub trending news item and adding a line about screenpipe’s ranking in September 2024 as the number one trending repository.

5. **Dependency Changes:**
   - A commit removed MSVC-specific dependencies from the `Cargo.toml` in the `screenpipe-vision` directory, likely as part of simplifying or cleaning up unnecessary dependencies.

Overall, these changes reflect efforts to improve the application's build process, UI, and documentation while eliminating support for certain environments where maintenance is deemed unnecessary.
