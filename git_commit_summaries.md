# Git Commit Summaries

-----------------------------------------------------------------------
-----------------------------------------------------------------------
 
PRESS CMD+SHIFT+V TO VIEW IN MARKDOWN
 
_______________________________________________________________________
-----------------------------------------------------------------------
Total number of commits: 47

<details>
<summary>Summary for commit 1 (85ee2bcd5a6044390a1df30d6dacd6351fe695e2)</summary>

The commit 85ee2bcd by Louis Beaumont on September 12, 2024, addresses a security issue in the `screenpipe-server` project. The change is in the `screenpipe-server/src/bin/screenpipe-server.rs` file, where the server's binding address has been updated. Instead of binding to all network interfaces (`0.0.0.0`), the server now only binds to the local loopback interface (`127.0.0.1`), which likely restricts access to the server, making it more secure.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 2 (508d2f4b96dbf0f52b046b0f14097780d3c84955)</summary>

This commit, authored by Louis Beaumont, introduces changes to `use-settings.tsx` in the `screenpipe-app-tauri` project. The primary updates include:

1. Importing `posthog` library to enable user identification.
2. Adding a `useEffect` to call `posthog.identify` whenever `settings.userId` changes, which helps in tracking users.
3. Some minor formatting adjustments for better code readability, specifically added line breaks in the variable assignments for `savedMonitorIds` and `savedAiUrl`.

These changes aim to fix the identification process and improve code maintainability.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 3 (27057dc332b4e38371254530cce4a519d3ec5b3d)</summary>

### Summary of Git Changes

**Commit Information:**
- **Commit:** `27057dc332b4e38371254530cce4a519d3ec5b3d`
- **Author:** Louis Beaumont <louis.beaumont@gmail.com>
- **Date:** Thu Sep 12 11:32:03 2024 -0700
- **Message:** Fix test & bench

#### Files Changed:

1. **screenpipe-audio/benches/record_and_transcribe_benchmark.rs**
   - Modified the `create_whisper_channel` function to include `VadEngineEnum::Silero` and `None` as additional parameters.
   - Updated the `worker_threads` setting and renamed `rt` to `runtime`.
   - Increased benchmark measurement time from 30 to 60 seconds.

2. **screenpipe-audio/benches/stt_benchmark.rs**
   - Added imports for `SileroVad` and `VadEngineEnum`.
   - Updated `create_whisper_channel` calls to include additional parameters `VadEngineEnum::Silero` and `None`.
   - Modified `stt` function calls to handle `vad_engine` and an additional parameter `None`.

3. **screenpipe-audio/tests/core_tests.rs**
   - Added import for `VadEngineEnum` and used it in tests.
   - Updated `stt` function calls with an additional parameter `None`.
   - Modified the `create_whisper_channel` calls to include `VadEngineEnum::WebRtc` and `None`.

4. **screenpipe-vision/benches/vision_benchmark.rs**
   - Removed unused `Arc` import.
   - Updated `OcrEngine::Tesseract` usage by removing `Arc`.

### Summary:
The commit includes updates to benchmark and test files to enhance the configuration of `create_whisper_channel` and `stt` functions with additional parameters for voice activity detection (VAD). Additionally, it makes minor tweaks for better clarity and performance, including increasing benchmark durations and fixing import issues.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 4 (43daa6b7ad66399117a8c788c411278ef13e6b45)</summary>

The commit `43daa6b7ad66399117a8c788c411278ef13e6b45` by Louis Beaumont with the message "fix windows" includes the following changes:

1. **`settings.tsx` Update**:
    - In the Tauri settings component, changed the local provider URL from `http://localhost:11434` to `http://localhost:11434/v1`.

2. **`Cargo.toml` Update**:
    - Bumped the version of the `screenpipe-app` package from `0.2.33` to `0.2.34`.

3. **`vad_engine.rs` Update**:
    - Reorganized imports for better readability.
    - Condensed the conversion of `audio_chunk` from `f32` to `i16` into a single line.
    - Reformatted a URL string for the SileroVAD model download into two lines.
    - Removed an unnecessary preprocessor directive `#[cfg(not(target_os = "windows"))]`.
    - Ensured consistency in brace style and spacing.
    - Added newline at file end.

These changes primarily aim to improve code readability and maintainability, as well as enhance compatibility with Windows.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 5 (55278bdd4c4d1681d2068b8f0d9010167c851941)</summary>

The commit by Louis Beaumont addresses and fixes an issue where the input of a forward slash ("/") stopped working in a specific component of the application. The changes are made in the `input.tsx` file located within the `components/ui` directory.

### Changes:
1. **New Event Handler**: A new function `handleKeyDown` is added to handle the slash key input specifically for input fields of type "url". When the slash key is pressed, it prevents the default behavior, inserts a slash at the cursor position, and updates the cursor position correctly.
2. **Integration of Event Handler**: The `handleKeyDown` function is integrated with the `onKeyDown` event of the input element.
3. **Minor Refactoring**: The code includes minor formatting adjustments such as adding semicolons and adjusting the import declarations for better readability and consistency.

### Purpose:
The primary purpose of this commit is to ensure that the slash key input works as intended in URL input fields by appropriately handling the keydown events.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 6 (89f7f0302325bf2ab9641c0a66921405f1bb969e)</summary>

The commit `89f7f0302325bf2ab9641c0a66921405f1bb969e` by Louis Beaumont makes several changes to the Screenpipe project. Here's a summary of the updates:

1. **Default OCR Engine for Windows**:
   - The project now defaults to using the Windows native OCR engine when on a Windows system.

2. **UI Guidance Enhancements**:
   - Improved user interface guidance by adding tooltips to explain certain input fields. Specifically, an informational tooltip has been added to the AI URL input field to guide users on what to enter and provide examples.

3. **Code Cleanup**:
   - Removed the `self_healing` feature from the resource monitor as it was dead code and not in use.
   - This involved deletion of related configuration options from the CLI, and removal of various structures and logical segments associated with `self_healing`.

4. **Dependency Cleanup**:
   - Removed unused dependencies from `Cargo.toml` files of different modules (`ndarray` from `screenpipe-audio` and `futures`, `dirs` from `screenpipe-core`).

5. **Refactor Resource Monitor**:
   - Simplified the `ResourceMonitor` struct by removing parameters related to health checks and self-healing.
   - Reduced the `ResourceMonitor::new` method to not require parameters related to the removed functionality.

By these changes, the codebase is more maintainable, with better user guidance and a cleaner dependency list.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 7 (11ca216b80da3e09ef4c8fea1cc159b91008d894)</summary>

The commit `11ca216b80da3e09ef4c8fea1cc159b91008d894` is a merge commit authored by Louis Beaumont. It merges changes from a pull request (#309) by Joe Goldin. The updates include distributing ONNX Runtime libraries with the build and re-enabling Silero VAD (Voice Activity Detector) on Windows.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 8 (b91ef6c1d7ee13291aa434d2a3ffd404d1832ba1)</summary>

The commit with hash `b91ef6c1d7ee13291aa434d2a3ffd404d1832ba1`, authored by Louis Beaumont, includes a modification to the `use-settings.tsx` file in the Tauri application. The change was made on September 12, 2024. The modification involves updating the default settings by adding a hardcoded Deepgram API key to the `deepgramApiKey` entry. The key `"7ed2a159a094337b01fd8178b914b7ae0e77822d"` is set as a placeholder, with a note indicating that it is temporarily hardcoded because they have ample credits.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 9 (cc826d2156c94a7bad1ef8a44fc781cda0e7e6b4)</summary>

The commit with hash `cc826d2156c94a7bad1ef8a44fc781cda0e7e6b4`, authored by Louis Beaumont on September 12, 2024, includes several changes to the `screenpipe-audio/src/stt.rs` file to fix the build. The changes are as follows:

1. **Refactoring of Conditional Check for `deepgram_api_key`:**
   - The conditional check for `deepgram_api_key` has been restructured for improved readability and maintainability by breaking it into multiple lines.

2. **Fixed API Key Handling:**
   - The `deepgram_api_key` is now cloned in method calls, ensuring that the original key remains unmodified.

3. **Corrected Parameter Passing in `create_whisper_channel`:**
   - In two places, the order and parameters passed to the `stt` function have been corrected by ensuring the `vad_engine` parameter is passed correctly.

These changes ensure that the Deepgram transcription engine integration and the `create_whisper_channel` function are handled correctly, likely resolving issues that were breaking the build.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 10 (2edbbebd5f4bfe2a656c881a9a1049ae27044eb4)</summary>

This commit, authored by Louis Beaumont, merges a pull request (#308) from user "joegoldin" into the main branch. The pull request adds a new feature that allows users to configure a Deepgram API key through the user interface. The merge was completed on September 12, 2024.

Key Points:
- Author: Louis Beaumont
- Date: September 12, 2024
- Pull request merged: #308
- Contributor: joegoldin
- New feature: Configure Deepgram API key in the UI
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 11 (2d450d7ecbcd6a9456597c89ad74bc0d267d9ea6)</summary>

The commit `2d450d7ecbcd6a9456597c89ad74bc0d267d9ea6` merges the 'main' branch into the 'feat-deepgram-api-key-ui' branch. The changes in the `screenpipe-audio` code involve updates to two Rust files related to the audio processing components. 

Key changes include:

1. **Configuration for VAD Engines**:
   - In `screenpipe-audio-forever.rs`:
     - Added conditional compilation attributes to use `Silero` for non-Windows platforms and `WebRtc` for Windows platforms as the Voice Activity Detection (VAD) engines.
     - The `deepgram_api_key` parameter is now passed to `create_whisper_channel`.

   - In `screenpipe-audio.rs`:
     - It previously hardcoded the usage of the `Silero` VAD engine, and this is now fixed to `WebRtc` with comments suggesting alternative choices.
     - Adjusted the code to pass the `deepgram_api_key` parameter correctly.

These changes likely aim to improve platform-specific behaviour and integrate a new feature related to the Deepgram API key.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 12 (6d7106df5cce2d248e188f1c6909dc3ac082feb8)</summary>

The commit with hash `6d7106df5cce2d248e188f1c6909dc3ac082feb8`, authored by Louis Beaumont, merges changes from a pull request (#310) submitted by user `sabrehagen`. The purpose of the pull request is to fix a type error that was causing the build to break. The merge integrates these changes into the main codebase. The commit occurred on Thursday, September 12, 2024, at 08:28:16 AM (-0700 timezone).
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 13 (734676b53fe6332798e6335bc161c4fde5fcdc37)</summary>

The Git commit with hash `734676b53fe6332798e6335bc161c4fde5fcdc37` and authored by Louis Beaumont includes changes to disable the use of the Silero voice activity detection (VAD) engine on Windows platforms. 

Here are the key changes:
1. **In `screenpipe-audio/src/bin/screenpipe-audio-forever.rs`:**
   - Modified the VAD engine selection to use `VadEngineEnum::Silero` for non-Windows platforms and `VadEngineEnum::WebRtc` for Windows platforms.

2. **In `screenpipe-audio/src/bin/screenpipe-audio.rs`:**
   - Changed the VAD engine from `VadEngineEnum::Silero` to `VadEngineEnum::WebRtc` unconditionally.

In summary, these changes ensure that the Silero VAD engine is disabled on Windows, opting for the WebRtc VAD engine instead.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 14 (b7290dd793bc7da214b258b6dfeddcb7695cd22e)</summary>

The commit `b7290dd793bc7da214b258b6dfeddcb7695cd22e` by Louis Beaumont on September 12, 2024, disables the Silero voice activity detection (VAD) engine on Windows. This is achieved by adding a conditional compilation attribute (`#[cfg(not(target_os = "windows"))]`) to the implementation of the `VadEngine` trait for `SileroVad` in the `screenpipe-audio/src/vad_engine.rs` file. This directive ensures that the `is_voice_segment` function is only compiled on non-Windows operating systems.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 15 (619127260fac4e7feea9f1a1275653e15236afc7)</summary>

The commit with hash `619127260fac4e7feea9f1a1275653e15236afc7` made by Joe Goldin on September 11, 2024, updates the `pre_build.js` script located in the `screenpipe-app-tauri/scripts/` directory. The modification specifically alters the command used for extracting the ONNX Runtime libraries in a Windows environment. 

The change replaces:

```javascript
await $`unzip ${onnxRuntimeLibs} || Expand-Archive -Path ${onnxRuntimeLibs} || echo "Done extracting"`;
```

with:

```javascript
await $`unzip ${onnxRuntimeLibs} || tar -xf ${onnxRuntimeLibs} || echo "Done extracting"`;
```

This update suggests replacing the `Expand-Archive` PowerShell cmdlet with the `tar` command as a fallback extraction method.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 16 (19e0508ae5d05b61961fb4ef57b8a9dc6122cefb)</summary>

The commit made by Joe Goldin on September 11, 2024, updates the `pre_build.js` script in the `screenpipe-app-tauri` project. The specific modification involves the line that handles the removal of the ONNX Runtime library's zip file after extraction on Windows. The change ensures that the "Done cleaning up zip" message is part of the command executed by `await $` rather than executed separately.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 17 (ba89535f3535989fdc7293778e2a4fb8fc895080)</summary>

The commit `ba89535f3535989fdc7293778e2a4fb8fc895080` by Joe Goldin on September 11, 2024, updates the `pre_build.js` script located within the `screenpipe-app-tauri/scripts` directory. Specifically, the changes modify the way ONNX Runtime libraries are handled for Windows:

1. **Extraction Process**: The code previously used `Expand-Archive` to extract the ONNX Runtime library zip file. The updated code now tries multiple extraction methods: `unzip`, `Expand-Archive`, or simply outputs "Done extracting" if successful.

2. **Cleanup Process**: The removal of the zip file now also tries multiple approaches: using `rm -rf` first, if it fails it falls back to `rm -Recurse -Force`, and finally confirms "Done cleaning up zip" if the cleanup is successful.

Overall, the changes make the script more robust by attempting different commands to ensure the extraction and cleanup processes are successful in various environments.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 18 (f6dcbfa6f4c198eea6f482d1b7af78e622f6d03e)</summary>

The commit `f6dcbfa6f4c198eea6f482d1b7af78e622f6d03e` authored by Joe Goldin on September 11, 2024, includes updates to the `pre_build.js` script. The changes are as follows:

1. **Windows ONNX Runtime Setup:**
   - The method for unpacking ONNX Runtime libraries has been changed from `tar -xf` to `Expand-Archive -Path`. This adjustment ensures the use of a better-suited command for handling archives in a Windows environment.

2. **Code Formatting:**
   - A newline character was added at the end of the file for proper formatting.

These updates likely enhance compatibility and codebase cleanliness for building the application in a Windows environment.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 19 (12b5922de472731009647a93ffa5b8cc298916fd)</summary>

The commit with ID `12b5922de472731009647a93ffa5b8cc298916fd`, authored by Jackson Delahunt on September 12, 2024, involves a single change to fix a broken build in the TypeScript example of a Vercel AI Chatbot. Specifically, the change is in the `codeblock.tsx` file located at `examples/typescript/vercel-ai-chatbot/components/ui/`. The modification corrects the `window.prompt` method call by fixing a misplaced logical OR operator, replacing:

```javascript
const fileName = window.prompt('Enter file name' || '', suggestedFileName)
```

with:

```javascript
const fileName = window.prompt('Enter file name', suggestedFileName)
```
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 20 (1e8bb95768ab0c665bf23e85817194cc15a627bf)</summary>

### Summary of Git Changes

**Commit Details:**
- **Commit ID:** 1e8bb95768ab0c665bf23e85817194cc15a627bf
- **Author:** Joe Goldin
- **Date:** Wed Sep 11 23:11:48 2024 -0700
- **Message:** Fix to copy resources for ONNX Runtime.

**Changes Made:**

1. **.gitignore:**
   - Added a rule to ignore `onnxruntime-win-x64-gpu` libraries.

2. **pre_build.js (in `screenpipe-app-tauri/scripts`):**
   - Updated the Windows configuration to include `onnxruntime-gpu` in the `vcpkgPackages` array.
   - Added setup procedures for ONNX Runtime libraries specific to Windows:
     - Download and unzip the ONNX Runtime libraries if they don't exist.
     - Remove the zip file after extraction.
     - Log messages for setup status.

3. **tauri.windows.conf.json (in `screenpipe-app-tauri/src-tauri`):**
   - Updated the list of resources copied during the build:
     - Added a rule to include ONNX Runtime DLL files (`onnxruntime*\\lib\\*.dll`).

These changes ensure the necessary ONNX Runtime resources are properly ignored, downloaded, and included during the build process for the Windows platform.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 22 (69e6b8bd40787c48c82632876f8fe4a773656d82)</summary>

The commit `69e6b8bd40787c48c82632876f8fe4a773656d82` by Joe Goldin made a simple version bump in the `Cargo.toml` file of the `screenpipe-app-tauri` project. The version was changed from `0.2.31` to `0.2.33`.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 23 (bb342643e47ce9f4d859ddac4fab81e2ffab6e26)</summary>

The commit by Joe Goldin addresses the enhancement of including ONNX runtime libraries with the build process. Here is a summary of the changes made:

1. **Version Rollback in Cargo.toml:**
   - The version of `screenpipe-app` was rolled back from `0.2.32` to `0.2.31`.

2. **Dependency Updates in `screenpipe-audio/Cargo.toml`:**
   - Added `vad-rs` version `0.1.3` to the dependencies.
   - Re-added dependencies for Windows under `[target.'cfg(target_os = "windows")'.dependencies]`:
     - `ort` with specific features like "download-binaries", "copy-dylibs", etc.
     - `esaxx-rs` version `0.1.10`
     - `samplerate` version `0.2.4`
     - `libsamplerate-sys` version `0.1.10`
   - Removed the redundant configuration for `vad-rs` under the non-Windows target.

3. **Code Adjustments in `screenpipe-audio/src/stt.rs`:**
   - Merged code paths, removing conditional compilation and ensuring `SileroVad` is available across all platforms.
   - Cleaned up the `create_whisper_channel` function by removing conditional `cfg` usage for the `SileroVad`.

4. **Modifications in `screenpipe-audio/src/vad_engine.rs`:**
   - Updated imports and removed conditional compilation for `SileroVad`.
   - Ensured the `VadEngineEnum` includes `Silero` for all platforms.
   - Adjustments in the implementation of `SileroVad` to standardize its initialization and usage across all platforms without conditions.

5. **CLI Adjustments in `screenpipe-server/src/cli.rs`:**
   - Removed conditional compilation directives around `Silero` for CLI VAD engine selection.
   - Ensured `Silero` is included as a default VAD engine option regardless of the operating system.

This commit ensures that ONNX runtime libraries and related functionalities are properly bundled and available across different platforms, removing previous environment-specific exclusions.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 24 (1d87fe95d5f2deaef52ff16de5554b91a48110a3)</summary>

The main changes in commit `1d87fe95d5f2deaef52ff16de5554b91a48110a3` involve disabling the Silero VAD (Voice Activity Detection) on Windows. The alterations affect several files within the project:

1. **Updated Project Version**:
    - `screenpipe-app-tauri/src-tauri/Cargo.toml`: The version number is incremented from `0.2.31` to `0.2.32`.

2. **Dependency Adjustments**:
    - In `screenpipe-audio/Cargo.toml`, dependencies specific to Windows, including `vad-rs`, are commented out, while `vad-rs` is added for non-Windows environments.
   
3. **Import and Use of `SileroVad`**:
    - In multiple Rust source files (`screenpipe-audio/src/stt.rs`, `screenpipe-audio/src/vad_engine.rs`, `screenpipe-server/src/cli.rs`), the `SileroVad` import and usage are conditioned to non-Windows environments using `#[cfg(not(target_os = "windows"))]`.

4. **Conditional Compilation**:
    - Code segments related to `SileroVad` (e.g., instantiation, method implementations, and enum variants) are wrapped in conditional compilation attributes to exclude them on Windows.

5. **CLI Argument Handling**:
    - Default VAD engine settings for CLI (`screenpipe-server/src/cli.rs`) are updated to use `WebRtc` on Windows and `Silero` for other systems.

Overall, the commit ensures that the `SileroVad` functionalities are disabled on Windows while being active on other operating systems, leading to conditional dependency and code adjustments accordingly.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 25 (5914dc028a9fa76e9739cbf1a82a0c8f58d85505)</summary>

The commit `5914dc028a9fa76e9739cbf1a82a0c8f58d85505` by Joe Goldin on September 11, 2024, with the message "fix settings command," introduces the following changes to the file `screenpipe-app-tauri/components/settings.tsx`:

1. **New Handler Function Added**: A new function `handleDeepgramApiKeyChange` has been added. This function handles changes to a new input field for the Deepgram API key.
2. **Updated State and Settings**: The function updates the local state with the new Deepgram API key value and also calls `updateSettings` to update the settings with this new value.

This update addresses improvements to the settings component by adding support for the Deepgram API key. This commit is cherry-picked from an earlier commit `06a4b2be31aa951e646f70c349d3b6b6cb33ad5d`.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 27 (58693ffcb3eabf858af5db73eb43cc7ae4fe17cf)</summary>

The commit introduces the following changes:

- **File Deletion:**
  - The file `settings.tsx` located at `examples/apps/screenpipe-app-tauri/components/` has been deleted.

- **Updates to Existing File:**
  - In the file `settings.tsx` located at `screenpipe-app-tauri/components/`, significant updates have been made:
    - The `deepgram` card section has been added to the `Settings` component. This includes:
      - User interface elements for inputting and managing the Deepgram API key.
      - Additional descriptive text and links related to Deepgram's services and its API key.
  
These changes seem to focus on improving the UI, especially by integrating settings for Deepgram's transcription models, simplifying the management of API keys, and enhancing user experience for choosing and configuring AI providers within the application.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 28 (7f31b627347c83b7d018ab4fa607f42297d6a6bb)</summary>

The commit `7f31b627347c83b7d018ab4fa607f42297d6a6bb` by Louis Beaumont includes several changes to the UI of a chat application. The key changes are as follows:

1. **Fix in `chat-message-v2.tsx`**:
   - Updated the condition to display the AI icon. It now checks if the `aiUrl` does not include "openai" before displaying a custom icon (🦙). If it does include "openai", it displays the `IconOpenAI`.

2. **Logging in `search-chat.tsx`**:
   - Added a `console.log` statement to print `openaiApiKey` and `aiUrl` from the settings for debugging purposes.

3. **UI Adjustment in `search-chat.tsx`**:
   - Removed the `max-h-[600px]` class from the div displaying chat messages, which potentially removes a height restriction and allows the div to grow as needed.

4. **Version Bump in `Cargo.toml`**:
   - Updated the version of the package `screenpipe-app` from `0.2.3` to `0.2.31`.

These changes appear to be focused on debugging and improving the user interface of the chat application, along with a version update in the configuration file.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 29 (ce6202cf2f57f270c1d021f9d4b38cfd1e321af3)</summary>

The commit `ce6202cf2f57f270c1d021f9d4b38cfd1e321af3` introduces support for configuring the Deepgram API key within the UI of the `screenpipe-app-tauri` application. The following summarizes the changes made in this commit: 

1. **New Settings Component**:
   - A new file `settings.tsx` was added which contains a settings dialog allowing users to configure various AI settings including API keys for OpenAI and Deepgram.

2. **Deepgram API Key Management**:
   - The `settings.tsx` component lets users enter Deepgram API keys, and these settings are managed and stored using the `useSettings` hook.
   - Updates in `use-settings.tsx` to accommodate the handling of the Deepgram API key alongside the existing settings.

3. **Server and Sidecar Updates**:
   - Modifications in `sidecar.rs` to handle an additional parameter for the Deepgram API key when launching sidecar commands.
   - Updates made to `screenpipe-audio` to pass the Deepgram API key as an option to the speech-to-text (stt) function.
   - `screenpipe-server` now includes an argument for `deepgram_api_key` to be passed when starting continuous recording.

These changes ensure that the application can leverage Deepgram's transcription services by allowing users to input the necessary API keys directly through an enhanced settings dialog in the UI. 

Overall, the commit extends the application’s functionality to support Deepgram's transcription features, improving its versatility and configurability.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 31 (21af441016f463831cb31b0b45a1aaf4fdf52054)</summary>

**Commit Summary:**

- **Commit Hash:** 21af441016f463831cb31b0b45a1aaf4fdf52054
- **Author:** Louis Beaumont
- **Date:** Wed Sep 11 19:49:32 2024 -0700
- **Message:** "fix limit"

**Changes Made:**
1. **File Modified:** `search-chat.tsx`
   - The limit value in the `queryScreenpipe` function call was changed from `limit * 3` to `limit * 1.5`.

2. **File Modified:** `Cargo.toml`
   - The version of the package "screenpipe-app" was bumped from `0.2.2` to `0.2.3`.

These changes suggest a modification in the query parameter to reduce the limit multiplier and a version increment in the project's Cargo file.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 32 (3aeb5f6f3f42931114644181d8f6d570e745cb9c)</summary>

The git changes involve a merge commit made by Louis Beaumont on September 11, 2024. The commit merges changes from a pull request (#307) by Joe Goldin that addresses and fixes a compilation issue related to static linking with MSVC (Microsoft Visual Studio Compiler) on Windows.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 33 (48d4fc97e40acbb453d170ed2da77af67e3f22da)</summary>

**Summary of Git Changes:**

Commit: 48d4fc97e40acbb453d170ed2da77af67e3f22da  
Author: Louis Beaumont  
Date: Wed Sep 11 19:15:47 2024 -0700  

**Description:**  
The commit includes a fix that involves changes to a custom prompt and user interface components.

**Modified Files:**

1. **search-chat.tsx:**
   - **Changes:**
     - The `limit` value for `queryScreenpipe` has been increased by multiplying it by 3 (noted as a temporary "huge hack").
     - A new `Separator` component has been added before displaying chat messages.
   
2. **use-settings.tsx:**
   - **Changes:**
     - Several lines of instructional comments regarding the usage of the "q" argument in queries, handling empty data, and narrowing down questions in chat have been removed.

**Key Changes by Section:**

- **search-chat.tsx:**
  - Query limit increased to `limit * 3`.
  - Added a `Separator` (`<Separator className="my-8" />`) in the UI.
  
- **use-settings.tsx:**
  - Removed instructions on using the "q" argument, handling empty data, and guiding users to ask more specific questions.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 34 (fb218717da0b91cada671e3ae3c42aa386e297b1)</summary>

The commit with ID `fb218717da0b91cada671e3ae3c42aa386e297b1` by Joe Goldin, dated September 11, 2024, includes a fix to statically link MSVC (Microsoft Visual C++) to resolve linking issues on Windows systems. The changes are made in the `.cargo/config.toml` file, specifically updating the `rustflags` under the `[target.x86_64-pc-windows-msvc]` section. The modifications include enabling `crt-static` and adjusting the `link-args` to incorporate multiple default libraries and exclude several conflicting libraries for successful static linking.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 35 (d98ae85b63426514a30bc0c6cf229a26a7f44cd2)</summary>

This Git commit, authored by Louis Beaumont on September 11, 2024, introduces two main changes to the `screenpipe-server` application:

1. **CLI Table Update**:
   - Additional information regarding the ignored windows, included windows, and video chunk duration is now displayed in the command-line interface (CLI). This involves cloning and printing these values.
   - The CLI output table now includes entries for `Video Chunk Duration`, `Use PII Removal`, `Ignored Windows`, `Included Windows`, and `Friend Wearable UID`.

2. **Configurable Video Chunk Duration**:
   - A new command-line argument (`video_chunk_duration`) is introduced, allowing users to specify the duration of video chunks in seconds. This argument defaults to 30 seconds if not provided.
   - The `start_continuous_recording` and `record_video` functions are updated to accept the `video_chunk_duration` parameter.
   - The `VideoCapture` struct and the `save_frames_as_video` function incorporate the `video_chunk_duration` parameter, replacing the previously hardcoded 30-seconds duration with a configurable option derived from the CLI argument.

These changes enhance the flexibility and usability of the `screenpipe-server` application by allowing users to control video chunk duration through the CLI and by providing more detailed runtime information.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 36 (06e4e3f5c28d84cc1a6110554af8be6d066529b6)</summary>

This commit, authored by Louis Beaumont, addresses an issue related to video length management in the `screenpipe-server` project. The key changes can be summarized as follows:

1. **Imports**: Added `timeout` from `tokio::time`.

2. **Frame Calculation**:
   - Changed the fixed `frames_per_video` value to be dynamically calculated as `(fps * 30.0).round()`, meaning 30 seconds of video based on the frames per second (fps).
  
3. **FFmpeg Process Management**:
   - Updated the condition to start a new FFmpeg process when `frame_count` reaches `frames_per_video` or if `current_ffmpeg` is `None`, instead of using a modulus operation.
   - Introduced a reset for `frame_count` each time a new FFmpeg process is started.

4. **Frame Writing**:
   - Implemented a timeout mechanism with `timeout(write_timeout, receiver.recv()).await` to handle frame reception.
   - Simplified the frame writing loop by removing the redundant check for yielding every 100 frames and now it yields within each iteration unconditionally.

These changes collectively aim to improve the handling of video frame segmentation and ensure smoother video chunk processing by FFmpeg, fixing the previously "weird" video length issue.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 37 (6ca90f87a656b5550c3b101c7901cc5236e5c372)</summary>

In the commit `6ca90f87a656b5550c3b101c7901cc5236e5c372`, made by Louis Beaumont on Sep 11, 2024, various small UI changes were introduced to the `DateTimePicker` and `SearchChat` components in a Tauri app. Additionally, a new hook `useInputHistory` was created for managing input history.

**Changes Overview**:

**1. DateTimePicker (`components/date-time-picker.tsx`):**
- Date formatting was modified to be in lowercase.

**2. SearchChat (`components/search-chat.tsx`):**
- Import statements were updated to include the `HelpCircle` icon and `useInputHistory` hook.
- Removed unused state `isChatEnabled`.
- Replaced element scroll listeners with window scroll listeners.
- Adjusted positioning elements:
  - Centered `VideoComponent` elements within divs.
  - Corrrected conditional rendering for `window_name` badge.
- Enhanced UI:
  - Added descriptive tooltips for various input fields such as `search query`, `content type`, `start date`, `end date`, `app name`, `window name`, `include frames`, `page size (limit)`, `min length`, and `max length`.
  - Updated placeholders to use lowercase for better consistency.
- Modified scrolling behavior to ensure the view scrolls down after specific actions like generating AI responses or starting a new search.
- Updated progress bar display during loading.
- Adjusted the "Scroll to Bottom" button's appearance.

**3. New File: Hook (`lib/hooks/use-input-history.tsx`):**
- Implemented `useInputHistory` hook:
  - Manages input value and history states.
  - Loads history from `localStorage` on initial render.
  - Saves new values into history and `localStorage` avoiding duplicates.

These changes aim to improve UI consistency, user experience, and functionality by adding more descriptive tooltips, standardizing format, and enhancing overall interaction with the search and date-time components.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 38 (32fd0753f91980a98e577c95adab4911499082ef)</summary>

The commit made by Louis Beaumont on September 11, 2024, introduces a new file `not-found.tsx` in the `screenpipe-app-tauri/app` directory to address some issues on Windows. This newly added file defines a React component named `NotFound`, which renders a simple user interface with a header saying "Not Found" and a paragraph stating "Could not find requested resource."
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 39 (5494a489dbf34a7303464b18415f5bec04a60d68)</summary>

The commit with hash `5494a489dbf34a7303464b18415f5bec04a60d68` made by Louis Beaumont on Sep 11, 2024, aimed to fix the Tauri build process for a GitHub Actions workflow. The changes specifically involved uncommenting the setup step for MSVC (Microsoft Visual C++) in the file `.github/workflows/release-app.yml`, making it active only for the Windows platform. Here's a summary of what was altered:

- Previously commented-out MSVC setup steps were uncommented.
- This modification ensures that when the workflow runs on a Windows platform, the MSVC development command setup (`ilammy/msvc-dev-cmd@v1`) is used.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 40 (9fd1e20d78384227cd76c9076dd40274f397e958)</summary>

The commit `9fd1e20d78384227cd76c9076dd40274f397e958` by Louis Beaumont makes changes to the GitHub Actions workflow file `release-app.yml` in an attempt to fix the Tauri build process. The key modifications are:

1. For the Ubuntu 22.04 platform, the build arguments `--features mkl` have been removed, leaving the args empty with a TODO note for adding CUDA and mkl features in the future.
2. For the Windows platform, the build arguments have been changed from `--target x86_64-pc-windows-msvc --features mkl` to `--target x86_64-pc-windows-msvc`. There is also a TODO note questioning whether CUDA and mkl features should be added.

These changes are likely made to troubleshoot or streamline the build process by temporarily removing certain features.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 41 (adfc33ed49065185070eca8a46055a90d3abbea0)</summary>

The commit authored by Louis Beaumont attempts to fix issues related to building the Tauri app. The changes include:

1. **GitHub Workflow Changes (`.github/workflows/release-app.yml`)**:
    - For the `ubuntu-22.04` platform, added the `--features mkl` argument.
    - For the `windows-latest` platform, added the `--features mkl` argument to the build commands, alongside `--target x86_64-pc-windows-msvc`.
    - Commented out the step for setting up MSVC and instead added a step for installing LLVM and Clang version 10.0.

2. **Cargo Configuration Changes (`screenpipe-app-tauri/src-tauri/Cargo.toml`, `screenpipe-audio/Cargo.toml`)**:
    - Updated the version of `screenpipe-app` from `0.2.1` to `0.2.2`.
    - Added a new feature `mkl` to the `screenpipe-app-tauri` Cargo configuration.
    - Enabled the `mkl` feature in `screenpipe-audio`, which now includes dependencies on relevant `candle` crates with the `mkl` feature enabled.

These changes aim to refine the build process across different platforms by leveraging the `mkl` feature and ensuring the appropriate compiler tools are available.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 42 (702b54f2a65bbb8bf86f703572b660d83e41bb1e)</summary>

The commit `702b54f2a65bbb8bf86f703572b660d83e41bb1e` by Louis Beaumont on September 11, 2024, addresses two main changes:

1. **Fixing Windows Build:** The commit likely includes changes necessary to successfully build the project on Windows operating systems.
2. **Custom Prompt:** Adjustments were made to customize the prompt, though the specific details are not entirely clear from the excerpt.

### Code Changes:
- **File Modified:** `screenpipe-app-tauri/src-tauri/tauri.conf.json`
- **Modification in CSP (Content Security Policy):** The `connect-src` policy was changed to include a wildcard `*`, allowing connections to any source. 

##### Old policy:
```json
"connect-src": "ipc: http://ipc.localhost https://youtube.com https://api.openai.com http://localhost:3030 https://web.crabnebula.cloud https://api.github.com https://eu.i.posthog.com https://github.com https://*.githubusercontent.com https://*.github.com http://*:11434 http://*:9000"
```

##### New policy:
```json
"connect-src": "ipc: http://ipc.localhost https://youtube.com https://api.openai.com http://localhost:3030 https://web.crabnebula.cloud https://api.github.com https://eu.i.posthog.com https://github.com https://*.githubusercontent.com https://*.github.com http://*:11434 http://*:9000 *"
```

The addition of the `*` at the end allows connections to any source, possibly aimed at fixing network issues during the build process on Windows or enabling more flexible development.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 43 (8c47e0bb21273a0082cd76e1af047df9b1d9bb33)</summary>

In commit `8c47e0bb21273a0082cd76e1af047df9b1d9bb33`, the following changes were made:

1. **`settings.tsx` File Updates:**
   - Removed functionality related to toggling and configuring "Ollama".
   - Simplified the UI for selecting and configuring AI provider settings.
   - Added a new section for custom prompts, including a text area for input and a reset button to clear the prompt.
   - Updated some placeholders for clarity and added input type for URL fields.

2. **`pre_build.js` Script Updates:**
   - Adjusted the paths for copying the `screenpipe.exe` binary to better support Windows builds.
   - Now includes potential paths with fewer directory layers, simplifying the build script.

These changes enhance the configuration user interface within the application and improve Windows build compatibility.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 44 (8198a5e8af1243dc4e7add5e78357c2a5e027a7e)</summary>

The commit `8198a5e` authored by Louis Beaumont on September 11, 2024, includes several updates to the `search-chat.tsx` file to fix the UI:

1. **Badge Component Updates**:
    - The `Badge` components for `app_name` and `window_name` are now interactive elements:
        - They have `cursor-pointer` class added.
        - They include `onClick` handlers to set `app_name` and `window_name` using respective functions.

2. **Container Styling**:
    - The main container (`div`) now includes additional padding (`p-4`).

3. **Form and Input Changes**:
    - The input within the form is now wrapped in a `div` to address layout and state issues:
        - Input adjustments such as right padding (`pr-10`), height (`h-12`), and focus border styling changes have been implemented.
        - Disabled state of the input is linked to the `isAiLoading` state.
    - The warning tooltip for content length now appears more elegantly positioned within the `div` wrapping the input:
        - The `TooltipTrigger` wraps an `AlertCircle` icon within an `absolute` positioned `div`.

These adjustments aim to enhance the usability, interactivity, and overall layout of the search chat component.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 45 (3ef95e8007468bd5b01bc3cdb55034f46e7a1c9c)</summary>

The commit with ID `3ef95e8007468bd5b01bc3cdb55034f46e7a1c9c` by `Louis Beaumont <louis.beaumont@gmail.com>` on `Wed Sep 11 14:15:13 2024 -0700` contains several changes aimed at improving the user experience (UX). Here are the key changes summarized:

1. **Binary File Update:**
   - `screenpipe-app-tauri/bun.lockb`: Updated with new binary differences.

2. **Enhancements in `search-chat.tsx`:**
   - Added new imports for UI components (`AlertCircle`, `ChevronDown`, `Send`, etc.) and some new components like `Accordion`, `VideoComponent`, and `Tooltip`.
   - Introduced a new constant `MAX_CONTENT_LENGTH` to manage maximum content length.
   - Adjusted the default value of the `limit` state from 50 to 30.
   - Implemented scroll handling to show a scroll-to-bottom button when the user scrolls up significantly (`>10%` of the content height).
   - Wrapped content display in an `Accordion` component for better organization.
   - Included more detailed handling for different content types (`OCR`, `Audio`, `FTS`), adding interactive elements like dialogs for images and tooltips.
   - Added a scroll-to-bottom button that appears when content is scrolled up.
   - Enhanced the user search form with UI cues using icons and tooltips.

3. **New Alert Component:**
   - Created a new file `screenpipe-app-tauri/components/ui/alert.tsx` with implementation for `Alert`, `AlertTitle`, and `AlertDescription` components which add alert functionality.

4. **Code Clean-up:**
   - In `use-pipes.tsx`, it commented out a console log statement that was printing local pipe information.

5. **Version Bump:**
   - Updated the `Cargo.toml` version for the `screenpipe-app` from `0.2.0` to `0.2.1`, indicating a minor update possibly related to UX improvements.

These changes collectively aim to improve the usability and functionality of the `screenpipe-app-tauri` application by enhancing the search-chat interface, adding new UI components, and refining content handling mechanisms.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 46 (24cd130efb462dfc9b30eaed899dbe0b8636e5cd)</summary>

The commit with the identifier `24cd130efb462dfc9b30eaed899dbe0b8636e5cd` is a merge commit. It merges in changes from a pull request (#305) authored by a user or a branch named `m13v` into the current branch. The merged changes involve something related to a "dmg background png". The merge was authored by Louis Beaumont on September 11, 2024.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 47 (05102911ac3e07b7165664e155fd6dc1708b3b45)</summary>

The commit with hash `05102911ac3e07b7165664e155fd6dc1708b3b45` by Louis Beaumont on September 11, 2024, addresses a fix for the Tauri build. The changes are made in the `Cargo.toml` file located in the `screenpipe-app-tauri/src-tauri` directory. Specifically, the commit adds two new empty feature arrays: `cudart` and `metal` (marked as unused). These additions are appended after the commented `cuda` and `metal` feature declarations and before the `default` feature specification.
</details>

------------------------------------------------------------------------

# Overall Summary of Changes

### Summary of Git Changes

**Commit 85ee2bcd by Louis Beaumont on September 12, 2024:**
- **File:** `screenpipe-server/src/bin/screenpipe-server.rs`
- **Purpose:** Address a security issue.
- **Changes:** The server's binding address is restricted to the local loopback interface (`127.0.0.1`), instead of all network interfaces (`0.0.0.0`).

**Commit 27057dc332b4e38371254530cce4a519d3ec5b3d:**
- **Author:** Louis Beaumont
- **Date:** September 12, 2024
- **Message:** Fix test & bench
- **Files Changed:** 
  1. `screenpipe-audio/benches/record_and_transcribe_benchmark.rs`
  2. `screenpipe-audio/benches/stt_benchmark.rs`
  3. `screenpipe-audio/tests/core_tests.rs`
  4. `screenpipe-vision/benches/vision_benchmark.rs`
- **Changes:** Enhanced configurations and parameters for VAD engine, improved readability, and performance adjustments.

**Commit 43daa6b7ad66399117a8c788c411278ef13e6b45:**
- **Author:** Louis Beaumont
- **Purpose:** Fix Windows compatibility.
- **Changes:** 
  1. `settings.tsx` - Updated URL.
  2. `Cargo.toml` - Version bump.
  3. `vad_engine.rs` - Reorganized imports for readability.

**Unnamed Commit:**
- **Purpose:** Fix input of the forward slash.
- **Changes:** 
  1. Added `handleKeyDown` function for slash key input in `input.tsx`.
  2. Refactor for consistency and readability.

**Commit 89f7f0302325bf2ab9641c0a66921405f1bb969e:**
- **Author:** Louis Beaumont
- **Purpose:** Enhance Screenpipe project.
- **Changes:** 
  1. Default to Windows OCR on Windows.
  2. UI enhancements, including tooltips.
  3. Removed `self_healing` feature and unused dependencies.
  4. Simplified `ResourceMonitor`.

**Commit 11ca216b80da3e09ef4c8fea1cc159b91008d894:**
- **Author:** Louis Beaumont
- **Purpose:** Merge PR #309 by Joe Goldin.
- **Changes:** Distributed ONNX Runtime libraries with the build, re-enabled Silero VAD on Windows.

**Commit b91ef6c1d7ee13291aa434d2a3ffd404d1832ba1:**
- **Author:** Louis Beaumont
- **Purpose:** Update `use-settings.tsx`.
- **Changes:** Added Deepgram API key hardcoding for temporary use.

**Commit cc826d2156c94a7bad1ef8a44fc781cda0e7e6b4:**
- **Author:** Louis Beaumont
- **Purpose:** Fix build issues.
- **Changes:** Refactored conditional checks and parameter passing in `stt.rs`.

**Unnamed Commit:**
- **Purpose:** Merge PR #308 by joegoldin.
- **Changes:** Feature to configure Deepgram API key in UI.

**Commit 2d450d7ecbcd6a9456597c89ad74bc0d267d9ea6:**
- **Purpose:** Merge 'main' into 'feat-deepgram-api-key-ui'.
- **Changes:** VAD engine updates and Deepgram API key integration.

**Commit 6d7106df5cce2d248e188f1c6909dc3ac082feb8:**
- **Purpose:** Merge PR #310 by sabrehagen.
- **Changes:** Fixed type errors causing build issues.

**Commit 734676b53fe6332798e6335bc161c4fde5fcdc37:**
- **Author:** Louis Beaumont
- **Purpose:** Disable Silero VAD on Windows.
- **Changes:** Adjusted VAD engine selection to use `WebRtc` on Windows.

**Commit b7290dd793bc7da214b258b6dfeddcb7695cd22e:**
- **Purpose:** Disable Silero VAD on Windows.
- **Changes:** Conditional compilation to exclude `SileroVad` implementation on Windows.

**Commit 619127260fac4e7feea9f1a1275653e15236afc7:**
- **Author:** Joe Goldin
- **Purpose:** Update `pre_build.js`.
- **Changes:** Replaced `Expand-Archive` with `tar` for ONNX Runtime libraries extraction.
  
**Unspecified Commits:**
- **General Purpose:** Enhanced Tauri app build processes, Windows compatibility, UI improvements, and added feature support for Deepgram API key input.
- **Files Affected:** Various files across `screenpipe-app-tauri`, `screenpipe-audio`, and configuration scripts.

These summaries provide an overview of significant changes in multiple commits led primarily by Louis Beaumont, focusing on security, functionality enhancements, performance improvements, and compatibility fixes.
