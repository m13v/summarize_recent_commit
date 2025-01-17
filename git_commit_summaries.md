# Git Commit Summaries

-----------------------------------------------------------------------
-----------------------------------------------------------------------
 
PRESS CMD+SHIFT+V TO VIEW IN MARKDOWN
 
_______________________________________________________________________
-----------------------------------------------------------------------
Total number of commits: 31

<details>
<summary>summary for commit 1 (085dd4bea9405f339ab72d9f29fe14c266b4974f)</summary>

The commit updates the `README.md` file with a few changes:

1. The tagline about the platform's features was modified to remove specific resource usage details: 
   - The original line mentioning "10% CPU, 4 GB ram, 30gb/m" was replaced with just "24/7 screen, mic, keyboard recording and control."

2. Details about resource usage were added to a different section explaining how the system works:
   - It now specifies "uses 10% CPU, 4 GB ram, 15 gb/m" in the context of its 24/7 local recording capabilities.

3. Minor wording change in the "why" section:
   - The phrase "data is the biggest bottleneck in AI right now" was replaced with "context is the dark matter of intelligence."
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 2 (d25717fc8d4fabcadfb3470d9de7351779238301)</summary>

The commit updates the `README.md` file by modifying a line in the "news" section. Specifically, the change involves replacing "Loom pipe" with "Reddit agent" and adding "Timeline" to the list of integrations available for the "pipe store stripe integration" entry from December 2024.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 3 (eed2e4b19d81cb1b024cce9d074b783edd9829c0)</summary>

The commit with ID `eed2e4b19d81cb1b024cce9d074b783edd9829c0` was made by Louis Beaumont on January 8, 2025. This commit involves changes to the `README.md` file. The updates include changing the source URL of one image and the removal of an extra blank line. The first image URL was updated to a new link, and a redundant line was deleted, creating a cleaner format in the markdown.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 4 (f7064974419d9860eb026f28ccbc751e26a67f7e)</summary>

The commit by Louis Beaumont on January 8, 2025, updates the `README.md` file. The change involves a minor revision to a sentence within a paragraph that describes the "screenpipe" library and platform. The ordering of the phrase about AI apps and examples has been altered. Previously, the sentence read: "library & platform to build, distribute, monetize ai apps (like rewind, granola, etc.) that have the full context." It was changed to: "library & platform to build, distribute, monetize ai apps that have the full context (like rewind, granola, etc.)."
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 5 (2cc80057e16121e5bb43bb8926075c92bb421368)</summary>

The commit made by Louis Beaumont on January 8, 2025, updates the `README.md` file. The changes revised the subtitle and description of the "ScreenPipe" project. Specifically, the changes included:

- Updating the description of the project to highlight it as a "library & platform to build, distribute, monetize AI apps" such as "rewind, granola, etc." that have full context.
- Changing the technical and operational details to emphasize it as "open source," "100% local," "dev friendly," with "10% CPU, 4GB RAM, 30gb/m" usage, maintaining features like 24/7 screen, mic, keyboard recording, and control.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 6 (c7db0a30e7528d302ef4896f5bd2ce7ba1801967)</summary>

The commit with ID `c7db0a30e7528d302ef4896f5bd2ce7ba1801967` addresses fixes related to the auto-destructor functionality in the `screenpipe-server` project. Here are the key changes:

1. **Auto-Destruct Behavior**:
   - Improved the process checking for Windows by updating the `is_process_alive` function. It now uses `GetExitCodeProcess` to accurately determine if a process is still active, handling errors and logging debug information when the process cannot be opened or its exit code cannot be retrieved.

2. **Logging**:
   - Replaced `println!` statements with `debug!` and `info!` logging macros for more structured logging output. Specifically, these changes were made to log when processes are not found or fail and whether specific processes are alive.

3. **Code Changes in `main` Function**:
   - Deferred the dropping of `vision_runtime` and `audio_runtime` to a blocking task (`tokio::task::block_in_place`) for proper resource cleanup, making sure they are dropped outside the main async context.

4. **Code Cleanup**:
   - Removed unnecessary early drops of `vision_runtime` and `audio_runtime` during the main function execution.

Overall, these changes improve the robustness and logging of the auto-destruct feature on Windows and ensure proper cleanup of runtime resources.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 7 (271247f047d0606a7ca98da868af199216da6aae)</summary>

The git commit `271247f047d0606a7ca98da868af199216da6aae` by Louis Beaumont includes changes to the GitHub Actions workflow file `release-app.yml`. The commit adds a new environment variable `CN_VERBOSE` with a value of `"2"` to two blocks of code in the workflow. This sets the verbosity level for CrabNebula operations to 2, likely enabling more detailed logging. Additionally, the `release upload` commands for both Windows and non-Windows uploads have been modified to include the `-vv` flag, which likely increases verbosity for those commands as well. Overall, these changes are aimed at increasing the verbosity of the logging output for troubleshooting or monitoring purposes.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 8 (480a3f0a175f3cd70552df1f30409e2467ebd7f4)</summary>

The commit `480a3f0a175f3cd70552df1f30409e2467ebd7f4` by Ezra Ellette introduces the Whisper-live-transcript feature as part of a larger pull request #1099. This update covers multiple changes across different files, highlighted as follows:

### Refactor and Code Reduction
- Several modules have been refactored to reduce code redundancy and improve structure.
- Deepgram URLs are made consistently available across the module, enhancing modularity.

### Real-Time Audio Features
- **Realtime Audio Settings Added**: The project now supports real-time audio transcription with newly added functions and logic.
- **Deepgram Live Transcription**: Integrated support for real-time transcription using Deepgram's API.
- **SSE Endpoint**: An SSE (Server-Sent Events) endpoint was created for streaming transcriptions live.

### Code Modifications
- New modules (`deepgram`, `realtime`, and `segments`) have been added to specifically handle live audio processing and integrations.
- Extensive refactoring occurred in `screenpipe-audio` to support use of various real-time transcription engines, including Whisper and Deepgram.
- A new command interface setup is integrated for better control over audio processing functions and handling of transcription events.

### Logging and Debugging Changes
- Adjustments to logging: Reduced unnecessary logs unless debugging is enabled. Ensures that "no speech detected" messages do not flood logs except when explicitly needed for debugging.
- Debugging facilities added to allow streamlined testing and message logging for various transcription stages.

### Deepgram Integration:
- **Deepgram Client Integration**: Added the Deepgram API client and related functionalities to subscribe to real-time audio feeds and process their transcriptions.
- Configuration includes API token management and support for custom endpoints or proxies.

### Server and CLI Adjustments
- New features within the server to support real-time transcription streaming and control configurations.
- CLI enhancements allow users to specify whether to utilize real-time audio transcription and select specific devices for such activities.

### Testing and Build System
- Updates in the build system, dependencies acquisition in `Cargo.toml`, and other files required to support real-time audio processing.

Overall, these changes significantly enhance the system's capability to handle live audio processing, improve integration with external transcription services, and offer improved configuration, error handling, and logging functionalities.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 9 (ae1ab1e837402913b689c8204d2e3dc502d29e8e)</summary>

The commit with hash `ae1ab1e837402913b689c8204d2e3dc502d29e8e`, authored by Louis Beaumont on January 7, 2025, updates the `Cargo.toml` file in the `screenpipe-app-tauri/src-tauri` directory. The version of the package "screenpipe-app" is incremented from "0.23.0" to "0.23.1". The commit message indicates that this change addresses properly ending the process in the Windows release of the application.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 10 (8554ab54e95be23eb84b5fdfd5530ebe785f1ace)</summary>

The commit with hash `8554ab5` introduces a fix to the `sidecar.rs` file within the `screenpipe-app-tauri/src-tauri/src` directory. The modification addresses the functionality of the "stop" button. In particular, it changes the command used to terminate processes: 

- The `taskkill` command in the code originally used the flags `"/F", "/IM", "screenpipe.exe"`.
- The change adds the `"/T"` flag to these arguments, making the new command `"/F", "/T", "/IM", "screenpipe.exe"`.

This adjustment ensures that when the "stop" button is pressed, the `taskkill` command not only forces the termination of the "screenpipe.exe" process but also any child processes started by it.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 11 (1936bee0e25af84db905cecd5133a8b2abf937c1)</summary>

The changes in this commit involve both a version update and a platform-specific code modification for the Tauri-based application "screenpipe-app":

1. **Version Update:**
   - The version of the application is incremented from `0.22.9` to `0.23.0` in the `Cargo.toml` file. This typically indicates a new release with possibly significant updates or fixes.

2. **Platform-Specific Code Modification:**
   - In the `main.rs` file, a segment of code that starts a health check service is now enclosed within a conditional compilation block specific to macOS (`#[cfg(target_os = "macos")]`). This means the health check service is only initiated on macOS systems, whereas previously it was executed regardless of the operating system.
   - This change likely targets a known issue related to the application, as suggested by the commit message "revert again windows tray menu crash," which hints at addressing a crash that occurred when using the tray menu on Windows.

Overall, the commit refines the application's behavior by including OS-specific logic and suggesting a release that's focused on stability improvements, especially concerning Windows.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 12 (7ae5a14dd4533690267334d5d84309eab044d63b)</summary>

The commit adds a changelog for version 0.22.9 of a project. It introduces a new feature where the memory pipe can provide AI-generated descriptions of recent memories, improving user experience. There are also fixes included, such as correcting the tray menu behavior on Windows to handle unhealthy states properly, and resolving an issue with the display of recent memories in the memory pipe. Changes were documented in the changelog files in two locations, reflecting updates for version 0.22.9.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 13 (973776d2b32bc37c06e06df5595546bc2f371522)</summary>

The commit introduces several updates and bug fixes across multiple files in a Git repository:

1. **`pipe-store.tsx` Changes:**
   - Added a new `isEnabling` state to manage the enabling process of a pipe.
   - Updated the `handleToggleEnabled` function to set the loading state when enabling a pipe and to reset it afterward.
   - Increased the timeout from 1000ms to 3000ms for pipe initialization.
   - Buttons for opening the pipe (in a browser or as an app) are now disabled during the enabling process and show "initializing..." instead of "open in browser" or "open as app".

2. **Dependency Updates in Rust Files:**
   - Updated the `once_cell` dependency from version 1.19.0 to 1.20.2 in `Cargo.lock`, along with its checksum.
   - Updated `screenpipe-app` version from 0.22.6 to 0.22.8 in `Cargo.lock`.
   - Updated the `screenpipe-app` version in `Cargo.toml` from 0.22.8 to 0.22.9.

3. **`health.rs` Modifications:**
   - Added a timeout of 5 seconds to the health check request to handle cases where it never times out on Windows.

4. **`main.rs` Adjustments:**
   - The health check service is now started on all operating systems, not just macOS. Previously, it was only enabled on macOS due to issues with Windows.

These changes collectively enhance the app's functionality and stability, particularly for Windows users, and include version updates for package dependencies.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 14 (6874e4c6dd1966634783ce919b925f9bfffa254f)</summary>

This Git commit introduces several changes to the `memories-gallery.tsx` and `video.tsx` components aimed at enhancing the functionality of the "memory pipe" feature.

### Key Changes:
1. **Dependencies Update**: 
   - Added new imports including `OpenAI`, `useRef`, and `useSettings`.
   - Updated existing ones like lucide-react icons and added `Loader2`.

2. **New Interfaces**:
   - `VideoDescription`: Added to manage video description states with `loading` and `content` properties.

3. **State Management**:
   - New states `videoDescriptions`, `collectiveDescription`, and `abortControllerRef` have been introduced for managing video descriptions and controlling asynchronous operations.

4. **Memory Fetching**:
   - Memories are now filtered to avoid showing recent ones, specifically omitting those from the last 5 minutes.
   - Introduced a new logic to handle memory fetching attempts with a cap on the number of attempts (`MAX_ATTEMPTS`).

5. **AI Descriptions**:
   - Introduced functions `generateVideoDescription` and `generateCollectiveDescription` to provide AI-generated descriptions for individual video memories and a collective summary of fetched memories.
   - Utilized the OpenAI API to generate these descriptions based on memory content.

6. **Enhanced User Feedback**:
   - Implemented loading states and error handling for the AI-generated content.
   - Display pending messages like "generating description..." using animations with `motion.div`.

7. **UI Components**:
   - Updated `MemoriesGallery` UI with additional elements and buttons for generating summaries.
   - Used `motion` for animations and improved user interactions with descriptions and summaries.

8. **Video Component Enhancements**:
   - Added an `onLoadStart` prop to initiate actions when a video starts loading.
   - Improved memory management with cleanup in `useEffect`.

9. **Error Handling**:
   - Enhanced error reporting in video loading and media validation processes.

Overall, this update significantly enhances the user experience by providing AI-assisted video descriptions and managing state more effectively to avoid recent memory display, alongside UI improvements and expanded functionality for video handling.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 15 (1c48f4d1e6edd3b21d7b4093bfd8045fe3908c7f)</summary>

The commit authored by Louis Beaumont made the following changes to the `screenpipe-app`:

1. **Version Update:**
   - The version of the `screenpipe-app` has been incremented from `0.22.7` to `0.22.8` in the `Cargo.toml` file.

2. **Health Check Service Adjustment:**
   - Modifications were made in `main.rs` to conditionally start the health check service only on macOS. This was done because the service was causing crashes on Windows. The code block that spawns the health check service is now wrapped with `#[cfg(target_os = "macos")]`, ensuring it only executes when the app is running on macOS.

3. **Tray Menu Icon based on Health Status:**
   - The application’s tray menu icon now changes according to its health status, but this feature is implemented only for macOS. 

These changes aim to enhance the application’s functionality on macOS while avoiding issues on Windows.
</details>

------------------------------------------------------------------------

# Overall Summary of Changes

The git log outlines a series of updates, enhancements, and bug fixes across various components of a project, primarily focused on the `README.md`, `screenpipe-app`, `screenpipe-server`, and associated files. Here are the key points:

1. **README.md Updates**:
   - Numerous wording changes for clarity and emphasis on project features, resource usage, and AI context.
   - Image source updates and formatting clean-up for better presentation.
   - Revised content about tools and integrations, such as replacing "Loom pipe" with "Reddit agent".

2. **`screenpipe-app` Changes**:
   - Version increments, reflecting new updates or bug fixes.
   - Conditional logic for macOS-specific operations to avoid crashes on Windows, such as health check services and tray menu icons.
   - Improvements in stop button functionality to ensure proper termination of processes and their children.

3. **`screenpipe-server` Enhancements**:
   - Improved logging mechanisms and process checking, especially for Windows, to ensure better auto-destruct functionality.
   - Code refactoring for cleaner and more structured execution paths.

4. **Real-Time Audio and Transcription Features**:
   - Introduced features like Whisper-live-transcript and Deepgram integration for real-time audio transcription.
   - Modular enhancements for better handling of live audio processing, logging, and error reporting.

5. **GitHub Actions Workflow Modifications**:
   - Added verbosity settings for better logging during release operations.

6. **UI and Functionality in Memory Handling**:
   - Updates to components like `memory-gallery.tsx` for improved user experience, AI-generated video descriptions, and enhanced state management.

7. **Dependency and Configuration Adjustments**:
   - Updated dependencies to newer versions for stability and security.
   - Configuration tweaks to align with platform-specific requirements.

Overall, these commits reflect a comprehensive effort to refine application functionality, enhance user interface components, improve platform-specific behaviors, and ensure robust operation across different environments.
