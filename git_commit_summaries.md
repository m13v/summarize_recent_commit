# Git Commit Summaries

-----------------------------------------------------------------------
-----------------------------------------------------------------------
 
PRESS CMD+SHIFT+V TO VIEW IN MARKDOWN
 
_______________________________________________________________________
-----------------------------------------------------------------------
Total number of commits: 84

<details>
<summary>summary for commit 1 (6e9a4d2c38e03c1b979d5780cf24aa644815aa49)</summary>

This commit, authored by Louis Beaumont, includes several changes across different files in a project:

1. **`app/layout.tsx`**:
   - The `ClerkProvider` is updated with a new `publishableKey`, switching from a production to a test key. Additionally, a new `allowedRedirectOrigins` array is added, which specifies permits for `http://localhost:3000` and `tauri://localhost`.

2. **`Cargo.lock` and `Cargo.toml`**:
   - The version of the package `screenpipe-app` is incremented from `0.14.9` to `0.14.11` in `Cargo.lock`, and from `0.14.10` to `0.14.11` in `Cargo.toml`.

3. **`tauri.conf.json`**:
   - The Content Security Policy (CSP) sections are expanded with more detailed source lists.
   - For `default-src`, additional sources like `http://localhost:*` and specific clerk URLs are added.
   - `connect-src` list is expanded to include different domains like `clerk.dev` and specific clerk subdomains, ensuring wider connectivity capabilities.
   - For `img-src`, sources are restructured into an array, with additions like `clerk.dev` URLs.
   - `style-src` and `frame-src` are clearly segregated into arrays, allowing a detailed specification of permissible content sources.
   - `script-src` is expanded significantly to include multiple safe resource locations and clerk domain references.

Overall, the changes update security configurations, change the authentication setup, and increment versioning.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 2 (8f7c8842a6ff1aa6e28fbe4508ae1d020947113a)</summary>

The commit by Louis Beaumont with the hash `8f7c8842a6ff1aa6e28fbe4508ae1d020947113a` includes the following changes to the project:

1. **Modifications in `pre_build.js`:**
   - Added logic to handle setup for both `arm64` and `x86_64` architectures on macOS.
   - Introduced an array of architectures and iterated over each to perform respective setup actions.
   - For both architectures, checks for the most recent binary paths and copies the appropriate binary.
   - Utilizes `install_name_tool` to modify dylib paths for binaries depending on whether it's in development mode or not, with appropriate error handling.

2. **Changes in `build.rs`:**
   - Large sections of code related to copying binaries and modifying dylib paths for both `x86_64` and `aarch64` targets were commented out.
   - This seems to indicate a shift from handling these operations in `build.rs` to handling them within the updated `pre_build.js` script.

Overall, these changes focus on improving the build process, specifically for macOS environments, by managing binary setup and dynamic library path adjustments more dynamically and effectively in the pre-build step.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 3 (d978e35a56cf4e0596438dd9d2a5bb6d57e37d75)</summary>

This git commit by Louis Beaumont with the message "fix: build" includes several changes across multiple files in the `screenpipe-app-tauri` project:

1. **TypeScript/React Components:**
   - In `layout.tsx`, `auth.tsx`, `pipe-store.tsx`, and `stripe-subscription-button.tsx`, the import of Clerk has been switched from `@clerk/nextjs` to `@clerk/clerk-react`.
   - The `"use client";` directive was added to some of these files, indicating these components should run on the client side.

2. **Configuration:**
   - `next.config.mjs` was updated to include an `images` property with `unoptimized: true`.

3. **Dependencies:**
   - In `package.json`, the dependency on `@clerk/nextjs` was replaced with `@clerk/clerk-react`, and its version has been changed from `^6.5.1` to `^5.17.1`.

4. **Other Files:**
   - The `bun.lockb` file shows binary changes.
   - The version in `Cargo.toml` was incremented from `0.14.9` to `0.14.10`.

These changes appear to adjust the build to use a different Clerk package across components, add a client-side directive, adjust configuration settings for image optimization, and update versions to reflect these changes.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 4 (af2edf9e49cb2312f0a4b88fe1eac15d7590f8e7)</summary>

The commit `af2edf9e49cb2312f0a4b88fe1eac15d7590f8e7` by Louis Beaumont addresses a build issue by modifying the file `settings.tsx` in the `screenpipe-app-tauri/components` directory. Specifically, it removes the import statement for `ExternalAuthButton` from this file. No other changes are indicated in this commit.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 5 (22105fe48a3be858c7be95d901af0aec2d8e39f1)</summary>

The recent commit introduces several changes and enhancements to the project:

### Features:
1. **Authentication with Clerk**: 
   - Integrated `Clerk` for authentication in `screenpipe-app-tauri`. This is reflected through imports and usage in components like `layout.tsx`, `auth.tsx`, and `settings.tsx`.
   - Introduced `ClerkProvider` and related authentication components (`SignInButton`, `SignOutButton`) for handling user sign-in and sign-out, with specific UI updates to include Clerk's authentication.

2. **Stripe Subscription**:
   - Added a feature for Stripe subscription to enable specific functionalities, like using the "Loom pipe". This involves the integration of a new `StripeSubscriptionButton` component and handling of subscription states.

3. **UI and Component Updates**:
   - Adjustments to text and styling such as changing headings from title-case to lowercase.
   - Implementation of a subscription-based model to enable certain features based on user subscriptions.

4. **External Plugin & Service Integration**:
   - Introduced `posthog` for analytics within several components, allowing for capturing and identifying user actions and states.

### Fixes and Modifications:
1. **Pipe Store Command**: 
   - Fixed the method of fetching, enabling, disabling, and updating pipes to utilize HTTP requests over direct command execution, shifting from a direct command-line interface to a more network-service based approach.

2. **Removed Unused Configurations**:
   - Cleaned up Rust target-specific configurations in `.cargo/config.toml`, specifically removing unnecessary entries for Apple platforms.

3. **Dependency Management**:
   - Upgraded and added dependencies related to dual-auth integrations and Stripe payments within `package-lock.json` and `package.json`.
   - Removed unused packages and updated several `@radix-ui` packages to their latest versions.

### Build and Architecture:
1. **macOS Build Process**:
   - Simplified build scripts in `pre_build.js` and `build.rs` by removing redundant steps and updating the handling of macOS library paths (`dylibs`).

2. **Version Updates**:
   - Updated the project's version in `Cargo.toml` from `0.14.8` to `0.14.9`.

3. **Security Policies**: 
   - Updated the Content Security Policy (CSP) to allow connections and images from new domains related to Clerk and Stripe integration.

These changes collectively enhance authentication, improve user experience through membership subscriptions, and streamline the build and deployment process for macOS while integrating necessary updates to dependencies.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 6 (e90d426e76439120371218773588fcb32195d4d5)</summary>

The commit with hash `e90d426e76439120371218773588fcb32195d4d5` authored by Louis Beaumont has a commit message indicating that documentation was fixed. Specifically, in the file `content/docs/pages/docs/plugins.mdx`, an import statement was modified. The import `Tabs from '@/components/tabs'` was changed to `import { Tabs, Tab } from 'nextra/components'`. This likely corrects or updates the way Tabs are imported for usage in the documentation.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 8 (998686fe4c3ed2a1cfa25d822b944e67ee32724e)</summary>

The Git commit made by Louis Beaumont updates a few components in the `create-pipe` example project. Here is a summary of the changes:

1. **README Updates**: In the `README.md` file within the `examples/create-pipe` directory, the instructions for creating a new screenpipe pipe have been modified. The versions of the `@screenpipe/create-pipe` package for npm, bun, pnpm, and yarn have been changed from a specific version (`0.0.5`) to `latest`, indicating a shift to always use the most up-to-date version of the package.

2. **Version Bump**: The `version` in the `package.json` has been updated from `0.0.5` to `0.0.6`.

3. **Dependency Addition**: A new dependency, `@scarf/scarf` with version `^1.4.0`, has been added to the `package.json` file's list of dependencies.

4. **Binary File Change**: The binary file `bun.lockb` has been changed, although the exact changes aren't detailed in the diff since it’s a binary file.

Overall, this commit updates the `create-pipe` example to use the latest version of the `@screenpipe/create-pipe` package in instructions and bumps the package version while adding a new dependency.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 9 (b82bc824e3a2de2a4a176fdccab71c20092389a9)</summary>

The git changes summarized are as follows:

1. **Refactor in `pipe-store.tsx`:**
   - A new pipe configuration named "pipe-for-loom" was added to the `corePipes` array. This pipe is designed to automatically summarize Loom videos and add them to notes.
   - A button, previously present in the `PipeDialog` component, which allowed users to tweet about how they use the `@screen_pipe`, was removed. This button had a heart icon and the text "support us."

2. **Version Update in `Cargo.lock`:**
   - The version of the "screenpipe-app" package was incremented from 0.14.7 to 0.14.8, indicating a new version release.

Overall, these changes include adding a new pipe feature, removing a social media sharing button, and updating the application version.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 10 (029101d3330bc2090a6f898758c77a722f8f064f)</summary>

The git commit `029101d3330bc2090a6f898758c77a722f8f064f` introduces updates to the documentation for a feature named "plugins." The changes focus on enhancing the documentation page `plugins.mdx`:

1. **Imports**: Added an import statement for a `Tabs` component (`import Tabs from '@/components/tabs';`).

2. **Content Changes**:
   - Expanded the description of plugins, categorizing "pipes" into two types: 
     - **UI-based**: Uses NextJS for user interaction.
     - **Headless**: Operates in the background without a visual interface, potentially utilizing cron jobs.
   - Removed the previous step-by-step guide for setting up a pipe manually using git, copy, and bun commands.
   - Introduced a fast creation method using a CLI with multi-package manager support (npm, bun, yarn, pnpm). The CLI helps in setting up, testing, and enabling the pipe.

3. **Instructions**: A new section utilizing the `Tabs` component is added to demonstrate how to create a new pipe using different package managers, with commands wrapped in `copy` blocks for easy copying. Users are guided to follow the CLI instructions to set up and enable the pipe.

These updates streamline the process of creating and managing plugins, emphasizing the use of a CLI tool for simplicity and versatility, and highlighting the different ways pipes can be implemented within the system.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 11 (5f7b9ee603bc8ee2d6585bf66690403ec46ad917)</summary>

This commit introduces new functionality and changes across several files and directories. Here's a summary of the key changes:

1. **Configuration Changes**:
   - Removed unused configuration related to macOS in `.cargo/config.toml`.

2. **New `create-pipe` Example**:
   - Added a new example called `create-pipe` within the `examples` directory.
   - Created necessary files for this example:
     - `.gitignore`: Contains standard patterns for ignoring files related to Node.js and related build tools.
     - `README.md`: Instructions and setup for creating a new screenpipe pipe using Next.js.
     - `index.ts`: A new script to automate the creation of pipes with options for user interface and headless setups. It supports installation via npm, bun, pnpm, and yarn.
     - `package.json` and `tsconfig.json`: Configuration files for managing dependencies, scripts, and TypeScript options.

3. **Archiving Old Examples**:
   - Moved the `pipe-meeting-summary-by-email` example to an `archive` directory, indicating it's now considered legacy or not actively maintained.

4. **Updated `pipe-obsidian-time-logs`**:
   - Introduced a new `package.json` and `tsconfig.json` file.
   - Added a `.gitignore` for ignoring build and dependency directories.
   - Created a `bun.lockb` binary file for package-lock specific to Bun.js.

5. **Refactoring in `pipe-store.tsx`**:
   - Replaced HTTP requests for managing pipes (e.g., list, download, enable, disable) with sidecar commands using the `screenpipe` executable.
   - Enhanced error handling and user feedback via toast notifications.
   - Adjustments made to support more reliable pipe management and configuration saving.

6. **Version Bumps**:
   - Incremented the version numbers in `Cargo.lock` and `Cargo.toml` for `screenpipe-app` to reflect updates.

Overall, the commit enhances the `screenpipe` ecosystem by introducing new tools for creating customizable pipes and modernizes existing infrastructure with command-line integrations.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 12 (3153d0ce240f413f2b0728b2a684047b4cb10da1)</summary>

The commit made by Louis Beaumont on November 29, 2024, updates the `README.md` file in the `examples/typescript/pipe-for-loom` directory. The update involves a significant reduction in the README's content, removing detailed instructions and information about the Next.js project. These instructions covered setting up a development server, links to resources for learning about Next.js, and deployment guidance on Vercel. In place of this content, a brief line was added: "replace loom by this pipe," along with an image embedded into the document.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 13 (8e65cf26727edac36669e805d7b45844c9d778d5)</summary>

The recent commit by Louis Beaumont primarily involves updating the Windows getting-started documentation for setting up and building a project from source. Here's a summary of the changes made:

1. **CONTRIBUTING.md Updates:**
   - Added instructions to install CMake using winget.
   - Provided environment variable setup commands using PowerShell to ensure paths are set correctly for vcpkg, LIBCLANG_PATH, and include GNUWin32's bin in the PATH.

2. **Getting Started Guide Modifications in getting-started.mdx:**
   - Expanded step-by-step instructions on installing necessary tools and dependencies on Windows, including Visual Studio Build Tools, Rust, LLVM, CMake, unzip utility, Git, and Bun.
   - Provided directions for setting environment variables with PowerShell commands for PKG_CONFIG_PATH, VCPKG_ROOT, LIBCLANG_PATH, and PATH.
   - Updated instructions on cloning and setting up the vcpkg package manager, including integration and FFmpeg installation.
   - Clarified the steps to clone the repository and build the CLI and desktop application using Cargo and Bun.
   - Added sections detailing optional GPU acceleration using CUDA for NVIDIA GPUs and Intel's MKL for CPU optimization.

These changes aim to streamline the setup process and ensure users have clear guidance for building the application from source on Windows.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 14 (50ec5df4da69444507cef3bc05c71d01c00ec56a)</summary>

The Git changes in commit `50ec5df4da69444507cef3bc05c71d01c00ec56a` include updates and fixes across the `screenpipe-server` project. Here's a summary of the changes:

1. **Edge Case Tests and Search Bug:**
   - Improved edge case tests.
   - Fixed a bug related to the search functionality, specifically ensuring that searches for audio content only proceed when `app_name` and `window_name` criteria are not specified.

2. **Code Cleanup and Optimizations:**
   - Removed extra parentheses and unnecessary code (e.g., code removed in the server).
   - Standardized usage of SQL operations, such as removing additional spaces and ensuring consistent formatting (e.g., proper alignment of `SELECT`, `FROM`, `WHERE`, etc.).

3. **Conditional Logic Adjustments:**
   - Adjusted the logic in the `DatabaseManager` to prevent unnecessary search queries, such as filtering audio results unless certain criteria are absent.
   - Removed logic enforcing `ContentType` to OCR based on `app_name` or `window_name` in `server.rs`.

4. **Testing Enhancements:**
   - Added and modified database test cases to test new or adjusted functionalities.
   - Ensured tests account for different combinations of search parameters and confirm expected behavior across the search queries.

These changes aim to enhance functionality, clean up the codebase, and ensure robust testing and execution of search operations within the application.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 15 (aaa45d135c3023bad16ca9abab47c5cd7033a9a0)</summary>

This git commit introduces a new feature that adds a "pipe for loom". Here's a summary of the changes:

1. **Project Structure:**
   - A new example project named `pipe-for-loom` has been added under `examples/typescript/`.
   - Several configuration files are introduced, including `.eslintrc.json`, `.gitignore`, `package.json`, and `postcss.config.mjs`.
   - `tailwind.config.ts` and `tsconfig.json` set up Tailwind CSS and TypeScript respectively.

2. **Core Files:**
   - **API**: A file API route (`app/api/file/route.ts`) is implemented for fetching video or audio files, with error handling for file access permissions, existence, and type support.
   - **Components**: Numerous React components have been added, including `Pipe`, `Header`, `DateTimePicker`, `Toaster`, and various UI components for buttons, badges, dialogs, etc.
   - `global.css` applies global styles using Tailwind CSS and includes root color variables for both light and dark themes.

3. **Functionality:**
   - **Video and Audio Handling**: Ability to merge video and audio files using a server API. Time filters (last 30/60 mins, 12/24 hours) help select content duration for merging.
   - **UI Enhancements**: A user interface is provided to manage videos and audios, including setting start and end times using a date-time picker and viewing merged output in dialogs with video/audio players.
   - **Notifications**: A toast notification system is implemented to provide user feedback on actions like errors in merging.

4. **Server Logic:**
   - Videos are validated using `ffmpeg` for errors before attempting to merge.
   - Error handling is added in the server's video utilities (`src/video_utils.rs`), including validation of videos and handling of directory creation failures.

5. **Dependencies:**
   - The project makes use of several external libraries, such as Radix UI, Date-fns, and Tailwind CSS. Dependencies are added for these libraries in `package.json`.

Overall, this commit establishes the initial setup for a feature that supports fetching, validating, and merging video and audio files with a user-friendly frontend.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 16 (6b88b4d11fcbe5f0b1ff1e63dc22f819630b3054)</summary>

The commit updates the `README.md` file by modifying the description of the project. Specifically, it changes the subtitle and a key descriptive phrase. The subtitle has been updated to emphasize that the project provides "one API to get all user desktop data" that is "local, cross-platform, 24/7," including various forms of data such as screen, voice, keyboard, mouse, and camera recording. The description was adjusted to highlight a "sandboxed JS plugin system" along with keyboard and mouse control capabilities, while the previous wording regarding "nextjs for desktop agents" was removed.
</details>

------------------------------------------------------------------------

# Overall Summary of Changes

The series of git commits by Louis Beaumont demonstrates a comprehensive update and feature enhancement across the project, particularly focusing on improving authentication, build processes, and user interface components. Key areas of change include:

1. **Authentication and Security Enhancements**:
   - The integration of Clerk for authentication with updates to components to reflect this switch, alongside modifications to Content Security Policies to accommodate new domains required for authentication and connectivity.
  
2. **Build Process Improvements**:
   - Major changes in macOS-specific build processes, moving some functionality from `build.rs` to `pre_build.js`, handling architecture-specific setup, and refining dynamic library paths.

3. **Enhancing User Experience**:
   - Introduction of Clerk authentication components and redesign of user flow with UI updates like subscription models and integration of analytics using `posthog`.

4. **Version Management and Configuration Changes**:
   - Incremental updates to package versions and configurations across various files to accommodate new features and improvements, like changing versions in `Cargo.toml` and `package.json`.

5. **Code Refactoring and Cleanup**:
   - Streamlining existing processes by removing obsolete elements, introducing utility functions for operations like HTTP requests, and cleaning up imports and dependency management.

6. **Documentation and Example Projects**:
   - Updated documentation, especially for plugins and setup on Windows, provides clearer, more streamlined installation instructions.

7. **Introduction of New Features**:
   - Examples such as `create-pipe` demonstrate new functionalities, including a setup script for different package managers and new project structure exemplified in `pipe-for-loom`.

Overall, these updates collectively aim to enhance functionality, streamline processes, improve build efficiency, and enrich user interactions with new features and cleaner interfaces.
