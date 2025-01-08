# Git Commit Summaries

-----------------------------------------------------------------------
-----------------------------------------------------------------------
 
PRESS CMD+SHIFT+V TO VIEW IN MARKDOWN
 
_______________________________________________________________________
-----------------------------------------------------------------------
Total number of commits: 16

<details>
<summary>summary for commit 1 (07f372b2c19b670b469b91abddf9c3f9bc6a5a34)</summary>

The Git changes in commit `07f372b2c19b670b469b91abddf9c3f9bc6a5a34` include several updates and additions:

1. **Version Updates**:
   - The version of the `screenpipe-app` package in `Cargo.lock` was updated from `0.22.5` to `0.22.6`.
   - The version in `Cargo.toml` was updated from `0.22.6` to `0.22.7`.

2. **Dependencies**:
   - A new dependency, `once_cell` version `1.20.0`, was added in `Cargo.toml`.

3. **New Health Check Feature**:
   - A new file `health.rs` was added, implementing a health check feature using a Tauri app handle. This feature checks the health of a service running at `http://localhost:3030/health` and updates tray icons based on the health status.
   - In `main.rs`, this health check service is started during the app initialization.

4. **Refactoring and Reorganization**:
   - The `mod store` line in `main.rs` was moved for organizational purposes.
   - The `health` module was newly added and utilized in `main.rs`.
   - Minor code style changes were made for clarity, such as formatting and moving code blocks related to handling shortcuts.
   - Comments were added or adjusted for clarity, particularly in the new `start_health_check` function and some function parameters in `main.rs`.

5. **Tray Menu Update Optimization**:
   - In `tray.rs`, a mechanism was implemented to cache the current menu state and only update the tray menu if the state has changed, improving efficiency.
   - `once_cell` is used to store the last menu state, ensuring updates only when necessary.

6. **Miscellaneous**:
   - Uses of `println!` were changed to use `info!` from `tracing` for more consistent logging.
   - Some unnecessary imports were removed, and significant whitespace and formatting changes were made for code clarity.

Overall, this commit mainly enhances the functionality of the `screenpipe` application with a new health check service, cleans up the code, and optimizes certain operations.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 2 (fb460edbf9b359734b7206dcf55f9ed43f1cc500)</summary>

The git commit with hash `fb460edbf9b359734b7206dcf55f9ed43f1cc500` adds a new changelog entry for version 0.22.6 of the Screenpipe application. The updates include:

### New Features:
- Added keyboard shortcuts to enhance user efficiency in the Screenpipe application.

### Fixes:
- Resolved a casing issue that could impact app performance.
- Fixed a minor logging issue to ensure accurate functionality.

Additionally, the commit updates the CHANGELOG.md files to reflect these new features and fixes. Full changelog details are available in a specified comparison link.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 3 (f22b7c7e5278773169ac0f966b8cacefe34bb995)</summary>

The git commit with ID `f22b7c7` by Louis Beaumont updates the version number of the "screenpipe-app" in the `Cargo.toml` file from `0.22.5` to `0.22.6`. This change suggests the release of a new patch version of the application.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 4 (7a06d53cf0abe6b62a4853029f77b8e1de4053b9)</summary>

The commit introduces multiple changes to the `screenpipe-app`, primarily adding shortcut-related enhancements:

1. **Shortcut Handling Improvements**: The commit refactors and organizes the code for handling shortcuts. This includes the introduction of a new `ShortcutRow` component for managing individual shortcut settings.

2. **Pipe Shortcut Functionality**: New functionality is added to associate keyboard shortcuts with "pipes" (likely referring to some kind of modular features or plugins), allowing users to trigger these pipes via shortcuts.

3. **UI Changes**: A new component `shortcut-row.tsx` is created to encapsulate the UI and logic for managing shortcuts. This component handles recording new shortcuts, enabling/disabling them, and provides visual feedback through UI elements like switches and buttons.

4. **API Integration**: A new API class `PipeApi` is implemented in `lib/api/index.ts`. This class provides methods to list pipe configurations, which the application can use to dynamically generate UI elements for managing pipe shortcuts.

5. **Settings Enhancements**: The application’s settings now include a `pipeShortcuts` field in the `Settings` type, allowing storage and retrieval of pipe-specific shortcuts.

6. **Backend Enhancements**: Tauri's backend (`main.rs`) is updated to handle the new pipe shortcuts. This includes updating methods for applying and managing global shortcuts, with new logic to register shortcuts associated with pipes.

7. **Package Version Update**: The version of `screenpipe-app` in `Cargo.lock` is updated from `0.22.3` to `0.22.5`, likely reflecting significant changes and new features added in this commit.

These updates improve the flexibility and usability of the application by allowing users to manage and utilize shortcuts more effectively, particularly in the context of customizable features like pipes.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 5 (fe7463a82b9c22ade722d9a5adde545959444498)</summary>

The given commit, authored by Louis Beaumont on January 7, 2025, removes a debug option in the `sidecar.rs` file of the `screenpipe-app-tauri` project. Specifically, the line of code that pushes the `--debug` argument to the `args` vector is commented out at line 363, effectively disabling this debug option during the execution of the `spawn_sidecar` function.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 6 (8fcd890d1dd6139c48ead8523e8bc20a79ea456b)</summary>

The commit identified by `8fcd890d1dd6139c48ead8523e8bc20a79ea456b` addresses a casing issue in the `main.rs` file of the `screenpipe-app-tauri` project. Specifically, it modifies the labels for two menu items: "Start Recording" and "Stop Recording", changing them to lowercase ("start recording" and "stop recording") to ensure consistent casing. The author of these changes is Louis Beaumont, with the commit dated January 6, 2025.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 7 (24b5bcf9f2603ee5fce04c17e9e18321345d3eb6)</summary>

This commit makes a small fix in the `pipes.rs` file within the `screenpipe-core` module. It updates a list of strings used for logging purposes by adding a new entry: `"$ next start"`. This change appears to address a minor issue with logging or pattern matching in the code.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 8 (6b9c4b238ad519e9b8983bbf18735f30caada70b)</summary>

The commit made by Louis Beaumont on January 6, 2025, includes a change in the `screenpipe-server/src/auto_destruct.rs` file. The modification was made within a function `is_process_alive`, which is intended to check if a process is alive on Windows operating systems. Specifically, the code was altered to ensure that when opening a process with `OpenProcess`, the result is unwrapped, likely to handle potential errors and ensure that the process handle is correctly obtained.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 9 (c02fa48601de43cbcfe0f97bb22638410c3a39ce)</summary>

The commit `c02fa48601de43cbcfe0f97bb22638410c3a39ce` involves updating documentation by adding a new changelog file for version 0.22.5 of the project. The changes include the creation of a new changelog document at `content/changelogs/0.22.5.md` along with updates to the existing `CHANGELOG.md` file in the `screenpipe-app-tauri/public/` directory. The changelog highlights several fixes:

1. **Auto Destruct Issue for Windows CLI**: Fixed as per issue #926 to ensure proper auto-destruction behavior.
2. **Benchmark Tests**: Adjustments made for improved reliability.
3. **Unit Test Issues**: Various tests have been corrected to ensure their accurate functionality and coverage.

Additionally, a link to the full changelog comparison from e147b to 06914 is provided. The changelog was generated by a GitHub Actions Bot on January 6, 2025.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 10 (069147c82c299f81720e68dd682f77dc7208865b)</summary>

The commit identified as `069147c82c299f81720e68dd682f77dc7208865b`, made by Louis Beaumont, addresses issue #926 by implementing fixes related to the automatic destruction of Windows CLI. The changes include:

1. **GitHub Actions Workflow (`release-app.yml`):**
   - The maximum number of retries (`MAX_RETRIES`) for a process has been increased from 3 to 5.
   - The delay between retries (`RETRY_DELAY`) has been increased from 10 seconds to 30 seconds.

2. **Version Update (`Cargo.toml`):**
   - The version number of the `screenpipe-app` has been incremented from `0.22.4` to `0.22.5`.

3. **Process Watching Enhancement (`auto_destruct.rs`):**
   - Added a function `is_process_alive` specifically for Windows that uses the Windows API to check if a process is still running.
   - Modified the `watch_pid` function to use the new Windows API check before falling back to the existing `Command` approach for process monitoring.
   - This consists of shell commands using `tasklist` to verify both the PID and application name (`screenpipe-app.exe`) on Windows.
   - Enhanced the logic to provide additional process termination checks to ensure robust monitoring.

These changes improve the reliability of process termination checks on Windows in the application, ensuring it handles retries more effectively and uses the Windows API for a more dependable process status check.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 11 (e42979135c89decee89e6f6c278f4b5ba9a8fad4)</summary>

The git commit `e42979135c89decee89e6f6c278f4b5ba9a8fad4` by Louis Beaumont includes fixes and updates to benchmark files in the `screenpipe-server` and `screenpipe-vision` projects.

1. **In `screenpipe-server/benches/db_benchmarks.rs`:**
   - Additional `None` parameters were added to a function call within the `setup_large_db` function to enhance or fix its configuration.

2. **In `screenpipe-vision/benches/apple_leak_bench.rs`:**
   - The `cidre::ns` namespace was imported for usage.
   - The `perform_ocr_apple` function call was modified to use a `ns::ArrayMut<ns::String>::with_capacity(0)` as an argument instead of an empty vector.

3. **In `screenpipe-vision/benches/vision_benchmark.rs`:**
   - `Arc` from the `std::sync` library was introduced, and `WindowFilters` from `screenpipe_vision::capture_screenshot_by_window` was imported.
   - The `benchmark_continuous_capture` function now creates `WindowFilters` using `Arc`, and this is passed into the `continuous_capture` function. This change might provide thread-safe shared ownership of `WindowFilters`.
   - The `continuous_capture` function call was adjusted to replace the empty arrays passed for window filters with the `window_filters` object and added a `false` flag at the end.

Overall, these changes seem to address improvements in setup parameters, the use of more appropriate data structures, and refinements to benchmark functions within the project.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 12 (393e1f04781ea88d4946a3af573fb174cb1d500f)</summary>

This commit `393e1f0` made changes primarily to fix and clean up unit tests in a project, involving both audio and vision modules. Here are the key changes:

1. **Audio Module:**
   - Removed unnecessary code related to `output_path` in several files, ensuring that temporary output paths are no longer specified or cloned. This was applied to example files (`stt.rs`) and test files (`accuracy_test.rs` and `core_tests.rs`).
   - Simplified function calls by removing arguments (`output_path`, `true`) that were unnecessary.
   - Adjusted function calls to use an updated interface, such as changing `&audio_input` to `&audio_input.data`.

2. **Vision Module:**
   - Restructured code in `websocket.rs` to use a new `WindowFilters` struct instead of handling ignored and included windows directly.
   - Removed the `window-filtering.rs` example file altogether, likely due to redundancy or integration of its features elsewhere.
   - Changed the visibility of the function `get_apple_languages` in `core.rs` to be `pub`, allowing it to be used outside of its current module.
   - Updated `apple_vision_test.rs` to include testing for both English and Chinese languages using `perform_ocr_apple`. The test confirms that OCR tasks identify specific keywords from images, using a new approach to handle language configurations.

Overall, the commit streamlines the test setup, enhances the flexibility of handling language settings, and removes deprecated or inefficient code paths.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 13 (e147b2ead56490714d3bb070a8b5365c0e8c4559)</summary>

The commit with the ID `e147b2e` introduces changes primarily related to documentation updates and fix implementations. It adds a new changelog file for version 0.22.4 and updates an existing changelog file in the `screenpipe-app-tauri` project. The key changes highlighted in the commit include:

- **Fixes:**
  - Correction of a modal display issue to enhance user guidance.
  - Resolution of problems related to building memories to improve functionality.
  - Rectification of import issues to ensure application stability.

The changelog was updated to reflect these fixes, and the previous entries for new features and improvements were removed or replaced, focusing the document on the recent fixes. The `Full Changelog` link was updated to a new comparison range between different commits.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 14 (6683130a6ba34bf1f05936bda883aebe3d404291)</summary>

The git commit with hash `6683130a6ba34bf1f05936bda883aebe3d404291` by Louis Beaumont updates the version number of the "screenpipe-app" package from "0.22.3" to "0.22.4" in the `Cargo.toml` file. This change is part of a fix related to a modal for instructions in the "release-app" component.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 15 (ee6bf72f5dd9b6fd5df3978c59389cac9fd9c1c8)</summary>

The commit by Louis Beaumont focuses on fixing the build for the "memories" component. Here's a summary of the changes made:

1. **package.json Updates**: 
   - Removed `eslint` and `eslint-config-next` dependencies.
   - `bun-types` remains as `latest`, and `typescript` still uses version `^5`.

2. **Memories Gallery Component (`memories-gallery.tsx`)**:
   - The `newMemories` array is now explicitly typed as `any[]`.
   - Added multiple `@ts-ignore` comments to ignore TypeScript errors possibly related to `item.content.frameId`, `item.content.timestamp`, `item.content.filePath`, `item.content.appName`, and using `item.content.text`.
   
3. **Breaking Changes Instructions Dialog (`breaking-changes-instructions-dialog.tsx`)**:
   - Combined the asynchronous `init` function with the logic to fetch pipe data. It now initializes and also checks pipes within a single `useEffect`, reducing redundancy.
   
4. **Pipe Store Component (`pipe-store.tsx`)**:
   - Removed a block of code involving an `iframe` that displayed content based on `selectedPipe.config.port`. This might have been unused or causing layout/build issues.

These changes aim to improve the component's stability or address specific build errors, especially related to TypeScript configurations and dependencies.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 16 (80ce4eb4d2c8422db6a7ed7cedb514f74650c863)</summary>

The git changes in the provided commit include:

1. Deletion of several `bun.lockb` files across multiple directories within the project. These include:
   - `pipes/archive/pipe-anthropic-computer-use-meeting-assistant`
   - `pipes/archive/pipe-digital-clone`
   - `pipes/archive/pipe-find-leads-with-exa-ai`
   - `pipes/archive/pipe-keyword-notification`
   - `pipes/archive/pipe-llama32-comment-linear-while-you-work`
   - `pipes/archive/pipe-obsidian-time-logs`
   - `pipes/archive/pipe-screen-time-storyteller`
   - `pipes/data-table`
   - `pipes/identify-speakers`
   - `pipes/linkedin_ai_assistant`
   - `pipes/memories`
   - `pipes/obsidian`
   - `pipes/pipe-email-exa-search`
   - `pipes/pipe-for-loom`
   - `pipes/pipe-notion-table-logs`
   - `pipes/pipe-post-questions-on-reddit`
   - `pipes/search`
   - `pipes/timeline`

2. An update to `pipes/memories/package.json`, where the version of `@screenpipe/js` was changed from `1.0.0` to `1.0.1`.

3. Changes in import statements within TypeScript files:
   - In `pipes/memories/src/app/api/settings/route.ts`, the import of `pipe` was changed from `@screenpipe/js/node` to `@screenpipe/js`.
   - In `pipes/obsidian/src/app/api/log/route.ts` and `pipes/obsidian/src/app/api/settings/route.ts`, similar updates were made to the `pipe` import statement, changing it from `@screenpipe/js/node` to `@screenpipe/js`. 

These changes primarily focus on the deletion of lock files and updating import paths for consistency or maintenance reasons.
</details>

------------------------------------------------------------------------

# Overall Summary of Changes

The provided git changes detail a series of updates and improvements for the "screenpipe-app" across various commits. Here’s a consolidated summary:

1. **Version and Dependency Updates:**
   - Incremental version updates in `Cargo.lock` and `Cargo.toml`, reflecting new patch releases and developments.
   - Addition of the `once_cell` dependency.

2. **Feature Enhancements:**
   - Implementation of a health check feature to monitor service health and integrate changes into the app's initialization.
   - Improvements in shortcut handling, creating UI components for managing shortcuts, and enhancing backend logic to support new functionalities.

3. **Refactoring and Code Optimization:**
   - Code reorganization and style improvements for clarity.
   - Optimization of the tray menu to update only when necessary, utilizing `once_cell`.

4. **Bug Fixes and Miscellaneous Improvements:**
   - Addressed casing issues and logging inaccuracies.
   - Improved process monitoring and automatic termination reliability on Windows, addressing specific issues (e.g., #926).
   - Minor fixes in unit tests and benchmarks, focusing on setup parameters and data structure optimizations.

5. **Documentation and Changelog Updates:**
   - Addition and updates of changelog entries to document changes made in each version.
   - Specific updates to changelogs to reflect new features, fixes, and improvements.

6. **Removed Debug Options and Deletions:**
   - Removal of a debug option in `sidecar.rs`.
   - Deletion of several lock files and adjustments to import paths in TypeScript files for maintenance.

Overall, these commits reflect a concerted effort to enhance the functionality, stability, and maintainability of the "screenpipe-app" project. They focus on new feature additions, especially regarding health checks and keyboard shortcuts, and include code optimizations and numerous fixes for robust application behavior.
