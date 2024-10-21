# Git Commit Summaries

-----------------------------------------------------------------------
-----------------------------------------------------------------------
 
PRESS CMD+SHIFT+V TO VIEW IN MARKDOWN
 
_______________________________________________________________________
-----------------------------------------------------------------------
Total number of commits: 11

<details>
<summary>summary for commit 1 (21d0137a281ec213588d6595ee5cb980fe3806dd)</summary>

The commit authored by Louis Beaumont on October 19, 2024, includes a modification aimed at fixing tests in the `screenpipe-audio/examples/stt.rs` file. The changes introduce the use of the `Language` module from `screenpipe_core`. Specifically, a new language parameter, `vec![Language::English]`, has been added to a function call within the `async fn main` function.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 2 (88b6d679d4612a1f3115816a1cef2273aada953f)</summary>

The commit titled "bump wayland support" involves several changes across two `Cargo.toml` files related to updating dependencies:
1. **Version Update**: The version of the `screenpipe-app` package was incremented from `0.6.0` to `0.6.1` in `screenpipe-app-tauri/src-tauri/Cargo.toml`.
2. **Dependency Updates**:
   - In `screenpipe-vision/Cargo.toml`, the `xcap` dependency was removed from the main dependencies section but was added or uncommented for specific operating systems.
   - For Windows, `xcap` version `0.0.12` is now an active dependency.
   - For macOS, `xcap` was updated to version `0.0.13`.
   - For Linux, `xcap` was updated to version `0.0.14`.

These changes likely aim to improve Wayland support across different platforms by updating package versions and dependencies accordingly.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 3 (eb0eb04b22af7f7d797799c3549ee42710fd65e3)</summary>

The commit adds Wayland support to the `screenpipe-vision` project. The specific changes include:

1. In `Cargo.toml`, a new dependency for `xcap` is added with the source code being referenced directly from a Git repository (`https://github.com/nashaofu/xcap.git`). This replaces hardcoded version references for `xcap` in platform-specific dependency sections for Windows, macOS, and Linux where they are now commented out.

2. The specific line `"xcap = "0.0.12"` for Windows and Linux, and `"xcap = "0.0.13"` for macOS are commented out, indicating that the project now relies on the Git version of the `xcap` library, which is presumably updated for better support or additional features like Wayland.

Overall, these changes suggest that the project is accommodating a platform-independent setup by utilizing the latest `xcap` library directly from the Git repository rather than using specific versions, possibly to address compatibility with Wayland.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 4 (9222b4c97d8dbce2080976adbb28cb3a662f5e3b)</summary>

The commit made by Louis Beaumont with the ID `9222b4c97d8dbce2080976adbb28cb3a662f5e3b` includes the following changes:

1. **Version Update**: 
   - The version of the "screenpipe-app" package in its Cargo.toml file was updated from "0.5.99" to "0.6.0". This suggests a minor version release.

2. **Code Simplification**:
   - In the file `screenpipe-audio/src/stt.rs`, the import statement was modified. The `DeviceType` import was removed from the list of imported components, indicating that `DeviceType` was likely unused in this portion of the code.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 5 (3e8aa37c9bf1310362c1eb490069dd30330aa034)</summary>

This commit merges a pull request (#534) from the contributor Ezra Ellette into the main branch. The purpose of the pull request was to fix an issue related to displaying audio by converging interleaved audio samples. The merge was performed by Louis Beaumont on October 19, 2024.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 6 (78987f8733ce5d092f221df413337c2878cafe9d)</summary>

The commit modifies the function `stereo_to_mono` to `audio_to_mono` in the file `audio_processing.rs`. This function is generalized to handle audio with arbitrary numbers of channels rather than just stereo (two channels). The updated function computes mono audio by iterating over audio data in chunks corresponding to the number of channels, summing and averaging the values in each chunk to create mono samples. The capacity for the mono samples vector is calculated based on the audio length and the number of channels.

In the `core.rs` file, all usages of `stereo_to_mono` are updated to use the new `audio_to_mono` function, reflecting the change in the function's name and its updated logic. This ensures the application correctly processes audio data with varying numbers of channels.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 7 (18fbf24120e36769f3688c28f9a90d6e2bb778f5)</summary>

The commit with hash `18fbf24120e36769f3688c28f9a90d6e2bb778f5` is a merge commit authored by Louis Beaumont on October 19, 2024. It merges changes from the pull request #533, which was submitted by the user 'devnoname120'. The purpose of the merge is to apply a fix that ensures the build process ignores the `~/.wgetrc` file to avoid any interference that could occur due to user-specific configurations.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 8 (3af2ff7840014a7ba2980b6494ac3b1017a86d46)</summary>

The commit made by Ezra Ellette on Fri Oct 18, 2024, updates the `stt.rs` file in the `screenpipe-audio` project. The change involves modifying the `encode_single_audio` function call to encode audio as mono instead of the previous channel configuration. This is achieved by changing the `audio_input.channels` parameter to `1`, which specifies a single audio channel.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 9 (510650c84d7d10ba530a667f02a7e5cac85929e9)</summary>

The commit made by Ezra Ellette on October 18, 2024, involves renaming a function in the codebase. The function `converge_to_mono` has been renamed to `stereo_to_mono`. This change is reflected in two files:

1. **`screenpipe-audio/src/audio_processing.rs`**: 
   - The function definition itself is changed from `converge_to_mono` to `stereo_to_mono`.

2. **`screenpipe-audio/src/core.rs`**:
   - All instances where the `converge_to_mono` function was called have been updated to use `stereo_to_mono` instead.

The purpose of this change is likely to improve clarity and accuracy in describing the function's behavior.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 10 (23be7622a02457440c776b423d3fc563035e5834)</summary>

The git commit with ID `23be7622a02457440c776b423d3fc563035e5834` introduces changes primarily aimed at processing audio samples to convert them to mono when dealing with interleaved audio data. Here are the key changes made in the commit:

1. **New Function Added:**
   - A new function `converge_to_mono` was added in `audio_processing.rs`. This function converts stereo (or multi-channel) audio data to mono by averaging the left and right channel samples.

2. **Integration in Core:**
   - The `converge_to_mono` function is used in several places within the `screenpipe-audio/src/core.rs` file, particularly in the `record_and_transcribe` function. The function ensures that the audio data passed around is converted to mono before it is pushed to the audio queue.

3. **Code Simplification in STT:**
   - Within `screenpipe-audio/src/stt.rs`, the code was simplified by removing a conditional operation that treated audio normalization differently based on the device type. The normalization process now uniformly uses the `normalize_v2` function, eliminating earlier complexity that only applied normalization to certain device types.

4. **Minor Fixes and Changes:**
   - Corrected the determination of output device types by moving from a specific property check to a more flexible `is_output_device` check on macOS.
   - Adjusted how channels are read to directly use the channel count as a `u16` for consistency.

These combined changes aim to enhance the processing of audio data by ensuring all audio, whether initially multi-channel or not, is treated uniformly as mono throughout the application, potentially reducing errors related to channel mismatches.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 11 (7a2d765e20a3fce9f6ab3133bcbb721ee0bbf48a)</summary>

The commit, authored by Paul, updates the `pre_build.js` script used in the build process for the `screenpipe-app-tauri`. The main change involves modifying several `wget` commands within the script to include the `--no-config` option. This addition prevents the local `~/.wgetrc` configuration file from interfering with network requests. These changes are applied across various setups within the script, including configurations for FFMPEG, Tesseract, ONNX Runtime libraries, OpenBlas, CLBlast, and Macintosh setups, as well as in the function `installOllamaSidecar`.
</details>

------------------------------------------------------------------------

# Overall Summary of Changes

Here is a summary of the described git changes:

### Commit by Louis Beaumont (October 19, 2024)
1. **Test Fix in `stt.rs`:**
   - Modified `screenpipe-audio/examples/stt.rs` to introduce a language parameter `vec![Language::English]` within the `async fn main` function.
   - Utilizes the `Language` module from `screenpipe_core`.

2. **Wayland Support and Dependency Updates:**
   - **Version Increment:** Updated `screenpipe-app` from `0.6.0` to `0.6.1` in `Cargo.toml`.
   - **Dependency Handling:**
     - Removed `xcap` from general dependencies in `screenpipe-vision/Cargo.toml`.
     - Added or updated platform-specific versions: `0.0.12` for Windows, `0.0.13` for macOS, `0.0.14` for Linux, signifying platform-specific Wayland setup.
   - Shifted to using the `xcap` library from a Git repository for consistent versioning across platforms, enhancing Wayland support.

3. **Function Generalization:**
   - Renamed `stereo_to_mono` to `audio_to_mono` in `audio_processing.rs` to handle arbitrary channel audio processing.
   - Updated all instances in `core.rs` to use the new function, allowing for varied channel processing.

### Pull Requests Merged by Louis Beaumont
1. **#534 - Audio Display Fix (Ezra Ellette):**
   - Converged interleaved audio samples in the main branch.
2. **#533 - Build Process Fix:**
   - Merged a change that ensures the build process ignores the `~/.wgetrc` configuration to prevent user-specific issues.

### Additional Commits
1. **Channel Adjustment by Ezra Ellette (October 18, 2024):**
   - Updated `stt.rs` to encode audio in mono (`channels` parameter set to `1`).
   - Renamed function `converge_to_mono` to `stereo_to_mono` across relevant files for clearer function behavior.

2. **`pre_build.js` Script Update by Paul:**
   - Added `--no-config` option to `wget` commands to prevent interference from local `~/.wgetrc` settings across various configurations in the build script.

These changes collectively aim to improve audio processing consistency, support for different platforms, and build process reliability.
