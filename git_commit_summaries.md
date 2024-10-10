# Git Commit Summaries

-----------------------------------------------------------------------
-----------------------------------------------------------------------
 
PRESS CMD+SHIFT+V TO VIEW IN MARKDOWN
 
_______________________________________________________________________
-----------------------------------------------------------------------
Total number of commits: 8

<details>
<summary>Summary for commit 1 (cdbd6472d0f9420fe4e378dd10ac9e8a07cdc902)</summary>

This commit, identified by hash `cdbd6472`, introduces integration tests for an end-to-end "screenpipe" feature, which includes both audio and screen functionalities for Linux and Windows environments. The commit adds several shell scripts and configuration files to automate testing and setup within GitHub Actions.

**Key Additions:**
1. **New Binary:**
   - `.github/scripts/audio_test.wav`: A binary file for audio testing purposes.

2. **Shell Scripts:**
   - `check_logs.sh`: Validates logs for errors such as crashes and confirms expected service startups.
   - `install_dependencies.sh`: Installs required dependencies on Linux using `apt-get`.
   - `run_screenpipe.sh`: Executes the screenpipe application and logs the output.
   - `setup_audio.sh`: Configures audio settings using PulseAudio for Linux.
   - `setup_display.sh`: Sets up a virtual display environment using Xvfb and Openbox for Linux.
   - `stop_screenpipe.sh`: Stops the screenpipe application.
   - `test_audio_capture.sh`: Tests audio capture capabilities by playing a sample audio file and checking logs.
   - `test_ocr.sh`: Tests OCR functionality by creating and displaying an image to be recognized.
   - `verify_tesseract.sh`: Confirms the installation and availability of Tesseract OCR.

3. **GitHub Actions Workflows:**
   - `linux-integration-test.yml`: Defines a workflow for running integration tests on Ubuntu, setting up dependencies, configuring a virtual display and audio, building the CLI, running tests, and capturing logs.
   - `windows-integration-test.yml`: A similar workflow for Windows, including steps for handling Windows audio services and using Chocolatey for dependency installation.

Overall, these changes aim to automate the process of testing the screenpipe feature's functionality on both Linux and Windows, ensuring compatibility and reliability across platforms.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 2 (08ca82909681cd34ee94d9fbc68cf417e6977601)</summary>

The recent commit primarily adds audio normalization for input devices, while also including several other updates and fixes:

1. **Version Update**: The version of the `screenpipe-app` package is incremented from `0.3.1` to `0.3.2`.

2. **Dependency Changes**:
   - Removed `pyannote-rs` from `screenpipe-audio/Cargo.toml`.
   - Added `tracing-subscriber` version `0.3.16`.

3. **Code Modifications**:
   - Changed the function `default_output_device` from asynchronous (`async`) to synchronous.
   - Integrated audio normalization only for input devices by updating the `stt` function.
   - Adjusted `VadSensitivity` speech ratio thresholds for `Low`, `Medium`, and `High`.

4. **Testing and Test Data**:
   - Added new test files `accuracy5.mp4` and `accuracy5.wav`.
   - Updated the `accuracy_test.rs`, adding a new transcription test case with these files and setting up tracing for debug purposes.
   - Adjusted test code to use `default_input_device` and synchronous `default_output_device`.

5. **Miscellaneous**:
   - Removed debug logging related to audio format detection in `pcm_decode.rs`.

Overall, the changes improve the audio processing capabilities and update the test suite to ensure accuracy and functionality.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 3 (6bb82de286f64ac82785166cbb29f4df1d41c591)</summary>

The commit by Louis Beaumont refactors the code to improve the Speech-to-Text (STT) accuracy by 4%. Key changes include:

1. **Audio Normalization**: A new function `normalize_v2` is added in `audio_processing.rs`, which normalizes the audio data by adjusting its root mean square (RMS) and peak values. This function is used in the STT process to standardize audio input.

2. **STT Modifications**: The STT functions `stt_sync` and `stt` in `stt.rs` are updated to incorporate the new normalization method. Audio data is normalized before processing, potentially enhancing accuracy.

3. **Encoding Toggle**: A new boolean parameter `skip_encoding` is added to the `stt` function, allowing control over whether audio encoding should be skipped or not.

4. **Test Updates**: The tests are modified to use the updated STT configuration. Specifically, a different audio transcription engine, `WhisperLargeV3Turbo`, is employed in tests, and the `skip_encoding` flag is set to `true` in some test scenarios.

5. **File Structure Changes**: The `audio_processing` module is added to the project, making the new normalization functions accessible in the core library.

Overall, these enhancements aim to streamline audio processing and improve STT functionality by ensuring audio input is consistently normalized and accurately transcribed.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 4 (f22a85fd9ff09c618716223ec6a0b8b162735e9e)</summary>

The commit introduces changes to the `screenpipe-audio` project to add a benchmarking feature focused on measuring transcription accuracy. Here is a summary of the modifications:

1. **Dependencies Update in `Cargo.toml`:**
   - Added new dependencies: `pyannote-rs` version `0.2.7`, `strsim` version `0.10.0`, and `futures` version `0.3.31`. These additions likely support the new transcription accuracy testing functionality.

2. **New Test Data Files:**
   - Added several new audio files to the repository for use in testing transcription accuracy:
     - `accuracy1.wav`
     - `accuracy2.wav`
     - `accuracy3.wav`
     - `accuracy4.wav`
     - `accuracy4.mp4`

3. **New Test Implementation `accuracy_test.rs`:**
   - A new test file `accuracy_test.rs` was created to automate transcription accuracy testing.
   - The test uses various Rust async features and libraries (`tokio`, `futures`) to execute concurrently.
   - It tests audio files against expected transcriptions by processing them through the Screenpipe Audio's speech-to-text pipeline.
   - The transcription accuracy is computed using the Levenshtein distance, and a minimum threshold of 55% average accuracy is asserted in the test.

By adding this benchmarking test, the project now has a structured way to evaluate the accuracy of its speech-to-text transcription capabilities, which is crucial for validating and improving its performance.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 5 (8212fbefec67a59fd3f93eef96e1ce948082ba23)</summary>

The commit titled "revert mkl" by Louis Beaumont modifies the `.github/workflows/release-app.yml` file, specifically affecting how certain platforms are built in a GitHub Actions workflow. The changes remove the `mkl` features from the build arguments for both the `ubuntu-22.04` and `windows-latest` platforms. 

- For `ubuntu-22.04`, the previous build argument `--features mkl` is now removed.
- For `windows-latest`, the previous build argument `--target x86_64-pc-windows-msvc --features mkl` is changed to `--target x86_64-pc-windows-msvc`, effectively removing the `mkl` feature.
- Additionally, the export of `RUSTFLAGS` with `-C target-cpu=native` for both `ubuntu-22.04` and `windows-latest` has been removed. 

These changes suggest a rollback or removal of support/build optimization related to Intel's Math Kernel Library (MKL) for these platforms.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 6 (1a5838ebc1214c323a7fe0b7b17c873ab2625bef)</summary>

The Git commit `1a5838ebc1214c323a7fe0b7b17c873ab2625bef`, made by Louis Beaumont on October 8, 2024, includes updates to the GitHub Actions workflow file `release-app.yml`. This commit adds platform-specific `RUSTFLAGS` configuration for Ubuntu 22.04 and Windows:

- For the `ubuntu-22.04` platform, it sets `RUSTFLAGS` to `-C target-cpu=native`.
- For the `windows-latest` platform, it also sets `RUSTFLAGS` to `-C target-cpu=native`.

These changes are likely meant to optimize the build process by using CPU-specific instructions on these platforms.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 7 (392dc3bfeec7013a9097a9a6f0831d3d4c3999b3)</summary>

The commit by Louis Beaumont on October 8, 2024, involves two main changes:

1. **GitHub Workflow Updates**:
    - In the `.github/workflows/release-app.yml` file, the build arguments for both Ubuntu and Windows platforms were modified. The `cuda` feature was removed from the build arguments, leaving only `mkl` specified. This change appears to be a refinement of the build options, possibly due to issues or updates with CUDA.

2. **Pre-build Script Enhancement**:
    - In the `screenpipe-app-tauri/scripts/pre_build.js` file, the command to download FFMPEG on the Windows platform was updated. The new command includes additional options for `wget`, such as `--tries=5`, `--retry-connrefused`, `--waitretry=10`, and `--secure-protocol=auto`. These options improve the robustness of the download process by specifying retry behavior and protocol security. 

These changes collectively aim to improve the build and test process, focusing on reliability and adjusting build feature sets.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 8 (83d81a3e83ed0e205af672fae1731b211bb6211a)</summary>

The commit titled "fix: build and test" made several changes to the codebase:

1. **GitHub Workflow:** 
   - In the `.github/workflows/release-app.yml` file, a change was made to the caching mechanism. The key used for cache identification was updated from `${{ matrix.platform }}-${{ matrix.args }}-pre-build` to `${{ matrix.platform }}-${{ matrix.target }}-pre-build`.

2. **Version Update:**
   - The version number in `screenpipe-app-tauri/src-tauri/Cargo.toml` was incremented from `0.3.0` to `0.3.1`.

3. **Test Module Imports:**
   - In the `screenpipe-audio/tests/core_tests.rs` file, an import statement was modified. The import of `stt` from `screenpipe_audio::stt` no longer imports `self`. Additionally, `AudioDevice` was removed from the imports, suggesting it might not be necessary in the test file anymore.

4. **Example Code:**
   - In `screenpipe-core/examples/llama.rs`, a configuration-dependent code block was added. If the "llm" feature is not enabled, it now prints a message indicating that the LLM feature is not available.

5. **Server Code Clean-up:**
   - In `screenpipe-server/src/bin/screenpipe-server.rs`, the import of `highlightio::HighlightConfig` was removed, and an unused item `Log`, `Metadata`, and `Record` from the `log` crate were cleaned up from the imports.

These changes collectively improve the build configuration, update the versioning, clean up code imports, and handle feature-specific conditions in the code.
</details>

------------------------------------------------------------------------

# Overall Summary of Changes

Here's a concise summary of the Git changes described:

1. **Integration Tests for Screenpipe:**
   - Introduced integration tests for the "screenpipe" feature, which includes audio and screen functionalities on Linux and Windows.
   - Several shell scripts and GitHub Actions workflows were added to automate testing and setup, enhancing cross-platform compatibility.

2. **Audio Normalization and STT Improvement:**
   - Added audio normalization for input devices to improve Speech-to-Text (STT) accuracy.
   - The STT functions were updated, resulting in a 4% increase in transcription accuracy.

3. **Benchmarking Feature:**
   - Introduced new dependencies and a test suite to benchmark transcription accuracy, using audio test files to compute metrics like Levenshtein distance.

4. **GitHub Actions Adjustments:**
   - Removed Intel MKL support for specific platforms, reverting build optimizations.
   - Refined build configurations, including platform-specific `RUSTFLAGS` and removal of the CUDA feature in workflows.

5. **Project Updates:**
   - Version updates and code refactoring for better code management and feature handling.
   - Enhanced robustness of scripts downloading dependencies, such as FFMPEG, with additional options to handle connectivity issues.

These changes focus on improving cross-platform support, enhancing audio processing capabilities, and refining the build and test processes for better reliability and performance in the screenpipe feature.
