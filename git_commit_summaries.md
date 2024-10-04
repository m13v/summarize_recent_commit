# Git Commit Summaries

-----------------------------------------------------------------------
-----------------------------------------------------------------------
 
PRESS CMD+SHIFT+V TO VIEW IN MARKDOWN
 
_______________________________________________________________________
-----------------------------------------------------------------------
Total number of commits: 20

<details>
<summary>Summary for commit 1 (0c0f4c403c27a6b73055b781aac5e9a90a518f9d)</summary>

The git commit involves several changes aimed at optimizing the download and caching process for the Silero model, refining build configurations, updating dependencies, and making minor adjustments to the code. Here is a summary of the key changes:

1. **GitHub Actions Workflow (`release-app.yml`)**:
   - Uncommented build configurations for Intel-based Macs, Ubuntu, and Windows platforms in the workflow. This re-enables these build targets in the CI pipeline.
   - Increased V8 memory limit size from 4096 to 8192.

2. **Version Bump**:
   - Incremented the version of `screenpipe-app` from `0.2.90-beta` to `0.2.91`.

3. **`screenpipe-audio` Cargo.toml Updates**:
   - Added new dependencies: `dirs` version `5.0.1` and `lazy_static` version `1.4.0`.

4. **Code Refactoring for Model Download (`vad_engine.rs`)**:
   - Refactored the VAD engine to use a caching mechanism, downloading the Silero model only once and storing it in a cache directory.
   - Introduced `lazy_static` and `tokio::sync::Mutex` to manage the path and state of the model download operation.
   - Improved logging from `debug` to `info` for certain operations.

5. **Miscellaneous Code Cleanups**:
   - Removed unused imports in `screenpipe-core/src/pipes.rs`, simplifying the module dependencies.

Overall, these changes enhance efficiency by reducing redundant downloads, improve build support across platforms, and tidy up unused code.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 2 (e2045a2f6ee277847f7d2e8a15a6efced67fc7fd)</summary>

The commit `e2045a2f6ee277847f7d2e8a15a6efced67fc7fd` is a merge commit by Louis Beaumont, done on October 4, 2024. It merges changes from a pull request (#428) submitted by a user named charpeni. The purpose of this pull request was to fix typos in the `CONTRIBUTING.md` file.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 3 (549aa5fadc8264895457fad125b3baf962687432)</summary>

This commit indicates a merge of the `QuincySan-CN_support` branch into the current branch. The merge was committed by Louis Beaumont on October 4, 2024. During the merge, the decision was made to retain the existing `bun.lockb` file from the current branch.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 4 (989ab32365b520b1a23b2e26e01f22f0bbb24015)</summary>

The commit by Louis Beaumont on October 4, 2024, involved the deletion of two files in the `screenpipe-app-tauri` project. The files removed were:

1. `bun.lockb` - a binary file used by the Bun toolkit, commonly associated with project dependency locks.
2. `acl-manifests.json` - a JSON configuration file containing detailed permission settings and schemas for different components and functionalities within a Tauri-based application. This file defined permissions for resources like notifications, filesystem access, process management, and more.

The commit was broadly labeled as a "fix," suggesting these deletions were part of a corrective measure in the project's development process.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 5 (325c68384e522c4023a73569e74c6dff846c23fc)</summary>

The git commit with hash `325c68384e522c4023a73569e74c6dff846c23fc`, authored by Nicolas Charpentier, makes two minor textual corrections in the `CONTRIBUTING.md` file. The word "personnally" was corrected to "personally," and the lowercase "i" in "i use" was corrected to "I use."
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 6 (b9e3bdf1d8905668a4056ed29b5810d08f0ea91c)</summary>

The git commit with hash `b9e3bdf1d8905668a4056ed29b5810d08f0ea91c` involves the deletion of the file `acl-manifests.json` from the `screenpipe-app-tauri/src-tauri/gen/schemas` directory. The author of the commit is Ren, and the deletion was carried out as requested on October 4, 2024. The file contained detailed configurations for various permissions related to autostart, CLI, app settings, events, notifications, file system, and other core functionality described in JSON. The removal indicates that these configurations are no longer needed or are being replaced.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 7 (6b6607289c1a712ee93f223b9968e1924fdbd676)</summary>

This commit includes the following changes:

1. **Cargo.toml Modifications:**
   - The `axum` dependency version "0.7" was removed from both `Cargo.toml` files located in the root and `screenpipe-vision` directories.

2. **New File Added:**
   - A new JSON file named `acl-manifests.json` was created under `screenpipe-app-tauri/src-tauri/gen/schemas/`. This file contains a comprehensive set of permission configurations for various application features such as auto-start, CLI interactions, application core functionalities, event actions, and more.

3. **screenpipe-vision's Cargo.toml Adjustments:**
   - In the `screenpipe-vision/Cargo.toml`, the specification of the `version`, `authors`, `description`, and `repository` fields were modified to refer to the workspace settings instead of specific values.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 8 (6093aa445f941843a169f527666c810f59698b66)</summary>

The commit "try fix" by Louis Beaumont updates the `entitlements.plist` file in the `screenpipe-app-tauri` project. Specifically, it adds three new entitlements related to Apple's security settings:

1. `com.apple.security.cs.allow-jit`: Allows just-in-time (JIT) compilation.
2. `com.apple.security.cs.allow-unsigned-executable-memory`: Allows the use of unsigned executable memory.
3. `com.apple.security.cs.disable-library-validation`: Disables library validation to allow dynamically loaded unsigned code bundles.

These changes are likely intended to provide the application with greater flexibility in executing code, possibly to support functionality related to the Deno runtime.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 9 (d6ba7a68ce89ee694f1dfd763fb76e624ed7f403)</summary>

The git commit with hash `d6ba7a68ce89ee694f1dfd763fb76e624ed7f403` by Louis Beaumont introduces changes to the `screenpipe-core/src/pipes.rs` file. The main update involves modifying the V8 JavaScript engine's memory configuration parameters within a function. Previously, the code set the maximum old space size to 4GB using a single `heap_limits` method call. 

The updated code now breaks down the memory configuration into two separate values: 
- `max_old_space_size` is adjusted to 2GB.
- `initial_old_space_size` is set to 512MB.

These values are then used to set the heap limits in the V8 `CreateParams`. Additionally, a new platform parameter, `v8_platform`, which uses a default V8 platform configuration, is introduced and added to `RuntimeOptions`. These adjustments aim to control memory usage more precisely.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 10 (a288b17866815c00e90147adc2c6c6be3ca06a15)</summary>

**Git Commit Summary:**

**Commit Hash**: a288b17866815c00e90147adc2c6c6be3ca06a15  
**Author**: Louis Beaumont <louis.beaumont@gmail.com>  
**Date**: Thu Oct 3 20:06:28 2024 -0700  
**Commit Message**: try fix

**Changes Made**: 
1. **File Modified**: `screenpipe-core/src/pipes.rs`

**Key Modifications**:
- **Imports Added**:
  - Added several `deno_core::v8` related imports: `Platform`, `SharedRef`, and `CreateParams`.
  - Added import for `deno_core::RuntimeOptions`.
- **Logic Changes**:
  - Introduced `platform_params` for setting up V8's heap limits using `v8::CreateParams`, with max old space size specified as 4GB.
  - Updated the creation of the `JsRuntime` to include `create_params`, allowing for custom V8 configuration.
- **Purpose**: These changes appear to be aimed at configuring the memory management behavior of the V8 JavaScript engine used within the Deno runtime environment, particularly by setting heap limits for performance and resource management.

Overall, the commit attempts to fix or optimize the execution environment by modifying the memory allocation settings through newly introduced parameters and imports.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 11 (524b7a252b4affd7562c37d5d6be01c38fbc8882)</summary>

The commit with hash `524b7a252b4affd7562c37d5d6be01c38fbc8882`, made by Louis Beaumont, renames files in different directories of example TypeScript projects, changing their names from `main.js` to `pipe.js`. The change note mentions that this update is "untested." The contents of the files remain unchanged as indicated by the similarity index of 100%.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 12 (390633ab28297c547a91ffcb90c953e4679d481e)</summary>

The commit by Louis Beaumont adds a new step to the GitHub Actions workflow file `release-app.yml`. Specifically, it introduces a step to upload build artifacts. This step is conditional, only executing when the build matrix specifies the `macos-latest` platform and the build arguments include targeting `aarch64-apple-darwin` with the `metal` features. The artifacts are uploaded using the `actions/upload-artifact@v3` action, and they are named `build-artifacts-2`, with the specified path for the artifacts being `./screenpipe-app-tauri/src-tauri/screenpipe-aarch64-apple-darwin`.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 13 (4d075d6f7c86b69ffaa67b0b0c81d610560412af)</summary>

The commit refactors a GitHub Actions workflow configuration file, `.github/workflows/release-app.yml`. Specifically, it uncomments a job named `draft` that was previously commented out. This job runs on the latest Ubuntu environment and consists of steps to check out the repository and create a draft release using the `crabnebula-dev/cloud-release` GitHub Action. The `draft` job makes use of some secrets, namely `CN_APP_SLUG` and `CN_API_KEY`, to execute the `release draft` command with the `tauri` framework. The `publish-tauri` job seems to be dependent on the `draft` job, although the `needs` parameter for the dependency is commented out in this commit.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 14 (cc12df6fd9e15f3b94133adeb4beba5affb26bef)</summary>

The commit made by Louis Beaumont on October 3, 2024, includes changes to the GitHub Actions workflow file `release-app.yml`. The notable changes in this commit are:

1. **Modification of Comments:**
   - A comment related to the Windows ARM platform was adjusted for formatting clarity.

2. **Uncommented Sections:**
   - Several previously commented-out sections related to build processes, dependency installation, and artifact uploading were uncommented:
     - A pre-build script is run to prepare the CLI.
     - Usage of `esaxx` is identified on Windows using `cargo tree`.
     - Installation of `vcpkg` for Windows builds.
     - Setup of `MSVC` and installation of LLVM & Clang on Windows.
     - The entire build process is now active using the `tauri-apps/tauri-action`.
     - Assets are being uploaded to CrabNebula Cloud.

3. **Conditional Artifact Upload:**
   - The upload of build artifacts is conditioned specifically for macOS with certain build arguments: targeting `aarch64-apple-darwin` with `metal` features.

This commit essentially re-enables previously disabled workflow steps to ensure they are executed when the build runs, particularly addressing different platform configurations.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 15 (4433eb236fecbc35e280210c473c193fcdedc235)</summary>

The commit, authored by Louis Beaumont, makes a change to the GitHub Actions workflow file located at `.github/workflows/release-app.yml`. Specifically, it comments out a line of code that previously set the `RUSTFLAGS` environment variable within a conditional block for the `macos-latest` platform. The `RUSTFLAGS` previously configured linker arguments related to dynamic library paths, but after this change, those flags will no longer be set during the build process.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 16 (9b38ecc8974f8d9b182e44765299992fc265f3b0)</summary>

The commit made by Louis Beaumont updates the GitHub Actions workflow file `.github/workflows/release-app.yml`. Specifically, the changes include:

1. Modifying the `cargo build` command within a job to include the `--release` flag, which means the project will be built in release mode instead of the default debug mode.

2. Updating the path for the upload of build artifacts to point to the release directory instead of the debug directory. The path changes from `./target/aarch64-apple-darwin/debug/screenpipe` to `./target/aarch64-apple-darwin/release/screenpipe`.

These changes adjust the workflow to build and upload release-mode artifacts.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 17 (f49141d12c7df710b9a2602aeda99799d0585019)</summary>

The `release-app.yml` GitHub Actions workflow file was updated in the following ways:

1. The entire `draft` job was commented out. This job originally ran on `ubuntu-latest` and included steps to check out the code and create a draft release using the `crabnebula-dev/cloud-release` action, leveraging secrets for the app slug and API key.

2. The dependency on the `draft` job was commented out in the `publish-tauri` job. This change means that `publish-tauri` no longer depends on the completion of the `draft` job.

3. In the `publish-tauri` job, the path for the artifacts uploaded by the `actions/upload-artifact@v3` step was modified. The path changed from `./target/debug` to `./target/aarch64-apple-darwin/debug/screenpipe`, indicating a specific target architecture for the build artifacts.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 18 (5e9f427c16c60935d801181a239cac03d9c35bfc)</summary>

The commit `5e9f427c16c60935d801181a239cac03d9c35bfc` made by Louis Beaumont on October 3, 2024, involves the deletion of the file `acl-manifests.json` located in the `screenpipe-app-tauri/src-tauri/gen/schemas/` directory. This file contained JSON data related to various permissions, commands, and configurations across different modules such as `acr`, `cli`, `core:app`, `core:event`, `core:image`, and others. Each module specified default permissions and detailed structures for granting or denying permissions to different commands and functions. The removal signifies that this schema configuration file is no longer needed in the project.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 19 (63b92d0bf86a2cc1e8dbcf796ff1445e0e2ec828)</summary>

The commit includes several changes to a Rust project and its components, focusing on adding support for OCR (Optical Character Recognition) of Chinese text. Here are the main changes:

1. **`Cargo.toml` Updates:**
   - Added the `axum` dependency version "0.7" to the main Cargo.toml file and also to `screenpipe-vision`'s Cargo.toml.
   - Updated the `tower` plugin from version "0.4" to version "0.5" with the "util" feature in `screenpipe-server`'s Cargo.toml.
   - Changed the version of `screenpipe-vision` from `{ workspace = true }` to "0.1.93".

2. **Source Code Changes in `screenpipe-vision`:**
   - Modified the `performOCR` function in `ocr.swift` to adjust image preprocessing. Commented out some filters to return directly the original image (`ciImage`). Enhanced text request capabilities by adding support for Chinese languages ("zh-Hans", "zh-Hant") and enabling language correction.

3. **Testing Enhancements:**
   - Added a new test `test_apple_native_ocr_chinese` in `screenpipe-vision/tests/apple_vision_test.rs` to verify OCR functionality with a Chinese test image.
   - Introduced a new test image file `testing_OCR_chinese.png` for the above test.

4. **Binary Files:**
   - The binary library files (`libscreenpipe.dylib`, `libscreenpipe_arm64.dylib`, `libscreenpipe_x86_64.dylib`) were updated, possibly recompiled with the changes.

These changes collectively aim to enhance the OCR capabilities of the project, especially for Chinese text recognition, by updating dependencies, fine-tuning the OCR pipeline, and introducing specific tests.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 20 (6478ff6c2962c3ba28fe5f116c57cf39dde366ea)</summary>

This commit includes several changes across multiple files in the `screenpipe-app-tauri` project:

1. **Binary File Change**: 
   - The binary file `bun.lockb` in the `screenpipe-app-tauri` directory was modified.

2. **Updates to `package.json`**:
   - A new field `"trustedDependencies"` was added with `@sentry/cli` as a trusted dependency.

3. **Changes to `acl-manifests.json`**:
   - The file contents were replaced entirely. The replacement still includes configurations for permissions and permission sets within the application, with a detailed JSON specifying various permission-related settings.

4. **Updates to JSON Schema Files**:
   - In both `desktop-schema.json` and `macOS-schema.json`, the word "programmatic" was corrected to "programatic" in the "Capability" object descriptions. 

These changes indicate an initial setup of a new project (`初始提交` means "initial commit" in Chinese), with updates that involve dependency management, permission configurations, and schema documentation corrections.
</details>

------------------------------------------------------------------------

# Overall Summary of Changes

The commits involve a variety of changes across different aspects of a project, primarily focusing on optimizing processes, updating configurations, and making textual corrections. Here's a concise summary of key updates:

1. **GitHub Actions Workflows**:
   - Re-enabled build configurations for multiple platforms, increased memory limits, and added artifact upload steps for specific macOS builds.
   - Commented out or modified various sections to streamline building and releasing processes.

2. **File and Dependency Modifications**:
   - Updated `Cargo.toml` files to adjust dependencies, focusing on introducing new packages like `dirs` and `lazy_static`, and removing or modifying existing ones like `axum`. 
   - Adjustments ensure support for OCR, particularly with Chinese text.

3. **Code Adjustments**:
   - Refactored model download logic to reduce redundancy by implementing caching.
   - Increased logging verbosity for better debugging and visibility.

4. **Typos and Textual Fixes**:
   - Corrected minor typographical errors in documentation files like `CONTRIBUTING.md`.

5. **File Deletions and Renames**:
   - Deletions of configuration files such as `acl-manifests.json`, suggesting either redundancy or a shift in configuration approach.
   - Renaming of JavaScript files from `main.js` to `pipe.js` in TypeScript projects.

6. **Security and Permissions**:
   - Updated macOS entitlements to enable certain security capabilities that support Deno runtime execution.

7. **Memory and Performance Optimization**:
   - Adjusted V8 JavaScript engine memory settings to better manage resources, particularly for old space and initial space sizes.

Overall, these changes aim to enhance the build process, improve efficiency through caching, and ensure smoother functionality across platforms while responding to project needs like language recognition and security flexibility.
