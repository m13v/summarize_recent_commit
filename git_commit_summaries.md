# Git Commit Summaries

-----------------------------------------------------------------------
-----------------------------------------------------------------------
 
PRESS CMD+SHIFT+V TO VIEW IN MARKDOWN
 
_______________________________________________________________________
-----------------------------------------------------------------------
Total number of commits: 255

<details>
<summary>summary for commit 1 (d56e58739042f41399cf4af70486c6ea8b1b4c1f)</summary>

The commit `d56e58739042f41399cf4af70486c6ea8b1b4c1f` by Louis Beaumont removes the functionality related to logging in the "memories" component of the project. Specifically, this commit deletes:

1. A cron job configuration from `pipe.json` that scheduled every 5 minutes to interact with the `/api/log` endpoint.
2. The entire log API route located at `src/app/api/log/route.ts`, which included logic for generating work log entries based on screen data, creating Markdown files, and syncing them to an Obsidian vault.

This effectively removes both the scheduling and execution of log generation within this component.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 2 (2b13370fda43063e2e91fc8832cc20a41d3f88fb)</summary>

The git commit introduces a changelog for version 0.22.2. Key updates include improvements and fixes:

### Improvements:
- **Noise Reduction in Storage:** Enhanced the quality of recorded content by reducing background noise.

### Fixes:
- **Memory Issues:** Addressed various memory-related problems to improve application stability and performance.

Additionally, the changelog reflects changes to simplify the handling of the Authorization header and updates the full changelog link to reflect these version changes. The commit does not include new features, as previously listed in the changelog, which have now been removed.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 3 (1105ac6f2de7652a1f15afca33b25491049bd776)</summary>

The git commit `1105ac6f2de7652a1f15afca33b25491049bd776` made changes to three files within the `screenpipe-app-tauri` project. Here are the key updates:

1. **pipe-store.tsx**:
   - The `id` of a `CorePipe` entry was modified from `"memories-gallery"` to `"memories"`.

2. **Cargo.lock**:
   - The version of the package named `screenpipe-app` was updated from `"0.22.0"` to `"0.22.1"`.

3. **Cargo.toml**:
   - The version of the `screenpipe-app` package was incremented from `"0.22.1"` to `"0.22.2"`.

These changes appear to be part of a release update, possibly minor version increments for package releases and a small update to the pipeline identification.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 4 (c7914943ced6aa3cc89a61694b62995743b0ddef)</summary>

The commit c7914943ced6aa3cc89a61694b62995743b0ddef, authored by Louis Beaumont, removes certain entries from the `corePipes` array in the `pipe-store.tsx` file. Specifically, the entries for the following pipes were deleted:

1. "obsidian time logger" which logs days in an Obsidian table.
2. "reddit question bot" for promoting content on Reddit.
3. "notion time logger" which logs days in a Notion table.

These changes indicate a cleanup or refactor, removing these specific features or functionalities from the project's codebase.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 5 (93c7d4ecb2966a72c599b5616158a1777c3d882c)</summary>

The commit `93c7d4ecb2966a72c599b5616158a1777c3d882c` by Louis Beaumont updated the `plugins.mdx` file in the documentation. The changes include updating the list of plugins and their descriptions. Several new plugins were introduced, including:

- **Memories Gallery:** A Google Photos-like gallery for screen recordings with AI insights.
- **Data Table:** A tool to explore data in a powerful table view with filtering and sorting options.
- **Search:** Allows searching through screen recordings and audio transcripts using AI.
- **Timeline:** Provides a visual AI-powered timeline of daily activities.
- **Speaker Identification:** Automatically identifies and labels different speakers using AI.
- **Meeting Assistant:** Organizes and summarizes meetings with AI-generated transcripts and insights.

Additionally, some existing plugin descriptions were modified for conciseness, such as the **LinkedIn AI Assistant** and **Obsidian Logs**. The descriptions for **Loom** and other plugins received minor updates or reordering.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 6 (0491dfe78afdce3e981e3ab17d0b74d08a60af1b)</summary>

The commit made by Louis Beaumont in the file `memories-gallery.tsx` contains refactoring and simplification of the component's structure:

1. **Removed Nested `div`:** The outer `div` with class `relative aspect-video` was removed, and the `VideoComponent` is now a direct child of the `motion.div`.

2. **Simplified Structure:** The component now lacks the nested structure and directly integrates the `VideoComponent`.

3. **Adjusted Classes:** The classes applied to the top-level `motion.div` were modified, removing the `relative` and `h-full` classes from the main container.

4. **Condensed Metadata Display:** The section displaying `Calendar` and timestamp information has been simplified into a single `div` that is centered without separate wrapped containers.

5. **Removed URL Display:** The display of the `memory.app_name` and its corresponding `title` attribute were removed. 

Overall, these changes simplify the component and reduce redundant wrapping elements.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 7 (4d38ea28af877670b587e575471e340314f7fc55)</summary>

The commit `4d38ea28af877670b587e575471e340314f7fc55` by author `neo773` addressed GitHub rate-limiting issues in the application. Key changes include:

1. **Version Update**: The `screenpipe-app` package has been updated from version `0.21.9` to `0.22.0`.

2. **Dependency Changes**: New dependencies have been introduced in `screenpipe-core/Cargo.toml`:
   - Added `http-cache-reqwest` version `0.15.0`.
   - Added `reqwest-middleware` version `0.4.0`.

3. **Code Modifications**:
   - Several changes were made in `screenpipe-core/src/pipes.rs`, including:
     - Transition from using `reqwest::Client` to utilizing a cached client setup with `reqwest_middleware` and `http_cache_reqwest` for improved API responses handling.
     - Enhanced the handling and downloading of files from GitHub, including checking for file and directory types and using a more efficient approach for processing files in parallel.
     - Added debug logging for better insight into API URL conversions, cache hits, download processes, and ignored files.
     - Refactored API call constructs to accommodate the changes in request handling.

4. **Debugging and Logging**:
   - Improved logging throughout the code by changing log levels from `info` to `debug` to reduce verbosity in standard operation and provide detailed insight during debugging sessions.

5. **Platform-specific Fixes**:
   - A note mentions a path fix on Windows, though specifics are not evident within the given changes.

This commit improves the robustness and efficiency of Git interactions by employing cached HTTP requests, handling files more effectively, and providing better debug insights.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 8 (4707469491bf5320d0384ea2259850f9fab70cef)</summary>

In this commit, the author, Louis Beaumont, updated the `README.md` file located in the `screenpipe-integrations/screenpipe-mcp` directory. The changes include the insertion of an empty line at the beginning and the addition of a URL `https://github.com/user-attachments/assets/7466a689-7703-4f0b-b3e1-b1cb9ed70cff` between the initial empty lines and the section titled "1. Configure Claude Desktop".
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 9 (76d8d58676fba797f50844e43baa84121b88b8fb)</summary>

The commit updates the `README.md` file in the `pipes/memories/` directory. The update adds an image by including an HTML `<img>` tag at the end of the file. The image is a screenshot (with specific dimensions) sourced from a URL that belongs to the user's GitHub assets.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 10 (8d10f579f16a495fc8a291715ae228fd28df4e82)</summary>

The commit by Louis Beaumont updates the `README.md` file in the `pipes/memories` directory. The change involves the removal of a line that contained an image link from the document, which previously displayed an image in the README file. The rest of the text remains unchanged.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 11 (6ea20e5e1ddf87d2620068667f0aaac0c6165aef)</summary>

The commit updates the `README.md` file located in the `pipes/memories` directory. The changes include:

1. The description of the project has been modified. It now describes the application as a tool that provides "Google Photo-like memories of your days" with resurfacing and reminders of important information. The previous description mentioned real-time analysis of screen/microphone activity to extract specific information related to CRM, customer, market research, or ideas.

2. An image has been added with the markdown syntax for embedding images, linking to a URL under `https://github.com/user-attachments/assets/`.

3. A link to another GitHub user attachment and an HTML `img` tag specifying a screenshot have been removed from the document.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 12 (7d0bd1a3ae9a1358a33e4609f167c532a0a4ef9b)</summary>

The commit with hash `7d0bd1a3ae9a1358a33e4609f167c532a0a4ef9b` by Louis Beaumont on December 30, 2024, addresses a memory-related fix in the `memories-gallery.tsx` file.

**Changes made:**
1. A class modification was made to the main container of a component:
   - Added `h-full` to ensure the element takes the full available height.
   
2. Adjustments in the child div elements:
   - Removed the `flex-shrink-0` class from the `aspect-video` div, likely to allow it to flex as needed.
   - Changed `flex-1` to `flex-grow` in the class attributes for a div, suggesting an improvement in the way space is allocated for this element to prevent underutilization or overflow of memory.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 13 (1f6b36e894ab8a5763dde0d9e89f8d0597737718)</summary>

The commit by Louis Beaumont, identified by hash `1f6b36e`, addresses memory handling in the `memories-gallery.tsx` component located in the `pipes/memories/src/components` directory. The main change introduces an initial loading state using skeleton components when `isLoading` is true. This loading state uses an array of skeleton elements to simulate the layout of loaded content, improving user experience while data is being fetched. The skeleton components include placeholder rectangles representing the media and text content. The existing logic for rendering the list of memories remains, but is now conditionally rendered based on the loading state.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 14 (964f3637e3d084943b081610d984b2e17ea6b301)</summary>

The commit `964f3637e3d084943b081610d984b2e17ea6b301` introduces a changelog for version 0.22.1. The key updates include:

### New Features:
- **Memory Gallery Pipe:** A new feature similar to Google Photos has been added for enhanced media organization.

### Improvements:
- **Authorization Header Handling:** The processing of the Authorization header has been simplified for better integration with the screenpipe-cloud AI provider.

### Fixes:
- **Pipe Search Header:** Issues related to the search header functionality have been resolved for improved usability.

A full comparison of changes can be viewed at the provided GitHub link: [276cb..0ebbc](https://github.com/mediar-ai/screenpipe/compare/276cb..0ebbc). 

Additionally, the changes reflect updates in the public changelog of the Tauri app to match these enhancements and fixes.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 15 (0ebbc53a128270479e744c5d54f54a9d370842a3)</summary>

The git commit with the hash `0ebbc53a128270479e744c5d54f54a9d370842a3` authored by Louis Beaumont on December 30, 2024, updates the `Cargo.toml` file in the `screenpipe-app-tauri/src-tauri` directory. The version of the `screenpipe-app` package is incremented from `0.22.0` to `0.22.1`. This suggests a minor release or patch update for the application.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 16 (79ff2aefdbd006af1b5c8bf7486ed37e45493255)</summary>

The commit introduces several updates and new features, focusing on the addition of a "memories gallery" and improvements to existing components. Here's a summary:

1. **Memories Gallery Feature**: 
   - A new gallery feature similar to Google Photos, called the "memories gallery," is added. It leverages AI to provide insights and timeline visualization of screen recordings.
   - The new component includes a lot of UI elements such as buttons, tooltips, and dialogs to interact with the gallery.
   - Various UI components were created or modified to support this feature, including accordion, badge, carousel, buttons, input fields, and more.

2. **New Files and Changes**:
   - A significant number of new files are introduced, largely markdown files, configuration files (`tailwind.config.ts`, `postcss.config.mjs`, etc.), and JSON files (e.g., `eslint.config.mjs`, `components.json`).
   - New source files for components and hooks are introduced to support the interaction and functionality within the memories gallery.

3. **Improved Video Component**: 
   - Updates are made to the video component across different pipe directories, enhancing the handling of file links and potentially integrating it into the new memories gallery workflow.

4. **Pipe Store Updates**:
   - Changes to the `pipe-store.tsx` suggest support for the new memories gallery within the core functionalities of the app. This involves the addition of the memories gallery to the list of core pipes.

5. **General Improvements**:
   - Common UI elements have been improved, with enhanced visual representations and greater interactivity.
   - Configuration files and settings management received updates to cater to new functionalities, including integration into existing APIs and logging mechanisms.
   - `.gitignore` was updated to accommodate new dependencies and framework-specific files.

Overall, this commit enhances the app's capabilities by adding a sophisticated photo-like gallery for screen recordings and enriching user interaction through a multitude of new and improved UI components.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 17 (3cd8bebc6000f8ee2c4198e6df74b238b509c0a5)</summary>

The commit `3cd8bebc6000f8ee2c4198e6df74b238b509c0a5` by the author "fortran01" with co-author "Louis Beaumont" refactors the handling of the Authorization header for the "screenpipe-cloud" AI provider. This change, which is referenced by the issue number #1068, simplifies the way the Authorization is processed. Specifically, in the file `search-chat.tsx`, an additional object containing the abort signal is now passed to the function call. This involves adding the `signal: abortControllerRef.current.signal` configuration to the object parameters within the `SearchChat` component function.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 18 (3717db0e2bf97f56e947f1f68697297483b41dd3)</summary>

The commit `3717db0e2bf97f56e947f1f68697297483b41dd3` by Louis Beaumont on December 30, 2024, makes a fix to the `search-chat.tsx` component within the `pipes/search` directory. Specifically, it removes an object from a function call, which included abort controller signaling and an `Authorization` header. The removed lines previously added the user's token to the headers for authentication. This change suggests a simplification or alternative handling of authentication or signaling within the function.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 19 (276cbe8a0c32f777981dfcfb8dbda80789cf64ff)</summary>

The commit `276cbe8a0c32f777981dfcfb8dbda80789cf64ff` authored by the GitHub Actions Bot adds a changelog for version 0.22.0 of a project called "Screenpipe". The changelog is introduced in two markdown files, one being newly created. Below is a summary of the documented changes:

### New Features
- Swapped configurations to enhance flexibility and allow for improved user customization.

### Improvements
- Made optimizations to improve the performance and efficiency of the store.

### Fixes
- Resolved an issue with Claude MCP time prompts to ensure correct functionality.
- Fixed an error in the video component to maintain proper operation.

Both files state the full changelog is available through a GitHub comparison link, offering a detailed commit history between two specified commit hashes.

The changelog is consistent across both the newly created and updated files within the repository.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 20 (0d277a876fb50277b1bc9d5ffdd7a9f624565691)</summary>

The commit with hash `0d277a8` updates the version of a package in the `Cargo.toml` file. The version is incremented from `0.2.22` to `0.2.23`. This change appears to be a simple bump in the command-line interface (CLI) version, typically done for release purposes or minor improvements.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 21 (04cc7632991ce07775e989669b7e86c41d6f92ce)</summary>

The Git commit with the hash `04cc7632991ce07775e989669b7e86c41d6f92ce` made by Louis Beaumont updates the version of the package named "screenpipe-app" in its `Cargo.toml` file from "0.21.9" to "0.22.0". The commit is labeled with the message "release-app," indicating that this change is likely part of a new release of the application.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 22 (0acb1290518e94c1093b4cdf211f5aef6cb0090c)</summary>

The changes in the given commit involve a major update that introduces profile management and configuration handling for a Screenpipe app built using Tauri. Here's a summary of the changes:

1. **New Feature: Profile Management**
   - A new feature to handle user profiles has been added through a new custom hook `useProfiles` and several updates to settings management.
   - The app now supports multiple profiles, allowing users to switch between different sets of configurations.
   - Profiles are stored in separate files, with mechanisms to create, delete, and copy profile settings.
   - Profiles can be switched using a UI dropdown menu, and screenpipe restarts automatically upon profile switching.
  
2. **Global Shortcuts Update**
   - The app's shortcut management was updated to accommodate profile-specific shortcuts.
   - Logic was added for assigning and storing shortcuts that allow switching between user profiles.
   - The backend is capable of registering and managing these shortcuts using the `ShortcutConfig` in Rust.

3. **UI Components Updates**
   - The UI components related to settings and shortcuts were updated to reflect the addition of profiles and their management.
   - A new profile dropdown was added to the settings to handle creating, selecting, and deleting profiles.
   - Existing shortcut management UI was updated to display and handle profile-related shortcuts dynamically.

4. **Backend and Store Logic Changes**
   - Changes in Rust include the use of `LazyStore` to simplify handling persistent data for both settings and profiles.
   - The backend now manages multiple store files based on the active profile, ensuring separation of settings.

5. **Infrastructure and System Updates**
   - Adjustments have been made to ensure cross-platform compatibility with potential management logic for data paths.
   - Updates include handling binary changes in `ui_monitor` presumably for compatibility or feature parity purposes.

Overall, the major thrust of this commit involves enabling users to manage multiple profiles, with seamless switching and persistence, along with corresponding UI, backend, and infrastructure changes needed to support this functionality.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 23 (d657fd4f50ea393e17e4c9dfc92f442e36564eb6)</summary>

The git changes involve archiving three directories of projects or "pipes" by moving them into an "archive" directory. Specifically, the directories `pipe-find-leads-with-exa-ai`, `pipe-llama32-comment-linear-while-you-work`, and `pipe-obsidian-time-logs` were all relocated from their original location under `pipes/` to `pipes/archive/`. Each file within these directories, such as `.gitignore`, `README.md`, `bun.lockb`, `package.json`, `pipe.json`, `pipe.ts`, and `tsconfig.json`, was moved correspondingly, maintaining their structure and content, as indicated by the 100% similarity index in the diff output.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 24 (776e5cca99c09b06264f6ef69bce329b4c615f2d)</summary>

The commit made by Louis Beaumont addresses changes related to handling time prompts in the `screenpipe-mcp` module. The following updates were made:

1. A new `.gitignore` file was added to the `screenpipe-integrations/screenpipe-mcp/` directory. This file ignores Python cache files, distribution and packaging directories, virtual environments, IDE-specific files, and other temporary files like `.DS_Store`.

2. Two Python cache files (`__init__.cpython-311.pyc` and `server.cpython-311.pyc`) were deleted from the `__pycache__` directory in `screenpipe_mcp/src/screenpipe_mcp/`.

3. In the `server.py` file, the descriptions for the `start_time` and `end_time` fields were updated to clarify that times should be in ISO format UTC and note the importance of using the user's time and timezone.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 25 (827c4f2c860a0fddf83ec16793d1f7ddd5487c0e)</summary>

The commit with hash `827c4f2c860a0fddf83ec16793d1f7ddd5487c0e` updates the `README.md` file located in the `screenpipe-integrations/screenpipe-mcp` directory. The changes primarily focus on instructions for configuring the Claude Desktop application. 

Key changes include:
- A new link to download the Claude desktop app has been added.
- Instructions for cloning the repository using Git have been introduced.
- The section for editing the Claude app configuration file has been expanded. It now includes both Windows and Mac-specific instructions, with additional text editors suggested for Mac (`cursor` or `vim`).
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 26 (8c5e7a857c57d8c647736268cd1342c5b35476ee)</summary>

The commit with ID `8c5e7a857c57d8c647736268cd1342c5b35476ee` authored by Louis Beaumont on December 29, 2024, updates the `README.md` file in the `screenpipe-mcp` directory. The change is a modification to the instructions for opening the `claude_desktop_config.json` file on Mac systems. Previously, the command was `code ~/Library/Application Support/Claude/claude_desktop_config.json`, and it has been updated to include quotation marks around the path: `code "~/Library/Application Support/Claude/claude_desktop_config.json"`. This change likely ensures proper handling of spaces in the file path when using the command.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 27 (a4cf24b70a0dd9d88ba02b981449b277f9058859)</summary>

Here is a summary of the changes made in the commit with hash **a4cf24b70a0dd9d88ba02b981449b277f9058859**:

1. **Video Components:**
   - In all files (`identify-speakers`, `search`, and `timeline` components), a log level change was made from `console.error` to `console.warn` for the error message "Failed to load media".

2. **Pipe Store Enhancements:**
   - Modified the behavior of the store in the `pipe-store.tsx` file.
   - Introduced a button that opens a pipe window when the conditions are met (pipe has a configuration port and is enabled).
   - If opening the window fails, it logs the error and displays a user notification. This change makes the conditional rendering and error handling more robust.

3. **Version Bump:**
   - Updated the version of the `screenpipe-app` in `Cargo.lock` from `0.21.8` to `0.21.9`.

4. **Schema Updates:**
   - The `capabilities.json` file was significantly expanded to include more detailed permission scopes related to file system and shell execution.
   - Added permissions to allow executing specific shell commands and accessing certain paths, improving the app's capability configuration.

These changes focus on improving the usability and stability of the application by enhancing error handling, feature adjustments, and permission management.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 28 (12de8b70a12d8df8715df10822a942c8f8534329)</summary>

The commit labeled `12de8b70a12d8df8715df10822a942c8f8534329` updates the documentation by adding a changelog for the version 0.21.9 of the project "Screenpipe." 

### Summary of Changes:

- **Fixes:**
  - A Next.js related error has been resolved to improve application stability.

- **Improvements:**
  - The default settings for the AI feature have been adjusted to enhance performance.

- The changelog does not list any new features added in this release.

Additionally, the changelog includes a link to the full list of changes in the version comparison [87f73..db544](https://github.com/mediar-ai/screenpipe/compare/87f73..db544). The update is reflected in two files: `0.21.9.md` (a new file) and `CHANGELOG.md` in the `screenpipe-app-tauri` directory.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 29 (db54437b08ac0f418a1cbdcb7b305a75b811e2c4)</summary>

The commit with ID `db54437b08ac0f418a1cbdcb7b305a75b811e2c4`, authored by Louis Beaumont, includes a change in the `screenpipe-app-tauri/src-tauri/Cargo.toml` file. The version number of the package "screenpipe-app" was updated from "0.21.8" to "0.21.9". The commit is likely part of a release process for the application.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 30 (7e8bb1aff859dbb855f529325c99cee384e60411)</summary>

The commit `7e8bb1a` by author `neo773` on December 29, 2024, addresses a Next.js error. The changes involve:

1. **Version Update**: The version of the package `screenpipe-app` in the `Cargo.lock` file was updated from `0.21.7` to `0.21.8`.

2. **Code Changes in `pipes.rs`**: In the file `screenpipe-core/src/pipes.rs`, modifications were made to the command execution logic that starts a Next.js project. Specifically, the `Command::new` invocation now includes an added argument `--bun` in two places where a child process is started using the `bun_path`. This likely provides a required flag to the `bun` command during execution.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 31 (146f6d2e278d40ef172dcb6230bddcef82a1c0ae)</summary>

The commit titled "small AI default change" modifies two files related to AI settings in the project:

1. **ai-section.tsx**: The tooltip content was updated to reflect new specifications for character limits in AI models. The estimate of characters per token was changed from 3 to 4. Additionally, the description for the maximum number of characters supported by OpenAI models was updated from "30k-40k characters" to "roughly 512k characters."

2. **use-settings.tsx**: The default setting for `aiMaxContextChars` was increased from 30,000 to 512,000 characters, aligning with the updated maximum character capacity mentioned in the tooltip.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 32 (87f7364eaf25a7e32122e5d3aa8170e0a39aaac6)</summary>

This commit introduces the changelog for version 0.21.8 of the software. The changelog highlights several updates across three main categories:

1. **New Features:**
   - Added support for Gemini AI models in Screenpipe Cloud, enhancing its capabilities.

2. **Improvements:**
   - Improved version extraction in the installation script for better accuracy.
   - Reduced noise in cursor rules to enhance tracking accuracy.

3. **Fixes:**
   - Fixed a scrolling issue in the audio device selection and AI models interface.

Additionally, the full changelog can be viewed via a provided GitHub link. Some redundant parts were also updated in the `screenpipe-app-tauri/public/CHANGELOG.md` file to align with these changes.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 33 (a5053721b99291b2aa87d2e479074609359da6a7)</summary>

The commit introduces several updates and fixes to the `screenpipe-app` and related components. Here's a summary of the changes:

1. **Bug Fixes and Features**:
   - Resolved an issue where scrolling was not functioning in the audio device selection and AI models sections.
   - Added support for "ad gemini" models in ScreenPipe Cloud, allowing for compatibility with Google's Gemini language models.

2. **Code Clean-up and Revisions**:
   - Removed a redundant `console.log` statement from the `search-chat.tsx` component.
   - Minor code formatting adjustments in `ai-proxy` to enhance readability and maintain consistency.

3. **Functionality Additions**:
   - Added an environment variable `GEMINI_API_KEY` to facilitate the integration of Gemini models.
   - Enhanced model selection options in the AI settings, including new Gemini models.

4. **UI and UX Adjustments**:
   - Tweaks to dialogue popups and authentication messaging, improving clarity and user guidance.

5. **Removed Features**:
   - Eliminated the `open_auth_window` command and corresponding authentication window handling logic.

6. **Version Updates**:
   - Updated the version in `Cargo.toml` from `0.21.7` to `0.21.8` and reflected this change in `Cargo.lock`, likely signifying a new release of the app.

These changes collectively aim to improve the app's functionality, compatibility, and overall user experience.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 34 (582d91bc879f0c44fb1479d1f7770be6e8fe488a)</summary>

The commit titled "trim noise from cursorrrules" by Louis Beaumont updated the `.cursorrules` file. The main change was the removal of a large block of comments and examples detailing coding practices and principles in various programming contexts. These examples included logging styles in Rust, error handling, async patterns, UI styling with NextJS/TailwindCSS, and API usage examples. The key principles advocated in these comments focused on performance (using channels over mutexes), clarity (consistent error messages and logging), UI design preferences, context awareness, and cross-platform considerations. After these changes, only three coding rules remain, focusing on using specific technologies, escaping HTML appropriately, and using HTML entities within React or HTML code when necessary.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 35 (a296dacfc59367a2ee93cef403fc05aa93187d92)</summary>

The commit `a296dacfc59367a2ee93cef403fc05aa93187d92`, authored by `fortran01`, introduces improvements to the version extraction logic within the `install.sh` script. The update focuses on enhancing the method used to parse the latest version number from a GitHub release. The script now uses a more robust combination of `grep` and `sed` commands, which improves cross-platform compatibility and resolves potential inconsistencies in version parsing. Previously, the version was extracted using `cut`, but this has been replaced with a purely `sed`-based approach for capturing the version number more accurately.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 36 (2c6682655605bd672da40901c7272f7e8ac0b8cd)</summary>

The commit by Louis Beaumont on December 24, 2024, titled "release-app fix," modifies the GitHub Actions workflow file, `release-app.yml`. The change removes a step that was intended to skip jobs for a self-hosted environment when the `tauri-args` is set to `--target x86_64-pc-windows-msvc`. Specifically, the code block responsible for this check, which consisted of a step named "skip self hosted" that exited with status 1, has been deleted. The rest of the workflow and other steps are unchanged.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 37 (23942d84e582fc5f4777974a85a0afe46534c188)</summary>

This Git commit, made by Louis Beaumont on December 24, 2024, introduces the following changes to the project:

1. **GitHub Actions Workflow (`release-app.yml`):**
   - A new step was added to the workflow to skip running certain jobs on a self-hosted runner for the `x86_64-pc-windows-msvc` target. This is achieved by exiting with code `1` when the condition is satisfied.

2. **Cargo.toml Update:**
   - The version number of the `screenpipe-app` package in the `Cargo.toml` file was incremented from `0.21.6` to `0.21.7`.

3. **Capabilities Configuration (`main.json`):**
   - A significant update was made to the capabilities configuration file. The changes include:
     - Added permissions for various file system paths using environment variables like `$XDG_DATA_HOME`, `$LOCALAPPDATA`, `$APPDATA`, `$HOME`, etc.
     - Defined multiple shell execution permissions, allowing specific shell commands and arguments to be executed as sidecar processes, such as `screenpipe pipe purge`, `screenpipe setup`, `screenpipe migrate`, etc.
     - Added HTTP permissions for various HTTP methods (GET, POST, etc.) to be used with URLs matching certain patterns.

These changes collectively enhance the release process, increase the version of the application, and expand the application's capabilities regarding file access, shell command execution, and HTTP requests.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 38 (4e881d3efce576a4737ad72f2515b37e23128eb3)</summary>

The commit `4e881d3efce576a4737ad72f2515b37e23128eb3` by Louis Beaumont on December 24, 2024, is titled "release-app 🎅 CHRISMAS TIME 🎅" and includes the following changes:

1. **File: `use-settings.tsx`**
   - Removed console error logging for platform detection within the `createDefaultSettingsObject` and `useSettings` functions.

2. **File: `Cargo.toml`**
   - Updated the package version from `0.21.5` to `0.21.6`.

3. **File: `main.rs`**
   - Removed the import of `DeepLinkExt` from the `tauri_plugin_deep_link`.
   - Added setup for tray menu updater with an update item.
   - Removed a previously existing but unresolved merge conflict block related to deep linking.
   - Removed code related to deep link registration and handling.

4. **File: `sidecar.rs`**
   - Removed unused `std::env` import and related code that previously obtained the executable's directory path.
   - Refactored sidecar spawning code, removing Windows-specific logic about executable directory paths.

5. **File: `tray.rs`**
   - Removed unnecessary `info` tracing, leaving only `debug` statements.

Overall, this commit refines existing functionality by cleaning up debug information, resolving merge conflicts, handling the tray menu updates, and updating the application's version for release.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 39 (798299f8e20fa68c46f81abcb340ea2972068ba0)</summary>

The commit introduces several changes and improvements related to global shortcut management in the Tauri application. Here's a summary of the key changes made in `main.rs`:

1. **New Imports**: Imports for `tauri::AppHandle`, `tauri_plugin_deep_link::DeepLinkExt`, and `tauri_plugin_global_shortcut` were added to enable deep linking and global shortcut functionalities.

2. **Shortcut Configuration**: A new `ShortcutConfig` struct is defined to manage the configuration of shortcuts. It includes fields for the shortcuts to show, start, and stop actions, as well as a list of disabled shortcuts.

3. **Shortcut Loading**: The `ShortcutConfig` struct includes an `async` method `from_store`, which loads the shortcut configuration from a stored file (`store.bin`). This method sets default values if none are stored.

4. **Shortcut Helper Functions**: Several helper functions were added:
   - `register_shortcut`: Asynchronously registers a global shortcut and defines a handler for when the shortcut is triggered.
   - `update_global_shortcuts`: Updates the global shortcuts based on user input.
   - `initialize_global_shortcuts`: Initializes global shortcuts from stored configuration at startup.
   - `apply_shortcuts`: Applies new shortcut configurations, unregisters old shortcuts, and registers the new ones.

5. **Shortcut Usage**: Global shortcuts for showing the app, starting, and stopping recording are registered. They have handlers that execute corresponding actions when triggered.

6. **Deep Link Handling**: Additional logic was introduced to handle deep linking, specifically for Linux and Windows platforms. URLs received through deep links are logged, and registration for a specific URL scheme (`screenpipe`) is attempted.

7. **Concurrency**: The initialization of global shortcuts is run in a separate async task using Tauri's async runtime to prevent blocking.

Overall, this commit enhances the global shortcut functionality in the application, making it more configurable and dynamic, while also adding initial support for deep linking abilities.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 40 (ce5d3187f57affa79084bbadecb95d312722b71b)</summary>

The commit with ID `ce5d3187f57affa79084bbadecb95d312722b71b` by Louis Beaumont on December 24, 2024, contains multiple changes focused mainly on refactoring and improvements related to deep linking in the `screenpipe-app-tauri` project. 

1. **Dependency Updates**:
   - The `Cargo.lock` has been updated to include `tauri-plugin-deep-link` among the dependencies.
   - The `Cargo.toml` was modified to adjust the version specification for `tauri-plugin-deep-link` from a fixed version `2.2.0` to support the latest compatible version `"2"`.
   - `tauri-plugin-single-instance` now includes a feature for `"deep-link"`.

2. **Code Clean-up**:
   - Removed unneeded whitespace in various source files, including `commands.rs` and `main.rs` to make the code more consistent and readable.
   - Neatened imports, ensured consistent usage of `use` statements, and reordered imports for better readability.

3. **Deep Link Integration**:
   - Integrated `tauri-plugin-deep-link` initialization into the application workflow, ensuring it is the first plugin initialized in `main.rs`.
   - Removed redundant or misplaced deep link handling code in `main.rs`; specifically, removed the section involved with registering URLs and handling deep link URL events for Windows/Linux.

4. **Removed Redundant Permissions**:
   - A significant amount of redundant permission entries were removed in the `capabilities/main.json`, considerably simplifying the permissions schema.

5. **Schema Additions**:
   - Added `windows-schema.json` which contains JSON schema definitions for capabilities and permissions, likely contributing to better validation or documentation of the capabilities system.

6. **General Improvements**:
   - The code formatting improvements across multiple files suggest an extensive code base clean-up and standardization effort.

These changes are part of a broader effort to streamline the handling of deep links in a Windows environment and to declutter the project from outdated or redundant configurations. Overall, these enhancements likely contribute to a more maintainable and cohesive code base with improved functionality for managing deep links.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 41 (7da94cbf3cd799baef60e49976f63cf5a5a91b47)</summary>

The commit `7da94cbf3cd799baef60e49976f63cf5a5a91b47` by Vivek Kumar addresses a couple of fixes and enhancements pertaining to query responses and the anthropic MCP integration. It also adds updates to the setup README file.

### Changes:
1. **README.md Update**:
   - Instructions were added for configuring `claude_desktop_config.json` on both Windows and Mac, including commands for opening the configuration file in editors.
   - A note was added to remind users to restart Claude Desktop after making changes.

2. **Server Python Script Modifications (`server.py`)**:
   - Imported the `json` module for handling JSON responses.
   - Code formatting was enhanced for consistency, such as ensuring uniform spacing in dictionary values and import statements.
   - Enhanced error handling where JSON responses are now explicitly parsed using `json.loads`, and a `JSONDecodeError` is caught with an appropriate error message.
   - Improved logic to append search results to the results list and error handling now includes logging to the console.
   - Fixed a bug where content types were retrieved directly from the result rather than nested content attributes.
   - Overall, structured and formatted the response building process for returned search results ensuring they are correctly compiled into formatted text blocks.

These updates collectively improve the robustness and user guidance within the MCP component of the project.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 42 (28964a39b778ab466f55582782b5a54118d58e92)</summary>

In this commit, new features related to global keyboard shortcuts have been implemented in the "screenpipe-app-tauri" application. Here's a summary of the changes:

1. **New Global Shortcuts and Event Handling:**
   - Introduced global keyboard shortcuts for starting and stopping screen recording using the `@tauri-apps/api/event` and `@tauri-apps/api/core` libraries.
   - The `useSettings` hook has been extended to handle new shortcuts: `startRecordingShortcut` and `stopRecordingShortcut`.

2. **Settings Updates and Default Configuration:**
   - Modified the settings state to include `startRecordingShortcut` and `stopRecordingShortcut`.
   - Default shortcut mappings are provided, such as "Super+Alt+R" for start recording and "Super+Alt+X" for stop recording.

3. **User Interface Enhancements:**
   - Updated the `ShortcutSection` component to allow users to configure and update their keyboard shortcuts for various actions like showing the interface, starting, and stopping recording.
   - Added visual feedback for shortcut recording sessions using hotkeys for consistent detection of key presses.

4. **Backend Command Enhancements:**
   - Extended Rust code in the `commands.rs` and `main.rs` files to handle commands related to hiding the main window and managing shortcuts registration.
   - Implemented shortcut registration and unregistration logic using the `tauri_plugin_global_shortcut` plugin.

5. **Error Handling and Notifications:**
   - Notifications are displayed when starting or stopping recording fails or succeeds, providing feedback to users.
   - Error handling is improved for shortcut updates and registration processes.

6. **Dependencies and Build Changes:**
   - Added `hotkeys-js` to handle keyboard input more reliably.
   - Updated the `Cargo.lock` for Rust dependencies and incremented the version from "0.21.4" to "0.21.5."

7. **Tray Menu Additions:**
   - The tray menu now includes options for starting and stopping recording, providing quick access to these features from the system tray.

In summary, these updates significantly enhance user interaction with the application by integrating global keyboard shortcuts for controlling key features such as starting and stopping screen recordings and toggling the app's main interface visibility.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 43 (ff523f7cf8536be234585a83519f783af2790d3e)</summary>

The commit by Louis Beaumont with the message "fix build (but windows deep link still does not work)" includes the following changes:

1. **GitHub Workflow Update:**
   - The `release-app.yml` file in the GitHub workflows directory was modified to include retry logic for the upload of assets using the `cn-cli`.
   - Two new environment variables `MAX_RETRIES` and `RETRY_DELAY` were introduced, with values `3` and `10` respectively.
   - The workflow now has a loop that attempts to upload assets multiple times (up to 3 times) with a delay of 10 seconds between attempts in case of failure. If all attempts fail, the script will exit with an error message.

2. **Rust Code Modifications:**
   - The code in `main.rs`: Adjusted the conditional compilation for deep link URL registration to include Windows (`#[cfg(any(windows, target_os = "linux"))]`). The deep link registration was updated to include the scheme `"screenpipe"`.
   - In `sidecar.rs`: Removed the setting of the `TESSDATA_PREFIX` environment variable on Windows. This involves removing the creation of a path to `tessdata` and its usage in configuring a sidecar process.

Overall, these changes appear to fix some build processes and adjust environment configurations, though the deep link functionality for Windows remains problematic.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 44 (30c510696921f8e088a6a05c62b5e5879604af9d)</summary>

The git commit `30c510696921f8e088a6a05c62b5e5879604af9d` made by author Louis Beaumont on December 24, 2024, reduces the verbosity of logging in the `tray.rs` file of the `screenpipe-app-tauri` project. Specifically, the change involves replacing an `info` level log statement with a `debug` level log statement. This change decreases the level of detail output in the logs under normal conditions, making them less verbose. Additionally, the `tracing` module import was updated to include the `debug` log level.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 45 (1962c7726e9cf0cd3c0385abba6985b47414a751)</summary>

The commit titled "release-app fix windows build" by Louis Beaumont addresses a couple of issues related to the development of the "screenpipe-app" project:

1. **Version Bump**: The version of the `screenpipe-app` package has been increased from `0.21.4` to `0.21.5` in the `Cargo.toml` file. This indicates the release of a new minor update to the application, which typically includes bug fixes or small improvements.

2. **Windows Build Fix**: A change was made to the `main.rs` file to fix issues related to building on Windows. The specific change involves modifying a conditional compilation directive related to deep link URL scheme registration. The original directive included both Windows and Linux, but the updated directive now only applies to Linux (`#[cfg(any(target_os = "linux"))]`). This suggests that the deep link registration should not be attempted on Windows, potentially resolving build errors or runtime issues specific to that platform.

Overall, these changes are part of a minor version update aimed at improving the application's compatibility or functionality, especially concerning its build process on Windows systems.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 46 (d09590e06942c386fc03f9dff040c38f778184d4)</summary>

The commit titled "pin deep link" by Louis Beaumont updates the `Cargo.toml` file for a Tauri application. The changes involve modifying the versions of two Tauri plugins:

1. The `tauri-plugin-http` was originally specified with a general version `"2"`, and it remains specified as such in its new location, suggesting a possible restructuring of dependencies.
2. The `tauri-plugin-deep-link`, initially also specified with `"2"`, has been pinned to an exact version `"=2.2.0"`, indicating a specific version requirement.

These changes likely aim to ensure compatibility or fix issues related to these plugin dependencies.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 47 (5a0770368eff6574f314c1ad1a73df205763017b)</summary>

This commit, authored by Louis Beaumont, addresses issues related to cron job execution within a Rust application (`release-app`). The update includes the following key changes:

1. **Version Increments**: 
   - Updated `Cargo.toml` in the workspace package from version "0.2.21" to "0.2.22".
   - Updated `screenpipe-app` version in two places:
     - In `Cargo.lock` from "0.21.2" to "0.21.4".
     - In `Cargo.toml` from "0.21.3" to "0.21.4".

2. **Code Changes in `updates.rs`**:
   - Removed an unused import (`std::env`).

3. **Main Cron Job Logic Modifications in `pipes.rs`**:
   - Moved the retrieval of the last cron execution time inside the loop for each iteration.
   - Added error handling for scenarios where the next execution time is not found and breaks the loop if this occurs.
   - Improved error handling for invalid durations by logging the error and falling back to a 60-second delay.
   - Enhanced logging to provide more debug information, such as the timing of the next execution.
   - Introduced a `tokio::select!` construct to wait for a duration or a shutdown signal, improving the robustness of the cron job execution and shutdown routine.
   - Updated the logic for executing the cron job, with success or failure results being logged accordingly.

Overall, these changes optimize the cron job handling by improving error handling, logging, and the control flow of the cron execution process.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 48 (5bc0af199aba77cb507b1ff3cb5fb4bcc198ed07)</summary>

The commit by Louis Beaumont on Dec 24, 2024, involves several updates to the ScreenPipe app:

1. **Binary File Update**:
   - The `bun.lockb` file under `screenpipe-app-tauri` was modified, though details of binary file changes are not visible in the diff.

2. **UI Changes in AccountSection Component**:
   - Within `account-section.tsx`, the "Manage Account" and "Login" buttons were altered. 
   - The "Manage Account" button now only appears if the `settings.user?.token` exists. When available, it directs users to the URL "https://accounts.screenpi.pe/user".
   - The existing login button was updated to now include an `ExternalLinkIcon`.

3. **Version Update**:
   - The version in `Cargo.toml` of the `screenpipe-app` package was incremented from `0.21.2` to `0.21.3`, indicating a minor update likely due to the new features or enhancements.

4. **Dependency Update**:
   - In `main.rs`, the `tauri_plugin_deep_link::DeepLinkExt` was added to the list of used imports, suggesting that the app now supports deep linking functionality, which aligns with the commit message mentioning "deep link auth".
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 49 (01fe7a10f480f0a32aff31f2117ac5a9e4acab2c)</summary>

This commit introduces deep linking functionality to the Screenpipe application. It includes several key modifications across various files:

1. **`account-section.tsx`**: 
   - Added deep link handling using the `@tauri-apps/plugin-deep-link` package. 
   - Modified the `useEffect` hook to listen for URLs, specifically looking for `api_key` in the deep link URL to automatically log in the user by updating settings with the API key.

2. **`package.json`**: 
   - Added the `@tauri-apps/plugin-deep-link` package as a dependency.

3. **Cargo Files**:
   - Modified `Cargo.lock` and `Cargo.toml` to add dependencies needed for deep linking, like `tauri-plugin-deep-link`, and updated the application version to `0.21.2`.

4. **Capabilities and Schemas**:
   - Updated the `main.json`, `acl-manifests.json`, `capabilities.json`, `desktop-schema.json`, and `macOS-schema.json` to include permissions and capabilities related to deep linking.

5. **`main.rs`**:
   - Integrated deep linking by initializing the `tauri-plugin-deep-link` and configuring event handling for when URLs are opened through deep links.
   - Registered URL schemes for Windows and Linux, handling potential registration errors.

6. **`tauri.conf.json`**: 
   - Added configuration for deep links specifying supported desktop schemes and mobile path prefixes.

These changes collectively enable the app to handle URLs for authentication directly through deep links, a feature useful for actions like auto-login via authentication tokens sent in a URL.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 50 (bafba74b4c9fb59256052355b8290884976dc8e9)</summary>

The commit `bafba74b4c9fb59256052355b8290884976dc8e9` by Louis Beaumont modifies the GitHub Actions workflow file `.github/workflows/release-app.yml` to fix the build process. 

### Key Changes:
- **Consistency and Style**: Comments in the script are now more consistent with the lowercase ('create' and 'download') in place of capitalized commands.
- **CLI Directory**: The temporary directory for the CLI is created using `Join-Path` to construct the path relative to the `GITHUB_WORKSPACE`.
- **CLI Download**: The process to download the CrabNebula CLI has been updated to use a variable `$cliExe` for constructing the path for the downloaded executable.
- **Execution Path and Commands**:
  - The `cd` command was separated from the asset upload command with clearer steps.
  - The upload command now uses a PowerShell execution operator (`&`) for clearer execution.
  - An additional variable, `$cliFullPath`, is introduced for better readability when uploading assets.
- **Console Output**: Some of the console messages were refined for better clarity ("Downloading" to "downloading" and "Uploading" to "uploading").

These alterations focus on improving clarity, consistency, and ensuring the script uses appropriate variable path combinations within the workflow.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 51 (ae3cacf4a208abc3e366efff560f0a66184bb58f)</summary>

The commit `ae3cacf4` introduces a changelog for version 0.21.2 of the project. The changes include:

### Improvements:
- **Increased default FPS on macOS:** This update enhances screen recording performance by increasing the default frames per second setting for macOS users.

### Fixes:
- **Fixed build and create-pipe issues:** The update resolves problems related to the application build process and the create-pipe functionality.

Additionally, the full changelog comparison has been updated to reflect the changes made between specific commits: [5cf86..69940](https://github.com/mediar-ai/screenpipe/compare/5cf86..69940). 

The changelog file was newly created, and similar updates were made to the existing `CHANGELOG.md` file in the `screenpipe-app-tauri/public` directory. Some previous entries were removed to align both documents with the latest changes, reflecting only the increased default FPS on macOS improvement and the described fixes.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 52 (6994005f52a2092f452de52c86d5e79d552c456c)</summary>

The git commit `6994005f52a2092f452de52c86d5e79d552c456c` made by Louis Beaumont on December 23, 2024, updates the GitHub Actions workflow script `release-app.yml`. The changes include fixing the syntax of the upload command for the CrabNebula CLI on Windows. Specifically, the command has been adjusted to change the directory to `screenpipe-app-tauri/src-tauri` before executing the upload command. The path to the `cn.exe` has been updated to reflect this change, ensuring the assets are uploaded correctly using the provided secrets for the app slug and API key.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 53 (96cfbf663c31c4f2284b34b43659b3dc46ca7e6f)</summary>

The commit made by Louis Beaumont updates the default frames per second (fps) setting for macOS in two files within a project: `use-settings.tsx` and `cli.rs`. Specifically, the default fps value for macOS is increased from 0.2 to 0.5. This change affects both the default settings object creation in the React hook file and the command-line interface options for the server. The change is meant to optimize performance specifically for macOS, reflecting a consideration for platform-specific behavior.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 54 (d3d72fba72a3eb847a5e8d0277d343f57cfd2d7e)</summary>

The Git commit with hash `d3d72fba72a3eb847a5e8d0277d343f57cfd2d7e` by Louis Beaumont updates the `Cargo.toml` file to change the version of the package "screenpipe-app" from `0.21.1` to `0.21.2`. This update likely signifies a minor release or patch update, as indicated by the version increment within the commit message "release-app-publish".
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 55 (7e0bf4b7683a12baaef43cc60d18f346bfc1720e)</summary>

The commit by Louis Beaumont involves two main changes:

1. **GitHub Actions Workflow Update**:
   - In the `.github/workflows/release-app.yml` file, the process of downloading the CrabNebula CLI was refined. It now uses a revised download URL and includes a user agent in the request header. Previously, the process involved downloading a ZIP file and extracting it, which has now been simplified to downloading an executable file directly.

2. **JavaScript Project Modifications**:
   - In the `screenpipe-js/create-pipe/index.ts` file, the logic for selecting a pipe type (UI or headless) was removed. This results in simplification by using only the "pipes/obsidian" template for creating pipes regardless of UI requirements.
   - References to specific instructions for running headless pipes were removed, now only referencing general `bun dev` or its alternatives.
   - Version updates in `package.json` for the `screenpipe-js/create-pipe` and `screenpipe-js` packages, incrementing the versions to "0.0.10" and "0.1.11" respectively.

These adjustments aim to streamline the workflow and simplify the process of creating pipes, along with updating package versions.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 56 (5cf86a5be664444e82ba25a472d854d8e89e8376)</summary>

The commit titled "release-app-publish fix port check" by Louis Beaumont makes several updates:

1. **Version Updates:**
   - The `Cargo.toml` in the root of the workspace has been updated to change the version from "0.2.20" to "0.2.21".
   - The `Cargo.toml` in the `screenpipe-app-tauri/src-tauri` directory has been updated to change the version from "0.21.0" to "0.21.1".

2. **Code Refactoring in `screenpipe-server`:**
   - The check for a free local IPv4 port, previously executed at the beginning of the `main` function, has been moved to occur later in the function. The logic of checking if the port is already in use, and potentially returning an error if it is, remains unchanged.

These changes imply a patch level release for both components, likely to fix the issue regarding the port check logic in the `screenpipe-server` application.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 57 (7ff016fae0083eb3421cf3b71a8e129cd4f35b30)</summary>

The Git changes involve two main updates:

1. **GitHub Actions Workflow**: The `release-app.yml` file in the GitHub workflows directory has been modified. Specifically, the shell used for a particular Windows job has been changed from `pwsh` (PowerShell Core) to `powershell` (Windows PowerShell).

2. **Cargo.toml Version Update**: The `Cargo.toml` file for the `screenpipe-app` has been updated to reflect a version change from `0.20.9` to `0.21.0`. This likely indicates the release of a new version of the application with potential updates or improvements.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 58 (91d48e56499c4ec8337602fe33ddbc8f16aa4b4e)</summary>

This Git commit, authored by Louis Beaumont, addresses two issues within the release workflow and the app's backend. The main changes involve:

1. **GitHub Actions Workflow (`release-app.yml`)**:
   - Updated how the CLI is downloaded and run for both Windows and non-Windows platforms. Previously, the task for downloading and running the CLI manually on Windows was duplicated later in the file. This has now been reorganized, with Windows-specific code appearing earlier, and a streamlined non-Windows configuration retained.
   - Ensures that the script’s execution logic properly differentiates tasks between Windows and other operating systems with the use of conditional instructions.

2. **Backend Code Updates**:
   - In the settings route (`route.ts`), added a default interval for the cron schedule and adjusted how settings are loaded and updated, ensuring the settings persist properly even if no prior settings exist.
   - Fixed the settings update logic in React hooks (`use-settings.tsx`) to correctly update and reset settings, with precautions for ensuring application stability when settings are undefined or not loaded.

3. **Handling Cron Jobs**:
   - In `pipes.rs`, a new `CronHandle` struct is introduced for managing cron jobs, maintaining a collection of handles to initiate a shutdown sequence for crons when a pipe is deleted.
   - Creates a `cleanup_pipe_crons` function to efficiently terminate cron jobs for a given pipe.
   - Adjusts the cron scheduling to utilize these handles for better management and cleanup.

4. **Pipe Manager (`pipe_manager.rs`)**:
   - Ensures that when attempting to stop a pipe, the associated cron jobs are also properly terminated, utilizing the newly introduced cleanup mechanism.

5. **Cargo.toml**:
   - Bumped the app version from `0.20.8` to `0.20.9`.

These improvements intend to fix the problem where pipe settings would be lost upon disabling them and ensure backend crons are properly terminated upon deletion.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 59 (2bb840993420030223b2f8f92b171bf882b7db8b)</summary>

The git commit `2bb840993420030223b2f8f92b171bf882b7db8b` introduces a changelog for the Screenpipe version 0.20.8. The changes include:

### Improvements:
1. Enhanced fallback mechanisms for store and credit options, providing a more robust user experience.
2. Updated the HTML timeline to improve clarity and usability.

### Fixes:
1. Disabled update checks in development mode to boost performance and usability.

This update reflects adjustments for more seamless operation and user experience, removing unnecessary update checks during development. The full changelog details are available through a comparison link provided within the message.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 60 (a48acb6185f0b1730aa3239473e2f44b453fd6b7)</summary>

The recent Git commit introduces several changes and improvements:

1. **Workflow Updates**: 
   - The GitHub Actions workflow file, `release-app.yml`, now includes a new step for downloading and running the CLI manually on Windows. This step is specifically for the Windows target and involves downloading the CrabNebula CLI, extracting it, and using it to upload assets. The previous step for uploading assets directly to CrabNebula Cloud is now limited to non-Windows targets.

2. **Component Changes**: 
   - In `pipe-store.tsx`, the order of the `corePipes` array has been modified. The "obsidian" and "meeting" pipes have been repositioned, along with the addition of a new pipe, "speaker identification", with updated details including credits and paid status.
   - The monthly plan price in `account-section.tsx` has been increased from "$20/mo" to "$30/mo".
   - In `ai-section.tsx`, a new AI model, "claude-3-5-haiku-latest", has been added to a list of models.

3. **Credit Purchase Dialog Enhancement**:
   - In `credit-purchase-dialog.tsx`, the function `handlePurchase` has been updated to append `client_reference_id` and `metadata[user_id]` parameters to the URL, which likely improves tracking or processing of purchases.

4. **Version Bump**:
   - The version of the application in `Cargo.toml` has been incremented from "0.20.7" to "0.20.8", indicating a new minor release.

These changes enhance the functionality and maintainability of the application, while adjusting pricing and versioning to reflect updates.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 61 (397ce99dc0c2f6ba815be9a799f47ce5edefdb9b)</summary>

The Git commit made by Divanshu Grover introduces a feature that disables update checks in a Tauri application when running in development mode. This is achieved by checking for the presence of an environment variable called `TAURI_ENV_DEBUG`. If this environment variable is set to `"true"`, the update check is skipped, and a log message noting the bypass is recorded. The change adds code to check this condition at the beginning of the update process in the `UpdatesManager` implementation, as shown in the `src/updates.rs` file.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 62 (6209eecdb6ae6bff4ac591a949235565f30252be)</summary>

The git commit updates two files in the `screenpipe-js/create-pipe` directory. In `index.ts`, a parameter in a function call was changed from `"pipes/pipe-simple-nextjs"` to `"pipes/obsidian"`. Additionally, in the `package.json` file, the version number has been incremented from `0.0.8` to `0.0.9`.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 63 (e584c93542acfa573aefab5fa7ef97947d8733fa)</summary>

The commit authored by Louis Beaumont includes improvements to the HTML timeline in the `index.html` file located in `screenpipe-server/examples/timeline_ui_simple`. The key changes are:

1. **Timestamp Dataset**: A new data attribute `dataset.timestamp` is added to the `frame-container` to store the timestamp of each frame.

2. **Code Commenting**: Sections of code responsible for displaying metadata and audio information have been commented out. This includes metadata about device, window, and app, as well as OCR text and audio details (such as device name, duration, and transcription).

3. **Frame Appending Logic**: The logic for appending frames to the timeline has been revised. Instead of simply appending new frames to the end, the frames are now inserted in order based on their timestamp. The code finds the correct position to insert the new frame within the existing frames, ensuring they are sorted by timestamp.

These changes enhance the management and display of timeline frames, likely making the timeline view more organized and logically ordered by time.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 64 (3cfc264e072b8c74b66083cefbfbfe35cd2a3269)</summary>

The commit introduces a new changelog for version 0.20.7 of the Screenpipe application. Key updates include:

### New Features:
- Enhanced multi-selection functionality within the application.
- Integration of the Ollama API for improved model retrieval.

### Improvements:
- Optimization of the application binary to reduce its size, enhancing download and installation efficiency.

### Fixes:
- Resolution of an issue with Deepgram integration in Screenpipe Cloud.
- Correction of a Windows NSIS installer error.
- Fixes for the cron job functionality related to the Obsidian pipe.

The changelog is provided in two places: a new file `content/changelogs/0.20.7.md` and an update to the `screenpipe-app-tauri/public/CHANGELOG.md` file. Both entries include a reference to the full changelog comparison link.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 65 (36866a88da19aaed840a19e6858f90916418e5c3)</summary>

The recent git commit includes several key changes:

1. **Cargo and Version Updates**:
   - Bumped the version in `Cargo.toml` and `src-tauri/Cargo.toml` from "0.20.6" to "0.20.7". Also, updated the workspace package version from "0.2.19" to "0.2.20".
   - Updated dependencies in `Cargo.lock`, removing packages like `const-random`, `dlv-list`, `rust-ini`, `tiny-keccak`, `trim-in-place`, and others. Some packages were downgraded or updated to unify dependencies.

2. **Code Refactoring in React Components**:
   - `cli-command-dialog.tsx`: Changed the prop name from `localSettings` to `settings` and updated the logic to use `settings` for configuring CLI commands.
   - `dev-mode-settings.tsx`: Simplified imports and logic, removed unused imports, adjusted the call to `CliCommandDialog`, and handled the `updateSettings` call asynchronously.
   - `recording-settings.tsx`: Commented out the call to `relaunch` after invoking the `spawn_screenpipe`.

3. **Schema and Permission Configuration**:
   - Revised schema files (`acl-manifests.json`, `desktop-schema.json`, `macOS-schema.json`) to remove deep-link permissions and references. Adjusted permissions around other plugins.

4. **Backend Improvements**:
   - `sidecar.rs`: Improved `User` struct to extract data more granularly with labels for each user attribute and moved user data extraction to a method, `from_store`. Added optional fields to `UserCredits`.
   - `sidecar.rs` now prints user data and pushes `--debug` to arguments when spawning the sidecar, facilitating better debugging.
   - `stt.rs`: Improved `transcribe_with_deepgram` function to omit mandatory `CUSTOM_DEEPGRAM_API_TOKEN` validation, allowing fallback to a default API token. Differentiated between using custom and default API keys.

The overall update seems to focus on cleaning up the code, updating dependency management, simplifying properties, and improving runtime flexibility and debugging capabilities for configuring deepgram API usage.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 66 (91636afb4bf741f824c5765182c909eff24e017d)</summary>

The commit 91636afb4bf741f824c5765182c909eff24e017d by the author Dhanus addresses a Windows NSIS error, as stated in the commit message "fix: windows nsis error (#1033)". Here are the changes made:

1. **GitHub Actions Workflow**:
   - In the `.github/workflows/release-app.yml` file, the section that installs NSIS has been removed. This included downloading NSIS, adding it to the PATH, and verifying the installation. 

2. **Tauri Configuration**:
   - In the `screenpipe-app-tauri/src-tauri/tauri.windows.conf.json` file, a new entry for "targets" has been added with "nsis" listed as a target. This potentially configures or specifies the NSIS as a packaging target within the Tauri app configuration.

Overall, the changes remove the explicit installation of NSIS from the CI/CD pipeline and update the Tauri configuration to include NSIS as a target.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 67 (d562cf000f657edb43d9cfcc64bc445fdd40b73a)</summary>

This commit, authored by neo773, focuses on enhancements to the multi-select component within a React project. Here's a summary of the key changes:

1. **New Features for MultiSelect:**
   - Implemented the ability for users to add custom values not present in the predefined options list.
   - Added a `validateCustomValue` function prop to validate custom values before adding them.
   - Introduced state management to track and manage custom options separately and combine them with default options for display.

2. **MultiSelect UI and UX Enhancements:**
   - Improved the UI by allowing users to distinguish custom entries via an "Add" option in search results.
   - Updated keyboard interaction for the `MultiSelect` input, allowing pressing "Enter" to add custom values to the selection.
   - Enhanced filtering in the command input by supporting substring matches and differentiating between entered text and existing selections.

3. **Code Refactoring:**
   - Reorganized option rendering to display selected items first, followed by unselected ones.
   - Added a custom search filter function to handle custom and predefined options more seamlessly. 
   - Split logic into modular functions like `createWindowOptions` and `addCustomValue` to improve code readability and maintainability.

4. **Recording Settings Component Adjustments:**
   - Replaced manual option handling in `RecordingSettings` with the newly designed option creation function, allowing for custom ignore/include window lists.
   - Enhanced the visual layout with minor fixes, like ensuring consistent spacing and alignment.

These changes address a feature request for better handling of dynamic and user-defined options, improving both the functionality and user experience of multi-selection elements within the application.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 68 (f44b8fab22858c82e80f084edaf6c1e4391168a5)</summary>

This commit by Louis Beaumont includes the following changes:

1. **Cron Schedule Update in `pipe.json`:**
   - The cron schedule for the path `/api/log` in the `pipes/obsidian/pipe.json` file was modified. The schedule was changed from running every second (`* * * * * *`) to every 5 minutes (`0 */5 * * * *`).

2. **Addition of `updateCronSchedule` Function in `route.ts`:**
   - A new function, `updateCronSchedule`, was added to the `pipes/obsidian/src/app/api/settings/route.ts` file. This function updates the cron schedule in the `pipe.json` file to run at a specified interval (in minutes) and ensures that the configuration file contains certain default properties, such as `enabled` and `is_nextjs`.

3. **Handling Partial Updates for Obsidian Namespace in `PUT` Method:**
   - The `PUT` method in the same file (`route.ts`) was extended to handle partial updates for the "obsidian" namespace. If the update contains a value for `interval`, it is converted from milliseconds to minutes (ensuring it’s at least 1 minute), and the new cron schedule is set according to the specified interval.

4. **Minor UI Change in `recording-settings.tsx`:**
   - In the `screenpipe-app-tauri/components/recording-settings.tsx` file, the class for the `Key` component label associated with `deepgramApiKey` was adjusted by removing unnecessary padding (`pl-2`) from the class name, simplifying its styling.

Overall, the commit focuses on fixing and enhancing the functionality of cron scheduling, especially concerning configuration updates and UI cleanup.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 69 (38366d74b0d98af1905fe00a01d9637934297217)</summary>

This commit updates the AI section of a Tauri-based application to integrate with the Ollama API for fetching available models dynamically. Key changes include:

1. **Import Changes**: Added `useEffect` from React to handle side effects.

2. **New Interface**: Introduced a new type `OllamaModel` to represent model data fetched from the Ollama API.

3. **getModelSuggestions Function**: Updated to switch the model suggestions for the "native-ollama" and "screenpipe-cloud" providers, utilizing the dynamic fetching for Ollama models.

4. **State Management**: Added a state `ollamaModels` to store the fetched models.

5. **Effect for Fetching Models**: Implemented a `useEffect` hook that fetches Ollama models from an API endpoint (`http://localhost:11434/api/tags`) when the AI provider type is "native-ollama". It parses the response and updates the `ollamaModels` state.

6. **Model Selection UI**: Adjusted the model selection UI to iterate over either the fetched Ollama models or the pre-defined suggestions based on the current AI provider type.

Overall, this update allows the application to dynamically fetch model suggestions from the Ollama API instead of relying solely on hard-coded model names.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 70 (1b2ce35dead0a0cd05f4aed7806af5d04d5e7452)</summary>

This commit introduces several changes aimed at reducing the size of the application binary. Here’s a summary of the changes:

1. **Build Configuration**:
   - Added configurations to the `Cargo.toml` files, specifying a release profile with `codegen-units = 1`, `lto = true`, `opt-level = "s"`, and `strip = true` to enable optimizations and stripping to minimize binary size.

2. **Dependencies**:
   - Removed various unused dependencies from the JavaScript `package.json` files across different components (`screenpipe-app-tauri`, `screenpipe-js`). These include several `@opentelemetry` and `@tauri-apps` plugins, as well as packages like `js-levenshtein`, `openai`, and `zod`.

3. **Workflow Adjustments**:
   - Updates to GitHub workflows included a corrected script for installing dependencies specifically on Ubuntu, checking and cleaning up unused Rust dependencies with `cargo-udeps`, which now requires the Rust nightly toolchain.
   - The `cargo` commands now use the nightly toolchain explicitly.

4. **Dependency Management Scripts**:
   - Removed some redundant dependencies installed in the `install_dependencies.sh` script.

5. **Miscellaneous**:
   - Minor code style and whitespace fixes were made, such as removing trailing spaces and ensuring comments have consistent formatting.

Overall, the changes are primarily focused on cleaning up and optimizing the application, removing redundant components, and configuring the build process to generate a smaller, more efficient binary.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 71 (bce13dffa3c00680a3ba2918e0b759adbe27e20f)</summary>

The commit `bce13dffa3c00680a3ba2918e0b759adbe27e20f` made by Louis Beaumont on December 21, 2024, updates the `README.md` file for the `pipes/obsidian` project. The changes include:

- The initial description of the functionality related to searching, reviewing, and summarizing digital context data has been replaced with a focus on real-time analysis of screen and microphone activity to log and extract specific information such as CRM details, customer data, and market research.
- The update emphasizes the privacy of this process, mentioning that it operates entirely on the user's computer using 4-5 AI models while utilizing a total of 5 GB of RAM, including existing components like llama3.2 and phi4.
- Several illustrative images have been updated; previous images showcasing various filtering methods (keyword, window/app, speaker) have been removed and replaced with a single new image.
- A link to a GitHub asset is included instead of showing several screenshots as in the previous version.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 72 (b914d83646aabb1f1b7de0e82e80fbb691164789)</summary>

The commit `b914d83646aabb1f1b7de0e82e80fbb691164789` authored by the GitHub Actions Bot adds a changelog for version 0.20.6. It includes:

- **New Features:** The Screenpipe application is now published on the store, making it available to users.
- **Improvements:** Access to Obsidian V2 through the store is now streamlined.
- **Fixes:** No specific fixes are documented in this commit.

This changelog is added both as a new file `0.20.6.md` in `content/changelogs` and an updated entry in `screenpipe-app-tauri/public/CHANGELOG.md`. The full changelog link is provided for more detailed comparison.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 73 (509f1d105369fa800fc853402e514334231820a4)</summary>

This commit updates and enhances functionality related to a git repository. Summarized below are the key changes:

1. **Obsidian Settings Component (`obsidian-settings.tsx`):**
   - A check was added to verify if the File System Access API (specifically the `showDirectoryPicker` method) is supported by the browser. If not, an error toast is displayed instructing the user to enter the path manually.
   - Added toasts for feedback:
     - A success toast when the Obsidian vault path has been set.
     - An error toast message when directory selection fails.
   - Minor formatting adjustments were made to the default value calculation for settings intervals.

2. **Pipe Store Component (`pipe-store.tsx`):**
   - Added a new core pipe entry named "obsidian v2," which includes details such as a description, URL, credits, and payment status.
   - Modified the conditional rendering logic for `PipeConfigForm`, adding an additional check for whether any config fields exist for a selected pipe.

3. **Screenpipe App Version Update (`Cargo.lock` and `Cargo.toml`):**
   - The version of the `screenpipe-app` was incremented from `0.20.5` to `0.20.6`.

These changes aim to improve feature availability and stability across the repository's components, specifically providing enhancements for the Obsidian pipe within the app store structure.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 74 (a9e4b018febcf529a719a3b9742e930d46b83755)</summary>

The git commit made by Louis Beaumont on December 21, 2024, includes the following changes:

1. **GitHub Workflow Adjustments**:
   - In the `.github/workflows/release-app.yml` file:
     - Several commented-out steps related to version extraction from `Cargo.toml`, uploading unsigned binaries for signing, and signing the artifact with SignPath have been removed.
     - Improved the workflow condition by removing the `always()` function in the `publish-release` job condition.
     - Added a new step to extract the application version from `Cargo.toml` and store it in the `GITHUB_ENV` environment variable during the `publish-release` job.
     - The command to publish a release to CrabNebula Cloud now includes the version of the application.

2. **Version Update**:
   - Increased the version of the `screenpipe-app` in both `Cargo.lock` and `Cargo.toml` files from `0.20.4` to `0.20.5`.

These changes focus on refining the release process in the GitHub Actions workflow, specifically related to handling versioning and publishing of the app.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 75 (ca28a133fbfc32f2917ad808f2d511560b4832bb)</summary>

The git commit titled "release-app-force" by Louis Beaumont introduces a few changes to the "screenpipe-app" project:

1. **recording-settings.tsx**: 
   - A new line is added after the `invoke("kill_all_sreenpipes")` function call which introduces a 1-second delay using `setTimeout`. This is likely to ensure any running processes are properly terminated before starting a new instance.

2. **Cargo.lock and Cargo.toml**:
   - The version of the "screenpipe-app" package has been updated from "0.20.3" to "0.20.4" in `Cargo.toml`.
   - Corresponding changes are made to `Cargo.lock` reflecting this version bump from "0.20.2" to "0.20.4". This indicates a patch-level change, likely to include minor bug fixes or improvements.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 76 (f3f13f94d527d4cf8816a5e36409db0c5287b129)</summary>

The changes in this commit involve updates to two files within the project, `globals.css` and `recording-settings.tsx`, as follows:

1. **`globals.css`**:
   - A small CSS fix was applied to the `body` element to ensure that `pointer-events-auto` is always applied forcefully using the `!` symbol in Tailwind CSS syntax.

2. **`recording-settings.tsx`**:
   - Added an import statement for the `relaunch` function from the `@tauri-apps/plugin-process` package.
   - Adjustment in the sequence of operations when updating settings: 
     - Removed the line that waits for 1000 milliseconds (1 second) right after invoking `kill_all_sreenpipes`.
     - After invoking `spawn_screenpipe`, the change now includes waiting for 2000 milliseconds (2 seconds) and then calls `relaunch()` to restart the application.

Overall, these changes appear to address an issue (possibly related to pointer events and the application’s relaunch behavior) to improve how the application updates its settings and restarts.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 77 (2b885b0629f4dd6ecc0785f3040bb9395b0a4fca)</summary>

The recent commit with the hash `2b885b0629` includes several significant changes and refactoring in the `screenpipe-app-tauri` project. Here's a summary of the changes made across various files:

### General Changes:
- **Version Update**: The version number in `Cargo.toml` is updated from `0.20.2` to `0.20.3`.

### Component Updates:
1. **Header Component (`header.tsx`)**:
   - Removed unused imports (`useHealthCheck`, `Skeleton`, `useSettings`, `useUser`).
   - Simplified the component logic by removing the `isLoading` state and its related conditional rendering.
   - Altered the HTML structure of the header component, refactoring parts of the UI for clarity and removal.
   - Consolidated the dropdown menu logic, and retained the primary functionalities, such as showing settings and triggering dialogues for feedback and support.

2. **Recording Settings Component (`recording-settings.tsx`)**:
   - Added a slight pause when calling `invoke("kill_all_sreenpipes")` to ensure proper termination of processes.
   - Forced a page refresh after restarting `screenpipe` with new settings.
   - Adjusted styles and fixed minor UI inconsistencies.

3. **Screenpipe Status Component (`screenpipe-status.tsx`)**:
   - Removed several unused imports.
   - Refactored the dialog functionality to improve performance and fix a bug preventing multiple clicks while loading.
   - Updated the `HealthStatus` badge to always show an icon and removed additional loading icons.
   - Enhanced error logging for dialog opens and improved the appearance of the status interface.

### Hook Updates:
- **useSettings Hook (`use-settings.tsx`)**:
  - Added error handling when retrieving the platform type to prevent crashes and unexpected errors.
  - Improved platform string handling logic for setting paths, including logging failures.

These changes collectively aim to improve the application's performance, streamline the user interface, and enhance error handling and feedback mechanisms.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 78 (d7b4dbbefbc58d084d30cc53bed082e1ae678e3e)</summary>

Here's a summary of the git changes made in commit `d7b4dbbefbc58d084d30cc53bed082e1ae678e3e`:

1. **Dependency Updates:**
   - Updated the version of `@screenpipe/js` from `0.1.10-beta.21` or `0.1.10-beta.23` to `0.1.10-beta.24` in several `package.json` files located within different modules: `pipes/data-table`, `pipes/identify-speakers`, `pipes/meeting`, `pipes/obsidian`, `pipes/search`, and `pipes/timeline`.

2. **Code Modifications:**
   - In the `pipes/search/src/components/ui/icons.tsx` file, modified the `IconClaude` component. The changes include setting specific height and width attributes, adding a style with a flexible layout, removing some attributes like `role` and adjusting the `path` element.

3. **Debugging and Logging:**
   - Added a `console.log` statement in the `useSettings` function located in `pipes/search/src/lib/hooks/use-settings.tsx` to log the fetched data.

4. **Feature Enhancements in `screenpipe-js/node.ts`:**
   - Introduced helper functions `flattenObject` and `unflattenObject` for flattening and unflattening objects.
   - Updated the `SettingsManager` to use these new helper functions when loading and saving settings. This helps in transforming the settings data to and from a flat structure for storage purposes.

5. **Bumped Package Version:**
   - In the `screenpipe-js/package.json` file, incremented the version of the package itself from `0.1.10-beta.23` to `0.1.10-beta.24`.

Overall, the commit focuses on updating package dependencies, refactoring code for improved settings management, and adding logging for debugging purposes.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 79 (842ee4e35e6258b4d4ce6281c44e50bd82fd851e)</summary>

The commit with hash `842ee4e35e6258b4d4ce6281c44e50bd82fd851e` by the GitHub Actions Bot updates the documentation by adding a new changelog for version 0.20.2. It involves two file changes:

1. A new changelog file `0.20.2.md` is created under `content/changelogs/`. This file currently contains a link to the full changelog comparison between two commits (from `e06e6` to `e68f6`).

2. The `CHANGELOG.md` file in the `screenpipe-app-tauri/public/` directory is updated. The existing link to the full changelog comparison is revised to reflect the changes between the new commit range (`e06e6..e68f6`).
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 80 (e68f6d370fa3c897f78768ff87efc1ca39c549c7)</summary>

The commit introduces several changes to various files in a project. Here's a summary of the changes:

1. **Workflow Update (`release-app.yml`):**
   - The NSIS installer script was modified. Instead of using `Invoke-WebRequest`, a new method utilizing `curl.exe` with `Invoke-Expression` is now being used to download the NSIS installer.

2. **Typescript Components:**
   - **`notification-handler.tsx`:**
     - A debugging `console.log` statement was removed.
   - **`pipe-store.tsx`:**
     - The definition of the `pipe-simple-nextjs` object, which appears to include metadata for a "keyword analytics" feature, was reordered within the `corePipes` array.
   - **`recording-settings.tsx`:**
     - Minor styling and text changes were made, such as adding padding to icons and changing the placeholder text to lowercase.

3. **Hooks and Settings (`use-settings.tsx`):**
   - Adjusted settings logic, specifically the `aiMaxContextChars` maximum context characters setting was reduced from `100000` to `30000`.
   - The logic for platform detection and settings initialization was refactored for clarity and error handling.
   - Changes to the `tauriStorage` object logic to handle storing and removing settings more effectively, using a flattening and unflattening approach for storage keys.

4. **Rust Components:**
   - **`Cargo.lock` and `Cargo.toml`:**
     - The version of `screenpipe-app` was incremented, suggesting a new release or update from `0.20.1` to `0.20.2`. This may reflect substantial changes or improvements in the application described above.

Overall, this commit appears to contain styling adjustments, refactoring for maintainability, updates to CI/CD processes, and improvements to the configuration and versioning of the app.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 81 (e06e6932982a75d629099e4b7355d8ced3d32b0f)</summary>

The commit labeled "release-app-publish" introduces several changes to the workflow and the project versioning:

1. **GitHub Actions Workflow (.github/workflows/release-app.yml):**
   - A new output `should_publish` is added, which determines if a release should be published based on the commit message containing "release-app-publish."
   - The workflow step that previously handled version retrieval and artifact signing for Windows has been removed.
   - A new job `publish-release` is added. This job depends on three other jobs (`check_commit`, `publish-tauri`, and `retry-win`) and will only run if `should_publish` is true and the preceding jobs are successful. It publishes the release on CrabNebula Cloud.

2. **Cargo.toml (screenpipe-app-tauri/src-tauri/Cargo.toml):**
   - The version of the `screenpipe-app` is bumped from "0.20.0" to "0.20.1".

These changes streamline the release publishing process and increment the application version for a minor release.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 82 (569386844d9f1220e067b6bad106e90279428614)</summary>

The commit with hash `569386844d9f1220e067b6bad106e90279428614`, authored by Louis Beaumont, updates the `Cargo.toml` file. Specifically, it involves a version bump in the `[workspace.package]` section, changing the package version from `0.2.18` to `0.2.19`.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 83 (41d72cc2090874ea867012839291e6169c994295)</summary>

The git commit made the following changes:

1. **Memory Management in Apple OCR**: The implementation of the OCR function in `screenpipe-vision/src/apple.rs` was modified to include a memory management strategy using `cidre::objc::ar_pool`. This change is likely aimed at fixing a memory leak issue. The original approach included manually dropping objects, while the updated code encapsulates operations within a memory management pool to automatically manage resources and likely prevent memory leaks.

2. **Version Update**: The `Cargo.toml` file in the `screenpipe-app-tauri/src-tauri` directory was modified to update the version number of the `screenpipe-app` package from "0.19.9" to "0.20.0", likely to reflect the new changes or enhancements made in the code.

3. **API Settings Update**: In `pipes/timeline/src/app/api/settings/route.ts`, a new user token setting was added to the list of settings as an empty string, indicating a possibly new feature or configuration in the application settings.

These changes aim to improve resource management, feature upgradation, and configuration handling in the application.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 84 (1f130d5e52160f1a1fcffd562fc86bcf7f1ca612)</summary>

The git changes in this commit include:

1. **GitHub Workflow Update:**
   - A new step was added to the `.github/workflows/release-app.yml` file to install NSIS (Nullsoft Scriptable Install System) when the target is `x86_64-pc-windows-msvc`. This involves downloading the NSIS installer, executing it silently, and adding NSIS to the system PATH.
   - Verification of NSIS installation is also included by checking the version of the installed NSIS.

2. **Version Bump:**
   - The version of the `screenpipe-app` was updated from `0.19.8` to `0.19.9` in both `Cargo.toml` and `Cargo.lock` files within the `screenpipe-app-tauri/src-tauri` directory.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 85 (cc62170301689ae1c14425ac89fcb0b8995c5eb6)</summary>

The commit titled "fix accuracy_test (#1025)" by Ezra Ellette addresses a few key issues in the `screenpipe-audio` module, specifically in `stt.rs` and `accuracy_test.rs`:

1. **Changes in `stt.rs`:**
   - The normalization function `normalize_v2` has been modified to take `audio_data` as a borrowed value instead of an owned one, indicated by the removal of `&` before `audio_data`.
   - The transcription result in the `stt` function now directly returns the result instead of wrapping it in an `Ok` result.

2. **Additions in `accuracy_test.rs`:**
   - New imports were introduced from the `candle_transformers` library, specifically for `metavoice::transformer` and `whisper`.
   - `resample` was added to the imports from `screenpipe_audio`.

3. **Modifications in test setup:**
   - The code now checks if the input audio sample rate matches the expected `whisper::SAMPLE_RATE`. If not, it resamples the audio data to the correct sample rate. This change ensures that the audio data is processed correctly regardless of its original sample rate.
   - The `prepare_segments` function now receives `audio_data` directly instead of `audio_input`, aligning with the changes made for resampling.
   - The call to the `stt` function within the loop is simplified by removing unnecessary arguments like `output_path` and the `bool` flag.

These changes aim to improve the audio processing accuracy and address issues in the transcription testing.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 86 (2740b2b53ccaf3531370d0254ca30e0dc9888eab)</summary>

The latest git commit, identified by commit hash `2740b2b53ccaf3531370d0254ca30e0dc9888eab`, introduces several modifications. Here's a summary of the key changes:

1. **File Movements:**
   - Files related to two pipes, `pipe-digital-clone` and `pipe-screen-time-storyteller`, are reorganized and moved to an archive directory under `pipes/archive/`.

2. **New Additions:**
   - A new directory `pipes/obsidian` has been created, containing a series of new files. These include configuration files (`.gitignore`, `tsconfig.json`, `package.json`, etc.), TypeScript source files under `src/app` and `src/components`, Tailwind CSS, components and utilities, and a `README.md` with description and images.

3. **Configuration and Code Setup:**
   - JavaScript/TypeScript configurations for Babel, ESLint, and other setup files for Next.js application setup.
   - Tailwind CSS is used for styling, including custom theme adjustments.
   - A server-side implementation of API routes for logging and settings management under `src/app/api/`.
   - Creation of utility, hooks, and action files under `src/lib/`.

4. **Functional Components:**
   - Components like buttons, inputs, accordions, code blocks, etc., are defined under `src/components/ui/`.
   - A notification system using `use-toast` and related components.
  
5. **Scheduler and Network Enhancements:**
   - `screenpipe-core` includes examples for cron job scheduling, and the file `pipes.rs` is updated to configure the execution of different pipes, including validation and cron scheduling.
   - Modifications to `screenpipe-js` extend browser and node-specific functionalities, indicating better configuration and support for custom settings and environment.

Overall, these changes introduce new modules, reorganize existing files, enhance configurations for a Next.js web environment, and introduce additional utilities and components for both server and client operations. The current version and dependencies of various packages have been updated accordingly.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 87 (885a74e6d0351417c73461001adaaba90230dfe9)</summary>

This commit introduces a new feature to the `pipe-store.tsx` component of a Tauri-based app. The primary change is the addition of an "update button" that enables users to update a selected pipe (presumably a software component or module).

### Key Changes:
- **New Functionality:**
  - A new function `handleUpdatePipe` is added. This function captures an update event, displays a progress toast notification, deletes the old pipe version, downloads the new version, and refreshes the pipe list. Error handling is included to display a toast notification in case of a failure.

- **UI Elements:**
  - The user interface now includes a button for updating a pipe. This button appears if the selected pipe's source URL starts with "http". It is implemented using a tooltip for user guidance, and styled using the `Button` and `RefreshCw` components.
  
- **Code Refactoring:**
  - An existing functionality to refresh code from a local disk was reorganized for better integration with the new update button. It was moved to a different section without apparent changes to its logic.

- **Minor Fixes:**
  - Some minor code formatting changes were made, such as removing trailing spaces in various lines.

Through these changes, users can seamlessly update their pipes via a new update button, enhancing user experience and functionality within the app.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 88 (a4c2b963a748ca17efbc6b160a9247095049f105)</summary>

The commit titled "release-app fix settings," authored by Louis Beaumont, makes several changes to the project:

1. **Meeting History Components**: In `meeting-history.tsx` files across different components (`pipes/identify-speakers`, `pipes/meeting`, `pipes/search`), messages previously using the role "system" have been changed to "user" with comments indicating that this change was due to "Claude" not supporting system messages.

2. **Chat Message Component**: In `chat-message.tsx`, an additional icon called `IconClaude` has been introduced to recognize when Claude is included in the AI model settings. The logic for determining which icon to display has been updated to support this new icon.

3. **Icons Component**: The new `IconClaude` SVG component has been added to represent Claude across the UI.

4. **Timeline Agents**: In `timeline/agents.tsx`, the same role change from "system" to "user" has been applied for various agent messages.

5. **Onboarding API Setup**: The role change from "system" to "user" was also made in `api-setup.tsx`.

6. **Recording Settings UI**: A minor text change was made for consistency, changing "Audio chunk duration" to "audio chunk duration" in `recording-settings.tsx`.

7. **Settings Hook**: Simplified storage logic in `use-settings.tsx` by reconstructing settings management without flattening/unflattening objects, directly handling `settings` object.

8. **Version Updates**: The `Cargo.lock` and `Cargo.toml` were updated to reflect a version bump of the application from `0.19.6` to `0.19.7`.

These changes primarily focus on improving compatibility with the Claude AI model and simplifying the settings handling logic.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 89 (ce4d7ff1fa86d3a04329116807e8d40da662476a)</summary>

The commit by Louis Beaumont addresses a change in the `ai-panel.tsx` file, which is part of the pipes cloud project. The modification involves updating the `settings` type within the `AIPanelProps` interface. Previously, `settings` was defined as an object with the properties `openaiApiKey`, `aiUrl`, and `aiModel`. The new change replaces this explicit definition with the `Settings` type imported from "@screenpipe/js". This likely indicates a refactoring to use a more standardized or centralized settings type, which could improve code maintainability and consistency.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 90 (832b2487f04a52aa22ce4ca937ade2696ab44687)</summary>

The commit by Louis Beaumont, titled "fix pipes cloud", introduces several changes across different parts of the codebase. The main change involves modifying how the OpenAI API key is determined. Specifically:

1. **Meeting History Component**: In `meeting-history.tsx`, the API key for initializing the OpenAI instance is now conditionally set based on the `aiProviderType`. If the type is `screenpipe-cloud`, the key is set to `settings.user.token`; otherwise, it defaults to `settings.openaiApiKey`.

2. **Search Chat Component**: Similar changes are made to `search-chat.tsx`, where the OpenAI API key assignment logic is updated to conditionally use `settings.user.token` for `screenpipe-cloud` provider type.

3. **AI Panel Component**: In `ai-panel.tsx`, the API key assignment is updated with the same conditional logic.

4. **AI Section of Settings**: In `ai-section.tsx`, the list of models returned for the `screenpipe-cloud` `aiProviderType` has been expanded to include `o1-mini`, `o1`, and `claude-3-5-sonnet-latest` in addition to the existing models `gpt-4o` and `gpt-4o-mini`.

5. **Bun Lock File**: There is also a change detected in the binary file `bun.lockb` within the `pipes/timeline`, although the specific nature of the change is not detailed in the diff provided.

These updates are aimed at enhancing the application's adaptability to different AI provider configurations, particularly for the "screenpipe-cloud" environment.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 91 (2813cab291636e59ecd5f8e69dba420fb3361bc5)</summary>

The commit `2813cab` made by Louis Beaumont on December 20, 2024, updates the version number of the "screenpipe-app" from "0.19.5" to "0.19.6" in the `Cargo.toml` file located in the `screenpipe-app-tauri/src-tauri/` directory. This change is associated with adding H.265 support on macOS for the release of the app.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 92 (71e1ecf99e3f46f73261dde9f6c2e421f06019dc)</summary>

The commit introduces support for H.265 (HEVC) video playback and encoding on macOS across multiple components of the application. Each video component in different modules (identify-speakers, search, and timeline) now includes new `<source>` tags to handle H.265 video formats using the `hvc1` and `hvec` codecs. Additionally, a modification has been made to the server-side ffmpeg processing script within the `screenpipe-server`. Previously, the code differentiated video codec settings between macOS and other operating systems, using `libx264` for macOS. This update consolidates the video codec usage by employing `libx265` across all operating systems for video encoding, with the specific inclusion of the `hvc1` tag for macOS. The goal of these changes is to enhance the application's ability to play and encode H.265 videos universally on macOS.

**Summary:**
- Added H.265 codec support (`hvc1` and `hvec`) to video components for consistent video playback.
- Unified video encoding using `libx265` across all platforms in the server-side video processing.
- Removed special handling for macOS that previously used `libx264`.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 93 (346f4b85d5c796f03d821126bdb028bb8a63f661)</summary>

The commit with hash `346f4b85d5c796f03d821126bdb028bb8a63f661` was made by the GitHub Actions Bot on December 20, 2024. It involves documentation updates related to the changelog for version 0.19.5 of the project. Specifically, a new changelog file `0.19.5.md` was added to the `content/changelogs` directory, and the full changelog link in `screenpipe-app-tauri/public/CHANGELOG.md` was updated to reflect changes from commit `8dacd` to `48ab3`.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 94 (48ab3cd2d0eeacd2d3e00029018dffbcc47b2fd9)</summary>

The commit made several changes to the `screenpipe-app` project:

1. **File Deletion:**
   - The file `screenpipe-app-tauri/app/migration/page.tsx` was deleted. This file contained a React component related to migration progress with animations and UI elements.

2. **Binary Change:**
   - There was a change in the binary file `screenpipe-app-tauri/bun.lockb`.

3. **Code Formatting Improvements:**
   - In `screenpipe-app-tauri/components/recording-settings.tsx`, `screenpipe-app-tauri/components/ui/command.tsx`, and similar files, the code was reformatted to improve readability. This includes adding semicolons and improving indentation and spacing in arrow functions.

4. **UI Text Change:**
   - In `screenpipe-app-tauri/components/settings/ai-section.tsx`, the text inside a `CommandEmpty` component was slightly modified to use HTML entity for quotation marks.

5. **Platform Detection:**
   - In `screenpipe-app-tauri/lib/hooks/use-settings.tsx`, the platform detection logic was adjusted to handle errors. If platform detection fails, it defaults to "unknown" and issues a warning.

6. **Dependency Downgrade:**
   - The `cmdk` package version in `screenpipe-app-tauri/package.json` was changed from `1.0.0` to `0.2.1`.

7. **Version Bump:**
   - The version of the `screenpipe-app` in `screenpipe-app-tauri/src-tauri/Cargo.toml` was incremented from `0.19.4` to `0.19.5`.

These changes seem to focus on cleaning up the code, handling platform-specific logic more gracefully, and preparing for a new release.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 95 (18659215684ace1f7210117ce8b3047095aeff5a)</summary>

The commit with ID `18659215684ace1f7210117ce8b3047095aeff5a` by author Louis Beaumont on December 20, 2024, includes the following changes:

1. **Unification of Style**:
   - The commit modifies text styles across several components, such as `Header`, `RecordingSettings`, `Settings`, `AccountSection`, `AISection`, and `ShortcutSection`, to use lowercase for titles and labels, aiming for a more unified appearance.

2. **Customization of AI Models**:
   - Introduced the capability to customize AI models by allowing dynamic changes to the AI model selection in `AISection`.
   - Included a new max context character count setting, adjusting its default value from 30,000 to 100,000 across components dealing with settings.

3. **Removed Unused Components**:
   - The files `search-chat.tsx` and `sql-autocomplete-input.tsx` from the path `pipes/timeline/src/components` were deleted, indicating their functionalities are either integrated elsewhere or no longer needed.

4. **Language Enumeration Refactor**:
   - The `Language` enum in `language.ts` was altered to use lowercase identifiers. This reflects a shift towards enforcing consistent styling across enumerations.

5. **Versioning Update**:
   - The `Cargo.lock` file in the `screenpipe-app-tauri/src-tauri` directory had its version number updated from `0.19.1` to `0.19.4`, suggesting minor updates or patches were applied to the Tauri application.

6. **Minor Adjustments**:
   - Some tweaks in components, such as handling borders and layouts, were implemented for better user experience and aesthetics.

This set of changes appears to focus on improving stylistic coherence and customization options within the application, especially concerning AI feature settings, while also cleaning up unused components.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 96 (2cda071880f0dd797a374afe4cbacc515dd1a949)</summary>

The recent commit primarily entails a comprehensive redesign of the settings for the application, encompassing updates to both the frontend and backend components. Here’s a concise summary of the main changes:

1. **Frontend Components:**
   - **Settings Redesign:** The settings interface has been overhauled. New sections for Account, AI, Recording, and Shortcuts have been introduced, allowing users to manage these aspects independently.
   - **Introduction of easy-peasy:** The `easy-peasy` package has been introduced for state management, replacing certain aspects of the existing setup.
   - **UI Improvements:** New UI cards have been added for selecting different plans and AI providers, along with improved navigation and prompts for user actions.

2. **Backend and Logic Changes:**
   - **Integration with easy-peasy:** The application state management using `easy-peasy` has been advanced with models for settings, enabling persistent storage and streamlined state updates.
   - **Tauri Plugin Update:** Updated `tauri-plugin-store` version to 2.2.0, incorporating enhancements and possibly fixing issues with store management.
   - **Code Cleanup:** Redundant code has been removed, and the logic for managing AI-related settings and commands has been made more efficient.

3. **New Features:**
   - **AI Integration Options:** Users can now select from various AI integration options, such as OpenAI, Screenpipe Cloud, or customize to use their own setups.
   - **Multi-Select Component:** A new custom multi-select component has been added, allowing users to select multiple options more flexibly within the UI.

4. **Bug Fixes and Improvements:**
   - **Debounce Decrease:** Improvements in debounce settings for smoother performance and responsiveness.
   - **Store Interaction:** Improvements in how the app interacts with data stores, including proper access management and storage handling.
   - **UI Polish:** Adjustment to the display and layout for better user interaction and visual consistency.

Overall, this update aims to enhance user experience, enable clear separation of functionalities, and support more robust data handling strategies with the adoption of newer libraries and practices.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 97 (8dacdc8f42e3a0c6cd59d2f74997f8b926c0069c)</summary>

In this commit by Louis Beaumont, the size of a window in the `commands.rs` file has been increased. Specifically, the `inner_size` of the window was changed from 800x600 to 1200x850. This update affects how the window is initially displayed when created.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 98 (88d877169794c2aa2e2eb17e14047dcb712ebbb8)</summary>

The commit introduces several updates and fixes:

1. **GitHub Actions Workflow:**
   - Added a step to enable Git long paths on Windows for the `release-app` workflow.
   - Modified LLVM installation process: Changed directory creation permissions, streamlined logging, and improved error messaging.
   - Updated the process for handling Intel OpenMP DLLs, including directory handling and verbose logging of actions.

2. **Frontend (TypeScript):**
   - Enhanced error handling in `pipe-store.tsx` for the `handlePipePurchase` function by adding detailed error logs and user notifications via toast messages if the purchase fails or returns no data.

3. **Rust Backend:**
   - Updated the package version in `Cargo.toml` from 0.19.3 to 0.19.4.
   - Enhanced logging in `commands.rs` by including error logging.
   - Improved error handling when managing windows in Tauri, particularly for creating, focusing, and showing windows, providing explicit error feedback.

These changes focus on improving reliability and transparency in both the build process and application behavior, particularly around error detection and handling.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 99 (7d22de75da757d122880a6dccb986dab2b6a0f0f)</summary>

The commit `7d22de75da757d122880a6dccb986dab2b6a0f0f` involves updating documentation related to the changelog for version 0.19.3 of a project. Specifically, it adds a new file `0.19.3.md` in the `content/changelogs` directory, which includes a link to the full changelog comparison between two commits `[1249d..cf1f2]`. Additionally, it updates an existing changelog entry in `screenpipe-app-tauri/public/CHANGELOG.md` to reflect the same commit range `[1249d..cf1f2]`.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 100 (cf1f20acbf5703315daab0d4fbb24a007fd0074b)</summary>

The commit with ID `cf1f20a` made two primary changes:

1. **File Deletion**: The script `clean_workflows.sh` located in `.github/scripts/` was deleted. This script was previously used to delete all GitHub Actions cache IDs in the repository `mediar-ai/screenpipe`.

2. **Version Update**: The `Cargo.toml` file for the Tauri-based `screenpipe-app` was updated. The version number of the package `screenpipe-app` was incremented from `0.19.2` to `0.19.3`.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 101 (816c07d8c09449e5a4fd59a52c9ee84bcd487964)</summary>

The commit identified by hash `816c07d8c09449e5a4fd59a52c9ee84bcd487964`, authored by Louis Beaumont on December 19, 2024, makes a small update to the `screenpipe-vision` project. Specifically, it modifies the file `capture_screenshot_by_window.rs`. The change involves adding the string `"Screenshot"` to the `SKIP_APPS` `HashSet`, which suggests that this application is now excluded (or skipped) during some operation within the module. The commit is categorized as a chore, indicating a maintenance update rather than introducing new features or fixes.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 102 (1249de1a5078cde642c7d3609f42e113e2d35c79)</summary>

The commit primarily involves fixes and optimizations related to the application build and functionality on macOS. Here are the key changes summarized:

1. **Version Updates**:
   - The version in `Cargo.lock` is updated from `0.19.0` to `0.19.1`.
   - The version in `Cargo.toml` is updated from `0.19.1` to `0.19.2`.

2. **Code Optimizations in `screenpipe-vision/src/core.rs`**:
   - Introduced a conditional compilation for macOS with `#[cfg(target_os = "macos")]` to import necessary modules only for macOS.
   - Added a static `APPLE_LANGUAGE_MAP` using `OnceLock` to initialize and store the language mapping for macOS only once, improving performance and readability.
   - Refactored the language conversion logic in `get_apple_languages` by using a static hash map to retrieve language codes, eliminating the repeated match cases, and making the code cleaner and more efficient.
   - Improved the handling of the `languages_slice` to scope the `ns` usage properly within a closure for local initialization.

These changes enhance the build process and optimize the handling of language mappings, specifically targeting improvements for macOS environments.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 103 (2dcf4b8a517c0d9dfc4fc1a645d04d1a49ccae72)</summary>

The recent commit by Louis Beaumont updates the `README.md` file. The change involves modifying a line under the section that explains "how it works." Specifically, the description has been expanded to note that recording is done "100% locally," in addition to happening 24/7.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 104 (b78abc016f92e73722d6ba2e688acae095d26e97)</summary>

This commit, authored by Louis Beaumont, removes various references and dependencies related to Swift and dynamic libraries from the project. 

Key changes include:

1. **GitHub Workflows:**
   - The `release-app.yml` and `release-cli.yml` files had references to `libscreenpipe.dylib` and its architectures removed.
   - The workflow configuration no longer creates separate directories for different dynamic libraries.

2. **Git Ignore Configurations:**
   - Updated `.gitignore` to no longer exclude dynamic libraries, suggesting they are no longer being used or generated.

3. **Installation Script (`install.sh`):**
   - Eliminated sections of the script that handled downloading and installing macOS libraries and fixing binary paths using `install_name_tool`.
   - Removed library setup for different architectures.

4. **JavaScript Pre-Build Script:**
   - Removed code involving `install_name_tool` that modified paths for dynamic libraries in both production and development environments for macOS.

5. **Rust Build Scripts and Configurations:**
   - Deleted numerous configurations in `Cargo.toml` and build scripts related to Swift library compilation and macOS linking.
   - The `Cargo.toml` version was incremented from "0.19.0" to "0.19.1", indicating a minor release update.

6. **Swift Source Files:**
   - The Swift source file (`ocr.swift`) was entirely removed, along with the compiled Swift libraries (`libscreenpipe.dylib`, `libscreenpipe_arm64.dylib`, and `libscreenpipe_x86_64.dylib`).

7. **Tauri Configurations:**
   - The `tauri.macos.conf.json` has had `frameworks` entries referring to the deleted dynamic libraries removed.

8. **Binary Changes:**
   - There was a binary change in `ui_monitor-aarch64-apple-darwin`, although the specifics are not included in the diff.

Overall, the commit focuses on simplifying the project's dependencies by removing reliance on several Swift and dynamic library components, which could streamline cross-platform compatibility or reduce complexity for the build process.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 105 (4ccd43beb53de514c609bc2f0240ff88aef79d9b)</summary>

The recent commit `4ccd43b` introduces a feature for using Apple's native OCR capabilities within a Rust-based project. Key changes include:

1. **Feature Addition**: Integration of Apple's native OCR functionality using Rust for macOS platforms.

2. **Dependency Updates**: 
   - Added `serde` version `1.0.200` for serialization and deserialization.
   - Added `cidre` (version `0.5.0`) from its GitHub repository for interfacing with Apple's vision and core types.

3. **Code Modifications**:
   - Refactored `perform_ocr_apple` function to utilize `cidre` for handling OCR operations.
   - Removed C-based FFI logic and replaced it with higher-level Rust abstractions using `cidre`.
   - Developed structures (`OcrResult`, `OcrTextElement`, `OcrResultBBox`) for managing OCR results in a serialized format.
   - Added support for different languages by mapping application languages to Apple language codes.

4. **Benchmarks**:
   - Updated OCR benchmark functions to incorporate the new `cidre` library and ensure accuracy.

5. **Miscellaneous**:
   - Removed redundant functions and constants.
   - General code cleanup and optimizations: Removed temporary files, print statements, and duplication in logic.

Overall, these changes enhance performance and maintainability by leveraging Apple's native capabilities more efficiently and introducing support for multiple languages in OCR tasks on macOS.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 106 (64b88be0a07d8c7af7224b16aae860353229150c)</summary>

The commit adds a new changelog file for version 0.19.0. It introduces a new file `content/changelogs/0.19.0.md` to the repository, which is meant to document the changes included in this version. Additionally, it updates the `CHANGELOG.md` located in the `screenpipe-app-tauri/public/` directory to reference the same range of commits for the full changelog link. The changelog now points to compare commit range [02b62..b0dcd] instead of the previous [9a5ac..e457c].
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 107 (b0dcdf690a8e7c376e0ef8eaadc6d63d7495cdfc)</summary>

The commit made by Louis Beaumont on December 19, 2024, introduces several changes to the "screenpipe-app" Tauri application:

1. **Version Bump**: The version of the "screenpipe-app" has been updated from `0.18.9` to `0.19.0` in the `Cargo.toml` file and from `0.18.7` to `0.19.0` in the `Cargo.lock` file. This indicates a new release or a set of new features being added.

2. **Feature Enhancement**: 
   - The `open_pipe_window` function in `commands.rs` was modified. The creation of the `WebviewWindowBuilder` now includes setting the window to be "always on top" (`.always_on_top(true)`) and "visible on all workspaces" (`.visible_on_all_workspaces(true)`). 
   - Additionally, after the window is created, it explicitly sets the focus and forces the window to show by calling `window.set_focus()` and `window.show()`.
   - For macOS, there's an added configuration to adjust the activation policy to `Accessory`, which helps manage how the app interacts with the macOS windowing system.

These changes improve the window management of the app by ensuring the "pipe" window is more visible and accessible over other applications, enhancing user experience.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 108 (9129fada7e1b15d4fd6179abce0d0d96eb3b8f1b)</summary>

The commit `9129fada7e1b15d4fd6179abce0d0d96eb3b8f1b` by Louis Beaumont adds a new script to the repository for cleaning GitHub Actions caches. The script, named `clean_workflows.sh`, is located in the `.github/scripts/` directory.

Here is a summary of what the script does:
- It executes a GitHub CLI command to fetch all cache IDs for the repository `mediar-ai/screenpipe` using the appropriate API call.
- The script uses `jq` to parse the JSON response and extract cache IDs.
- For each cache ID obtained, it iterates through the list and deletes the cache by making another API call with the DELETE method.
- During this process, it prints out messages indicating the cache IDs being deleted.

This script is marked executable (mode 100755), indicating that it can be run directly from a terminal or script execution environment.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 109 (02b62f913aeb739753fd41dad72ec292464c6230)</summary>

The commit, authored by the GitHub Actions Bot, adds a changelog for version 0.18.9. This involves the creation of a new file, `0.18.9.md`, in the `content/changelogs` directory. Additionally, the commit updates the `CHANGELOG.md` file located in the `screenpipe-app-tauri/public/` directory to reflect the latest changes between the specified commit range `[9a5ac..e457c]`. The commit does not alter other existing content in the project.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 110 (e457c602818b73e876dd8ffe08c7aaf47bdb0b11)</summary>

The recent git commit introduces a few changes to the project:

1. **`pipe-store.tsx` file**:
   - Added a new entry for a core pipe with the ID "search". This entry provides functionality to search through screen recordings and audio transcripts with AI.
   - The "timeline" core pipe's credits have been changed from 20 to 0, and it is no longer marked as paid.

2. **`Cargo.toml` file**:
   - Updated the package version from "0.18.8" to "0.18.9", indicating a minor version release.

Overall, the changes involve enhancements to the pipe store components, including a new "search" functionality and adjustments to the "timeline" pipe settings, as well as a version bump of the project.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 111 (fb1980a396014a7f2e4d73291b7ace7919c66a90)</summary>

The commit includes two main changes:

1. **Configuration Updates:**
   - Added a `devIndicators` section to the `next.config.ts` files across multiple directories (`data-table`, `identify-speakers`, `meeting`, `search`, and `timeline`) within the `pipes` directory. This section includes:
     - `buildActivity: false`
     - `appIsrStatus: false`
   These additions serve to disable specific development indicators in the configuration.

2. **Text Correction:**
   - Fixed a grammatical mistake in the `recording-settings.tsx` file within the `screenpipe-app-tauri/components` directory by removing a duplicate sentence: "do not collect any screen data, microphone, query data."

Overall, the commit addresses toggling development indicators off in Next.js configuration files and corrects a text redundancy in a component file.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 112 (fd098e9c88f8947552713a7e23887d2381f43d1a)</summary>

The recent commit, identified by hash `fd098e9`, introduces a new feature focusing on media validation and updates to the video component. Below is a summary of the changes made:

1. **Video Component Enhancements**: 
   - Media validation functionality was added to the video components located in both `identify-speakers` and `search` directories. 
   - A new asynchronous function `validateMedia` verifies the existence and validity of media files by contacting a server endpoint.
   - Logic now includes handling of different validation statuses such as valid media, nonexistent media, and incomplete media files with appropriate error handling.

2. **Server-Side Support and Media Validation**:
   - The `screenpipe-server`'s Rust backend has a new endpoint `/experimental/validate/media` to support media validation requests.
   - The `validate_media` function checks if media files exist and are valid using `ffmpeg` via an asynchronous command.

3. **Minor Changes**:
   - The `favicon.ico` in two directories, namely `pipe-for-loom` and `search`, were modified.
   - The metadata in the `layout.tsx` for the `search` app was updated with a more descriptive title and description.

4. **Refactoring and Error Handling**:
   - Error messages were added to improve debugging and ensure that failures are communicated back when media files are invalid or missing.
   - The Rust utilities for video operations now include media validation as part of the merging process, ensuring the integrity of media inputs during operations.

These changes contribute to more robust media handling in the video components and establish backend support for media validation, enhancing the system's ability to process and verify video data effectively.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 113 (a9bd3d1a950eb049333a7ec1954eac271954137a)</summary>

The commit by Louis Beaumont updates the `README.md` file in the `pipes/meeting` directory. The changes include the addition of text emphasizing that the meeting assistant is the first to work without internet, either by embedding AI locally or by using services like Anthropic and OpenAI. It also highlights privacy concerns by stating that other meeting assistants may expose users' data to US data centers and the government. The order of the text and the image in the file has been slightly adjusted, but the image itself remains unchanged.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 114 (f2e5959c79d28c02ec50c33475b6426a8338d530)</summary>

The commit `f2e5959` by Louis Beaumont updated the `README.md` file in the `pipes/meeting` directory. The previous content "show your daily meetings" was replaced with a description of the project as "The AI notepad for people in back-to-back meetings." It now includes additional information on how the "meeting pipe" processes and enhances raw meeting recordings. An embedded image, possibly a screenshot relevant to the project, was also added to the README.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 115 (778130175f0d59c0743cd348fe8dc1ab01a1dea8)</summary>

The commit updates the file `pipes/meeting/README.md` by replacing its contents. Previously, the README included several sections with descriptions and screenshots related to various features like search, keyword filtering, window/app filtering, and speaker filtering. The entire content has been replaced with a single line: "show your daily meetings."
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 116 (9a5acc96720208afec05ff2f7695b0b60380cabf)</summary>

The commit by Louis Beaumont involves updating the version number of the package in the `Cargo.toml` file. The version of the "screenpipe-app" has been incremented from "0.18.7" to "0.18.8", likely indicating a new release of the app.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 117 (403718d345b5e71e1944b3fcaf52e32597fa368b)</summary>

The commit made by Neptune addresses duplicate entries in both front-end and back-end components of a project. Here are the key changes made:

1. **Front-end (TypeScript/React components)**:
   - In the `pipes/search/src/components/search-chat.tsx` and `pipes/timeline/src/components/search-chat.tsx` files, the mapping of `speakers` has been updated to remove duplicates. This is achieved by converting `speakers` to a `Set` before mapping, ensuring that each speaker is unique. 

2. **Back-end (Rust/SQL components)**:
   - In the `screenpipe-server/src/db.rs` file, several SQL queries have been modified to use `DISTINCT` to ensure unique results:
     - The `SELECT` queries that count distinct elements in various tables now enforce uniqueness with `DISTINCT` clauses.
     - In the `search_speakers` function, the SQL query has been updated to use `SELECT DISTINCT *` to avoid duplicate speaker entries when searching by name prefix.

Additional changes include minor formatting adjustments (such as whitespace), improving code cleanliness and readability. Overall, the commit enhances the system by ensuring that duplicate entries are filtered out in both data presentation and data retrieval processes.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 118 (3caa7a0de6d02173f07c314faca3d31a18c954b4)</summary>

The Git commit by Louis Beaumont addresses a build fix for the "release-app". The changes made are significant, involving the removal of several TypeScript files from the "screenpipe-app-tauri" directory, alongside some key supporting files. The removed files include components for "identify-speakers", "app-sidebar", "context-usage-indicator", "meeting-history", and several hooks and utility files related to conversation and search history. These deletions suggest a clean-up or restructuring effort that results in a simpler or more efficient application.

Additionally, the `Cargo.lock` and `Cargo.toml` files in the `src-tauri` directory have been updated. The version number is incremented from "0.18.6" to "0.18.7," signaling a new release that possibly correlates to the removal of unused or deprecated functionalities.

In summary, this commit is a build fix that removes a significant number of files to possibly streamline the application, accompanied by a version update.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 119 (609b4401a666f65517a7239ad8e207c32782d1dd)</summary>

The commit made by Louis Beaumont with the ID `609b4401a666f65517a7239ad8e207c32782d1dd` is focused on updating version numbers in two `Cargo.toml` files.

1. In the root `Cargo.toml` file:
   - The version of the workspace package has been updated from "0.2.17" to "0.2.18".

2. In the `screenpipe-app-tauri/src-tauri/Cargo.toml` file:
   - The version of the `screenpipe-app` package has been updated from "0.18.5" to "0.18.6".

These changes are likely in preparation for a new release of the application.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 120 (e8ae3bc2ccfdaca85ca2c373b51d973d68690840)</summary>

This commit focuses on improving the audio transcription feature, particularly by incorporating start and end time functionality throughout the application. Here are the key changes:

1. **Components Updates:**
   - Added use of `startTime` and `endTime` for audio handling in the `VideoComponent` and `AudioPlayer` components, primarily for rendering and playback.
   - Enhanced the `ExampleSearchCards` component to manage examples using hooks.
   - Introduced a custom audio player that shows audio progress with markers for start and end time.
   
2. **Backend Enhancements:**
   - Refactored audio processing functions to include start and end time information during audio segment preparation and transcription.
   - Added new SQL migrations to update the database schema, incorporating `start_time` and `end_time` columns for `audio_transcriptions`.
   - Modified database access methods to handle the new columns and support retrieval and storage based on these timestamps.
   - Adjusted logic for handling transcription results in server processes, ensuring the inclusion of time markers in the stored data.

3. **Rust Code Adjustments:**
   - Updated several Rust functions in the server and library components to manage and process start and end times of audio.
   - Created new functionality for writing audio with time-based segments to files.

4. **Testing and Types:**
   - Updated the test suites to accommodate changes in audio processing, ensuring all tests inject start and end times where necessary.
   - Modified types to support optional times in related data models.
  
Overall, these changes aim to provide more precise and controlled audio playback and transcription functionalities, allowing for better synchronization and usability across the application.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 121 (f67af93d228cdc519d94e2c2e74143feff0aa8f6)</summary>

The commit `f67af93d228cdc519d94e2c2e74143feff0aa8f6` made by Louis Beaumont addresses a couple of issues in the "screenpipe-app" project and includes the following changes:

1. **Functionality Improvement**:
   - Fixed an issue where settings were not refreshing properly in the pipe component of the `screenpipe-app`.
   - Enhanced the logic around handling user authentication and subscription checks when toggling features in the application, such as handling login prompts and checking for user credits, together with adding detailed logs.

2. **Refactoring and Debugging Enhancements**:
   - Added console logs throughout the `pipe-store.tsx` file to better trace the logic flow of enabling/disabling pipes and handling purchases.

3. **Infrastructure and Maintenance**:
   - Updated the version of the `screenpipe-app` from `0.18.4` to `0.18.5` in both `Cargo.lock` and `Cargo.toml` files, indicating a new minor release that likely incorporates bug fixes or small improvements without major changes.

4. **Code Formatting**:
   - Adjusted some multiline strings for better readability without altering the content.

Overall, the commit enhances the user authentication logic, better supports purchasing logic with improved feedback mechanisms, and provides additional logging to help with debugging.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 122 (eb8ea53b2bb09b40ee4a03285e4897b3d951b46b)</summary>

The commit `eb8ea53b2bb09b40ee4a03285e4897b3d951b46b` by Louis Beaumont updates the `README.md` file in the `pipes/identify-speakers` directory. The update makes significant changes by removing most of the existing content, including descriptions of features like search, review, keyword filtering, window/app filtering, and speaker filtering, along with several accompanying images. The revised version introduces a brief description of the functionality, stating that it allows you to teach AI to assign names to voices for use across ScreenPipe, with a new screenshot added.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 123 (924f97fef0d85830a94816e6a17e4824f7fcd640)</summary>

The commit with ID `924f97fef0d85830a94816e6a17e4824f7fcd640` contains various fixes and improvements across multiple files in the project. Here's a summary of the changes made:

1. **Dependency Updates**:
   - Updated the `@screenpipe/js` dependency in several `package.json` files from version `0.1.10-beta.18` or `0.1.10-beta.20` to `0.1.10-beta.21`.

2. **`use-settings.tsx` Hook Updates**:
   - Common updates to `use-settings.tsx` in several modules (`data-table`, `identify-speakers`, `meeting`, `timeline`, `search`) to include:
     - Added a `user` property with a `token` in default settings.
     - Implemented settings auto-refresh on window focus and optionally every 30 seconds.
   
3. **Removal of Console Logs**:
   - Removed `console.log` statements from API settings routes in several modules (`identify-speakers`, `meeting`, `search`).

4. **Function Imports Adjusted**:
   - In `identify-speakers.tsx`, adjusted the imports for `getFileSize` and `keysToCamelCase`.

5. **Video Actions Enhancements**:
   - In `video-actions.ts`, improved the `getMediaFile` function for better error handling and added a new `getFileSize` function to retrieve the size of a file.

6. **Capabilities Configuration Updates**:
   - Modified `local` property in `main.json` and `capabilities.json` from `false` to `true`.

7. **Settings Manager Enhancement**:
   - Removed a conditional initialization check in the `getAll` method of `SettingsManager` class in `node.ts`.

8. **Increment Version in `screenpipe-js`**:
   - Updated the version of `@screenpipe/js` to `0.1.10-beta.21` in `package.json`.

Overall, these changes address dependency updates, setting management improvements, code cleanup, and logging optimizations.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 124 (702f3b3f945e2304bbe6fc45439fc217cfddbf98)</summary>

The commit includes several changes across different files in a project, which can be summarized as follows:

1. **File: `pipes/identify-speakers/src/lib/utils.ts`**
   - Removed unused imports including `@tauri-apps/plugin-fs`, `@tauri-apps/plugin-os`, and others.
   - Deleted functions such as `getCliPath`, `removeDuplicateSelections`, `parseKeyboardShortcut`, and `getFileSize`.
   - Retained utility functions like `cn`, `convertHtmlToMarkdown`, and `stringToColor`.

2. **File: `pipes/meeting/src/lib/utils.ts`**
   - Removed imports of `ContentItem` and `js-levenshtein`.
   - Removed the `removeDuplicateSelections` function.
   - Added two new utility functions: `stripAnsiCodes` for removing ANSI codes from strings, and `toCamelCase` for converting strings to camel case.
   - Added functions `encode` for URL encoding, `convertHtmlToMarkdown`, and `stringToColor`.

3. **File: `screenpipe-app-tauri/components/pipe-store.tsx`**
   - Added two new entries to the `corePipes` array:
     - "meeting assistant" for organizing and summarizing meetings.
     - "speaker identification" for identifying and labeling speakers in recordings.

4. **Files: `screenpipe-app-tauri/src-tauri/capabilities/main.json` and `screenpipe-app-tauri/src-tauri/gen/schemas/capabilities.json`**
   - Changed the `"local"` property from `true` to `false` in the `migrated` capabilities configuration.
   
The changes primarily involve cleaning up unused code, adding new utilities, updating pipeline configurations, and modifying permissions in capability files.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 126 (16c4e362b0b4e97c9d07029d03a3d5fa93b4e8e7)</summary>

The git commit by Louis Beaumont added documentation to the `integrations.mdx` file. Specifically, a new section was introduced for the "mcp / anthropic app," including a link to its GitHub repository: https://github.com/mediar-ai/screenpipe/blob/main/screenpipe-integrations/screenpipe-mcp.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 127 (199e4ea31504873368e768e565a506ea775060c8)</summary>

The commit `199e4ea31504873368e768e565a506ea775060c8` by Louis Beaumont is titled "fix mcp" and includes several changes related to the `screenpipe-mcp` project.

1. **`pyproject.toml` Modifications:**
   - Unnecessary new lines at the beginning were removed.
   - The required Python version was updated from `>=3.9` to `>=3.10`.
   - Added configuration options under `[tool.hatch.build.targets.wheel]` and `[tool.hatch.build]` for packaging.

2. **File Renaming and Relocation:**
   - The files `__init__.py` and `server.py` were relocated from the root of the `screenpipe-mcp` directory to `src/screenpipe_mcp/`. This change also included the necessary path adjustments in the project structure.

3. **New Binary Files:**
   - Two new compiled Python files `__init__.cpython-311.pyc` and `server.cpython-311.pyc` were added into the `__pycache__` directory.

4. **`uv.lock` Modifications:**
   - The required Python version was decreased from `>=3.11` to `>=3.10`.
   - Added a new dependency `exceptiongroup` with a version specification, and a marker to include it only when `python_full_version < '3.11'`.
   - The use of `typing-extensions` was specified for Python versions `< 3.11`.

5. **Package Metadata Adjustments:**
   - The dependencies `httpx`, `mcp`, and `nest-asyncio` were adjusted to not have version specifiers in the project's metadata section.
   - Some dependencies and wheels related to `pydantic_core` were included with details for multiple platforms.

Overall, these changes seem to be focused on updating dependencies, adjusting the Python version requirements, restructuring the code layout, and preparing the package for an updated build system with `hatch`.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 128 (f2e9ce11a94eaaca513c218ef4c8bafba99f4c85)</summary>

The commit `f2e9ce11a94eaaca513c218ef4c8bafba99f4c85` by Louis Beaumont on December 18, 2024, deletes the file `hello.py` located in the `screenpipe-integrations/screenpipe-mcp` directory. The deleted file contained a simple Python script with a `main` function that printed "Hello from screenpipe-mcp!" when executed.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 129 (8cd68a734d5f4a56520ee44598985e2ee2e8682f)</summary>

The commit with hash `8cd68a7` primarily updates documentation related to the changelog of version 0.18.4. Here's a summary of the changes:

1. A new changelog file for version 0.18.4 has been added at `content/changelogs/0.18.4.md`. The file includes a link to the full changelog comparison on GitHub.

2. The existing changelog in `screenpipe-app-tauri/public/CHANGELOG.md` has been updated to reflect the new version, changing the reference link to the commit comparison for version 0.18.4.

Both updates include modifying links to direct to the appropriate commit comparison range for version 0.18.4, from `[22632..647c9]`.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 130 (647c98d6c7ddf5c81995bb464233bf8d1bdf5cee)</summary>

The commit updates the version number of the "screenpipe-app" package in the `Cargo.toml` file from "0.18.3" to "0.18.4". The purpose of this change is to release a new version, presumably with a feature called "retry pipe download" as mentioned in the commit message.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 131 (9619b4b50036a90d0ded0c0347e17d658031959e)</summary>

The commit by Louis Beaumont adds a new feature called "anthropic mcp" to the project. It involves creating a new directory structure under `screenpipe-integrations/screenpipe-mcp`, intended to integrate the screenpipe with the Model Context Protocol (MCP) for improved search capabilities on screen recordings and audio transcriptions.

Key changes include:

1. **Configuration Files:**
   - A `.python-version` file is added specifying the Python version 3.11.
   - A `pyproject.toml` defines the project configuration and dependencies, targeting Python version >=3.9 and includes packages like `mcp`, `httpx`, and `nest-asyncio`.

2. **Documentation:**
   - A `README.md` file provides setup and usage instructions, detailing configuration requirements for Claude Desktop and testing procedures using the MCP Inspector. It highlights key features like search parameters, response formatting, and error handling.

3. **Python Modules:**
   - An `__init__.py` module sets up the main entry point for the package. It initializes an asynchronous server function.
   - A `hello.py` script is a simple program that prints a greeting message.
   - A `server.py` script implements the core server logic using `asyncio` and `httpx`. It defines tools for searching content within screen recordings and processing requests to the screenpipe API.

4. **Dependency Management:**
   - A `uv.lock` file records the resolved dependencies and their specific versions, ensuring the environment can be recreated reliably.

Overall, these changes introduce a new MCP server to enhance the screenpipe integration, enabling advanced search features and interaction with other tools via the MCP specification.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 132 (d3937dd80e7a3675ca23749e6860699182c51aca)</summary>

The commit made by Louis Beaumont adds a retry mechanism for downloading pipes in the `screenpipe-core` project. It specifically introduces a new asynchronous function called `retry_install` in the `pipes.rs` file. This function attempts to install packages using `bun install` with a maximum of three retries and includes exponential backoff for retry delays.

The code modification changes the installation process in the `download_pipe` function to use `retry_install` instead of a single execution of the installation command. This ensures that if the installation fails, it will automatically retry up to the specified number of times before giving up, thus improving the robustness of the installation process.

Additionally, the code formatting for the `get_last_cron_execution` function is slightly altered for better readability, but its logic remains unchanged.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 133 (e1e2bfa98330e43c6dd5dccb1419b21e555c332f)</summary>

This commit addresses a warning by making a few changes across different components of the project:

1. **Cargo.lock Update**: 
   - The version of the `screenpipe-app` package has been bumped from `0.18.2` to `0.18.3`.

2. **Binary Changes**:
   - The binary files `ui_monitor-aarch64-apple-darwin` and `ui_monitor-x86_64-apple-darwin` have differences, indicating they have been updated or rebuilt. These changes are not detailed due to the binary nature of the files.

3. **Code Enhancement in `pipes.rs`**:
   - A section of code responsible for logging has been updated. The list of patterns used to determine if a log line should be categorized as an info log has been expanded. Additional patterns such as "Webpack is configured", "See instructions", and "https://nextjs.org" have been added.
   - Redundant comments indicating not to wait for a child process to finish have been removed.

Overall, this commit primarily refines logging behavior in the `pipes.rs` file while updating project binaries and incrementing the app version.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 134 (22632a40a42634c8daacd811a5e9828faa4f69b7)</summary>

This commit introduces a new changelog for version 0.18.3 of a project. A new file, `0.18.3.md`, was added to the `content/changelogs` directory, outlining the full changelog. Additionally, the existing `CHANGELOG.md` in the `screenpipe-app-tauri/public` directory was updated to reflect the new version's changes. The changelog references the differences between commit hashes `6f903` and `8861b`.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 135 (8861bc905db9b903337b61c3e1157fbf4ec611fb)</summary>

This commit introduces several updates and enhancements to the `screenpipe-app-tauri` project, particularly focusing on the tray menu functionality. Here's a summary of the changes:

1. **Updates in Versioning**:
   - The version of the `screenpipe-app` package has been updated from `0.17.9` to `0.18.2` in `Cargo.lock`, and from `0.18.2` to `0.18.3` in `Cargo.toml`.

2. **File Exclusion**:
   - Changes to the `Cargo.toml` file reflect the formatting of the `exclude` list for improved readability (no functional change).

3. **Tray Menu Enhancements**:
   - A new module `tray` has been introduced to manage tray menu functionalities.
   - The tray menu now dynamically updates to include menu items for each "enabled pipe" fetched from an API.
   - The `update_tray_menu` function periodically updates the tray menu to reflect the current state of pipes.

4. **Main Application Logic**:
   - Integration of a function `get_pipe_port` that retrieves the port information for a given pipe from an API.
   - Logic added to handle the selection of tray menu items associated with individual pipes. When such an item is chosen, it attempts to open a window corresponding to the pipe's port.
   - The `setup_tray_menu_updater` is called to regularly refresh the tray menu.

5. **Sidecar Management**:
   - Commented out some code related to killing processes which might improve user experience during development.

Overall, these changes improve how the application interacts with users via the system tray, making the app more dynamic and responsive to the current state of available features (pipes).
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 136 (6f9038a0e388b3da28294117c2c8c0366ace2082)</summary>

The commit introduces changes to the Screenpipe application to support passing a custom API token to the Deepgram service. Key changes include:

1. **Version Update**: The application's version in `Cargo.toml` has been incremented from `0.18.1` to `0.18.2`.

2. **Environment Variable for API Token**: The code now includes logic to set the environment variable `CUSTOM_DEEPGRAM_API_TOKEN` with the user's token when launching the sidecar, if the user has a token available and the `screenpipe-cloud` service is selected.

3. **Handling Missing API Token**: In the `transcribe_with_deepgram` function, a check is added to ensure that a custom API token is present. If not, an error is returned indicating that `CUSTOM_DEEPGRAM_API_TOKEN` is not set.

4. **API Token Selection**: The API token used for authorization in the request to Deepgram is updated to select between the custom API token from the environment variable, if available, or the default token.

These modifications facilitate improved interaction with the Deepgram API by enabling support for user-specified tokens.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 137 (db1cef5763611229473f0865e188287b4ced7b9e)</summary>

The latest changes introduce authentication to the API via Clerk, update dependencies, and make several modifications and deletions in the project files. Here's a summary of the key changes:

1. **API Authentication**: 
   - Added authentication to the API endpoints. A user's token is validated using Clerk's `verifyToken` function, and requests lacking valid tokens receive a 401 Unauthorized response.
   - Updated API proxy to support Anthropic AI alongside OpenAI. Tokens are used to determine user authorization.

2. **Dependency Updates**: 
   - Updated the version of `@screenpipe/js` from `0.1.10-beta.18` to `0.1.10-beta.20`.
   - Added new dependencies: `@anthropic-ai/sdk` and `@clerk/backend`.

3. **Enhancements**:
   - Integrated support for Anthropic AI models into the AI proxy, allowing API requests for two different AI providers: OpenAI and Anthropic.
   - Altered how AI responses are handled, with adaptations for handling Anthropic API responses alongside OpenAI responses.
   - The search function in the app is now capable of filtering results based on selected speakers from audio data.

4. **Code Deletions**:
   - Removed several components from the `/screenpipe-app-tauri/` directory, including `auth.tsx`, `chat-message-actions.tsx`, `chat-message-v2.tsx`, `example-search-cards.tsx`, `rag-example.tsx`, and `search-chat.tsx`.
   - Removed the `stripe-subscription-button.tsx`, which suggests changes in handling subscriptions or a redesign.

5. **User Settings**:
   - Included a `user` field in the settings object to store user information and authentication token.

6. **Debugging & Logging**:
   - Added console logging for various operations, possibly for monitoring or debugging purposes, especially related to settings retrieval and AI response streaming.

Overall, this commit significantly improves security by introducing authentication, introduces a more flexible AI proxy supporting multiple AI providers, updates package dependencies, and promotes code refactoring by removing redundant components.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 138 (81d716af197fdd51fb8dbfaa93eadf127266306f)</summary>

The commit made by Louis Beaumont updates the `README.md` file by adding a new point in a list under the "why?" section. Originally, the section contained one point about missing context for AGI when not recording. The update introduces a new first point stating that "data is the biggest bottleneck in AI right now," and shifts the existing point to the second position in the list.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 139 (ceb5fb78e297da889da0ece44300d680f8b32132)</summary>

The commit `ceb5fb78e297da889da0ece44300d680f8b32132` involves adding a changelog for version 0.18.1 of the project. Key changes include:

1. A new changelog file `0.18.1.md` is created in the `content/changelogs/` directory.
2. The public changelog file at `screenpipe-app-tauri/public/CHANGELOG.md` is updated by modifying the "Full Changelog" link to reflect changes comparing two different commit references: from `[61218..c7053]` to `[1104c..8f9a7]`.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 140 (8f9a7e300f41ce45b39b5fff8e8dd14babe9c3d3)</summary>

The recent git commit, authored by Louis Beaumont, introduces two main changes:

1. **GitHub Actions Workflow Update**: The `release-app.yml` file within the GitHub workflows directory has been modified to include a new step for setting up the Rust path when the `--target x86_64-pc-windows-msvc` argument is used. This step updates the system's PATH environment variable and saves it to GitHub's PATH, followed by verifying Rust's availability with a version check.

2. **Cargo.toml Version Bump**: In the `Cargo.toml` file located at `screenpipe-app-tauri/src-tauri/`, the `version` of the `screenpipe-app` package has been incremented from `0.18.0` to `0.18.1`. This indicates a minor version update, likely due to the changes in the workflow setup.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 141 (d63e40e71e44d1d0896d7593ffefd2947116fa54)</summary>

The commit by Louis Beaumont on December 16, 2024, updated the `README.md` file. The specific change involved adding a new screenshot to the document. The added line inserts an image with a width of 1312 pixels, taken on December 16, 2024, at 2:39 PM. This image is now included between two existing images in the README.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 142 (543b223cfd5898275e56e733adb0f6edcb15a9e7)</summary>

The commit made by Louis Beaumont improves the user experience for adding a custom pipe in the `pipe-store` component of a React-based application. The main changes include:

1. **Enhanced Toast Notification**: The process of adding a custom pipe now features a more detailed toast notification with a visual progress bar. The toast initially informs the user that the installation is starting and updates periodically to show the installation progress.

2. **Progress Update**: An interval updates the progress bar every 500 milliseconds, increasing the value by 3 until the operation completes or fails.

3. **Error Handling**: If an error occurs during the process, an error message is displayed, and the progress update is halted.

4. **Completion Notification**: Upon successful addition of the pipe, the progress bar reaches 100%, and the toast updates to indicate completion before disappearing after a short duration.

5. **Code Reordering**: The `setNewRepoUrl` function call, which clears the input field, has been relocated to after the successful completion of the process, ensuring it only runs if no errors occur.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 143 (aa172a5eaa0642401822772a5bb9c705a1868970)</summary>

The commit, authored by Louis Beaumont, updated the `README.md` file in the `pipes/data-table` directory. The changes involved removing the introductory content about setting up and running a Next.js project, including installation instructions and various Next.js-related resources. The new version of the README now briefly describes the purpose of the project as a tool to "visualize your data in a table" and includes a screenshot image to presumably illustrate the application's interface or output.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 144 (ca8c38ce159d298ddca209fa9ef3bae4b2c4bf90)</summary>

The git commit with the hash `ca8c38ce159d298ddca209fa9ef3bae4b2c4bf90` by Louis Beaumont updates the `README.md` file. The specific changes involve modifying the "how it works?" section. The previous version mentioned daily shipping and provided a way to give feedback via email and booking calls. The updated version focuses on the process with three main points: recording everything 24/7, indexing it into an API, and enabling developers to build AI apps using users' full context, with capabilities for desktop native development using Next.js, along with publishing and monetization options.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 145 (7ce697e924017808473bc54439fbd103c65e897a)</summary>

The commit made by Louis Beaumont on December 16, 2024, updates the `README.md` file by adding new content. Specifically, two additional images with specific dimensions and source URLs have been included into the file. These images are appended to the existing HTML code block, which previously contained just one image.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 146 (1047777f999e6b13710cff17a0b3ee2133359c18)</summary>

The git commit `1047777f999e6b13710cff17a0b3ee2133359c18` updates the `README.md` file located in `pipes/timeline/`. The update involves modifying an image included in the document. Specifically, one screenshot link has been removed and replaced with a new screenshot link. The content and format of the rest of the document remain unchanged.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 147 (76a0e11d86eb3971209da03bedd89f8129ecc8d6)</summary>

The commit made by Louis Beaumont updates the `README.md` file located in the `pipes/timeline` directory. The update adds a new screenshot image to the document. This screenshot is displayed with dimensions of 1119 pixels in width and corresponds to a state captured on December 16, 2024, at 2:01:35 PM. The change is supplementary and does not alter or remove any existing content in the README file.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 148 (1104c5af74f9dfad13f19bb51fa02e631fa0c73a)</summary>

The git commit with hash `1104c5af74f9dfad13f19bb51fa02e631fa0c73a` authored by Neptune on December 16, 2024, addresses ONNX linking issues. The changes introduce a new file, `build.rs`, in the `screenpipe-server` directory. This file contains a Rust build script that ensures, on Windows platforms, a linker search path is added for the ONNX runtime libraries. The path specified is relative to the project structure, directing the compiler to locate the necessary ONNX library in the `screenpipe-app-tauri/src-tauri/onnxruntime-win-x64-gpu-1.19.2/lib` directory.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 149 (ae07d0e4240c557b1f9369817b55110253073749)</summary>

The commit `ae07d0e4240c557b1f9369817b55110253073749` involves updating documentation to include a changelog for version 0.18.0. A new file has been added at `content/changelogs/0.18.0.md`, which references a full changelog link comparing two commits. Additionally, the `CHANGELOG.md` file located in `screenpipe-app-tauri/public` was updated to change the commit comparison in the full changelog link from `adcb9..19f4c` to `61218..c7053`.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 150 (c7053ae0761bd2354c76536ead71c5a46c961e07)</summary>

The commit by Louis Beaumont titled "release-app" updates several files in the project:

1. **`pipe-store.tsx`**: A new `CorePipe` entry has been added to the `corePipes` array. This new pipe has the following properties:
   - `id`: "data-table"
   - `name`: "data table"
   - `description`: "explore your data in a powerful table view with filtering, sorting, and more"
   - `url`: Points to the "data-table" pipe's GitHub URL.
   - `credits`: 0
   - `paid`: `false`

2. **`Cargo.toml`**: The version number for the Tauri app has been updated from `0.17.9` to `0.18.0`.

3. **`package.json`**: The version number for the `@screenpipe/js` package has been updated from `0.1.10-beta.18` to `0.1.10-beta.19`. 

These changes indicate a version update and the addition of a new feature (data table) in the `pipe-store.tsx` component.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 151 (2b56d7da987c71ab0f4bd4c08c7647107059da0b)</summary>

The commit introduces a new feature for a data viewer table focused on handling screen pipe data within a Next.js application. Here are the key changes:

1. **Project Setup:**
   - A new Next.js project has been set up using `create-next-app`.
   - A `.gitignore` file has been added to exclude node modules, build outputs, and other unnecessary files.

2. **Components:**
   - Several UI components have been created, including tables, buttons, dropdown menus, and more, using libraries such as `@radix-ui` and `lucide-react`.
   - Components specifically tailored for data representation:
     - `AudioTranscriptionsTable`: Displays audio transcription data.
     - `DatabaseSidebar`: Lists database tables.
     - `OcrDataTable`: Shows OCR processing results.
     - `UiMonitoringTable`: Displays UI monitoring data.
     - `VideoChunksTable`: Handles video chunks data.

3. **UI and Theming:**
   - A global CSS file with Tailwind CSS configurations for theming and component styles, including media support for dark mode.
   - Components and hooks for managing UI states, like toasts and tooltips.

4. **Server Functionality:**
   - `GET` and `PUT` endpoints in `route.ts` to fetch and update settings data from the server using Next.js API routes.

5. **Utilities and Hooks:**
   - Custom utility functions and React hooks for operations such as debounce handling, clipboard copy, and SQL data fetching.
   - Implementations of data-fetching hooks tailored for health checks and SQL autocomplete queries.

6. **Package Management and Configuration:**
   - Various dependencies are handled in `package.json`, including Next.js, Radix UI, and Tanstack Table. Development configurations are specified in `tsconfig.json`.

7. **Static Assets:**
   - SVG icons for different purposes included in the `public` directory for UI rendering.
  
The project seems to focus on building a robust interface for querying and managing screen pipe data with optimally designed components and efficient data handling practices.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 152 (c18deb1d8cf89378a225c9ae1551c6bac22d36b8)</summary>

The commit with hash `c18deb1d8cf89378a225c9ae1551c6bac22d36b8` includes updates to the `README.md` file in the `pipes/search` directory. The author of this commit is Louis Beaumont. The update adds a section on "speaker filtering," which mentions the ability to identify people you speak to continuously. Additionally, a new screenshot has been included to illustrate this feature. The commit also contains two additional lines labeled "etc."
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 153 (e252cd9bb17adc425963cdc6834c2a06a1730553)</summary>

The commit by Louis Beaumont updates the `README.md` file within the `pipes/search` directory. The update involves a significant reduction in content, removing introductory and instructional information about setting up and deploying a Next.js project. Instead of the original detailed setup instructions, the new version includes concise topics such as "search, review, summarize," "audio," "keyword filtering," and "window/app filtering," alongside several images. The removed sections previously contained details about running the development server, editing the main page, learning more about Next.js, and deploying on Vercel.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 154 (bedb8b8f23ba584483f32304b8f59e1e3d21822f)</summary>

The git commit by Louis Beaumont updates the `README.md` file located in `pipes/timeline`. The previous content, which detailed steps to get started with a Next.js project, instructions for running a development server, and information about Next.js resources and deployment on Vercel, has been entirely removed. It has been replaced with a brief description of a feature allowing users to scroll back in time, select time ranges, and use cmd+k to interact with AI about screen activity or audio conversations. Additionally, a screenshot image link has been added.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 155 (81a68639aaab63b86db383f6a3b5bda4bf9a38c8)</summary>

The commit with ID `81a68639aaab63b86db383f6a3b5bda4bf9a38c8` was authored by Ezra Ellette on December 16, 2024. The commit updates the `AudioContent` interface in the `screenpipe-js/types.ts` file by adding two optional properties: `startTime` and `endTime`, both of which are numbers.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 156 (6121844d6b595d14ce179e126c210a6a9bd97b9b)</summary>

The recent Git commit (ID: 6121844d) introduces updates to the `Cargo.toml` files across several crates within the project. The key changes include:

1. **Dependency Updates**:
   - The `hf-hub` dependency version was updated to `0.3.2` in the root project's `Cargo.toml`, as well as in `screenpipe-core` and `screenpipe-server`.
   - The `native-tls` feature was added to the `hf-hub` dependency in these crates to enhance compatibility with the latest version and its requirements.
   - The `hf-hub` dependency configuration now includes both the `git` source and specific features.

2. **Project Structure Adjustments**:
   - The formatting of the `exclude` section in the `Cargo.toml` of the root project was slightly adjusted to a single-line style.

3. **Binary File Updates**:
   - Binary files located in `screenpipe-app-tauri/src-tauri` for `ui_monitor` on both `aarch64` and `x86_64` Apple Darwin platforms were modified.

4. **Miscellaneous**:
   - Minor formatting adjustments were made in the `Cargo.toml` files to follow a consistent style, such as multiline feature lists.

These changes aim to ensure smoother integration and usage of the `hf-hub` library across the project's different modules while maintaining updated dependencies and source inclusions.

</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 157 (6fb9f087d8ecdb92332da1d93e5dd11bf7e2b021)</summary>

The commit with ID `6fb9f087d8ecdb92332da1d93e5dd11bf7e2b021`, authored by Louis Beaumont, updates the version of the "screenpipe-app" package. The `Cargo.lock` file previously indicated a version of "0.17.6", which is now updated to "0.17.9". Similarly, the `Cargo.toml` file version has been changed from "0.17.8" to "0.17.9". These updates likely correspond to a new release of the application or package titled "release-app timeline".
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 158 (9f8fae1d3891dd07af19cc9ef49e3200a218af8a)</summary>

The commit made by Louis Beaumont on the specified date includes several changes related to the pipe store functionality in the Screenpipe app. Here is a summary of the key modifications:

1. **New Feature for Pipe Store:**
   - A new pipe named "timeline" is added to the `corePipes` array. This pipe is described as a tool for visualizing daily activities using an AI-powered timeline, which is intended for time tracking and productivity analysis. It is a paid feature requiring 20 credits.

2. **Enhancements to Existing Functionality:**
   - A normalization function `normalizeId` is introduced to ensure consistent pipe IDs by removing any 'pipe-' prefix and converting them to lowercase.
   - The user credits system is improved. If a user does not have enough credits for a pipe, a `CreditPurchaseDialog` is shown instead of redirecting to an external URL for purchasing credits.

3. **User Interaction Improvements:**
   - A new component `CreditPurchaseDialog` is added. This component provides users with the option to purchase credits either through a monthly subscription (15 credits/m) or a one-time purchase (50 credits).
   - The `AccountSection` component now attaches the user’s ID and email as query parameters when redirecting to purchase URLs.

4. **User Data Handling:**
   - The `useUser` hook now includes a `refreshUser` function that explicitly refreshes the user’s credit information when needed.

Overall, the changes enhance the user experience by allowing users to handle credit purchases directly within the app interface, providing additional functionality with the new "timeline" pipe, and ensuring that user credit information is up-to-date and handled seamlessly.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 159 (67ee9c87f6766500fa904bce29f15aee4d547972)</summary>

The commit made significant updates to the project, notably restructuring and expanding the "pipes/timeline" module with a substantial amount of new files and components. Here's a summary of the main changes:

1. **New Files and Components**:
   - Added a variety of components under the "pipes/timeline/src/components" directory. These include UI components like buttons, inputs, cards, accordion, and a video component, among others.
   - Implemented several "hooks" into the "pipes/timeline/src/lib/hooks" directory for functionalities like copy-to-clipboard, debounce, health checks, and managing search history.
   - "ui" directory includes multiple components such as "accordion", "button", "calendar", "dialog", and more for building user interfaces.
   - Extensive set of utility files under "pipes/timeline/src/lib/utils.ts" for common functions needed across the application.
   - "actions" related to video handling were added in "pipes/timeline/src/lib/actions/video-actions.ts".

2. **Layouts and Configurations**:
   - A root layout file `layout.tsx` was added, configuring the global layout for the timeline application.
   - A global styles file `globals.css` was created, setting foundational styles using Tailwind CSS utilities.

3. **Added Functionality**:
   - Added gitignore, package configurations, and dependencies specific to the timeline module.
   - Configured ESLint and PostCSS, ensuring code quality and styling consistency.
   - Introduced TypeScript-specific files like "tsconfig.json" for type-checking.
   - Added Tailwind CSS configuration file "tailwind.config.ts" for styling the application.

4. **Removed or Modified Elements**:
   - Refactored and renamed some components and directories, indicating a restructuring of the codebase.
   - Removed Tauri-specific commands and configurations for the timeline view, transitioning it to a server-based application.
   - Changed server-side handlers to handle app icon fetching asynchronously.

Overall, these changes indicate a move towards enhancing the functionality and modularity of the timeline module, leveraging modern web technologies and best practices. This likely sets the groundwork for creating a standalone, feature-rich timeline component within the application ecosystem while simplifying or removing previously Tauri-managed processes.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 160 (57459a6c751dc35d450347a14392e4233d414b19)</summary>

The git commit made by Louis Beaumont on December 16, 2024, updates the `README.md` file. The changes involve modifying the text under the ScreenPipe project title and subtitle. The original description, which highlighted functionalities such as being an API for user desktop data, a sandboxed JS plugin system, and emphasizing quick deployment and monetization of desktop agents, has been altered. The new description focuses on building AI agents with full context, emphasizing that the project is open source, runs locally, is developer-friendly, and includes features like 24/7 screen, mic, and keyboard recording and control.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 161 (e084e0a960a674bcca4e55949bf34e8f45c12fd2)</summary>

The commit by the GitHub Actions Bot adds a changelog for version 0.17.8 to the repository. It involves two main changes:

1. A new file `0.17.8.md` is added under `content/changelogs/`, which includes a reference link to the full changelog comparing two commits (`adcb9..19f4c`).

2. Updates are made to the existing changelog file located at `screenpipe-app-tauri/public/CHANGELOG.md`. The previous content, which detailed improvements and fixes, is removed. This removed content included improvements to the pipe download user experience and fixes for self-hosted builds, node/browser compatibility, and search functionality issues. The file now only contains a reference link to the full changelog for the same commit range (`adcb9..19f4c`).
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 162 (19f4cbee1411f4e9ce7fa8c1cc98f6e77ab32b83)</summary>

The commit `19f4cbee1411f4e9ce7fa8c1cc98f6e77ab32b83` by Louis Beaumont updates two files:

1. **`.github/scripts/generate_changelog_md.sh`:** The script is modified to change the API model being used from `"gpt-3.5-turbo"` to `"gpt-4o-mini"`.

2. **`screenpipe-app-tauri/src-tauri/Cargo.toml`:** The version of the `screenpipe-app` package is incremented from `0.17.7` to `0.17.8`.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 163 (1ba5e68b8059d795079f61b9c96ba9521b2f57db)</summary>

The commit introduces several changes and new features to the screenpipe-app-tauri project:

1. **Subscription and Payment Handling**:
   - A new feature has been added to check existing subscriptions for users using Supabase. If a user tries to enable a paid pipe, the system now checks if they have already purchased it.
   - If a purchase is required and the user doesn’t have enough credits, they are redirected to a purchase link. If sufficient credits are available, the application attempts to make a purchase, deducting the necessary credits if successful.

2. **Refactoring and Code Organization**:
   - The `AccountSection` component has been moved to a separate file (`account-section.tsx`) for improved organization and readability.
   - The settings component (`settings.tsx`) has been updated to include the newly separated `AccountSection`.

3. **User Interface Enhancements**:
   - The UI in `account-section.tsx` now includes better organized sections for API key management, credits & billing, and developer options.
   - Users are informed of their credits and can refresh their credit information. There are options for purchasing credits via Stripe, including subscription models and one-time credit purchases.
   - Developer options, such as enabling Stripe Connect to sell pipes, are indicated as upcoming features.

Overall, this commit adds new subscription management functionality, improves the UI for account management, and cleans up the codebase by refactoring components into separate modules.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 164 (87ff7d796b14876c2b377acad1a05e2beb625e7a)</summary>

The commit with ID `87ff7d796b14876c2b377acad1a05e2beb625e7a` was made by Louis Beaumont on December 16, 2024. The update was to the `README.md` file, where a sentence was modified. The change added the phrase "deploy and monetize desktop agents in seconds" to the description of the software, which now reads:

- Old: "sandboxed js plugin system. keyboard and mouse control"
- New: "sandboxed js plugin system. keyboard and mouse control. deploy and monetize desktop agents in seconds"
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 165 (0469d45c2996b898f283149bc9aea7b3491b0265)</summary>

The git changes involve version updates for two Cargo.toml files. Specifically:

1. In the root `Cargo.toml`, the version of the package in the workspace has been updated from "0.2.16" to "0.2.17".
2. In `screenpipe-app-tauri/src-tauri/Cargo.toml`, the version of the "screenpipe-app" package has been updated from "0.17.6" to "0.17.7".

These changes likely indicate a minor release update for the application "screenpipe" referenced in the commit message, which mentions "release-app using screenpipe under vpn/firewall".
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 166 (a04ed4177bcb85a2d604538f2f2ab53492d4e62b)</summary>

The commit introduces changes primarily to the `Cargo.toml` file by:

1. Updating the version of the `hf-hub` crate from "0.3.0" to "0.3.2".
2. Modifying the `[patch.crates-io]` section to switch the `hf-hub` source from the repository `https://github.com/king-jingxiang/hf-hub` to `https://github.com/neo773/hf-hub` and enabling the "native-tls" feature.
3. Adding a note in the comments that highlights the use of a Chinese mirror for `hf-hub`, which might be necessary as the service is banned in China, and also mentions the inclusion of "native-tls".

This commit likely aims to improve TLS support and accommodate users in China by using a specific GitHub repository with desired features.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 167 (c820c8b4650f970fa565c7809c2668b6d32f41c2)</summary>

The git commit made by Louis Beaumont on December 15, 2024, addresses a fix related to the release of an app. Specifically, the change involves modifying the script `.github/scripts/generate_changelog_md.sh` to correct the command used to fetch the last published release. The correction involves adjusting the command to list releases for `mediar/screenpipe` instead of `screenpipe` alone. This update ensures the accurate retrieval of information about the most recent published release.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 168 (674c01f08ba8730a94a288970613482327e844dc)</summary>

The commit with the hash `674c01f08ba8730a94a288970613482327e844dc`, authored by Louis Beaumont, updates the `.gitignore` file. Specifically, it adds three new lines to the end of the file: the string "cn" followed by two blank lines. These changes are likely intended to ignore certain files or directories in the project that match the newly added patterns.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 169 (add293a9baae3abe52b8cd08ecc901f202bfb069)</summary>

The git commit `add293a` made several changes primarily focused on removing features related to the "search" functionality in a Tauri application. Here’s a summary of the changes made:

1. **SearchChat Component**:
   - A console log statement for `settings` was removed from `search-chat.tsx`.

2. **Search Page**:
   - The `page.tsx` file in the `screenpipe-app-tauri/app/search` directory was deleted. This file previously contained a component for rendering a search page with various functionalities like search history management.

3. **Header Component**:
   - The search menu item and its associated functionality were removed from `header.tsx`. This included removing a `DropdownMenuItem` for search and the `handleShowSearch` function.

4. **Cargo Files**:
   - The version of `screenpipe-app` was incremented from `0.17.5` to `0.17.6` in `Cargo.toml`.
   - Correspondingly, the version was updated in `Cargo.lock`.

5. **Commands**:
   - The `show_search` command was removed from `commands.rs` and its invocation from `main.rs`, indicating the feature for showing the search window was deprecated.
   - Adjustments were made to the `open_pipe_window` function to synchronize window handling logic.

6. **Tauri Configuration**:
   - The configuration settings related to the "search" window in `tauri.conf.json` were removed. This included window properties like dimensions and visibility.

Overall, these changes indicate a cleanup or deprecation of the search functionality within the Tauri application, paving the way for either a new approach to search or a strategic refocus away from this feature.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 170 (bea702b0c1e192375fc6957edaaa5e0ccda9ebec)</summary>

The commit `bea702b0c1e192375fc6957edaaa5e0ccda9ebec`, authored by "tribhuwan", addresses a timeout issue in the `screenpipe-audio` application related to downloading the ONNX Runtime on Windows platforms.

### Key Changes:
1. **Timeout Configuration**: 
   - Introduced a custom HTTP client with a timeout capability (300 seconds) using `reqwest::blocking::Client` to prevent timeout issues when fetching the ONNX Runtime zip file from GitHub.

2. **File Handling Enhancements**:
   - Added proper error handling when writing the downloaded file to disk.
   - Ensured all operations involved in fetching, writing, and extracting the ONNX Runtime zip file are done with error checks.

3. **Unzipping and File Management**:
   - Adjusted unzip command arguments to include `-o` for overwriting files during extraction.
   - Added logic to check for and remove existing ONNX Runtime directories before renaming the extracted files, ensuring a clean install process.

4. **Path Management**:
   - Used `std::path::Path` for managing and manipulating file paths.

These changes aim at enhancing robustness and ensuring that the installation process for the ONNX Runtime completes successfully without timing out or failing due to existing files or directories.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 171 (1f5b9d8d691527fd1e326fe808eb0f41f7e0c50e)</summary>

In the commit `1f5b9d8d691527fd1e326fe808eb0f41f7e0c50e`, authored by Tribhuwan, several changes were made to address connection and scrolling issues in a timeline UI:

1. **Introduction of Messages**: A new state variable `message` is introduced to display status messages to users while connecting to the server and while data is being collected.
   
2. **Connection Handling Improvements**:
   - A `connectionTimeout` was implemented to handle situations where the connection might not be established within a specific time frame (5000 ms). If the connection is not open after this duration, an error is set, and the event source is closed.
   - On successful opening of the connection (`onopen` event), error messages are cleared, and the loading state is set accordingly. 

3. **Scrolling Enhancements**:
   - A `containerRef` is added to manage `wheel` event listeners more effectively. The scroll handling was optimized by adding `preventDefault()` to avoid undesired scrolling behaviors within specific UI panels.
   - A `useMemo` hook is used for `handleScroll` to throttle scrolling, improve performance, and ensure proper stop propagation when necessary.

4. **User Feedback UI**:
   - The UI now shows messages like "connecting to the server..." and "please wait, we are collecting data to stream timeline..." to inform users about the current app status.
   - A loading spinner (`Loader2`) is displayed when a message is active.

5. **Minor Change in Audio Transcript**: 
   - The audio transcript component was updated to handle potential `undefined` values safely by using optional chaining (`currentFrame?.timestamp`).

Overall, these changes enhance the robustness of the timeline UI by improving connection stability, providing clearer user feedback, and refining scroll interactions.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 172 (adcb99195e26a14002993e38f1f8fd0b33fe2593)</summary>

The Git commit addressed two main changes:

1. **Version Update**: The version number in the `Cargo.toml` file for the `screenpipe-app-tauri` was incremented from `0.17.4` to `0.17.5`.

2. **Dependency and Code Adjustments**:
   - In `screenpipe-vision/Cargo.toml`, the Windows-specific dependency on the `xcap-win` (custom branch `fix-pr` from a Git repository) was replaced with a standard versioned dependency `xcap` at `0.0.12`.
   - Code modifications in several files (`capture_screenshot_by_window.rs`, `core.rs`, `monitor.rs`, and `utils.rs`) changed how the `xcap` library was imported. Previously, separate imports for different operating systems were specified (`xcap_macos` for macOS, `xcap_win` for Windows, and `xcap` for Linux). This was simplified to import `xcap` for all operating systems except macOS, thereby removing the `xcap_win` imports for Windows.

Overall, the commit aimed to fix build issues, particularly for the Windows platform, by simplifying and consolidating the use of certain dependencies and imports.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 173 (2b8e8479c0f678d16b15dd41145d5d64d4cb4502)</summary>

The commit made by Louis Beaumont updates the `screenpipe-app-tauri` project to version 0.17.4 and addresses some configuration issues related to dependency imports for different operating systems within the `screenpipe-vision` project. 

Here's a summary of the changes:
1. **Version Update**:
   - The version in `screenpipe-app-tauri/src-tauri/Cargo.toml` is incremented from `0.17.3` to `0.17.4`.

2. **Dependency Adjustments**:
   - In `screenpipe-vision/Cargo.toml`:
     - For Windows targets, changes the xcap dependency source from a specific revision to a branch (`fix-pr`) using a different repository: from `https://github.com/mediar-ai/xcap` to `https://github.com/nashaofu/xcap`.
     - Splits the `xcap-macoswin` dependency into `xcap-macos` and `xcap-win`, recognizing them separately for macOS and Windows.

3. **Code Conditional Compilation Adjustments**:
   - In several source files within `screenpipe-vision`, changes related to conditional compilation are made to reflect these dependency changes:
     - The previous imports using `xcap_macoswin` are replaced with platform-specific imports: `xcap_macos` for macOS and `xcap_win` for Windows.
     - These changes occur in `capture_screenshot_by_window.rs`, `core.rs`, `monitor.rs`, and `utils.rs`.

Overall, the update refines the use of platform-specific dependencies to improve build compatibility across targeted operating systems.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 174 (6d03f291f102a034e89225ac7d8dfba7dfbae269)</summary>

The commit made by Louis Beaumont on December 14, 2024, modifies the project files to address window focus issues on Windows and update package dependencies. Here's a summary of the changes:

1. **Version Update:**
   - The version of the "screenpipe-app" package was incremented from `0.17.2` to `0.17.3` in the `Cargo.toml` file of the `screenpipe-app-tauri` project.

2. **Dependency Updates:**
   - In the `screenpipe-vision` project, the dependency for the `xcap` library was modified:
     - For Windows and macOS, a new consolidated package `xcap-macoswin` is being used, pointing to a GitHub repository with a specific revision (`965bc99`).
     - For Linux, the import continues to use the regular `xcap` package.

3. **Codebase Changes:**
   - Adjustments were made in multiple source files (`capture_screenshot_by_window.rs`, `core.rs`, `monitor.rs`, `utils.rs`) within the `screenpipe-vision` project:
     - The import of the `xcap` package was unified for macOS and Windows into `xcap_macoswin`.
     - The conditional compilation attributes were changed accordingly to support the above import adjustments. 

These changes collectively aim to resolve focus issues on Windows while also refining how platform-specific dependencies are managed by the project.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 175 (7787edfbf82cae5176929483b25deef6ed749401)</summary>

The commit `7787edfbf82cae5176929483b25deef6ed749401` by Louis Beaumont deletes the file named `cn`. The file had executable permissions (mode 100755) before it was removed.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 176 (47309f8ec128588f612d59265add7bd8d73d8f91)</summary>

The commit `47309f8ec128588f612d59265add7bd8d73d8f91` introduces a new changelog for version 0.17.2. The main changes documented in this version include:

### Improvements:
- **Improved Pipe Download User Experience (UX):** Enhancements were made to the pipe download process to provide a better user experience.

### Fixes:
- **Self-Hosted Builds Fixes:** Issues with self-hosted builds have been addressed to ensure a smoother deployment process.
- **Compatibility Improvements:** Screenpipe-js has been updated to ensure compatibility with both Node.js and browsers.
- **Search Functionality Fixes:** Resolved problems with the search feature to provide more accurate search results.

A new file `0.17.2.md` was added to `content/changelogs/`, and some redundant lines were removed from the `screenpipe-app-tauri/public/CHANGELOG.md`, reflecting these updates.

For the full list of changes, readers can refer to the linked full changelog comparison [cabec..d24be](https://github.com/mediar-ai/screenpipe/compare/cabec..d24be).
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 177 (d24be50be7085ec4017761139d50a9139a78f851)</summary>

The commit with hash `d24be50be7085ec4017761139d50a9139a78f851` introduces several changes aimed at improving the user experience for downloading pipes in the Screenpipe application.

1. **Pipe Download UX Improvements**:
   - A progress bar feature was added to provide users with real-time feedback during the pipe downloading process. 
   - The initial toast message now includes a progress bar and a status message "starting download...".
   - A periodic update function has been implemented to incrementally update the progress bar with the message "installing dependencies...".
   - Upon completion, the toast updates to indicate that the pipe has been downloaded successfully with the message "completed successfully".

2. **Version Update**:
   - The version of the `screenpipe-app` was incremented from `0.17.1` to `0.17.2` in both the `Cargo.toml` and `Cargo.lock` files, reflecting the new changes made in this release. 

These changes enhance the interactivity and visibility of the pipe download process, providing a smoother and more informative user experience.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 178 (4253eb08f12317f09fcb841fe4ad97b58fda6422)</summary>

The commit by Neptune addresses a fix for self-hosted builds in a GitHub Actions workflow, specifically for the `.github/workflows/release-app.yml` file. The changes include:

1. **Setup Python**: 
   - Adds a step to use the `actions/setup-python@v5` action when building for the `x86_64-pc-windows-msvc` target. 
   - Specifies the Python version to be `3.13`.

2. **Python Package Management**:
   - Introduces a command to upgrade `ensurepip`, which prepares Python environments to have the `pip` tool installed (`python -m ensurepip --upgrade`).
   
3. **Dependency Installation**:
   - Enhances the logic for copying MKL libraries by attempting to install `intel-openmp` via pip into a directory called `omp` if existing DLLs are not found.

Overall, these changes aim to improve the handling of dependencies and ensure the successful building of MSVC target builds in self-hosted environments.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 179 (02c04e3bad63e6d67133c20c25eedc6374ec2bcb)</summary>

The commit by Louis Beaumont introduces multiple changes to improve compatibility between node and browser environments for the `screenpipe-js` package. Here's a summary of the key changes:

1. **Configuration Changes:**
   - Updated `next.config.ts`: Modified the Webpack configuration to utilize aliases for `@screenpipe/js` dependencies, directing server-side to `node.js` and client-side to `browser.js`.
   - Updated package version of `@screenpipe/js` from `0.1.10-beta.11` to `0.1.10-beta.18` in `package.json`.

2. **Dependencies and Build:**
   - Added new `bower.ts`, `next.ts`, `node.ts`, and `types.ts` files for better separation between node and browser-specific functionalities.
   - A new `package-lock.json` was added to manage dependencies more effectively.
   - Updated `tsconfig.json` to include new types and align with new module structures.

3. **Code Modifications:**
   - Within `screenpipe-js`, refactored parts of `main.ts` splitting node-specific implementations into `node.ts`.
   - Implemented `browser.ts` specifically for functionalities that need to run in a browser environment, and made appropriate exports in `main.ts`.
   - Removed `index.d.ts`, suggesting type definitions moved into separate files like `types.ts`.

4. **Settings Handling:**
   - Created `route.ts`, a new route API managing application settings via `GET` and `PUT` requests, forcibly set to Node.js runtime.
   - Deleted `settings-actions.ts`, consolidating setting management directly with API handling and utility functions.

5. **Enhanced Capability Management:**
   - Reorganized capability files to provide expanded access, e.g., altering window permissions in `main.json` and updated schema capability definitions.

6. **Miscellaneous:**
   - Optimized console debugging messages in search functionality.
   - Usage and import of the `pipe` object were adjusted to reflect these charges, ensuring compatibility and modular separation.

In sum, the commit refactors and reorganizes the `screenpipe-js` library for better environment compatibility while updating dependencies and consolidating settings management approaches.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 180 (bf3c83885aa9575e520a68a0cd94724fb4d6f6f0)</summary>

The commit by Louis Beaumont updates the `CONTRIBUTING.md` file. The change involves a modification to the message intended for potential contributors. The revised sentence now specifies a preference against contributions from individuals who do not use or will not use the product and are only interested in the bounties. The section reflects a slight clarification of the project's contribution policy.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 181 (81bee69f6e6767541d143d9cf1aefbeb865d682f)</summary>

The commit with ID 81bee69f6e6767541d143d9cf1aefbeb865d682f by Louis Beaumont updates the `CONTRIBUTING.md` file. The change adds a line advising potential contributors that preference is given to those who actively use the product. It discourages contributions from individuals interested solely in collecting bounties.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 182 (975cbf129c3448ab36d06b562aa50dbcd07934eb)</summary>

The commit `975cbf1` by Louis Beaumont on December 14, 2024, includes changes to the `settings-actions.ts` file within the search pipe module. The key updates are:

1. **Code Style Changes**:
   - Converted single quotes to double quotes for strings.

2. **Uncommented Code**:
   - Functionality to manage and use settings was restored by uncommenting sections of code, specifically the logic to get and clone settings with default values.

3. **Error Handling**:
   - Added checks to ensure the `settingsManager` object exists before attempting operations. If not found, an error is thrown.

4. **Refactored Logic**:
   - Code now creates a deep clone of settings to avoid prototype pollution, and functions for obtaining and resetting settings handle errors more robustly by logging errors and returning default settings when necessary.

These changes improve the robustness and reliability of settings management within the search module.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 183 (cabec012eaec63d8c96a46ef709527ef06dafd06)</summary>

The git commit `cabec012eaec63d8c96a46ef709527ef06dafd06` adds a changelog for version 0.17.1 of the project. It introduces new features, including H.265 support for macOS to enhance video compression and ensures compatibility with Next.js for the screenpipe-js framework. Improvements have also been made to the search functionality and speaker argument handling in screenpipe-js. No specific bug fixes were identified in this update. The changes were documented in two files: a newly created changelog file `content/changelogs/0.17.1.md` and an update to an existing changelog `screenpipe-app-tauri/public/CHANGELOG.md`.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 184 (34f5e8025b7f5206352244217027b698a2cd527f)</summary>

The commit contains several updates and improvements to the project:

1. **Version Updates:**
   - Updated the version in `Cargo.toml` from `0.2.15` to `0.2.16`.
   - Updated the version in the `screenpipe-app-tauri/src-tauri/Cargo.toml` from `0.17.0` to `0.17.1`.

2. **Debug Logging Removal:**
   - Removed `console.log` debugging statements from `pipe-config-form.tsx` and `use-user.ts`.

3. **Pipe Store Logic Changes:**
   - Added a `Loader2` component from `lucide-react`.
   - Added `port` property to the `Pipe` interface.
   - Improved state update logic for enabling/disabling pipes and ensured consistency by fetching fresh state from the server.
   - Added debug logs for clarifying the fetching and selection of pipes.

4. **UI Enhancements:**
   - Added a new feature in `pipe-store.tsx` to open pipes as standalone apps using a new button, utilizing the `open_pipe_window` command.

5. **Video Component:**
   - Changed the `source` type in `video.tsx` from `video/mp4; codecs=hvc1` to `video/mp4`.

6. **New Command:**
   - Added a new Tauri command `open_pipe_window` in `commands.rs` for opening a pipe window with specific dimensions and title.

7. **Binary Changes:**
   - The binary files `ui_monitor-aarch64-apple-darwin` and `libscreenpipe_arm64.dylib` were modified.

Overall, the changes improve the application's functionality, cleaning up console logs, enhancing UI interactions, and updating binary files while providing more robust management of application state and new capabilities.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 185 (294eb22d4d9608211f8f1a0df1fade0a9bb93d1c)</summary>

The commit `294eb22d` introduces new functionality to support H.265 video format (also known as HEVC) specifically for macOS. This change is made in the `screenpipe-app-tauri/components/video.tsx` file. The alteration consists of modifying the `<video>` element's `<source>` tag, updating the `type` attribute from `"video/mp4"` to `"video/mp4; codecs=hvc1"`. This change allows the video component to handle H.265 encoded videos.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 186 (e00e5224776fbdd051270e5497593a82776c0441)</summary>

The commit introduces changes to two TypeScript files within a codebase. 

1. **Component: `search-chat.tsx`**
   - Inside the `SearchChat` function, where certain conditions are met, the content type is explicitly set to "audio" by adding `setContentType("audio")` in two places. 
   - A new console log statement, `console.log("content type", contentType);`, is added to log the current content type to the console.

2. **Component: `pipe-store.tsx`**
   - A new core pipe is added to the `corePipes` array. This new object represents a "search" functionality that allows searching through screen recordings and audio transcripts using AI. It has attributes like `id`, `name`, `description`, `url`, and indicates that it requires 0 credits and is not paid.

Overall, these changes address an issue with the search functionality and update the pipe store configuration to include a new search feature.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 187 (6e32e5a577ffbb83d543d8b318abfedea6af9bce)</summary>

This commit includes several changes made to fix speaker argument handling in the `screenpipe-js` project and updates to related project files. Here's a summary of the changes:

1. **`screenpipe-js` Changes:**
   - Updated `screenpipe-js/main.ts` to handle `speakerIds` more robustly. It now checks if the `value` is not an empty string before appending it to query parameters. Also, when appending `speakerIds`, it checks if the array has more than zero elements before joining it into a comma-separated string.
   - Updated the version of `screenpipe-js` in its `package.json` file from `0.1.10-beta.9` to `0.1.10-beta.11`.

2. **Dependency and Version Updates:**
   - In `pipes/search/package.json`, the version of the `@screenpipe/js` dependency is updated to `0.1.10-beta.11`.
   - The version of `screenpipe-app` is incremented from `0.16.9` to `0.17.0` in the `Cargo.lock` file.

3. **Capability Changes:**
   - Added the ability `core:webview:allow-internal-toggle-devtools` to `screenpipe-app-tauri/src-tauri/capabilities/main.json`.
   - Corresponding change was made in `screenpipe-app-tauri/src-tauri/gen/schemas/capabilities.json`.

4. **Binary File Updates:**
   - There are changes in the binary files `bun.lockb`, `ui_monitor-aarch64-apple-darwin`, and `ui_monitor-x86_64-apple-darwin`, but specific content changes are not detailed as these are binary files.

Overall, the commit focuses on fixing handling for the `speakerIds` parameter in `screenpipe-js`, updating package versions, and modifying capabilities for the `screenpipe-app`.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 188 (38149b487f5cabe4ad3e0945ea98c875a8436061)</summary>

The commit `38149b487f5cabe4ad3e0945ea98c875a8436061` by Louis Beaumont introduces a variety of changes to the project, mainly focusing on making the screenpipe-js Next.js client/server compatible. Here's a summary of the key changes:

1. **GitHub Action (`release-app.yml`)**:
   - Updated functions from `println!()` to `Write-Host` in the script responsible for handling DLLs, for better compatibility with PowerShell.

2. **Project Restructuring**:
   - Renamed many files to move usages of 'pipe-anthropic-computer-use-meeting-assistant' and 'pipe-keyword-notification' to an 'archive' directory. This indicates an archiving or deprecation of these pipes.

3. **New `search` Pipe**:
   - Created a new Next.js project under `pipes/search` with base files like `README.md`, `package.json`, and various TypeScript and JSON configuration files.
   - Includes key setup for Next.js such as `.gitignore`, `tailwind.config.ts`, and `tsconfig.json`.
   - Introduced various UI components such as `Accordion`, `Badge`, `Button`, `Input`, `Dialog`, `Tooltip`, all following Tailwind CSS styling.
   - Implemented functionalities like video handling, SQL autocomplete, and user settings management, which are tailored for the Next.js application.

4. **Screenpipe-js Module Enhancements**:
   - Added a new file `inbox-server.ts` to manage server actions for inbox messages.
   - Defined types and setup for the @screenpipe/js module, providing detailed types for different content (OCR, Audio, UI) and settings management.
   - Refactored `main.ts` to better support Node.js and browser environments by dynamically importing Node.js specific modules and providing Node.js specific functionalities only when applicable.
   - Improved TypeScript configuration with a `tsup` setup to build both ESM and CJS formats, along with declarations.

5. **Packed New Functionalities & Fixes**:
   - Adjusted various configurations for improved compatibility and better module resolution.
   - Emphasized on server compatibility, ensuring functionalities like settings management and input control can gracefully handle execution environments (browser vs. Node.js).
   - Added a modernized TypeScript setup to build and manage the library more effectively.

In summary, this commit enhances the project by restructuring it for Next.js compatibility, archiving old components, adding new features with modern UI components, and refining the TypeScript configuration for better library management.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 189 (1d1fe9ab2f87902d03862db66218b80067161f75)</summary>

The commit introduces a new `.gitattributes` file, which is used to configure how GitHub's Linguist tool detects and categorizes files in the repository. The changes in the commit are as follows:

1. **Primary and Secondary Language Detection**:
   - Sets Rust (`*.rs` files) as the primary language for detection.
   - Sets TypeScript (`*.ts` files) as a secondary language that is still detectable.

2. **Vendor and Build Output Ignoring**:
   - Marks several directories (e.g., `dist/*`, `target/*`, and `node_modules/*`) as vendored code, causing them to be ignored.

3. **Documentation and Configuration Files**:
   - Specifies that markdown (`*.md`) and text (`*.txt`) files are documentation, which affects how they are treated in language statistics.
   - Sets various configuration files (`*.toml`, `*.json`, `*.yaml`, `*.yml`) to be non-detectable by Linguist, meaning they won't contribute to the language breakdown.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 190 (ddac89a4d2bcf771a9ed0d2952466baeaa36afd9)</summary>

The commit `ddac89a4d2bcf771a9ed0d2952466baeaa36afd9` adds a changelog for version 0.17.0. It outlines improvements, specifically enhanced focus control by ignoring unfocused windows and improved audio selection by automatically switching to audio when a speaker is selected in search. The commit also updates the `CHANGELOG.md` file in the `screenpipe-app-tauri` project, removing previous entries like new features and fixes, focusing instead on the same improvements listed in the new changelog document. A new file `0.17.0.md` is created in the `content/changelogs/` directory to log these changes.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 191 (f492aa0faa8cfd65ce1483ea1dc207b1e15654d8)</summary>

The Git commit introduces a change made by Louis Beaumont to the project on December 13, 2024. The main update involves removing a file from the codebase. Specifically, the file `window-properties.rs` located in the `screenpipe-vision/examples` directory has been deleted. This file was responsible for examining properties and states of windows such as title, application name, identifiers, minimized status, visibility, dimensions, etc., in an asynchronous loop using the Tokio library and a dependency `xcap`. The commit message indicates a default behavior change for the release application, where unfocused windows are now ignored, suggesting that this deletion is part of that adjustment in the application's behavior.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 192 (db2232041985df05a73d6f8bac8fac8b1000daa7)</summary>

This commit introduces the following changes to the project:

1. **Features and Enhancements:**
   - Added support for capturing windows that are not focused by default, controlled via a new CLI flag `capture_unfocused_windows`.
   - Improved audio transcription engine handling by defaulting `screenpipe-cloud` to `deepgram`. 
   - Introduced platform-specific window filters to ignore system windows for MacOS, Windows, and Linux.
   - Implemented window property monitoring with a new example script (`window-properties.rs`) to list all window details periodically.
   - Added the `once_cell` crate as a new dependency in `Cargo.toml` for efficient lazy static initialization.

2. **Code Refactoring and Clean-up:**
   - Removed obsolete CLI option `--save-text-files` and related code, simplifying file handling.
   - Consolidated window capture logic with a new `WindowFilters` struct and revised the function `capture_all_visible_windows`.
   - Updated and refactored core logic within several modules (`core.rs`, `capture_screenshot_by_window.rs`), enhancing clarity and maintainability.
   - Affected version bumps in `Cargo.toml` files for packages and improved version tracking (`screenpipe-app` from 0.16.9 to 0.17.0, `workspace` from 0.2.14 to 0.2.15).

3. **Dependencies and Package Management:**
   - Added a more specific xcap dependency for MacOS that references a commit on GitHub, enhancing cross-platform support.

This commit focuses on improving functionality related to window capture, specifically targeting performance and usability enhancements.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 193 (a5e04bc1455b00eddd8a928016078d744414beac)</summary>

This Git commit introduces several changes to the `screenpipe-app` repository:

1. **Functionality Update in `search-chat.tsx`:**
   - A new `useEffect` hook is added to automatically set search result filters to prioritize audio when any speakers are selected. This ensures that when a speaker is selected, the application focuses on audio results by disabling the `ocr` and `ui` options.
   - A tooltip has been added to inform users that selecting speakers will limit the search results to audio only. This tooltip appears next to the "speakers" label, providing guidance on the functionality.

2. **Version Update:**
   - The `screenpipe-app` version has been incremented from `0.16.8` to `0.16.9` in the `Cargo.lock` file, indicating a minor release, likely to address a feature enhancement or bug fixes.

3. **Binary File Changes:**
   - Several binary files have been updated, including:
     - `bun.lockb`
     - `ui_monitor` binaries for different architectures in both `src-tauri` and `screenpipe-vision` directories
     - `libscreenpipe_arm64.dylib` in the `screenpipe-vision/lib` directory
   - These changes likely involve compiled binaries or dependency updates relevant to the application's functionality across different platforms or architectures.

Overall, this commit primarily focuses on enhancing the audio search feature tied to speaker selection within the application's search functionality.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 194 (6ff4cf88cf90953c34b0397f68daa62b1cb10154)</summary>

This Git commit made a modification to a GitHub Actions workflow file, specifically `.github/workflows/release-app.yml`. The change involved updating the shell used for a particular step in the workflow. The shell was changed from `pwsh` (PowerShell Core) to `powershell` (Windows PowerShell) for the step named "Copy library for MKL". This change affects the execution of a command when the `matrix.tauri-args` is set to `--target x86_64-pc-windows-msvc`.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 195 (56e02cab0ed8edbfc0bd162d44de8f9c08d07d10)</summary>

In this commit, Louis Beaumont has removed a block of outdated instructions from the `pre_build.js` script in the `screenpipe-app-tauri/scripts` directory. Specifically, the removed code set several environment variables (`FFMPEG_DIR`, `OPENBLAS_PATH`, `LIBCLANG_PATH`, and `PATH`) for the Windows platform. The remaining code now simply logs 'bun install' and ensures that if `GITHUB_ENV` is not set, 'bun tauri build' is logged as well.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 196 (3f520b8b0586b53ea1a27f9444e153c55e00a34c)</summary>

The git commit `3f520b8b0586b53ea1a27f9444e153c55e00a34c`, authored by Louis Beaumont, addresses issue #955 with the following changes:

1. **`screenpipe-app-tauri/bun.lockb`**: This binary file was updated, but the specific changes are not visible due to its binary nature.

2. **`screenpipe-app-tauri/lib/hooks/use-user.ts`**: Added a condition to return early from the function if the `userData.id` has not changed compared to the existing `user?.id`. This prevents unnecessary updates if the user data remains the same.

3. **`screenpipe-app-tauri/src-tauri/Cargo.lock`**: Updated the version of the `screenpipe-app` package from `0.16.6` to `0.16.8`. This likely reflects changes in dependencies within the Cargo.lock file.

4. **`screenpipe-app-tauri/src-tauri/Cargo.toml`**: Updated the version of the `screenpipe-app` package from `0.16.8` to `0.16.9`, indicating a new package release or a minor update.

These changes primarily consist of version updates and an optimization to avoid redundant state updates in the `use-user.ts` file.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 197 (4ea536f0886394da5a1f06ccba67c4a512a3b30c)</summary>

The commit `4ea536f0886394da5a1f06ccba67c4a512a3b30c` by Louis Beaumont, dated Thu Dec 12 13:43:45 2024, removed a section of the GitHub Actions workflow configuration file `.github/workflows/release-app.yml`. Specifically, it deleted the steps responsible for setting up Python for a certain job within the workflow. The removed steps included configuring the Python setup using `actions/setup-python@v5`, setting the Python version to 3.11, specifying the architecture as x64, and enabling pip caching based on dependency files like `requirements.txt` and `pyproject.toml`. The removed setup was conditional, executed only when `matrix.tauri-args` equaled `--target x86_64-pc-windows-msvc`.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 198 (838703841f8b5c0d08c3836fd72af1757a9404f6)</summary>

The commit updates the logic for determining if audio is active in the `screenpipe-server/src/server.rs` file. Previously, audio was considered active if it had been captured within the last 5 seconds. The new code introduces a "grace period" of 2 minutes from the application's start time. During this grace period, audio is automatically considered active, regardless of the capture status. After the grace period, the previous logic applies, and audio is considered active only if captured in the last 5 seconds. This change likely aims to prevent false negative audio statuses shortly after the application starts.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 199 (e48bbb251fc48605df1b8512c243b84748740f31)</summary>

The commit, authored by Louis Beaumont, addresses a fix in the "release-app" workflow specifically for a "screenpipe cloud" issue. Here's a summary of the changes made:

1. **GitHub Workflow Updates (`release-app.yml`)**:
    - The shell used for a step was changed from `powershell` to `pwsh`.
    - The logic for copying the MKL (Math Kernel Library) DLLs was modified:
        - A check is added to use existing DLLs from a temporary directory ("C:/temp_mkl_setup") if available, otherwise it defaults to installing `intel-openmp` via pip.
        - There's a verification step to ensure the DLLs are copied, throwing an error if no DLLs are found in the target directory.

2. **Version Update (`Cargo.toml`)**:
    - The version of `screenpipe-app` was incremented from "0.16.6" to "0.16.8".

3. **Sidecar Logic Modification (`sidecar.rs`)**:
    - The logic for selecting the audio transcription engine was updated. If the engine is "screenpipe-cloud", it sets the model to "deepgram" instead of using the provided string.

These changes are aimed at enhancing the build process and ensuring the application's functionality with "screenpipe cloud" is correct.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 200 (7c39f3ef44e38b7b8ad6eafb2896959be51a83e8)</summary>

This commit introduced several important changes to the project:

1. **GitHub Actions Workflow**: 
    - Added an environment variable (`RUNNER_TOOL_CACHE`) to the `release-app.yml` workflow for Windows build settings. This variable appears to specify the tool cache directory for the GitHub runner.

2. **Dependency Management**:
    - Updated `package.json` to add a dependency on `@tauri-apps/plugin-http`. 
    - Updated `Cargo.lock` to add several new dependencies related to HTTP capabilities, such as `cookie_store`, `data-url`, and `percent-encoding`. 
    - Updated the version of `screenpipe-app` from `0.16.2` to `0.16.6`.

3. **Source Code Changes**:
    - Imported and used the `fetch` function from the `@tauri-apps/plugin-http` in a TypeScript file (`use-user.ts`).
    - Modified Rust source files to improve code formatting and style, such as removing trailing whitespace, and using consistent formatting for match statements and loops.

4. **HTTP Permissions**:
    - Updated permissions to include configurations for HTTP operations in `main.json`, `acl-manifests.json`, and related schemas. These allow HTTP requests with a wide range of methods across all domains (e.g., `http://**/*` and `https://**/*`).

5. **Configuration Updates**:
    - Added HTTP plugin initialization to the main Tauri application in `main.rs`.
    - Improved code structure by rearranging module imports in `main.rs`.

6. **File Formatting**:
    - Generally improved JSON formatting for readability by reformatting arrays and objects consistently across configuration files, such as `tauri.conf.json`.

Overall, the changes seem to improve HTTP functionality and permissions, refine build configurations, and enhance the code quality and formatting across the project.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 201 (b42787b78cfb1049ff67e62524c1ba9348f5ebc7)</summary>

The commit made by Louis Beaumont updates the GitHub Actions workflow file for the `release-app` process specifically targeting Windows builds:

1. **Python Setup:**
   - The name of the setup step for Python is capitalized from "setup python" to "Setup Python".
   - The Python version used is downgraded from "3.13" to "3.11", presumably to use a more stable version.
   - Additional configurations for Python are added:
     - The architecture is explicitly set to 'x64'.
     - Pip caching is enabled to optimize dependencies installation, and cache paths are specified for `requirements.txt` and `pyproject.toml`.

2. **Tauri Action Version:**
   - The version of `tauri-apps/tauri-action` used in the build steps is updated from `v0.5.16` to `v0.5.17`.

Overall, these changes focus on improving the setup and build process for Windows by making the Python environment more stable and the Tauri apps action more up-to-date.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 202 (d5a907c7ce620d5287cfc6fb0ceddafec8d68670)</summary>

The commit by Louis Beaumont introduces several changes across different components of the project, focusing primarily on feature additions and improvements. Here's a summary of the changes:

1. **GitHub Actions Workflow**:
   - A setup step for Python 3.13 was added to the `release-app.yml` workflow file specifically for the Windows MSVC target.

2. **AI Proxy**:
   - Introduced a new `RateLimiter` class using a `DurableObject` in the Cloudflare Workers script (in `index.ts`). This class tracks and enforces rate limits for incoming requests based on their endpoint paths.
   - The script now checks the rate limiter before processing requests and responds with a rate limit error (HTTP 429) when exceeded.
   - Error handling was improved during API requests, changing the logging and response message handling.
   - Additional response properties such as `error` and `retry_after` were added for clarity.

3. **Wrangler Configuration**:
   - Updates in `wrangler.toml` include a new durable object binding for the rate limiter.
   - Added a migration tag `v1` for the new `RateLimiter` class.

4. **Tauri Application**:
   - Version bump of the `screenpipe-app` from `0.16.4` to `0.16.5` in the `Cargo.toml` file to reflect updates.

5. **Screenpipe Server Changes**:
   - Improved error handling in `record_audio` function within `core.rs`. The script now checks if `current_transcript` is non-empty before updating `previous_transcript`.

6. **Binary Files**:
   - Two binary files were updated, `ui_monitor-aarch64-apple-darwin` and `libscreenpipe_arm64.dylib`, but the specific changes weren't detailed (binary diff).

Overall, the commit fixes the audio processing logic, introduces a rate limiting feature for certain API endpoints, updates dependencies for Windows builds, and bumps the app version to reflect these changes.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 203 (30800edb42f365882f3e55ff1b59e5166996ba22)</summary>

The recent git commit by Louis Beaumont includes two main changes:

1. **Update to GitHub Workflow**: In the `release-app.yml` file located in the `.github/workflows` directory, the command for installing the `intel-openmp` Python package has been modified. The original command used a specific path to the Python executable (`C:\Users\screenpipe-windows\AppData\Local\Microsoft\WindowsApps\python.exe`), while the updated command uses a more generic `python`, likely relying on the system's PATH variable to locate the Python interpreter.

2. **Version Bump in Cargo.toml**: In the `Cargo.toml` file located in `screenpipe-app-tauri/src-tauri/`, the version of the `screenpipe-app` has been incremented from `0.16.3` to `0.16.4`. This indicates a new release or update to the application.

These changes suggest streamlining the setup process in the workflow and a minor version update for the application.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 204 (b5758c34eaca55349886ebd554bf000856d9eb1f)</summary>

The commit with ID `b5758c34eaca55349886ebd554bf000856d9eb1f`, authored by Louis Beaumont on December 12, 2024, introduces a new feature to the AI proxy codebase. Specifically, it adds the ability to specify a sample rate for audio processing.

The change involves modifying the `screenpipe-actions/ai-proxy/src/index.ts` file. A new variable, `sampleRate`, is extracted from the request headers. If the `sample_rate` header is not provided, a default value of `16000` is used. This `sampleRate` value is then appended as a query parameter to the Deepgram API URL used for audio processing, enhancing the existing query parameters that determine the language detection.

</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 205 (e1bd8c77fcbc76db0598e2616b5c51fcfc620839)</summary>

The commit updates the function `transcribe_with_deepgram` in the Rust file `stt.rs`, which is part of the `screenpipe-audio` project. The changes made involve:

1. Adjusting the fallback mechanism for setting the `sample_rate` in the `WavSpec` structure:
    - Previously, for any non-88.2kHz sample rates, the code divided the sample rate by 3. This calculation is now replaced by directly using the `sample_rate` value. However, the exception for 88.2kHz is still in place, where it is set to 16000.
  
2. Modifying the query parameters for a request:
    - The string containing parameters for a request has been updated. An additional parameter `sample_rate=16000` was added to what was initially `model=nova-2&smart_format=true`.

These changes aim to improve the compatibility and functionality of the Deepgram speech-to-text integration.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 206 (92032227c2f4e2e8a44649965f495e14266b8d6b)</summary>

The commit titled "fix ffmpeg darwin arm64 (#947)" by user `neo773` made changes to the `install.sh` script to improve the installation process for `ffmpeg` on macOS systems, specifically adding support for ARM64 architecture. Here are the key updates made in the changes:

1. **Architecture Detection**: The script now checks the architecture of the system (`aarch64` for ARM64 or default).

2. **Download URL Update**: It uses different URLs to download the appropriate `ffmpeg` binary depending on whether the system is ARM64 or AMD64. The URLs point to `ffmpeg.martin-riedl.de` instead of the previous default.

3. **Download and Extraction**: The script now includes error handling during downloads and extraction of the `ffmpeg` binary, displaying messages for failures and exiting the script if necessary.

4. **Code Signing Verification**: Added a step to verify the code signature of the `ffmpeg` binary on macOS. If the binary is not signed or has an invalid signature, the user is prompted whether to continue the installation.

5. **Binary Execution Verification**: Verifies that the `ffmpeg` binary runs by checking for its version, and if successful, outputs the detected version.

6. **Binary Placement**: The binary is moved to `"$HOME/.local/bin/"`, ensuring it is added to the user's path.

Overall, these changes make the installation process more robust, especially on macOS ARM64 devices, by ensuring the correct binary is downloaded and verified.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 207 (fee0c97f2dcd1360c5b5a75a28c3100b6bc725a8)</summary>

The commit identified by hash `fee0c97f2dcd1360c5b5a75a28c3100b6bc725a8`, authored by Louis Beaumont, deletes a file named `cn`. The file was previously a binary file with mode `100755`, indicating it was likely an executable.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 208 (006bbab7595a4aeacefcb9ac3d05cd680bf98239)</summary>

The commit made by Louis Beaumont adds a "feat" (feature) to `install.sh` that involves incorporating color. Additionally, the changes in the commit update the command used to run the installation script in both `README.md` and `getting-started.mdx` documentation files.

Specifically, the `curl` command used to download and execute the installation script has been changed from `curl -fL` to `curl -fsSL`. This change likely improves error handling and silence execution of the command, as the `-s` (silent) and `-S` (show error) flags are added to the existing `-f` (fail) and `-L` (location) options.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 209 (e66aaf624b12ae188764b4e35fe6c8cdc5081bbc)</summary>

The commit made by Louis Beaumont consists of the following changes:

1. **README.md and Getting Started Documentation:**
   - The installation command instructions have been updated. Previously, the script was piped using `bash` and included the `-fsSL` curl options. Now, it uses `sh` and the simplified `-fL` options instead.

2. **install.ps1 Script:**
   - The output messages presented to users on successful installation have been updated to be more user-friendly. 
   - The initial "screenpipe installed successfully!" message is replaced with:
     - "installation complete! 🚀"
     - Instructions to restart the terminal and run the `screenpipe` command.
   - The information regarding joining the Discord server and checking the documentation now appears within a decorative box for improved readability.

Overall, the changes improve readability, enhance instructional guides, and provide a more visually appealing presentation during script execution. Additionally, there's a minor modification in the installation command in the documentation.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 210 (887accb6bd362337b313a18bc1194030d6b60f88)</summary>

The git commit made by Louis Beaumont introduces changes to the `install.sh` script, enhancing its visual output. The commit message indicates a new feature: adding color to the script. Here's a summary of the changes:

- The script now uses an ASCII box to display important URLs more prominently.
- Previously, URLs for joining the Discord and checking the documentation were printed on separate lines with blue color codes.
- These lines have been replaced with a visually distinct box format that frames the URLs and related information using ASCII characters for clarity and emphasis.

Overall, these changes improve the readability and aesthetics of the terminal output generated by `install.sh`.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 211 (b1325b8dffa4da97b12bbaf6ff4fc79c556465fe)</summary>

The commit `b1325b8dffa4da97b12bbaf6ff4fc79c556465fe` by Louis Beaumont on December 11, 2024, updates the `install.sh` script by adding color to some output text. Previously, the script defined colors with named variables `BLUE` for the color blue and `NC` for no color (reset). These variables have been removed, and the color codes are directly embedded in the echo statements. Specifically, URLs in the text are now highlighted in blue without relying on predefined variables.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 212 (5c8de2f86a1eff08d23482fa3522ccb59f4de2a4)</summary>

The commit titled "feat: add color to install.sh" introduces colored text to the `install.sh` script. Here's a summary of the changes:

- Two new variables, `BLUE` and `NC`, are defined to represent the blue color and "no color" reset, respectively, using ANSI escape codes.
- The Discord invitation link and the documentation URL in the script's output are now displayed in blue. This is accomplished by wrapping these URLs with the defined `BLUE` and `NC` variables to enhance their visibility.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 213 (2a211e4f0a3d4dd8b88ec59458d27bfeb812e139)</summary>

The commit `2a211e4f0a3d4dd8b88ec59458d27bfeb812e139` by Louis Beaumont updates the GitHub Actions workflow file `release-app.yml`. The changes involve modifying how Python and pip are executed in the workflow's PowerShell scripts:

1. In one script, the Python executable path is changed from using a hardcoded path to `C:\Users\screenpipe-windows\AppData\Local\Microsoft\WindowsApps\python.exe` to simply `python` when running the `get-pip.py` script. This likely makes the script more portable by relying on the system's PATH to locate the Python executable.

2. Conversely, the path to the Python executable is added when using pip to install the `intel-openmp` package in another script. This change explicitly specifies the path to the Python executable instead of relying on just `python`, likely for consistency or because the PATH wasn't resolving correctly.

Overall, the commit adjusts the way Python and its package manager, pip, are invoked in the workflow scripts to ensure the correct execution environment.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 214 (344c031d9a233257a8a0893155ae62cc226318f3)</summary>

The commit introduces several changes aimed at improving the install scripts (`install.ps1` for PowerShell and `install.sh` for Shell) used for the installation of the `screenpipe` application.

1. **Lowercasing Messages**: Various messages in both scripts are changed from Title Case to lower case to maintain consistency and perhaps align with a coding style or preference.

2. **Bun Installation**: The commit adds a step to install Bun (a JavaScript runtime) if it is not already installed. This involves adding an installation check and command for both Windows (in `install.ps1`) and Unix-like systems (in `install.sh`).

3. **ASCII Art and Additional Information**: An ASCII art meme has been added near the end of both scripts, intended as a celebratory display. Additionally, messages directing users to join Discord and check documentation for further assistance have been added at the end of the scripts.

4. **Simplification and Error Handling**: Improved error messages and consistent use of small case are employed for better readability and possibly align with a specific output format style.

5. **Silent Curl Operations**: The `curl` commands for downloading resources have been updated to use `-sL` rather than `-L`, making them operate in silent mode to reduce unnecessary output during execution.

Overall, these changes are focused on improving user experience during installation by providing better messages, ensuring all necessary tools are installed, and offering more guidance for getting started and seeking help.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 215 (c48d54310e7b1e54702001ab784c012dd6dae8f9)</summary>

The recent commit to `install.sh` primarily addresses enhancements related to macOS dependency management in the installation script.

### Key Modifications:

1. **Streamlined Command Execution**: The check for Xcode Command Line Tools now redirects output more efficiently by combining two commands into a single line:

   - Changed from:
     ```bash
     xcode-select -p & 
     >/dev/null
     ```
   - To:
     ```bash
     xcode-select -p &>/dev/null
     ```

2. **Dynamic Library Management**: The script adds functionality to download the required libraries specific to the user's architecture (either `arm64` or `x86_64`):

   - **ARM64 Architecture**: For `aarch64`, it downloads `libscreenpipe_arm64.dylib`.
   - **x86_64 Architecture**: For `x86_64`, it downloads `libscreenpipe_x86_64.dylib`.

3. **Linker Tool Adjustments**: The `install_name_tool` commands are now conditional based on architecture to properly update library paths:

   - Adjusts library path changes and IDs based on whether the architecture is `arm64` or `x86_64`.

These changes enhance the script's flexibility, ensuring that the appropriate libraries are downloaded and configured based on the user's system architecture during installation on macOS.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 216 (26a6707ed2e88718a174dea07348f0fa3f022d80)</summary>

The commit `26a6707ed2e88718a174dea07348f0fa3f022d80` introduces the changelog for version 0.16.3. This update includes:

### New Features:
- A speaker functionality was added to the SDK for improved audio management.

### Improvements:
- The API reference was updated to ensure ease of integration with external services.

### Fixes:
- Resolved app build errors related to User object inconsistencies, improving the user experience.
- Fixed a package resolving error in the pipeline to prevent disruptions.

The full changelog is accessible via a comparison link: [86b2f..f7529](https://github.com/mediar-ai/screenpipe/compare/86b2f..f7529).

Additionally, a file `cn` was created, and the existing changelog file `screenpipe-app-tauri/public/CHANGELOG.md` was updated to reflect these new changes for version 0.16.3, replacing the previous full changelog.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 217 (f752940b13b0fb7099a357c17954863c4fbe6320)</summary>

The commit titled "release-app," authored by Louis Beaumont, includes several changes related to the deployment workflow and version updates for the "screenpipe-app" project:

1. **GitHub Actions Workflow Update**:
   - In the `.github/workflows/release-app.yml` file, the method to install Python's `pip` has been updated. Previously, a generic Python command was used; it has been replaced with a more specific command that uses an absolute path to a Python executable on a Windows system.

2. **Version Updates**:
   - In `screenpipe-app-tauri/src-tauri/Cargo.lock`, the version of the "screenpipe-app" package has been updated from `0.16.0` to `0.16.2`.
   - In `screenpipe-app-tauri/src-tauri/Cargo.toml`, the version has further been incremented from `0.16.2` to `0.16.3`.

3. **Binary File Modifications**:
   - The `ui_monitor-aarch64-apple-darwin` and `ui_monitor-x86_64-apple-darwin` binaries have changed, indicating updates or recompilations for these macOS-specific binaries.

These changes suggest a minor version update for the application, along with some adjustments to the release workflow and rebuilds of macOS binaries.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 218 (71774976f07d8c149bef1b839135ea826b96264f)</summary>

The recent commit by Basit Mustafa addresses app build errors related to inconsistencies with the `User` object. Specifically, the changes include:

1. **In the `auth.tsx` component**:
   - Replaced the `user?.user_id` property with `user?.id`. This change ensures consistent identification of users with the `id` property when using Posthog for analytics.

2. **In the `stripe-subscription-button.tsx` component**:
   - Removed the call to `checkLoomSubscription`, and instead, a hardcoded `false` value is used to determine `hasSubscription`.
   - Updated the Stripe checkout URL to use `user?.id` instead of `user?.user_id` as the `client_reference_id`, ensuring the consistency of user identification across the app.

These changes help to maintain coherence in user identification and temporarily disable subscription checking logic by removing `checkLoomSubscription`.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 219 (f5e6742f8cdb08d7818af2962f1ec881ad456b69)</summary>

The recent update to the "Getting Started" documentation introduces several changes and simplifications to the installation process for "screenpipe". Here is a summary of the changes:

1. **Section Additions and Reorganization**:
   - Added a new subsection "### install now" at the beginning to emphasize downloading the [desktop app](https://screenpi.pe).
   - Some headings have been adjusted, such as renaming "manual build" to "build from source" and reorganizing the content significantly for clarity.

2. **Removal of Command-line Installation Instructions**:
   - The detailed instructions for CLI installation options, including using Homebrew for macOS, and comprehensive steps for building from source on Windows, have been removed.
   - The instructions for CUDA and Intel MKL GPU acceleration features remain but have been simplified.

3. **MacOS and Windows Installation**:
   - Consolidated macOS instructions to building from the source and downloading the pre-built desktop app.
   - Clarified that issues on Windows may necessitate downloading the pre-built desktop app.

4. **Linux Installation**:
   - Streamlined instructions for building from source and directed Docker users to an existing setup link.

5. **Removed Detailed Alternative Installation Options**:
   - Removed the detailed instructions for alternative installation using Nix package manager and Docker, simply offering a link for Docker setup.

6. **Simplification and Focus Shift**:
   - Overall, the document now focuses more on using the desktop app and offers less step-by-step guidance for command-line and source-code based installations.

7. **Emphasis on Desktop App**:
   - A clearer call to action for downloading and using the desktop app, allowing users to extend its functionality via plugins.

These changes aim to make it easier for users to get started with "screenpipe" by installing the desktop application and cater to a broader audience by minimizing technical setup instructions.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 220 (b522bd5ef6c5dd06995a32adeed2c5522ab9671e)</summary>

The commit by Louis Beaumont removes the "update-homebrew" job from the GitHub Actions workflow file `.github/workflows/release-cli.yml`. This job was responsible for updating the Homebrew Formula for the project, and it ran on macOS with various steps to set the version and bump the formula. Additionally, in the `Cargo.toml` file, the version number is incremented from "0.2.13" to "0.2.14".
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 221 (662f921dfad977a36892d5fcdb01cf5c4ae164b7)</summary>

The commit `662f921dfad977a36892d5fcdb01cf5c4ae164b7` updates the API reference documentation. Key changes include:

1. **Query Parameters Enhancements**:
   - More detailed descriptions for existing parameters like `q` and `content_type`.
   - Added new parameter `speaker_ids` for filtering by specific speaker IDs.

2. **Sample Requests**:
   - Updated and added sample requests for searching across different content types and filters.

3. **New "Speakers" API Section**:
   - Introduced multiple endpoints for managing speaker data, including:
     - Listing unnamed speakers.
     - Searching, updating, and deleting speakers.
     - Finding similar speakers and merging speaker records.
     - Marking a speaker as a "hallucination" (incorrect identification).

This update enhances the clarity and functionality of the API documentation by providing more comprehensive examples and detailing new features related to speaker management.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 222 (594c3914ff7f18b2d0da5f326dfc711666e13fac)</summary>

The commit by Louis Beaumont introduces several changes to the `screenpipe-js` library, updating it to version 0.1.5. Here are the key changes made in this commit:

1. **ContentType Expansion**: 
   - The `ContentType` union type has been extended to include combinations of types such as `"ui"`, `"audio+ui"`, `"ocr+ui"`, and `"audio+ocr"`. 

2. **Query Parameters Update**: 
   - A new optional parameter `speakerIds` has been added to the `ScreenpipeQueryParams` interface, allowing queries to filter based on speaker IDs.

3. **AudioContent Update**: 
   - The `AudioContent` interface now optionally includes a `speaker` field of type `Speaker`.

4. **New Interfaces**: 
   - Introduced `UiContent` to describe UI content structure with fields such as `id`, `text`, `timestamp`, `appName`, and `windowName`.
   - Introduced `Speaker` to define speaker information including `id`, `name`, and optional `metadata`.

5. **ContentItem Update**: 
   - The `ContentItem` type now includes a new type variant for `UI` content items, containing `UiContent`.

6. **Query String Handling**: 
   - Special handling for `speakerIds` within the `queryScreenpipe` function, converting the array of speaker IDs into a comma-separated string when generating URL query parameters.

7. **Version Update**: 
   - The version in `package.json` was updated from 0.1.4 to 0.1.5, reflecting these changes. 

These updates add functionality related to identifying and managing different types of content and speakers, improving the library's ability to handle a wider range of use cases.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 223 (d3e1d9448f4acb26ffcf47406ddd97769037e09a)</summary>

The git commit made by Louis Beaumont on December 11, 2024, involves a documentation update for plugins in the Screenpipe project. The change consists of a small revision to a bullet point in the Markdown file located at `content/docs/pages/docs/plugins.mdx`. Specifically, the phrase describing the "no lock-in" feature has been revised to include a promise of assistance: the original text was modified from "export it later as a desktop native app using screenpipe as a library" to "export it later as a desktop native app using screenpipe as a library (we will even help you with that)."
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 224 (3092e6b18d9e1d68a14660b1e6bbfde3d9894b0a)</summary>

The commit titled "docs: plugins" by Louis Beaumont updates the documentation related to plugins. Here are the key changes:

1. **Description of Plugins**:
   - The text describing UI-based plugins has been slightly modified. It now specifies that these plugins are "desktop native apps" using NextJS for user interaction.

2. **Reordering and Addition of Features**:
   - The order of feature points has been adjusted. "Monetization ready" was moved after "open source" and before "no lock-in."
   - A new feature called "no lock-in" was added, highlighting the ability to ship a startup quickly in screenpipe's store and later export it as a desktop native app using screenpipe as a library.

These amendments subtly adjust the focus of the documentation on the nature of UI-based plugins and enhance the description of feature offerings, stressing flexibility and ease of use for developers.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 225 (c201c2e6d626f80dfd76c07bdd38bbdfc1771248)</summary>

In the given commit, the documentation for plugins was updated. Specifically, the description of "headless" plugins has been modified to indicate that they are now deprecated. This annotation was added to the section discussing the different types of native plugins, clarifying that headless plugins, which run in the background without a visual interface, are considered deprecated.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 226 (8ce9aac39feca42f743af94ead0670e647a9c511)</summary>

In the commit by Louis Beaumont, there were updates made to the documentation for plugins located at `content/docs/pages/docs/plugins.mdx`. The changes include:

1. The `pipe.input` feature now has a more detailed description, specifying that it allows programmatic UI control using the keyboard and mouse.
2. A new feature, `pipe.settings`, was introduced, which allows getting and setting app settings, such as AI model and port configurations. 

These modifications enhance the clarity and functionality of the plugin documentation.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 227 (6290f2236bc19975d80e3e2c6b6e20ff1e435b07)</summary>

The git commit adds a new section to the `plugins.mdx` documentation file, providing example implementations for integrating AI functionalities using the `ollama-ai-provider` with the `screenpipe` SDK. Two main examples are included:

1. **Simple Hourly Summary with Ollama**: This example demonstrates how to retrieve the last hour of screen/audio data and generate a summary using a local Ollama model. The summary is then written to a markdown file.

2. **Smart Meeting Assistant with Ollama**: This example illustrates monitoring meeting applications to provide real-time AI assistance. It analyzes the last 60 minutes of meeting data and generates insights, which are then sent to an app's AI inbox with actionable suggestions.

These examples highlight how to:
- Query screen and audio data using `pipe.queryScreenpipe`.
- Utilize local AI models through Ollama.
- Send interactive notifications using `pipe.inbox`.

This update enhances the documentation by giving practical, code-based guidance on utilizing local AI models in conjunction with data querying and notification features.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 228 (7b9ae36d7d36c84630681abb7b694b183c796465)</summary>

The commit `7b9ae36d7d36c84630681abb7b694b183c796465` includes several changes related to the `screenpipe` application, focusing largely on integrating Stripe for handling credits and updates to existing features. The changes can be summarized as follows:

1. **Stripe Integration and Payment Handling:**
   - Added features to check and manage user credits using Stripe's API.
   - Updated the user interface to display and manage Stripe credits, including buttons for purchasing credits.
   - Modifications for handling paid features and pipes requiring credits.

2. **Code and Dependency Updates:**
   - Updated the code to streamline Stripe integration.
   - Installed new dependencies, such as `@supabase/supabase-js` for handling database interactions.
   - Refactored settings management within the application, introducing a `SettingsManager` for handling application configurations.

3. **Application Features Enhancements:**
   - Improved handling of deep linking and user authentication through `open_auth_window` and added support for a new `User` struct with credits.
   - Swapped from using FormData to raw audio buffer for Deepgram API integration, with the capacity to handle settings via environment variables for API endpoints.
   - Extensive changes in UI components, focusing on user account management and settings panels.

4. **Bug Fixes and Stability Improvements:**
   - Fixed issues with clickable elements on Windows systems.
   - Removed redundant imports and cleaned up unused components in several TypeScript and JavaScript files.

5. **Documentation and Configurations:**
   - Added migration scripts to support update and delete triggers for Full-Text Search (FTS) tables.
   - Updated various project configuration files such as Cargo.lock, package.json, and main.json to reflect new dependencies and project settings.

6. **UI/UX Adjustments:**
   - Modified UI components such as headers and settings dialogs to better accommodate the changes related to account and payment management.

Overall, this commit implements significant backend and frontend changes to integrate Stripe more effectively, enhance existing application features, and improve user experience.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 229 (5e2f8f7263c4ea9220e96c02313a56567a01a8a7)</summary>

The commit made by Louis Beaumont on December 11, 2024, updates the `README.md` file. The change involves modifying a sentence to provide more detail about the plugin system called "pipe" in the `screenpipe` project. The description now specifies that the system allows you to create a desktop app in Next.js within a sandboxed environment using Rust code.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 230 (89d1ec17dc0405baede39e6aa503de0d56ce157a)</summary>

The commit by Louis Beaumont updates the `README.md` file with several changes:

1. **Images Section:**
   - The markdown images are modified to HTML `<img>` tags with specified widths. The first image now has a width of 800 pixels, and the second image has a width of 400 pixels.

2. **Installation Instructions:**
   - The instruction to run "screenpipe" directly after executing the installation script on macOS and Linux is removed.
   - A comment about allowing permissions on macOS (screen, mic) is moved and rephrased as an independent statement after the installation instructions.

3. **Additional Information:**
   - The rest of the installation instructions and links remain unchanged.

Overall, these changes improve the formatting of images and clarify instructions regarding permissions on macOS.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 231 (9d82fe88ceb31c0b3dd266f68f745c8685a5149f)</summary>

The commit by Louis Beaumont updates the `README.md` file. The installation instructions for macOS and Linux have been changed from using `brew install screenpipe` to using a `curl` command that downloads and runs an installation script from a URL (https://raw.githubusercontent.com/mediar-ai/screenpipe/main/install.sh). Additionally, a note has been added to ensure that permissions (for screen and microphone) are allowed on macOS.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 232 (83dc6a8a4324def1ff7d9311551ad9cc0331d717)</summary>

The commit with ID `83dc6a8a4324def1ff7d9311551ad9cc0331d717` by Louis Beaumont involves the deletion of the file `content/docs/NOTES.md`. This file contained troubleshooting information and frequently asked questions (FAQs) relating to the application "screenpipe". The document provided solutions for various issues users might encounter, such as problems with file permissions, library loading errors, issues specific to Windows and macOS, and guidance on locating application data. The file has been entirely removed from the project.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 233 (ac210c5b0d7440aa39d5c30466679aa2c1f32a9a)</summary>

The commit made by Louis Beaumont updates the "getting-started.mdx" file in the documentation. The key change is the modification of the installation instructions for the screenpipe tool. The tabs for operating systems have been simplified from separate "macos", "linux", and "windows" tabs to a combined "macos & linux" and a "windows" tab.

For macOS and Linux users, the previous installation method using Homebrew (`brew install screenpipe`) has been replaced with a command that uses `curl` to download and execute an installation script from the screenpipe GitHub repository. Additionally, there is a note advising macOS users to ensure they allow the necessary permissions for the screen and microphone. The "windows" tab remains unchanged in this excerpt.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 234 (d52e9fe1ed373561f1a963f0932287612f88e44c)</summary>

The commit `d52e9fe` by Louis Beaumont on December 11, 2024, deletes a binary file named `cn`.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 235 (47b7a1233287ae6215ead3eed0801a4e59b73e22)</summary>

The commit adds a new installation script for macOS and Linux systems. The script, `install.sh`, is a POSIX-compatible shell script that automatically detects the operating system and architecture, fetches the latest version of the `screenpipe` application from GitHub, and installs it.

Key points include:

1. **OS and Architecture Detection:** The script uses the `uname` command to determine the OS (macOS or Linux) and the architecture (x86_64 or arm64/aarch64). It only supports arm64 on macOS, and x86_64 on both macOS and Linux, while Linux arm64 is not supported yet.

2. **Dependency Management:**
   - On macOS, it checks for the Command Line Tools and installs FFmpeg if needed, downloading it from a specified URL.
   - On Linux, it checks for the presence of required libraries (libasound and FFmpeg) and uses available package managers (`apt-get`, `dnf`, `pacman`, `zypper`) to install any missing dependencies.

3. **Fetching the Latest Version:** The script uses `curl` to access GitHub’s API, fetch the latest release, and constructs the download URL for the appropriate binary file.

4. **Installation Process:**
   - Downloads the compressed binary file from the constructed URL, verifies its integrity, and extracts it.
   - Copies the binary and necessary libraries into a local directory under the user's home (`$HOME/.local/screenpipe`).
   - Modifies library paths on macOS using `install_name_tool` to ensure proper linking.

5. **Symlink Creation and PATH Update:**
   - It creates a symbolic link in `$HOME/.local/bin` to allow easy access to the `screenpipe` binary.
   - The script attempts to append this directory to the user's PATH by modifying shell configuration files (such as `.zshrc` or `.bashrc`), prompting the user to reload their shell environment.

6. **Refinements and Bug Fixes:**
   - The script includes various tweaks and fixes, such as making it POSIX compliant, handling shell detection correctly, and minimizing verbose output logs.

This automated installer streamlines the setup process, handling dependencies and PATH updates to provide a smoother user experience.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 236 (4965dff8e95e015e1d4f041d8ad308d6f8bffb9e)</summary>

The git changes consist of two modifications:

1. **Version Increment in `Cargo.toml`:**
   - The version of the `screenpipe-app` was incremented from "0.16.1" to "0.16.2" in the `screenpipe-app-tauri/src-tauri/Cargo.toml` file.

2. **Formatting Adjustment in `screenpipe-server.rs`:**
   - Two lines in the `screenpipe-server/src/bin/screenpipe-server.rs` file were modified to adjust the length of box-drawing characters in printed tables, making sure they align neatly. Specifically, the number of hyphens was increased by one to ensure consistent formatting.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 237 (6631a97e1d1825d53f9d40d886c1ddd078859229)</summary>

The commit with hash `6631a97e1d1825d53f9d40d886c1ddd078859229` includes a series of bug fixes to the project. These changes were authored by a user named Tribhuwan, and co-authored by tribhuwan-kumar. The changes address the following issues:

1. **Clickable Issue with Status Button on Windows**: Details of this fix are not included in the diff provided, but it's mentioned in the commit message.

2. **Package Resolving Error**: The key focus of the changes is resolving package errors. Specifically, the changes are applied to the `pipes.rs` source file in the `screenpipe-core` module.

    - The code has been refactored to improve error handling during the downloading of pipes. The function now checks the result of the download attempt. If the download fails, it deletes the temporary directory and logs an error message.
    
    - The `download_github_folder` function call has been updated to align with the changes in asynchronous handling.
    
    - Previously, the `bun install` command was conditional upon detecting a Next.js project. The update makes running the `bun install` command mandatory for all projects containing a `package.json` file to unify the project's package setup procedure.

These changes collectively enhance error handling and streamline package installation regardless of the project's framework dependencies.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 238 (c80459a93c34e1b7a2e0f877c74bcb16e1db8555)</summary>

The commit with hash `c80459a9` by Louis Beaumont updates the `README.md` file. The change involves adding a new image to the README. Specifically, an `<img>` tag was added within a `<p>` block, which includes an image with a width of 1312 pixels. The image is a screenshot taken on December 11, 2024, at 1:39 PM, and the source URL is a GitHub user asset.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 239 (86b2fda0043dc47043340df96cb1bfc596f38d97)</summary>

The git commit titled "fix build" by Louis Beaumont modifies a single file: `screenpipe-server/src/bin/screenpipe-server.rs`. In this commit, the error handling code within the `main` asynchronous function was altered. Specifically, when a port is already in use, the error handling mechanism was changed from using `std::io::Error` with `ErrorKind::AddrInUse` to using `anyhow::anyhow!` to create a more general error message: "port already in use". This likely simplifies the error propagation by using the `anyhow` crate, which provides a more flexible error handling framework in Rust.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 240 (0aa776f80981f60e8792fe37cd5f15ce7d280a63)</summary>

The commit includes the following changes:

1. **.gitignore Update:** Added `.dev.vars` to the `.gitignore` file to ignore Cloudflare workers' environment variables.

2. **New File:** A new binary file `bun.lockb` was added under `screenpipe-actions/ai-proxy`.

3. **Code Changes in `index.ts`:**
   - Changed double quotes to single quotes for consistency in strings and key names.
   - Added a new endpoint `/v1/transcribe` to handle audio transcription requests using the Deepgram API. It checks for an audio file and optional languages, sends a request to the Deepgram API, and returns the transcription result.
   - Refined the existing `/test` and OpenAI chat completion logic with better error handling and response formatting.
   - Adjusted CORS headers for improved HTTP response management.
   - Enhanced error logging for more clarity.

4. **Interfaces and Documentation:**
   - Updated the `Env` interface to include a new environment variable `DEEPGRAM_API_KEY`.
   - Revised inline comments to provide usage instructions for new and existing API functionality.

5. **Example Usage:**
   - Added example commands in comments for testing the `/v1/transcribe` and chat completion endpoints locally and remotely. 

These changes improve functionality by adding new features and enhancing code readability and maintainability.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 241 (5520a2dafbba074ceaa4cbd296c8f93e46fd940d)</summary>

The commit `5520a2dafbba074ceaa4cbd296c8f93e46fd940d` introduces a new feature to prevent running multiple instances of the server simultaneously. The author added a check using the `port_check` crate to determine if the specified port is already in use, helping avoid starting the server more than once on the same port.

### Summary of Changes:

1. **Dependency Addition:**
   - Added `port_check` version `0.2.1` to the dependencies in `Cargo.toml`.

2. **Functionality Implementation:**
   - In `screenpipe-server/src/bin/screenpipe-server.rs`, the function `is_local_ipv4_port_free` from the `port_check` crate is used to check if the specified port is free.
   - If the port is in use, an error message is logged: "you're likely already running a screenpipe instance in a different environment, e.g., terminal/IDE, close it and restart or use a different port."
   - The program will return an error with the kind `AddrInUse` if the port is not free, preventing the server from starting when the port is occupied.

This change ensures that the server doesn't start multiple times on the same port, which can help avoid conflicts and improve usability by alerting the user when the server is already running.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 242 (ad061e14335667476e439f444f115736675f647a)</summary>

The git commit, identified by hash `ad061e14335667476e439f444f115736675f647a`, introduces changes made by the author Yashashwini2003 on December 11, 2024. The commit adds a new SQL migration script to the project located at `screenpipe-server/src/migrations/20241210111055_add_fts_update_delete_triggers.sql`.

This migration script creates UPDATE and DELETE triggers for several Full-Text Search (FTS) tables in the database. Specifically, it does the following:

1. **Temporary Foreign Key Suspension:** Begins by turning off foreign keys to perform the operations safely.

2. **Triggers for `ocr_text`:**
   - An UPDATE trigger (`ocr_text_update`) to synchronize changes to the `ocr_text_fts` table when modifications occur in `ocr_text`.
   - A DELETE trigger (`ocr_text_delete`) to remove entries in `ocr_text_fts` when corresponding entries are deleted from `ocr_text`.

3. **Triggers for `audio_transcriptions`:**
   - An UPDATE trigger (`audio_transcriptions_update`) for updating `audio_transcriptions_fts` when changes are made in `audio_transcriptions`.
   - A DELETE trigger (`audio_transcriptions_delete`) for deleting records from `audio_transcriptions_fts` upon deletions in `audio_transcriptions`.

4. **Triggers for `ui_monitoring`:**
   - An UPDATE trigger (`ui_monitoring_update`) to update the `ui_monitoring_fts` table reflecting changes in `ui_monitoring`.
   - A DELETE trigger (`ui_monitoring_delete`) to delete entries from `ui_monitoring_fts` when entries are removed from `ui_monitoring`.

5. **Re-enabling Foreign Keys:** The script concludes by re-enabling foreign keys to maintain database integrity going forward.

These changes are intended to maintain FTS tables in sync with their respective source tables, ensuring consistency and correctness of the data in the application.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 243 (a4f97e31c71dd96333847f458f7530a5e87c2666)</summary>

The commit `a4f97e31c71dd96333847f458f7530a5e87c2666` involves the addition of a changelog entry for version 0.16.1. The changes include:

1. **Documentation Update:**
   - A new changelog file `content/changelogs/0.16.1.md` was created to document the changes in version 0.16.1.
   
2. **Fixes:**
   - The changelog notes a fix for a clickable issue with the status button on Windows, resolving a problem where the button could not be clicked.
   
3. **Modification of Existing Changelog:**
   - The changelog in `screenpipe-app-tauri/public/CHANGELOG.md` was updated to reflect this fix and now points to a new comparison link for the full list of changes from `c47b5..6817d`.

This commit was automatically generated by the GitHub Actions Bot to document the latest fixes in the changelog.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 244 (6817d630331959bd09534eb6afc16e8db20ff5f8)</summary>

In the given Git commit, the author Louis Beaumont made a change to the `Cargo.toml` file within the `screenpipe-app-tauri/src-tauri` directory. The version number of the `screenpipe-app` package was incremented from `0.16.0` to `0.16.1`. The commit message indicates that this update includes a fix for an issue where the status button on Windows could not be clicked.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 245 (ca2ded55729a3162c032208e1638b848acf4f1ef)</summary>

The commit with hash `ca2ded55729a3162c032208e1638b848acf4f1ef` focuses on fixing an issue related to the status button's clickability on Windows in the `screenpipe-status.tsx` component. The main changes involve:

1. **Refactoring for Safety**: The code was updated to handle cases where the `health` object might be `null` or `undefined`, by using optional chaining (`health?.status`) and nullish coalescing operators (`health?.status ?? ""`). This ensures that the code doesn't break due to null reference errors and makes it more robust.

2. **Removed Code**: An initial block of code that likely returned a badge element when `health` was not available has been removed, streamlining the logic flow.

3. **Safety Checks Added**: Conditional statements and strings now account for potentially missing or undefined properties by providing default or fallback values.

4. **KPI Indication**: Visual elements indicating the status of processes such as screen recording, audio recording, and UI monitoring have been adjusted to reflect the optional presence of `health` data, ensuring proper UI feedback even if complete information is not available.

Overall, these modifications enhance the reliability and user interaction experience of the status button feature, particularly addressing environments using Windows.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 246 (c47b544e7934f48f96a6fee6186118cc4d9e1229)</summary>

The commit adds a changelog for version 0.16.0 to the project. The following changes are documented in the new version:

### New Features
- Introduced a speaker identification UI and API which enhances user experience.

### Improvements
- Code has been refactored to include a retry mechanism, increasing reliability.
- Several minor edits were made for better performance.

### Fixes
- Corrected issues with the audio transcriptions table to ensure accurate data processing.
- Resolved problems with the audio search function for improved search functionality.
- Fixed issues related to header and unit test compilation for smoother operation.

Additionally, a binary file was introduced, and the changelog was updated in both the new `content/changelogs/0.16.0.md` file and the `screenpipe-app-tauri/public/CHANGELOG.md` file. The full changelog can be reviewed by comparing commits [e73cc..41756].
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 247 (417564015ba047dedd183cde02aa64c6e14cbcaf)</summary>

The commit includes the following changes to the `screenpipe-app` project:

1. **Introduction of LinkedIn AI Assistant in Preview Mode:**
   - The LinkedIn AI Assistant is now marked as "(preview)" in the app. Its description has been updated to indicate that it is "coming soon."

2. **Modifications to Pipe Handling:**
   - The code handling the toggle and subscription logic for the Loom pipe has been removed, suggesting that the Loom service may temporarily be free or no longer subject to subscription checks.
   - Special handling logic has been added for the LinkedIn AI Assistant pipe to direct users to an onboarding link when selected.

3. **Code Cleanup:**
   - Minor code formatting improvements were made, including changes to the lambda syntax and the removal of unnecessary comments.

4. **Version Update:**
   - The version of the `screenpipe-app` has been incremented from `0.15.9` to `0.16.0` in both `Cargo.lock` and `Cargo.toml`.

These changes focus on preparing the LinkedIn AI integration for future release, removing Loom subscription checks, improving code readability, and indicating a new version of the app.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 248 (c97e01cd7c6f4cd17ef0e56c252adcbd13e33020)</summary>

The commit by Ezra Ellette introduced a migration script to modify the `audio_transcriptions` table in the database. The key changes in this script include:

1. **Table Redesign**: A new table `audio_transcriptions_new` is created with the desired schema where the `id` column becomes the primary key with auto-increment functionality.

2. **Data Transfer**: Existing data from the old `audio_transcriptions` table is copied over to this new table, `audio_transcriptions_new`.

3. **Table Replacement**: The old `audio_transcriptions` table is then dropped, and the new table is renamed to `audio_transcriptions`.

4. **Indexing**: Several indices are created to optimize queries, including indices on `audio_chunk_id`, `timestamp`, and `transcription`.

5. **FTS Integration**: A full-text search (FTS) virtual table `audio_transcriptions_fts` is created, and a trigger `audio_transcriptions_ai` is established to update this FTS table on insertions.

6. **Foreign Keys**: Foreign key constraints were temporarily disabled during the migration process to facilitate the restructuring, then re-enabled afterward.

7. **Default Values**: Columns such as `device` and `transcription_engine` have default values set to empty string and 'Whisper', respectively.

Overall, this migration aims to enhance the functionality, performance, and data integrity in managing audio transcriptions within the system.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 249 (cc2a041c5d67f66f1b3acbe474c421f2f447a670)</summary>

The commit by Louis Beaumont introduces several changes, both functional and style-related, to the GitHub repository. The main updates include:

1. **Workflow Update**: 
   - Modified the `.github/workflows/release-app.yml` to use `tauri-apps/tauri-action@v0.5.16`, instead of the previous unspecified version. 
   - Added a retry mechanism for the build step with `retryAttempts: 3`.

2. **Code Refactor in `pipe-store.tsx`**:
   - Added a `name` attribute to the `CorePipe` interface, providing more descriptive labels for the pipes within `corePipes`.
   - Implemented a `getFriendlyName` function to enhance readability by converting pipe IDs into user-friendly names, which is used in the display headers for better presentation.
   - Updated the JSX to use these friendly names instead of raw IDs.
   - Made a minor style adjustment to button classes, configuring them to change appearance based on the pipe’s enabled state by making use of conditional styling.

These changes collectively improve the robustness of the build process and enhance the clarity and user experience of the application's component UI.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 251 (966bea20117ff13ab522f6617278b8ec5b7edb95)</summary>

The recent git commit with hash `966bea20117ff13ab522f6617278b8ec5b7edb95` includes a significant update to the `screenpipe-server` program. The update entails the removal of a previous HF warning. Additionally, the main function of the server has undergone extensive modifications and expansions. Notable changes include:

1. **Initialization and Configuration Changes**: 
   - The main function is now asynchronous, using the `tokio::main` macro.
   - Added setup of a `Highlight` object for debugging purposes.
   - Extensive logging setup, depending on command configuration.

2. **Command Handling Logic**: 
   - A robust command handling mechanism is established with a variety of commands such as `Pipe` subcommands and `Setup`.
   - Enhanced functionalities for audio and screen capture permissions.
   - Server setups are refined with checks for essential software like FFmpeg.

3. **Configuration Parameters**: 
   - Numerous configuration parameters are introduced or refactored, impacting features like audio/vision processing, file handling, and device management.
   - Introduced flags for functionalities such as listing audio devices, monitors, and handling permissions.
   
4. **Server and Pipeline Management**: 
   - A server instance and related components are initialized and connected for processing pipelines.
   - Additional handling and management for pipe commands like list, download, info, enable, disable, update, and delete, all of which interact with a REST API.
   
5. **Error Handling and Warnings**: 
   - Various error-handling protocols for different operations, including model loading and FFmpeg configuration.
   - Improved user guidance through warnings and informational messages regarding telemetry and privacy.

6. **Startup and Shutdown Logic**: 
   - Improved startup display messages, highlighting settings configuration.
   - Implementation of graceful shutdown mechanisms on user input (such as `ctrl+c`) or other conditions, with data privacy considerations.

Overall, this update significantly enhances the `screenpipe-server` by incorporating better configuration management, command handling, and robust logging and debugging capabilities.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 252 (9684f4a6e6d60ae5f9e1234c6a229096ccfeaa3c)</summary>

The commit with ID `9684f4a6e6d60ae5f9e1234c6a229096ccfeaa3c` made by Louis Beaumont on December 10, 2024, includes the following changes to the `screenpipe-server/src/bin/screenpipe-server.rs` file:

1. **Logging Configuration Update**:
   - A new logging directive was added to the logging setup within the `setup_logging` function. It specifically adds "hf_hub=error" to the logging configuration. This implies that any log messages related to "hf_hub" will now be logged as errors.
  
2. **Removal of Code**:
   - A significant portion of the main function and related code logic was removed. This includes the `main` function, which appears to manage command-line interface (CLI) commands and server operations related to the Screenpipe server. This may include setup actions for logging, initiation of various components (like highlighting, pipe manager), checks for available tools (like FFmpeg), handling of CLI commands, and additional operational logic for handling and processing audio and video data.
  
These changes simplify the file by streamlining the logging setup and removing unused or redundant logic, possibly in preparation for a refactor or different architecture implementation.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 253 (62c971ece1aca322f7d637e24ecccab2bc6de68d)</summary>

The commit with hash `62c971ece1aca322f7d637e24ecccab2bc6de68d`, authored by Ezra Ellette, includes a fix to the `audio_search` function in the `screenpipe-server` project. The modification is within the `src/db.rs` file. Specifically, the condition for filtering audio transcriptions based on speaker data has been altered:

- Previously, the condition required that `speakers.hallucination = 0`.
- The updated condition now checks that either `speakers.id` is `NULL` or `speakers.hallucination = 0`.

This change accommodates cases where `speakers.id` might be `NULL`, allowing the condition to evaluate to true more flexibly.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 254 (373c768a95a961f4ba8cfa9ea128760451d854cd)</summary>

The commit `373c768a95a961f4ba8cfa9ea128760451d854cd` made by Louis Beaumont on Tue Dec 10, 2024, includes two main changes:

1. **Header Component (`header.tsx`):**
   - A significant block of code has been removed related to a timeline button wrapped in a `TooltipProvider` and `Tooltip`. This includes the entire logic for displaying a tooltip message when the timeline is not enabled in the settings. The button to show meeting history remains, but the timeline-related logic has been simplified or removed.

2. **Endpoint Test (`endpoint_test.rs`):**
   - The test code has been refactored for better readability and maintenance. Specifically, the `count_search_results` function and similar asynchronous calls have been modified to include additional `None` parameters, potentially indicating added arguments for flexibility or future use.
   - Several code formatting improvements have been made, such as better alignment, spacing, and wrapping of function calls over multiple lines.
   - Inline comments have been slightly adjusted for consistency.

Overall, this commit addresses code simplification in the header component and enhances the readability and flexibility of endpoint tests in the codebase.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 255 (1146c2b10d0bede217630d24b4807427d9ea5f68)</summary>

### Summary of Changes in Commit `1146c2b`

#### New Features:
1. **Speaker Identification UI/API**:
   - Addition of a UI for identifying unnamed speakers from audio samples.
   - New API endpoints for managing speakers:
     - Get unnamed speakers.
     - Search and rename speakers by name prefix.
     - Delete speakers.
     - Mark a speaker as hallucination.
     - Merge speakers.
     - Fetch and identify similar speakers.

2. **UI Enhancements**:
   - Updated meeting UI to include speaker identification.
   - Meeting history and identify speakers are accessible via the side menu.
   - Icons for speaker-related actions.
   - UI elements for setting speaker names and handling similar speakers.
   - Button visibility based on speaker identification status.
   - Utilizes the longest audio segment for speaker identification.

#### Code Structure Changes:
1. **New Files**:
   - Added `identify-speakers.tsx` and `meeting-history.tsx` for speaker identification and meeting history UI components.

2. **Modularization**:
   - New `types` folder introduced to manage types for meetings and speakers.
   - `utils` updated with methods to handle operations like file size calculation.

3. **Database and Server**:
   - New server endpoints for speaker management.
   - Database queries and structure updated to accommodate speaker-related operations.

#### Testing:
- Added tests for the new database features involving speakers like merging, searching, and deleting speakers.

#### Fixes and Maintenance:
- Fixed merge conflicts and UI-related problems.
- Updated binaries and configuration files.

#### Dependencies:
- Updated Rust dependencies, e.g., `thiserror` and others.
- New schema migrations for the database to support speaker features.

#### Miscellaneous:
- Changes to UI appearance and positioning.
- Improved handling of fetching and displaying speakers based on audio data.
- Enhanced logging for better diagnostics.

Overall, this commit primarily enhances the application's ability to manage and identify speakers in audio recordings, providing both a user-friendly UI and comprehensive underlying API support.
</details>

------------------------------------------------------------------------

# Overall Summary of Changes

This extensive summary captures updates across various components of a project involving changes to version control, feature development, code refactoring, and documentation updates. The alterations are documented under different commit identifiers, showcasing a myriad of improvements and feature additions to several parts of the application, touching upon UI enhancements, database and server functionality, as well as installing and setup scripts.

Key Features:
1. **Speaker Identification Enhancements**:
   - New UI and API components for speaker identification, management, and enhancements across general meeting functionalities.
   - Features include identifying unnamed speakers, fetching similar speakers, merging, marking hallucinations, and renaming functionalities.

2. **Backend Enhancements**:
   - Modifications and additions to SQL migrations to accommodate new speaker functionalities.
   - Improvement in API endpoints related to speaker management.
   - Handling and setup of additional background and setup features, including server functions and CLI interactions.

3. **Workflow and Build Updates**:
   - Several updates to GitHub Actions and build processes involving dependencies and build script adjustments, highlighting a focus on improving developer workflows and resolving previous build inconsistencies or errors.

4. **UI and UX Improvements**:
   - Enhanced integration of new features with the app’s UI, including new components and streamlined existing elements to align with new functionalities.
   - Clean-up of unused components and refactoring for consistent linking and streamlined user interaction in UI components.

5. **Documentation and Version Control**:
   - Significant documentation updates, reflecting new functionalities and refined instructions for facilitating ease of use and understanding among users and developers.
   - Managing version control updates across various application modules, indicating multiple minor and major release updates as features and enhancements get integrated.

6. **Miscellaneous**:
   - Improvements in code readability and maintainability, including formatting enhancements, comment updates, and bringing coherence across multiple modules for a cleaner codebase.
   - Installation improvements including enhanced scripts for better system compatibility and user experience.

Overall, these updates significantly contribute to enhancing the application’s capabilities, user experience, and developer ergonomics, continuing to iterate upon both functional and aesthetic aspects of the project while ensuring compatibility and modernization across various system platforms and usage scenarios.
