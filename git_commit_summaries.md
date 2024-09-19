# Git Commit Summaries

-----------------------------------------------------------------------
-----------------------------------------------------------------------
 
PRESS CMD+SHIFT+V TO VIEW IN MARKDOWN
 
_______________________________________________________________________
-----------------------------------------------------------------------
Total number of commits: 26

<details>
<summary>Summary for commit 1 (b16149887161ceeefb5d1bb264109f6a770b9fa3)</summary>

**Commit Summary:**

- **Commit Hash:** b16149887161ceeefb5d1bb264109f6a770b9fa3
- **Author:** Louis Beaumont
- **Date:** Wed Sep 18 19:27:17 2024 -0700
- **Type:** Chore
- **Description:** Added documentation (`README.md`) for the `pipe-phi3.5-engineering-team-logs` example in the TypeScript directory of the project.
- **Changes:** 
  - A new `README.md` file was created under `examples/typescript/pipe-phi3.5-engineering-team-logs/`.

**New README.md Highlights:**
1. **Project:** Automates logging of engineering work to Notion using Screenpipe and Phi 3.5 AI.
2. **Setup Instructions:**
   - Install Screenpipe and clone the repository.
   - Install and run Ollama with Phi 3.5.
   - Set up a Notion integration and database.
   - Configure environment variables for Notion API key and database ID.
   - Run the pipe to start continuous logging every hour.
3. **Customization Options:**
   - Adjust the logging frequency by modifying the `INTERVAL` in `pipe.ts`.
   - Modify the prompt to refine AI output.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 2 (89e627dc9ebf612f86c335fabb354d4667c3ed4c)</summary>

The commit enhances the user experience (UX) of the meeting page with the following changes:

### Codebase Changes:

1. **Imports:**
   - Added new icon imports from `lucide-react` such as `RefreshCw`, `Trash2`, `Users`, `FileText`, and `PlusCircle`.
   - Imported tooltip components from `@/components/ui/tooltip`.

2. **Interface Updates:**
   - Renamed interface fields to follow camelCase naming convention:
     - `meeting_group` to `meetingGroup`
     - `meeting_start` to `meetingStart`
     - `meeting_end` to `meetingEnd`
     - `full_transcription` to `fullTranscription`
     - `chunk_id` to `chunkId`
     - `file_path` to `filePath`
     - `offset_index` to `offsetIndex`
   - Added new fields `tags`, `deviceName`, and `deviceType` to the `AudioContent` interface.

3. **State and Methods:**
   - Removed debounce logic for `posthog` event tracking.
   - Added new states `isRefreshing` and `isClearing`.
   - Updated the function handling meetings to no longer distinguish live meetings.
   - Introduced new methods `handleRefresh` and `handleClearMeetings` for refreshing and clearing meeting data.

4. **UI Enhancements:**
   - Redesigned the meeting dialog with additional buttons for clearing and refreshing meetings using tooltips for better usability.
   - Removed the "live meeting" feature and related UI elements.
   - Enhanced transcription and summary sections with buttons to copy content and support for customizable prompts.
   
5. **Version Update:**
   - Incremented the package version from `0.2.51` to `0.2.52`.

### Configuration Changes:

- **`screenpipe-app-tauri/lib/screenpipe.ts`:** 
  - Updated `AudioContent` type by adding `device_name` and `device_type` fields.

- **`screenpipe-app-tauri/src-tauri/Cargo.toml`:**
  - Bumped the version of the package to `0.2.52`.

Overall, these changes aim to refine the UX of the meeting history page, provide more options for user interaction, and streamline handling of meeting data.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 3 (8937ba465bdb5276a3bbbeffb8370a57c44a8253)</summary>

In this commit, the following changes were made to the documentation:

1. **File: `content/docs/pages/docs/architecture.mdx`**
    - Expanded the content to include a detailed "status" section.
    - Added detailed information on different integrations and features, such as support for different OCR engines, audio input devices, remote capture capabilities, and more.
    - Included a list of supported integrations that are either completed or still in progress, such as ollama, openai, and others.
    - Provided explanations on additional features like screenshots, audio and STT support, optimized screen & audio recording, and local and cloud computing options among others.
    - Listed some pending features like security enhancements, multimodal embeddings, and energy-efficient modes.

2. **File: `content/docs/pages/docs/faq.mdx`**
    - Added detailed FAQ sections covering:
        - Differences with adept.ai and rewind.ai
        - Data storage locations and policies
        - Encryption status
        - Customization options for reducing storage and energy usage
        - Practical use cases for screenpipe

3. **File: `content/docs/pages/docs/getting-started.mdx`**
    - Updated the Windows installation steps with more detailed and experimental instructions.
    - Provided a step-by-step guide including installation of prerequisites, cloning the repository, and building the project.
    - Elaborated on the specific tools required from MS Visual Studio Build Tools and other dependencies.
    - Added troubleshooting guidance with references to opening issues if the installation does not work.

These changes enhance the comprehensiveness and clarity of the documentation, providing more detailed installation steps, feature descriptions, and integration statuses.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 4 (51e48a5c5495f17d6616999c27b19a7301d85478)</summary>

The commit `51e48a5c5495f17d6616999c27b19a7301d85478` by Louis Beaumont on September 18, 2024, updates the `README.md` file. The update adds a new entry to the "Latest News" section, stating that Screenpipe has reached 60 daily active users in September 2024. The specific change is the addition of the line:

```markdown
- [2024/09] Screenpipe hit 60 daily active users!
```
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 5 (f44270e40e9aa6a401d497b6861204de300c1d84)</summary>

The commit made by Louis Beaumont on September 18, 2024, updated the README.md file. The changes include:

1. Added a new entry under the "Latest News" section announcing the release of version 0 (v0) of the documentation with a link to it.
2. Removed the statement indicating that the feature was still experimental.

These adjustments improved clarity and provided an update on the documentation's availability.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 6 (911bbcd0cacadb8e0fcecc6d639077141b3bb58e)</summary>

The changes in the commit "911bbcd0cacadb8e0fcecc6d639077141b3bb58e" by Louis Beaumont can be summarized as follows:

1. **File Renaming**:
   - `motionDiv.tsx` was renamed to `motion-div.tsx` and moved to a new directory (`components` instead of `pages`).

2. **Updates to `_app.tsx`**:
   - Added a new import: `import { Metadata } from "next";`
   - Included metadata configuration for the app with fields such as title, description, icons, and Twitter card data.
   - The component structure within the `App` function was reformatted.

3. **Modifications in `_meta.js`**:
   - Updated the "plugins" key to "plugins (pipes)".

4. **API and Documentation Updates**:
   - Updated several MDX files to import `MotionDiv` from the new path (`../../components/motion-div`).
   - In the `faq.mdx` file, a large section of the FAQ content was removed and replaced with a link to external notes.

5. **Plugins Documentation Edits**:
   - Made several text changes in the `plugins.mdx` file to align with a more lowercase, simplified style.
   - Added more detailed instructions and streamlined text for better readability.

6. **Addition of Custom Favicon**:
   - Added a new favicon file `favicon.ico`.

7. **Theme Configurations**:
   - Reformatted the `theme.config.tsx` file without changing its core content.

These changes encompass code restructuring, text updates, new metadata configurations, adjustments in imports, and some stylistic edits in the documentation.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 7 (9185977455074ef94f94e6b73859b01d1fcaab76)</summary>

**Summary of Git Changes:**

**Commit:** `9185977455074ef94f94e6b73859b01d1fcaab76`
**Author:** Louis Beaumont
**Date:** Wed Sep 18 16:32:01 2024 -0700
**Description:** Chore: bunch of docs fix

**Changes:**

1. **New Files:**
   - `content/docs/NOTES.md`: Added new documentation containing troubleshooting tips and product FAQs for various issues users may face, with detailed steps to resolve each problem.

2. **Binary Files:**
   - `content/docs/bun.lockb`: Added a new binary file.

3. **Modifications to Existing Files:**
   - `content/docs/pages/_meta.js`: Renamed the index label from "Introduction" to "intro".
   - `content/docs/pages/docs/_meta.js`: Standardized the section titles to lowercase (e.g., "API Reference" to "api reference").
   - `content/docs/pages/docs/api-reference.mdx`: Reformatted headers and descriptions to be in lowercase and made minor text corrections.
   - `content/docs/pages/docs/architecture.mdx`: Reorganized the architecture overview section, reducing content and focusing on a high-level description of key components and a diagram overview.
   - `content/docs/pages/docs/contributing.mdx`: Replaced the detailed contributing guidelines with a link to the [contributing guide](https://github.com/mediar-ai/screenpipe/blob/main/CONTRIBUTING.md).
   - `content/docs/pages/docs/examples.mdx`: Reformatted headers to lowercase and adjusted text for consistency in describing use cases and benefits.
   - `content/docs/pages/docs/getting-started.mdx`: Reformatted headers to lowercase and adjusted narrative to be consistent in user instructions.
   - `content/docs/pages/index.mdx`: Updated introduction, focusing on key features and target audience with consistent lowercase formatting and slightly modified descriptions.
   - `content/docs/theme.config.tsx`: Updated text in the theme configuration for consistency, changing titles to lowercase.

4. **Deleted Files:**
   - `content/docs/pages/docs/troubleshooting.mdx`: Removed the troubleshooting guide, presumably because its content was incorporated into the new `NOTES.md` file.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 8 (05e6e4074b9f27cdb7e33c411fd6f83d57b6c043)</summary>

The commit `05e6e4074b9f27cdb7e33c411fd6f83d57b6c043` was made by Louis Beaumont on September 18, 2024. It involves merging a pull request (#331) from a branch named `main` by the user `bhupesh-sf`. The primary change introduced by this merge is the addition of a documentation website using Nextra.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 9 (795bc54153f22c92c424de7479b68fd4985082f7)</summary>

The commit `795bc54153f22c92c424de7479b68fd4985082f7` is a merge commit authored by Bhupesh Gupta on September 19, 2024. It merges changes from the branch `mediar-ai:main` into the local `main` branch.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 10 (f06ccfecce059e1f7cccc5d4784a0471989bbf0b)</summary>

The commit by Bhupesh Gupta incorporates feedback into a new documentation file, `plugins.mdx`, for the Screenpipe project. This documentation outlines the usage and development of "Pipes" and "Standalone Scripts" in Screenpipe, highlighting their roles in extending the platform's capabilities through custom workflows and integrations.

### Key Highlights:
- **Introduction to Plugins:** Explains the essential role of plugins in extending Screenpipe’s functionality through custom workflows that process screen and audio data.
- **Types of Plugins:**
  - **Pipes:** Integrate directly with Screenpipe and offer features like real-time processing, custom workflows, and integration capabilities.
  - **Standalone Scripts:** Run independently and offer flexible ways to automate and extend functionality.
  
### Details on Pipes:
- **Overview & Features:** Describes how Pipes can automate tasks like tagging, summarizing, and CRM integration, along with real-time data processing.
- **Running a Pipe:** Instructions to start a local server, download, and enable a pipe using curl commands.
- **Available Pipes:** A table listing different available pipes with descriptions and GitHub links.
- **Developing Pipes:** Guidance on creating custom pipes, including the required file structure and steps to set up.
- **Getting Featured:** Instructions for submitting custom pipes to the Pipe Store for sharing or monetizing.

### Standalone Scripts:
- **Overview:** Standalone Scripts help automate and extend functionality separately from the core app.
- **Available Standalone Scripts:** A table with examples, descriptions, and GitHub links.
- **Usage Instructions:** Steps to start using standalone scripts, including dependency installation and environment setup.
- **Contributing:** Encourages contributions and improvements to both Pipes and Standalone Scripts and provides instructions for submitting pull requests.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 11 (6ac3e579a6b2bcf30c8d1bc1e2dc155bbf6ee1e4)</summary>

Here's a summary of the git changes:

**Commit Overview:**
- **Commit Hash:** 6ac3e579a6b2bcf30c8d1bc1e2dc155bbf6ee1e4
- **Author:** Bhupesh Gupta
- **Date:** Thu Sep 19 02:40:10 2024 +0530
- **Message:** Incorporated the feedback

**Changes Breakdown:**

1. **File:** `_meta.js`
   - **Removed Entries:**
     - "core-concepts"
     - "installation guides"
     - "integrations"
     - "pipes"
     - "security"
     - "bonus features"
   - **Added/Reordered Entries:**
     - "plugins" added
     - Rearranged some entries like "troubleshooting" and "api-reference"

2. **File:** `api-reference.mdx`
   - **Change:** Added a missing newline at the end of a file.

3. **File:** `architecture.mdx`
   - **Change:** Commented out a section wrapped in `<MotionDiv delay={0.3}>`

4. **File:** `bonus-features.mdx`
   - **Status:** Deleted entire file.

5. **File:** `core-concepts.mdx`
   - **Status:** Deleted entire file.

6. **File:** `examples.mdx`
   - **Added YouTube iframes**:
     - For sections: Meeting Transcriptions and Summaries, Productivity Tracking, Compliance and Security Monitoring, Educational Material Organization, Real-Time Collaboration, and Annotation.

7. **File:** `getting-started.mdx`
   - **Additions:** 
     - Expanded sections detailing installation options per OS - macOS, Windows, and Linux.
     - More detailed and structured installation instructions with code samples.
     - Also moved the previously contained "installation guidelines" here.

8. **File:** `installation.mdx`
   - **Status:** Deleted entire file. The content has been merged into `getting-started.mdx`.

9. **File:** `integrations.mdx`
   - **Status:** Deleted entire file.

10. **File:** `pipes.mdx`
    - **Status:** Deleted entire file.

11. **File:** `security.mdx`
    - **Status:** Deleted entire file.

12. **File:** `index.mdx`
    - **Added:** Embedded a YouTube video for an introduction to Screenpipe.

**Summary:**
- Reorganized and cleaned up documentation by removing unnecessary files.
- Simplified menu structure in `_meta.js`.
- Merged and expanded installation instructions into the "Getting Started" guide.
- Added helpful YouTube video embeds to the "Examples" and "Welcome" sections for better user guidance.

</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 12 (35cd6e298b8b0ed630f1576342547f572f03e980)</summary>

The commit `35cd6e298b8b0ed630f1576342547f572f03e980` by Louis Beaumont, dated September 18, 2024, includes a bug fix. 

The change is made in the `settings.tsx` file located in the `screenpipe-app-tauri/components` directory. The modification adds a `defaultValue` attribute to a `Textarea` component, setting it to `localSettings.customPrompt`. This ensures that the `Textarea` element displays `localSettings.customPrompt` as its default value, addressing the prompt's default value issue.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 13 (9936ae35ce396e044edeae80a76c60b530ae478c)</summary>

The commit 9936ae35ce396e044edeae80a76c60b530ae478c by Louis Beaumont introduces a new feature allowing for Voice Activity Detection (VAD) sensitivity settings to be configurable both through the CLI and app UI.

- **File Affected**: `screenpipe-app-tauri/src-tauri/src/sidecar.rs`
- **Change Summary**:
  - Added logic to read the `vadSensitivity` value from a store. If the value is not set, it defaults to "high".
  - Included a new command-line argument `--vad-sensitivity` to the sidecar process if the VAD sensitivity value is other than "high".
  
These changes allow users to specify the sensitivity level of the VAD feature, either through the app's UI settings or directly via the command line, enhancing the customization options available to the user.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 14 (fc1f92797b10921f8a6cd1516b6c5bf1bbd656e8)</summary>

### Summary of Git Changes

**Commit Details:**
- **Commit:** fc1f92797b10921f8a6cd1516b6c5bf1bbd656e8
- **Author:** Louis Beaumont <louis.beaumont@gmail.com>
- **Date:** Wed Sep 18 12:22:19 2024 -0700
- **Description:** Added Voice Activity Detection (VAD) sensitivity feature to CLI & app UI settings.

**Files Modified:**

1. **`log-viewer-v2.tsx`:**
    - **Changes:**
      - Introduced log persistence using `localforage` to store and retrieve logs.
      - Added a button component to clear the logs.
      - Enhanced the log viewer to automatically save logs to `localforage` and retrieve them on initialization.
      - Slightly adjusted import formatting.

2. **`recording-settings.tsx`:**
    - **Changes:**
      - Adjusted the mapping of VAD sensitivity options.
      - Corrected the sensitivity labeling to be consistent.

3. **`Cargo.toml`:**
    - **Changes:**
      - Bumped the application version from `0.2.50` to `0.2.51`.

**Overall Enhancements:**
- **Log Viewer:**
    - Persistence has been added for log messages using `localforage`.
    - Added functionality to clear logs via a new button.
  
- **Recording Settings:**
    - Fixed and updated VAD sensitivity level mappings to a more logical structure.

- **Application Version:**
    - Updated the version to `0.2.51` reflecting the new features and fixes introduced.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 15 (c76bab75dc7a87a5c0d240a42674db72d07c0d8e)</summary>

### Summary of Changes

#### Commit Details:
- **Commit Hash**: `c76bab75dc7a87a5c0d240a42674db72d07c0d8e`
- **Author**: Louis Beaumont <louis.beaumont@gmail.com>
- **Date**: September 18, 2024

#### New Feature:
- **Voice Activity Detection (VAD) Sensitivity**: This commit introduces the ability to adjust VAD sensitivity both through the CLI and the application’s UI settings.

#### Key Changes:

1. **Frontend Changes (`recording-settings.tsx`)**:
    - **Imports**:
        - Added `VadSensitivity` import.
    - **Settings Update**:
        - Included `vadSensitivity` in the `settingsToUpdate`.
    - **Handler Methods**:
        - Added methods to handle changes in VAD sensitivity.
    - **UI Elements**:
        - Integrated a slider in the UI for adjusting VAD sensitivity with three options: low, medium, high.
        - Added tooltips for the VAD sensitivity slider to guide the user.

2. **Settings Hook (`use-settings.tsx`)**:
    - **Default Settings**:
        - Added `vadSensitivity` with a default value of `"high"`.
    - **Interfaces**:
        - Defined `VadSensitivity` type.
        - Updated `Settings` interface to include `vadSensitivity`.
    - **Settings Management**:
        - Logic to save and load `vadSensitivity` value from the settings store.

3. **Backend Changes (Cargo & Audio Files)**:
    - **Version Bump**: 
        - Updated app version to `0.2.50`.
    - **VAD Engine (`stt.rs`, `vad_engine.rs`)**:
        - Integrated VAD sensitivity settings, allowing configuration through both Silero and WebRTC VAD engines.
        - Methods to set VAD sensitivity and handle sensitivity-specific thresholds and ratios.
    - **VAD Configuration in Audio Transcription (`screenpipe-audio-forever.rs`, `screenpipe-audio.rs`)**:
        - Added `VadSensitivity` as an argument in the `record_and_transcribe` function.
        - Passed this argument where VAD engines are initialized.

4. **CLI and Server Changes (`screenpipe-server`)**:
    - **CLI Argument**:
        - Added a command-line argument to set VAD sensitivity.
    - **Server Initialization**:
        - Updated server startup to include VAD sensitivity in the configuration parameters.
        - Displayed VAD sensitivity status in server information prints.

### Overall Impact:
- Users can now configure the sensitivity of VAD via both command-line arguments and the application’s UI, enhancing control over audio detection quality and behavior.

</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 16 (68e6c9431592a062c5cc4f13787c0fbc9a53ce8b)</summary>

The git changes summarized in the commit (ID: 68e6c9431592a062c5cc4f13787c0fbc9a53ce8b) are as follows:

- A merge occurred, combining changes from the branches represented by commits `a9df45b` and `5f593bd`.
- The merge was authored by Louis Beaumont on September 18, 2024.
- The changes involved the removal of all references to blocking `reqwest`, replacing it with asynchronous `reqwest`.
- These changes were part of a pull request (#330) submitted by the user kerosina, aimed at fixing errors related to the Tokio asynchronous runtime.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 17 (5f593bd0e025178e4545951ff0c3fbeee0cfcd72)</summary>

The commit with ID `5f593bd0e025178e4545951ff0c3fbeee0cfcd72` addresses a merge mishap in the `screenpipe-audio/src/stt.rs` file. The author, `kerosina`, made the following changes:

1. **Function Signature Change**:
   - The function `transcribe_with_deepgram` was modified to be asynchronous. The change in the function definition is from:
     ```rust
     fn transcribe_with_deepgram(
     ```
     to:
     ```rust
     async fn transcribe_with_deepgram(
     ```

2. **Function Call Adjustment**:
   - In the `stt` function, the call to `transcribe_with_deepgram` now includes an `.await` to handle the newly async nature of the function. The modification is from:
     ```rust
     ) {
     ```
     to:
     ```rust
     )
     .await
     {
     ```

These changes ensure that the `transcribe_with_deepgram` function operates asynchronously and its call within `stt` is properly awaited.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 18 (878d725f5480cd8875db42db709c7cde9ee61222)</summary>

The recent Git commit `878d725f5480cd8875db42db709c7cde9ee61222` merges the `main` branch into a feature branch named `patch-fix-tokio-errors`. The commit was made by a user named Kerosina on September 18, 2024. 

The merge particularly modified the file `screenpipe-audio/src/stt.rs`. The main change observed in the diff is:

- The function `transcribe_with_deepgram`, which was previously an asynchronous function (`async fn`), has been changed to a synchronous function (`fn`). Additionally, a TODO comment regarding using an asynchronous request library (reqwest) to avoid blocking and potential crash issues has been removed.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 19 (27a792ede72d76a2c00f7427f0ec6d217d42e591)</summary>

The commit with hash `27a792ede72d76a2c00f7427f0ec6d217d42e591` was made by "kerosina" on September 18, 2024. The commit message indicates that it resolves a merge conflict.

The diff shows modifications in the file `screenpipe-audio/src/stt.rs`.

The specific change made:
- In the `transcribe_with_deepgram` function, a new parameter `sample_rate` of type `u32` was added to the function's parameters.

Before:
```rust
async fn transcribe_with_deepgram(
    api_key: &str,
    audio_data: &[f32],
    device: &str,
) -> Result<String> {
```

After:
```rust
async fn transcribe_with_deepgram(
    api_key: &str,
    audio_data: &[f32],
    device: &str,
    sample_rate: u32,
) -> Result<String> {
```

This code adjustment suggests that the function will now also require a sample rate value to process the deepgram transcription.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 20 (49eaf46bf7bdcda7b141b48a8cb2391a597052e9)</summary>

This commit introduces several changes to `stt.rs` in the `screenpipe-audio` module:

1. **Mutex Addition**: The `VadEngine` type in the parameters is now wrapped in a `Mutex`.

2. **Synchronous STT Function**:
    - A new function `stt_sync` is added that performs synchronous speech-to-text transcription.
    - It utilizes a new thread and employs `tokio` runtime for asynchronous operations within a synchronous context.
    - Various types were adjusted to be cloned and captured appropriately for the thread.

3. **Modification in Existing Functions**:
    - Within the `create_whisper_channel` function, the instantiation of `vad_engine` is updated to be wrapped in an `Arc` and `Mutex`.
    - Changes were made to call the new `stt_sync` function instead of the existing asynchronous `stt` function, especially for macOS targets within an `autoreleasepool`.

These changes essentially aim to switch certain parts of speech-to-text processing to synchronous execution, likely to address threading or performance issues on specific platforms such as macOS.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 21 (b47fff668c8caf90fd9cfd5500fca2a9cbd4a565)</summary>

Commit `b47fff668c8caf90fd9cfd5500fca2a9cbd4a565` is a merge commit authored by Bhupesh Gupta. This commit merges changes from the `main` branch of the `mediar-ai` repository into the current `main` branch. The merge incorporates new commits from `mediar-ai:main` into this branch, combining their histories. The merge was completed on Sun, Sep 15, 2024.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 23 (30cc98b239347f5bfa2d0c31e722cc63cb1c2f9d)</summary>

The commit by user kerosina on September 15, 2024, attempts to fix compilation issues on MacOS. The change involves modifying the `screenpipe-audio/src/stt.rs` file by adding an `.await` keyword to a function call. The modification is specific to the MacOS target configuration.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 24 (94d9d49d3e642a69a91d8d57636cc62766042425)</summary>

The given commit `94d9d49d3e642a69a91d8d57636cc62766042425` introduces a change to the `create_whisper_channel` function to make a block run asynchronously in the `screenpipe-audio/src/stt.rs` file. Specifically, inside the `if cfg!(target_os = "macos")` conditional, the `autoreleasepool` block has been modified to run its contents asynchronously by changing:

```rust
autoreleasepool(|| {
```

to:

```rust
autoreleasepool(|| async {
```
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 25 (43260f067ca2a815f849e8219fe39ea83e03c7c1)</summary>

The commit with hash `43260f067ca2a815f849e8219fe39ea83e03c7c1` by author `kerosina` on Sun Sep 15 14:35:06 2024, includes a fix to enable successful compilation on MacOS. Specifically, in the file `screenpipe-audio/src/stt.rs`, an asynchronous call to the `stt` function was updated. The change involves adding an `.await` to a match statement within a `#[cfg(target_os = "macos")]` block, ensuring the function call is properly awaited when compiled for MacOS.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 26 (b685cb36ae6686548e3d28c2f88b09e6487249a1)</summary>

The commit `b685cb36ae6686548e3d28c2f88b09e6487249a1` makes several updates related to switching from blocking to asynchronous (`async`) code in the `screenpipe-audio` and `screenpipe-integrations` components of the project. Below is a summary of the key changes:

1. **Updated imports**:
   - Replaced `use reqwest::blocking::Client` with `use reqwest::Client`.

2. **Updated functions to async**:
   - In `screenpipe-audio/src/stt.rs`:
     - Changed the function `transcribe_with_deepgram` to be async.
     - Adjusted the internal logic to await the requests and responses.
     - Changed other functions (`stt` and `create_whisper_channel`) to async and updated their internal calls to match.
   - In `screenpipe-audio/src/vad_engine.rs`:
     - Made the function `SileroVad::new` async.
     - Updated `SileroVad::download_model` to be async.
     - Changed the `create_vad_engine` function to async.

3. **Added dependency**:
   - Added `mime_guess` crate to `screenpipe-integrations`'s `Cargo.toml`.

4. **Updated `unstructured_ocr.rs`**:
   - Changed the `perform_ocr_cloud` and `unstructured_chunking` functions to be async.
   - Updated the multipart form creation to handle asynchronicity and proper MIME type determination.

These changes aim to improve the performance and responsiveness of the application by leveraging asynchronous HTTP requests instead of blocking calls, thus making the application more suitable for non-blocking, high-concurrency environments.
</details>

------------------------------------------------------------------------

# Overall Summary of Changes

### Summary of Git Changes

This series of commits primarily involve improvements to documentation, refactoring for better application performance, enhancements to UI/UX, and codebase changes. Here's a detailed summary:

1. **Documentation Enhancements:**
   - **New `README.md` (b16149887161cee...):**
     - **Project Documentation:** Added to describe a TypeScript project that automates logging of engineering work to Notion via Screenpipe and Phi 3.5 AI.
     - **Setup Instructions:** Detailed installation and setup steps.
     - **Customization Options:** Instructions to adjust logging frequency and AI output via configuration.

   - **Overall Documentation Updates:**
     - Expanded content for various `mdx` (Markdown with JSX) files.
     - Added FAQs, integration details, architecture overview, and detailed "getting started" guides for different platforms.
     - Standardized titles and organized content for improved readability and consistency.
     - Removed experimental feature notices.

2. **Codebase Changes:**
   - **Interface and State Updates:**
     - Renamed interface fields to camelCase and introduced new fields (`tags`, `deviceName`, `deviceType`) for better data handling.
     - Added new UI states (`isRefreshing`, `isClearing`) and methods (`handleRefresh`, `handleClearMeetings`) for enhanced functionality.
     - Incorporated new icon and tooltip imports to improve the UI.

   - **Async Function Conversion:**
     - Replaced blocking calls with asynchronous requests (`reqwest`) to enhance performance and prevent runtime errors, particularly handling asynchronous function calls properly in Rust using `.await`.

   - **Voice Activity Detection (VAD):**
     - Introduced VAD sensitivity settings in both CLI and application UI, improving customization in audio processing.
     - Added support for configuring VAD in the recording settings and sync functionality for VAD across various components.

3. **UI/UX Improvements:**
   - Redesigned meeting dialogs adding buttons for refreshing and clearing data.
   - Enhanced transcription sections with copy functionality and customizable prompts.
   - Added persistence to log viewer data using `localforage` for better user experience.

4. **Version Updates and Merge Fixes:**
   - Incremented package versions to reflect new features and fixes.
   - Addressed merge conflicts and compilation issues, particularly for MacOS, ensuring alignment with the main codebase.
   - Incorporated metadata configuration in `_app.tsx` for better SEO and social media sharing.

5. **Refactoring for Better Maintenance:**
   - Moved and renamed files for better clarity and maintainability (e.g., `motionDiv.tsx` to `motion-div.tsx`).
   - Simplified and streamlined code structure in various component and function definitions.

These changes aim to improve the documentation's usability, enhance application performance, and provide a more interactive and user-friendly interface for users.
