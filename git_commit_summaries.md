# Git Commit Summaries

-----------------------------------------------------------------------
-----------------------------------------------------------------------
 
PRESS CMD+SHIFT+V TO VIEW IN MARKDOWN
 
_______________________________________________________________________
-----------------------------------------------------------------------
Total number of commits: 19

<details>
<summary>Summary for commit 1 (ec408a0b8e1b9c96a5d84f2bac7307e3ebfb4877)</summary>

The commit `ec408a0b8e1b9c96a5d84f2bac7307e3ebfb4877` is a merge commit authored by Louis Beaumont. It merges a pull request (#329) by the user kerosina. The primary change introduced by this merge ensures that the system does not panic if the host computer lacks a default input device. The commit was made on September 14, 2024.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 2 (479604c2d16167c0f2c11055db78f4119b8d1e3a)</summary>

This Git commit mainly involves refactoring how video merging is handled within the `screenpipe-server` application by consolidating several separate functions and simplifying related processes.

Key changes include:

1. **Refactoring Video Merging Endpoint**:
   - The `merge_frames_from_video` function and its associated request and response structures have been replaced with `merge_videos`, simplifying the merge operation to handle multiple video paths.
   - This includes changes in `screenpipe-server/src/server.rs` and `screenpipe-server/src/video_utils.rs`.

2. **Update to Merge Handler**:
   - The `merge_frames_handler` function within `screenpipe-server/src/server.rs` has been updated to call `merge_videos` instead of `merge_frames_from_video`.
   - This handler now accepts a `MergeVideosRequest` and returns a `MergeVideosResponse`.

3. **Shell Script Changes**:
   - The example shell script has been updated to incorporate changes in how video search and merging are handled.
   - The script now gathers multiple video paths from several search responses and constructs a JSON payload for merging videos.

4. **Removed Frame Merging Logic**:
   - Removed detailed logic for extracting specific frames and merging them (e.g., `extract_frame_ranges`, `merge_frames_into_video`) from `screenpipe-server/src/video_utils.rs`.

5. **Improved Logging**:
   - Reduced log levels from `info` to `debug` for some non-critical messages and changed wording for consistency, such as using lowercase for errors.

6. **Simplification in File Handling**:
   - Simplified how temporary files and paths are handled during merging operations.
   - Improved error handling and file cleanup post-merge operations.

Overall, these changes enhance the video merging functionality by making it more straightforward and easier to manage, focusing on merging entire videos rather than specific frames.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 3 (7ece166d4d635234de786e7886435d9961809c3f)</summary>

In commit `7ece166d4d635234de786e7886435d9961809c3f`, the author `kerosina` updated the `core.rs` file in the `screenpipe-audio/src` directory. The change specifically enhances the `default_input_device` function by adding error handling. Instead of unwrapping the `default_input_device` (which would panic if there is no default input device), it now returns an error with the message "No default input device detected" if there is no default input device available.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 4 (4b8fd2416096e1db8f0c79ced59e4c2d62165b1a)</summary>

This commit by Louis Beaumont introduces new experimental functionality for the Screenpipe server to generate clips from recordings. Here are the main changes:

1. **Cargo.toml**:
   - Added the `uuid` dependency.

2. **src/cli.rs**:
   - Adjusted the default fps value for `macOS` from 0.2 to 0.5.
   - Changed the default `video_chunk_duration` from 30 to 60 seconds.

3. **src/lib.rs**:
   - Introduced a new module: `video_db`.

4. **src/server.rs**:
   - Added a handler for merging frames: `merge_frames_handler`.
   - Introduced a new route `/experimental/frames/merge`.

5. **src/video.rs**:
   - Changed frame calculation logic from `round()` to `ceil()`.

6. **New File: video_db.rs**:
   - Added new database query methods for fetching video frames and ordered video sequences.

7. **src/video_utils.rs**:
   - Numerous changes including introducing `MergeFramesRequest` and `MergeFramesResponse` structs.
   - Added functions to merge frames from video, check file accessibility, calculate frame ranges, and extract frames.
   - Introduced logic to merge extracted frames into a new video file.

8. **Other Noteworthy Changes**:
   - Enhanced logging and error handling.
   - Added functions for generating an output video from selected frames using temporary directories.

The update includes several necessary utilities and routing for creating video clips by merging frames from recordings, making the feature experimental and intended for further development.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 5 (e210a37312505b34a58d508fb5b545dc36c82c93)</summary>

This commit, authored by Louis Beaumont, contains two primary changes:

1. **README.md Updates**:
   - Removed instructions related to the experimental Apple native OCR and the option to use the Unstructured.io cloud OCR engine.
   - Simplified sections on disabling audio recording and using a cloud engine for audio transcription.

2. **Code Enhancements in `screenpipe-audio`**:
   - Code refactor to improve audio logging details, notably by adding `device` context to various log messages for better traceability.
   - Adjustments in the `record_and_transcribe` function to specify the data type being cast in the `extend_from_slice` method.
   - Enhanced `transcribe_with_deepgram` to include the audio device in the logging output, making the logs more descriptive regarding the device being used.
   - Additional detailed logging in the `stt` function for significant events like resampling, filtering non-speech segments, and various steps in the transcription process.

These changes aim to improve readability, logging clarity, and overall maintainability of the project.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 6 (e43fe936bb3f5e516ccb00d145615c9a0926e664)</summary>

In this commit, Louis Beaumont merged a pull request (#326) from the user "joegoldin" into the main branch. The commit ID for this merge is `e43fe936bb3f5e516ccb00d145615c9a0926e664`, and it combines changes from the parent commits `29dc3ed` and `c827c65`. This merge was completed on September 14, 2024.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 7 (c827c65fdd989a658b5272f182032e26a76e980c)</summary>

This commit (c827c65fdd989a658b5272f182032e26a76e980c) is a merge commit performed by Joe Goldin on September 13, 2024. It indicates that changes from the 'main' branch of the 'mediar-ai' repository have been merged into the current 'main' branch. The merge involves integrating the contents from two branches: one identified by commit 6dc64a3 and the other by commit 29dc3ed.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 8 (6dc64a3670a1e4eba435e097e5af161cfb19f812)</summary>

This Git commit, authored by Joe Goldin, addresses a fix related to the WAV audio sample rate. The change updates the sample rate of an audio transcription function in the `stt.rs` file within the `screenpipe-audio` project. Specifically, the sample rate has been adjusted from 16,000 Hz to 32,000 Hz in the `transcribe_with_deepgram` function. This update was cherry-picked from a previous commit identified by hash `0aa78116cb2343f72de288746e778073b0641094`.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 10 (29dc3ed9f45be44aaffe1e236a37c9a3b729ae3c)</summary>

The commit `29dc3ed9f45be44aaffe1e236a37c9a3b729ae3c`, authored by Louis Beaumont on September 13, 2024, introduces a new feature that enables keyword searches for app names and window names within the `screenpipe` project. The following changes were made:

1. **Version Update**:
   - The version in `screenpipe-app-tauri/src-tauri/Cargo.toml` was updated from `0.2.37` to `0.2.38`.

2. **Database Query Enhancement**:
   - In `screenpipe-server/src/db.rs`, adjustments were made to the way `app_name` and `window_name` parameters are handled in SQL queries.
   - The specific SQL logic was changed from using strict equality checks (`=`) to partial matches using the `LIKE` operator, enabling more flexible keyword-based searches:
     - From: `ocr_text.app_name = ?` and `ocr_text.window_name = ?`
     - To: `ocr_text.app_name LIKE '%' || ? || '%'` and `ocr_text.window_name LIKE '%' || ? || '%'`
   - These changes impact two functions within the `DatabaseManager` implementation, enhancing their ability to perform case-insensitive partial matches on the `app_name` and `window_name` fields.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 11 (b8aee0907213578b9f46023a9ac3a9e0f6ce6105)</summary>

The commit made a small UI change in the `screenpipe-status.tsx` file within the `screenpipe-app-tauri/components` directory. Specifically, the change involved updating the text within a paragraph in the `DevModeSettings` component to include additional information. The updated text now informs users that in developer mode, the backend won't automatically start when the app is launched. This change was authored by Louis Beaumont on September 13, 2024.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 12 (e5b452f2b38e567a789aa778d1392c9c0c66c1f2)</summary>

**Commit Summary:**

- **Commit ID:** e5b452f2b38e567a789aa778d1392c9c0c66c1f2
- **Author:** Louis Beaumont <louis.beaumont@gmail.com>
- **Date:** Fri Sep 13 17:32:47 2024 -0700
- **Type:** Feature
- **Description:** Introduced an environment variable to enhance logging for development purposes.

**Changes Made:**

1. **Updated Imports:**
   - Added the `env` module to the imports in `screenpipe-server/src/bin/screenpipe-server.rs`.

2. **Enhanced Logging Functionality:**
   - Modified the logging setup to include the ability to customize log levels for specific modules using the `SCREENPIPE_LOG` environment variable.
   - Added parsing logic to read directives from the `SCREENPIPE_LOG` environment variable and apply them to the logging filter.
   - Improved guidance with example usage comments:
     - `SCREENPIPE_LOG=screenpipe_audio=debug ./screenpipe`
     - `SCREENPIPE_LOG=screenpipe_audio=debug,screenpipe_vision=trace ./screenpipe`

3. **Adjusted Existing Log Directives:**
   - Removed the hardcoded `external_cloud_integrations=debug` directive from the logger setup.

This commit enhances the logging flexibility, allowing developers to dynamically set log levels for different modules via environment variables, improving the debugging and development process.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 13 (743b0bbea7d266786f5e3307e9ff308592eb5cbc)</summary>

This Git commit includes various changes across multiple files, mainly focusing on standardizing lowercase usage for UI and logging texts, adding new TypeScript definitions, introducing a new feature to purge all pipes, and making extensive tweaks to the `screenpipe-core` and `screenpipe-server` modules. Below are the key changes:

1. **CONTRIBUTING.md**:
   - Added a new rule to always use lowercase for logging and UI.

2. **TypeScript Changes**:
   - Added a new TypeScript definition file (`screenpipe.d.ts`) for globally declaring various utilities and environment variables.
   - Added a new `tsconfig.json` for TypeScript compiler options specific to a TypeScript example project.

3. **Rust Changes**:
   - **screenpipe-core**:
     - Added `dirs` dependency (version `5.0.0`) to `Cargo.toml`.
     - Removed commented-out code related to checking if a pipe is enabled.
     - Set additional environment variables like `HOME`, `CURRENT_DIR`, `TEMP_DIR`, and `PIPE_FILE`.
     - Improved error handling and logging, ensuring errors do not crash `screenpipe` but might disable broken pipes.

   - **screenpipe-server**:
     - Made all logging and UI messages lowercase.
     - Added a new command (`Purge`) in `cli.rs` to allow purging all pipes.
     - Implemented the `purge_pipes` method in `pipe_manager.rs` to delete all pipes.

Overall, the changes emphasize consistent lowercase usage in user interfaces and logging, the introduction of TypeScript support, enhanced error handling, and the new capability to purge all pipes.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 14 (9407d62ce64e706f48ad1a228b9068846a63f50e)</summary>

The commit made by Louis Beaumont on September 13, 2024, includes the following changes:

1. **GitHub Workflow Update**:
   - In the `.github/workflows/benchmark.yml` file, the section for analyzing STT (Speech-to-Text) benchmarks has been commented out. This section included configurations for running the `benchmark-action/github-action-benchmark` GitHub action to analyze STT benchmarks. The commenting out signifies that this feature is currently broken and is temporarily disabled.

2. **Test Modifications**:
   - In the `screenpipe-audio/tests/core_tests.rs` file:
     - Removed the `test_speech_to_text` test function which previously included a comprehensive test for loading an audio file and performing speech-to-text conversion with Whisper Model and WebRtcVad.
     - In the `test_record_and_transcribe` test functions, various improvements and cleanups were made, like removing unnecessary parameters from function calls and correcting assertions to check the length of audio data instead of output path.

   - In the `screenpipe-core/tests/pipes_test.rs` file:
     - Removed the `test_download_pipe_raw_file` test function which tested the downloading of a raw file from a given URL into a temporary directory and verified the presence of certain files (`main.ts`, `main.js`, `pipe.ts`, or `pipe.js`).

These changes are summarized by the commit message "fix tests," indicating a general improvement and cleanup of the test suite.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 15 (836f559ff804f81195a8c4f1debe61129755e658)</summary>

This commit by Louis Beaumont includes the following changes:

1. **Version bump**: The version number in `Cargo.toml` is incremented from `0.2.36` to `0.2.37`.
   
2. **Text color change in meeting-history.tsx**: The color of a div element indicating a live meeting in the `meeting-history.tsx` file has been changed from green (`text-green-500`) to black (`text-black`).

3. **Update to `bun.lockb`**: The binary file `bun.lockb` has been modified, but specific changes are not detailed due to the binary nature of the file.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 16 (f8484b767ea0e0991c6864857718e3436b54d522)</summary>

This commit merges a pull request (#323) into the main branch. The pull request, contributed by a user named "m13v," introduces "Meetings feature 3." Louis Beaumont is the author who performed the merge on September 13, 2024.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 17 (5dd01699620b438e86963479b87fda2d0cb61013)</summary>

### Summary of Git Changes

#### Commit Overview
- **Commit Hash**: 5dd01699620b438e86963479b87fda2d0cb61013
- **Author**: Louis Beaumont
- **Date**: Fri Sep 13 14:45:42 2024 -0700
- **Message**: "make pipe store technical preview only for now"

#### Changes in `pipe-store.tsx`
- **Modified Behavior**: The pipe store feature is restricted to a technical preview.
- **Details**:
  - Commented out or removed references to UI elements like `FeatureRequestLink`, certain console logs, and parts of the dialog header related to pipe functionality.
  - Added a centered message in the dialog stating that pipes must be enabled using `screenpipe pipe` commands or `/pipes` API.
  - Included a GitHub link for more examples.

#### Changes in `pipes.rs`
- **Modified Behavior**: Enhancements around pipe name handling and downloading pipes.
- **Details**:
  - Introduced a `sanitize_pipe_name` function using a regex to clean pipe names.
  - Updated logic to use sanitized pipe names when downloading GitHub folders or single files.
  - Enhanced error handling and sanitization for different pipe sources.

#### Changes in `screenpipe-server.rs`
- **Modified Behavior**: Improved display of monitors and audio devices in the server's output.
- **Details**:
  - Removed old print statement for monitor IDs.
  - Added detailed sections for both monitors and pipes, ensuring the output displays up to five items, with a count of additional items if more than five exist.
  - Enhanced visibility of available monitors and pipes, including status indicators for enabled/disabled pipes.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 18 (0caa32169fa69cb0e1da4f6f745a6d199a3e76a6)</summary>

The commit with hash `0caa32169fa69cb0e1da4f6f745a6d199a3e76a6` authored by Louis Beaumont on September 13, 2024, includes small UI fixes in the project. Specifically:

1. **File `app/page.tsx`:**
   - Modified the component rendering logic: Added a headline "where pixels become magic" above the `SearchChat` component when certain conditions (`settings.useOllama` or `settings.openaiApiKey`) are met.

2. **File `components/search-chat.tsx`:**
   - Removed the headline "where pixels become magic" from within the `SearchChat` component.
   - Changed the placeholder text from "Ask a question about the results..." to "ask a question about the results..." to ensure consistent casing.
   - Updated tooltip content text to have lowercase for consistent styling: "Content exceeds 30k tokens..." changed to "content exceeds 30k tokens...".

These changes focus on improving the user interface's consistency and presentation.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 19 (ab409281f82fb014f46d06c4530c822c0fc231a1)</summary>

### Summary of Git Changes

#### Commit Details
- **Commit:** `ab409281f82fb014f46d06c4530c822c0fc231a1`
- **Author:** Louis Beaumont
- **Date:** Fri Sep 13 14:20:32 2024 -0700
- **Message:** Try to fix Windows video embedding

#### Modified Files
1. `screenpipe-app-tauri/components/video.tsx`
2. `screenpipe-server/src/db.rs`

### Changes in `video.tsx`
1. **Imports Removed:**
   - `Button` from `./ui/button`
   - `Link` from `lucide-react`

2. **Sanitize File Path Function:**
   - Added a replacement to convert Windows backslashes (`\`) to forward slashes (`/`).

3. **Mime Type Function Added:**
   - `getMimeType`: Determines file type based on extension for better handling of media files.

4. **Load Media Function Update:**
   - Updated to use the new `getMimeType` function instead of hardcoding the mime type as `audio/mpeg` or `video/mp4`.

### Changes in `db.rs`
1. **Removed Debug Print Statements:**
   - Lines that printed the search progress and results length during audio search were deleted for cleaner code without debug information.

These changes are primarily focused on improving file path handling and media type determination in the Tauri component and cleanup of the server's database-related debug print statements. This should aid functionality, particularly for Windows, and ensure cleaner, more efficient logging and debugging in the server code.
</details>

------------------------------------------------------------------------

# Overall Summary of Changes

### Summary of Git Changes

#### Key Overall Changes
1. **Merge Commit: `ec408a0b8e1b9c96a5d84f2bac7307e3ebfb4877`**
   - **Author:** Louis Beaumont
   - **Date:** September 14, 2024
   - **Description:** Merged PR (#329) by user kerosina to ensure the system does not panic if no default input device is found. The change refactors video merging in the `screenpipe-server` for improved and simplified video merging.

2. **Commit: `7ece166d4d635234de786e7886435d9961809c3f`**
   - **Author:** kerosina
   - **Description:** Enhanced error handling in `default_input_device` function to return an error instead of panicking if no default input device is found.

3. **Commit:**
   - **Description:** Added experimental functionality for generating clips from recordings. Introduced new module `video_db` and updated frame calculation logic.

4. **Commit:**
   - **Description:** Updates in `README.md` and logging improvements in `screenpipe-audio` for better traceability and clarity.

5. **Commit: `e43fe936bb3f5e516ccb00d145615c9a0926e664`**
   - **Author:** Louis Beaumont
   - **Date:** September 14, 2024
   - **Description:** Merged PR (#326) authored by Joe Goldin. This includes merging changes from two branches with a commit hash `c827c65fdd989a658b5272f182032e26a76e980c`.

6. **Commit:**
   - **Author:** Joe Goldin
   - **Description:** Fix to increase WAV audio sample rate to 32,000 Hz in `screenpipe-audio`.

7. **Commit: `29dc3ed9f45be44aaffe1e236a37c9a3b729ae3c`**
   - **Author:** Louis Beaumont
   - **Date:** September 13, 2024
   - **Description:** Introduces keyword search functionality for app and window names within the `screenpipe` project using the `LIKE` SQL operator for partial matches.

8. **Commit:**
   - **Description:** Minor UI update in `screenpipe-status.tsx` to inform users that the backend won't start automatically in developer mode.

9. **Commit: `e5b452f2b38e567a789aa778d1392c9c0c66c1f2`**
   - **Author:** Louis Beaumont
   - **Date:** September 13, 2024
   - **Description:** Introduced an environment variable `SCREENPIPE_LOG` for enhanced, customized logging during development.

10. **Commit:**
    - **Description:** Standardized lowercase use for UI and logging, added new TypeScript definitions, purging all pipes feature, and enhanced `screenpipe-core` and `screenpipe-server` modules.

11. **Commit:**
    - **Description:** GitHub workflow update to disable STT benchmark analysis due to issues. Cleaned up and improved test functions in `screenpipe-audio` and `screenpipe-core`.

12. **Commit:**
    - **Description:** Version bump, UI color change in `meeting-history.tsx` from green to black, and `bun.lockb` file update.

13. **Commit:**
    - **Description:** Merged PR (#323) introducing "Meetings feature 3" by user "m13v".

14. **Commit:**
    - **Description:** Made `pipe-store.tsx` a technical preview and enhanced pipe name handling and downloading in `pipes.rs`.

15. **Commit:**
    - **Description:** Small UI fixes to improve consistency, adding a headline and modifying placeholder text in various components.

16. **Commit:**
    - **Description:** Fix related to Windows video embedding by improving file path handling and media type determination.

### Detailed Changes
1. **Video Merging Refactor**:
   - Simplified video merging by consolidating functions.
   - Updated handlers and structures to support multiple video paths.

2. **Error Handling in Audio**:
   - Modified `default_input_device` to handle errors gracefully.
   
3. **Experimental Features**:
   - Introduced new module and changed frame logic for generating clips.

4. **Logging and Documentation**:
   - Improved logging detail and simplified README instructions.

5. **Keyword Search Feature**:
   - Enabled keyword-based searches for app and window names using SQL `LIKE`.

6. **Maintenance and Enhancements**:
   - Standardized logging/UI text.
   - Introduced TypeScript support.
   - Enhanced error handling.
   
7. **Testing Improvements**:
   - Cleaned up test files and commented out broken benchmark analysis.

8. **UI/UX Changes**:
   - Updated UI text and colors for better consistency and clarity.
   - Technical preview enabled for certain features.

9. **Windows Compatibility**:
   - Improved file path handling for Windows in video embedding.
   
These changes collectively enhance the functionality, usability, and maintainability of the `screenpipe` project, focusing on error handling, logging, and user interface consistency.
