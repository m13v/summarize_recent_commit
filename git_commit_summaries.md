# Git Commit Summaries

-----------------------------------------------------------------------
-----------------------------------------------------------------------
 
PRESS CMD+SHIFT+V TO VIEW IN MARKDOWN
 
_______________________________________________________________________
-----------------------------------------------------------------------
Total number of commits: 35

<details>
<summary>Summary for commit 1 (e525abfa8e086d7cd1ecdc8f013efaa246a26499)</summary>

The commit identified by hash `e525abfa8e086d7cd1ecdc8f013efaa246a26499`, authored by Louis Beaumont on August 12, 2024, updates the version number of the `"screenpipe-app"` package in the `Cargo.toml` file from `0.1.37` to `0.1.38`. This change was made to force a new release due to internet issues. No other modifications were made.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 2 (a82ab0c79a2e2a03f2f1e6711b1c1c663c833e4b)</summary>

The commit `a82ab0c79a2e2a03f2f1e6711b1c1c663c833e4b` includes the following changes:

1. **Code Enhancements in `chat-message-v2.tsx`:**
   - Imported `memo` from React to optimize the `VideoComponent`.
   - Removed background color classes for code blocks, fixing a weird color issue.
   - Refactored `VideoComponent` to use `memo`, which prevents unnecessary re-renders and fixed a blinking issue with video embedding.
   - Added missing cleanup for the URL object in the `useEffect` cleanup return function.

2. **Version Update in `Cargo.toml`:**
   - Updated the
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 3 (2449a755fb64223fdec0921eaff16696b7e143a2)</summary>

The commit `2449a755fb64223fdec0921eaff16696b7e143a2` by Louis Beaumont addresses a fix related to the `media-src` directive in a Tauri application's configuration. Here are the changes summarized:

1. **Version Update**:
    - The version in `Cargo.toml` was updated from `0.1.35` to `0.1.36`.

2. **Configuration Fix**:
    - In `tauri.conf.json`, the `media-src` directive was updated.
    - The original `media-src` value: 
      ```json
      "'self' mediadevices: asset: http://asset.localhost file:"
      ```
    - The updated
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 4 (fc394dd0f2bacd2a5c8cf0692c59cf9225e30798)</summary>

The commit `fc394dd0f2bacd2a5c8cf0692c59cf9225e30798` authored by Louis Beaumont on August 12, 2024, includes the following changes:

1. **TypeScript Fixes**:
   - Replaced `// @ts-expect-error` annotations with `// @ts-ignore` in several locations within the `chat-list-openai-v2.tsx` and `function-call-message.tsx` files.
   - This change might be aimed at suppressing TypeScript errors more broadly without specifying expected errors.

2. **Version Update**:
   - Bumped the version number of the `screenpipe-app` from `0.1.34` to
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 5 (a22bc68910e87de2d2457cb52152960a18d6b5f1)</summary>

The commit with hash `a22bc68910e87de2d2457cb52152960a18d6b5f1` was made by Louis Beaumont on August 12, 2024. The commit message indicates a fix related to the last error type message.

Changes were made to the file `chat-list-openai-v2.tsx` located in `examples/apps/screenpipe-app-tauri/components/`. Specifically, lines were added that include `// @ts-expect-error` comments. These comments indicate that TypeScript errors are expected and explicitly acknowledged at various places where `setMessages`, message handling, and rendering logic (for `ChatMessage` and `FunctionCallMessage` components) occur. This seems to
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 6 (1be7b442f71a95adf84082f5a45091a5a981f073)</summary>

The commit made by Louis Beaumont on August 12, 2024, updates the version of the "screenpipe-app" package in the `Cargo.toml` file. Specifically, the version number is incremented from "0.1.33" to "0.1.34". This change aims to bump the Tauri version used in the project, although no specific details about Tauri itself are included in this commit.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 7 (2d64d3e7433d1538072edeaf16a28304d72ddb92)</summary>

### Summary of Git Changes:

**Commit:** 
- **ID:** 2d64d3e7433d1538072edeaf16a28304d72ddb92
- **Author:** Louis Beaumont <louis.beaumont@gmail.com>
- **Date:** Mon Aug 12 14:40:36 2024 +0200
- **Message:** fix: ux on chat + onboarding video

**Key Changes:**

1. **File:** `examples/apps/screenpipe-app-tauri/app/page.tsx`
   - **Changes:**
     - Imported `Link` from "next/link".
     - Added an HTML link to an onboarding video in the `CardDescription` component.

2. **File:**
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 8 (10cd59e1e0adbf41806fb8a34355c1f44c758841)</summary>

The commit identified as `10cd59e1e0adbf41806fb8a34355c1f44c758841` is a merge commit that integrates changes from two parent commits (`1280dee` and `3239c36`). Authored by Louis Beaumont on August 12, 2024, this merge incorporates a pull request (#161) from the user `lucasfernog-crabnebula`. The primary purpose is to fix the continuous integration (CI) process by creating the CrabNebula Cloud release in a separate job.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 9 (3239c36500d266c95f6ea73294113725d793d6e3)</summary>

The commit by Lucas Nogueira modifies the GitHub Actions workflow configuration for releasing the application. The main change is the splitting of the release process into separate jobs within the workflow file `.github/workflows/release-app.yml`. 

Key changes include:
1. **New Draft Job**:
    - A new job named `draft` is introduced to create a draft release using the `crabnebula-dev/cloud-release` Action.
    - This job runs on `ubuntu-latest` and depends on secrets for `CN_APP_SLUG` and `CN_API_KEY`.
  
2. **Modification of the `publish-tauri` Job**:
    - The `publish-tauri` job now depends on the `draft` job (`needs:
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 10 (1280deeec202859cdf320e89bd3c170abeb59784)</summary>

The commit `1280deeec202859cdf320e89bd3c170abeb59784`, authored by Louis Beaumont on August 12, 2024, with the message "chore: bump tauri app," updates the version of the `screenpipe-app` in the Cargo.toml file. Specifically, it changes the version from `0.1.32` to `0.1.33`.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 11 (374dadf5a351c3525d2eb590cc384b2d796a59d4)</summary>

The commit `374dadf5a351c3525d2eb590cc384b2d796a59d4` authored by Louis Beaumont on Mon Aug 12 14:03:11 2024 made several changes primarily aimed at fixing video embedding and updating various dependencies. Here's a summary of the key changes:

1. **GitHub Workflow Updates:**
   - `.github/workflows/release-app.yml`: 
     - Updated the version of `crabnebula-dev/cloud-release` from `v0.1.0` to `v0.2.0` across several workflow steps.

2. **Component Updates:**
   - `chat-list-openai-v2.tsx`: 
     - Updated imports
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 12 (2251d4284add78cf50b6e4e0a360dd4e20ae4649)</summary>

In this commit, several changes were made to the codebase, focusing on correcting function argument names related to time parameters across various files in the project. Specifically, the modifications involved renaming `start_date` and `end_date` to `start_time` and `end_time` respectively. This update was applied consistently throughout the codebase, including TypeScript files, JavaScript files, and Rust server code. Additionally, minor improvements and fixes, such as the addition of console logs for debugging and better handling of query parameters, were also integrated.

**Key changes include:**

1. **Function Argument Updates**:
   - Renamed `start_date` to `start_time` and `end_date` to `end_time` across various TypeScript
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 13 (c04f87442e93f42ac416da20204efa41a889df19)</summary>

The commit (c04f87442e93f42ac416da20204efa41a889df19) is a merge commit by author Louis Beaumont, carried out on August 12, 2024. It merges a pull request (#158) from branch `louis030195/update-formula-aarch64-apple-darwin-673280e88f811ec6713b0bd8a0727d858ae3ed80`. The purpose of the pull request was to update the Homebrew formula for the `aarch64-apple-darwin` platform. Two commits (db1636a and 2943600) were merged as part of this process.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 14 (2943600923cd4a995e4e52a7b1048a9cc8034ab2)</summary>

This commit represents a merge from the 'main' branch into the 'update-formula-aarch64-apple-darwin-673280e88f811ec6713b0bd8a0727d858ae3ed80' branch. The key change in this merge affects the `Formula/screenpipe.rb` file, specifically updating the SHA256 checksums for downloading the `screenpipe` binaries.

### Key Changes:
1. Updated the SHA256 checksum for the `aarch64-apple-darwin` binary from:
   ```sha256 "9816c964a059abab8e2cef8e85c91b734dc27fb7ba46cc7cbdb49db5f13d17
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 15 (db1636a674a238f8d85e2cfd56f7e904c6b1823b)</summary>

This commit represents a merge operation where Louis Beaumont integrated changes from pull request #159 into the main branch. The pull request involved an update to the Homebrew formula specifically for the x86_64-apple-darwin platform. The commit is a combination of changes from two parent commits—identified by hashes `673280e` and `575162c`.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 16 (575162c60d54135d3e236923ed50c4a3cd1e3213)</summary>

The commit with the hash `575162c60d54135d3e236923ed50c4a3cd1e3213` made by the GitHub Actions Bot on Mon Aug 12, 2024, updates the Brew formula for `screenpipe` from version 0.1.60 to version 0.1.61 specifically for the x86_64-apple-darwin platform. 

The changes include:
- Updating the version from "0.1.60" to "0.1.61".
- Updating the sha256 checksum for the x86_64-apple-darwin tar.gz file to "22ce24afea2c806df743469368fa1d78172
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 17 (449788b47104f3f45acb127fd0691e9d10c55549)</summary>

This commit updates the `screenpipe` formula in the Homebrew repository. Key changes include:

- Bumped the version of `screenpipe` from 0.1.60 to 0.1.61.
- Updated the download URL to reflect the new version.
- Changed the SHA256 checksum for the ARM architecture (aarch64-apple-darwin) to ensure integrity and security of the new version. 

</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 18 (673280e88f811ec6713b0bd8a0727d858ae3ed80)</summary>

The commit `673280e88f811ec6713b0bd8a0727d858ae3ed80`, authored by Louis Beaumont, updates version numbers in two files:

1. **Cargo.toml** in the root directory:
   - The version is updated from "0.1.60" to "0.1.61".

2. **Cargo.toml** in the `examples/apps/screenpipe-app-tauri/src-tauri` directory:
   - The version is updated from "0.1.31" to "0.1.32".

The purpose of these changes is a version bump for the respective packages.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 19 (e653aa33848d1e509e28864b0277170e6cbdb4de)</summary>

**Summary:**

This Git commit, authored by Louis Beaumont, addresses a crash issue in text chunking functionality when processing Chinese characters. 

**Key Changes:**
1. **Source Code Adjustments (`screenpipe-server/src/chunking.rs`):**
   - Reformatted import statements for consistent style.
   - Improved error handling when initializing the `Device`.
   - Reformatted code for better readability and consistency.
   - Altered the `text_chunking_simple` function to handle Chinese characters correctly by:
     - Collecting characters into a vector before chunking.
     - Removing the previous method of ensuring valid character boundaries, which was problematic for multi-byte character sets like Chinese.
     - Adjusting the chunking logic to work with the
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 20 (8c6bbe0287f072d290eecfb3be865372a6a3fdec)</summary>

The commit 8c6bbe0287f072d290eecfb3be865372a6a3fdec, authored by Louis Beaumont, addresses an issue related to the Windows Continuous Integration (CI) setup. 

The specific changes made in the file `screenpipe-vision/tests/windows_vision_test.rs` include:
1. Adding the `screenpipe_vision::monitor::get_default_monitor` import to the test module.
2. Modifying the creation of a mock monitor within the test to use `get_default_monitor().await.id()` instead of `get_monitor().await`.

These changes aim to fix issues with the Windows-specific tests in the CI pipeline by ensuring the correct monitor is retrieved and used in the tests.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 21 (be3cdeca025cc9da688cb3f1418a4ff04b59e17c)</summary>

The commit `be3cdeca025cc9da688cb3f1418a4ff04b59e17c` is a merge commit authored by Louis Beaumont on August 12, 2024. This merge integrates changes from pull request #156, specifically updating the Homebrew formula for the `x86_64-apple-darwin` platform.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 22 (85db448d104020b430d6557b3f16445c97722795)</summary>

The commit identified by hash `85db448d104020b430d6557b3f16445c97722795` merges changes from the `main` branch into a branch named `update-formula-x86_64-apple-darwin-e42532ec2aaef0d5503313d629acb0491ffd6a8c`. This merge was conducted on August 12, 2024, by Louis Beaumont.

The specific changes were made to the file `Formula/screenpipe.rb`, particularly within the macOS-specific section of the formula:

1. For arm64 architecture (Apple silicon), the `sha256` checksum for the corresponding tar.gz file was updated from `f8d10d
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 23 (8995c65c79778584234be8957e5a5e2a3c3fe4d6)</summary>

The git changes in commit `8995c65c79778584234be8957e5a5e2a3c3fe4d6` record a merge that incorporates updates from a pull request (#157). The primary purpose of this merge is to update the Homebrew formula for the `aarch64-apple-darwin` architecture. The merge consolidates changes from commit `e42532e` and `189dba7`, as conducted by the author, Louis Beaumont, on August 12, 2024.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 24 (189dba77122176cc199b39e94dbf0a9ac619862f)</summary>

This Git commit updates the formula for the Screenpipe library in the Homebrew package manager to version 0.1.60 for `aarch64-apple-darwin`. The key changes are:

- Updated the URL to refer to version 0.1.60 of the Screenpipe library.
- Updated the version number from 0.1.59 to 0.1.60.
- Updated the SHA-256 checksum for the arm64 version to ensure the integrity of the downloaded file.

Overall, the changes ensure that the formula correctly points to the new version of Screenpipe with the appropriate checksum for the arm64 architecture.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 25 (90a9e06d3af3e32facbd926d5f030d5f6b0bd4f5)</summary>

The commit `90a9e06d3af3e32facbd926d5f030d5f6b0bd4f5` authored by the GitHub Actions Bot on August 12, 2024, updates the `screenpipe` formula in the Homebrew repository.

**Changes made:**
- The version of the `screenpipe` formula is updated from `0.1.59` to `0.1.60`.
- The SHA256 checksum for the x86_64 binary for macOS is updated to reflect the new version:
  - Previous checksum: `3dd4a732ab1f4b5993571c4132c52c81389a1f1ab5
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 26 (e42532ec2aaef0d5503313d629acb0491ffd6a8c)</summary>

The commit `e42532ec2aaef0d5503313d629acb0491ffd6a8c` by Louis Beaumont on August 12, 2024, addresses several fixes related to the GitHub Actions workflow and project versioning:

1. **GitHub Actions Workflow (`release-cli.yml`):**
   - The `RUSTFLAGS` environment variable was moved from a global environment definition to being defined directly in the `Build with Metal feature` step.
   - An additional line was added to copy the `libscreenpipe.dylib` file to the release directory for all targets, ensuring the necessary library is included.

2. **Project Version Bump (`Cargo.toml`):**
   - The
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 27 (4d7acbf99673a7b36ba46d325c409a04bd38cc5e)</summary>

The provided git changes involve a merge commit authored by Louis Beaumont, dated August 12, 2024. This commit merges the pull request #154 from the branch `louis030195/update-formula-x86_64-apple-darwin-be21c3c27f7d8aaccec91fba01b8ea73b0f92282`. The purpose of the pull request is to update the Homebrew formula specifically for the `x86_64-apple-darwin` platform.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 28 (13cd994103406e68f30fa02a72b1efd921b176ca)</summary>

The changes involve merging the 'main' branch into the 'update-formula-x86_64-apple-darwin-be21c3c27f7d8aaccec91fba01b8ea73b0f92282' branch in the repository. The updated file is `Formula/screenpipe.rb`, specifically focusing on updating the SHA256 checksums for different architecture downloads of the `screenpipe` software.

- For ARM-based macOS systems (`if Hardware::CPU.arm?`), the SHA256 checksum is updated from `ac7ee831cb4ba85915d7a563d5f8462634e4939c8a4648e2ff94d4f994e73da9
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 29 (7f94b50c21fdf4918992890941642cb0d5ce567e)</summary>

The git commit `7f94b50c21fdf4918992890941642cb0d5ce567e` is a merge commit by Louis Beaumont, made on August 12, 2024. It incorporates changes from pull request #155 by the user `louis030195`. The purpose of the merge is to update the Homebrew formula specifically for the `aarch64-apple-darwin` architecture. The changes include commits from branches `33cd451` and `3b51e27`.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 30 (3b51e27acfdc60e8814f8156d6bb2c16e3663938)</summary>

The git commit with hash `3b51e27acfdc60e8814f8156d6bb2c16e3663938` involves a minor update to the screenpipe Homebrew formula. The changes include:

- Updating `screenpipe` to version `0.1.59` from `0.1.57`.
- Modifying the download URL to reflect the new version `0.1.59`.
- Updating the SHA-256 checksum for the `aarch64-apple-darwin` architecture to ensure the integrity of the new download.

The commit was made by the GitHub Actions Bot on August 12, 2024, as part of a chore to keep the Homebrew formula up to
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 31 (c566362e1081a1af7d06b13348272cf75bea4218)</summary>

This commit updates the Homebrew formula for the "screenpipe" library. The changes include:

1. **Version Update**: The version of the "screenpipe" library is updated from `0.1.57` to `0.1.59`.
2. **Checksum Update**: The SHA256 checksum for the x86_64-apple-darwin version of the tarball is updated to reflect the new version.

The purpose of this update is routine maintenance, ensuring that users who install this library via Homebrew receive the latest version.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 32 (33cd451a0d781a387991f3b5f79d7704fc881588)</summary>

The commit made by Louis Beaumont on August 12, 2024, updates the version of the `screenpipe-app` project within the `Cargo.toml` file from `0.1.30` to `0.1.31`. This change is likely to indicate a minor update or release for the application. The author and other metadata such as description and license remain unchanged.
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 33 (be21c3c27f7d8aaccec91fba01b8ea73b0f92282)</summary>

The git commit identified by hash `be21c3c27f7d8aaccec91fba01b8ea73b0f92282`, authored by Louis Beaumont on August 12, 2024, involves updating the version number of the project in the `Cargo.toml` file. The version was changed from "0.1.57" to "0.1.59".
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 34 (3d8b3a1eca9ef1bd9badc653def61bba65c050fa)</summary>

The commit titled "fix: audio crash when computer awake from long sleep time by catching error better" by Louis Beaumont includes the following changes:

### README.md
- Updated the usage section to include a new "--ocr-engine apple-native" flag for using Apple native OCR.
- Adjusted the instructions for disabling audio recording to improve clarity.

### pcm_decode.rs
- Changed an `expect` call to `ok_or_else` with a custom error message to handle cases where no supported audio tracks are found, thus preventing a crash.

### stt.rs
- Reformatted code to improve readability by adjusting the line breaks.
- Enhanced error logging to provide clearer messages when interacting with the Deepgram API.
- Updated various `info` and `debug`
</details>

------------------------------------------------------------------------

<details>
<summary>Summary for commit 35 (417a4990cba1ff2bea1c5cb7367f461e463388b0)</summary>

The commit with hash `417a4990cba1ff2bea1c5cb7367f461e463388b0`, authored by Louis Beaumont, addresses a bug that caused the application to crash when the computer woke from sleep and encountered unusual screen capture sizes. 

**Key Changes:**

1. **Error Handling in Compare Functions:**
   - Added error handling to `compare_with_previous_image` and `compare_images_histogram` functions.
   - Utilizes `anyhow::Result` to handle and propagate errors.
   - If `compare_with_previous_image` fails, it logs the error and assigns a default value (0.0).

2. **Modifications in Function Signatures:**
   - `compare_images
</details>

------------------------------------------------------------------------

# Overall Summary of Changes

Here's a summary of the provided git changes:

### Commit `e525abfa8e086d7cd1ecdc8f013efaa246a26499`
- **Author:** Louis Beaumont
- **Date:** August 12, 2024
- **Changes:** Updated version of `"screenpipe-app"` in `Cargo.toml` from `0.1.37` to `0.1.38` to force a new release due to internet issues. No other changes.

### Commit `a82ab0c79a2e2a03f2f1e6711b1c1c663c833e4b`
- **Changes:**
  1. **`chat-message-v
