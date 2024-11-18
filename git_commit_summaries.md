# Git Commit Summaries

-----------------------------------------------------------------------
-----------------------------------------------------------------------
 
PRESS CMD+SHIFT+V TO VIEW IN MARKDOWN
 
_______________________________________________________________________
-----------------------------------------------------------------------
Total number of commits: 8

<details>
<summary>summary for commit 1 (3e1932baeb24ad80c66a8656479eeba2116bb9c5)</summary>

This commit, authored by the GitHub Actions Bot, introduces a changelog for the 0.11.0 release. The following changes were made:

1. A new file `0.11.0.md` was added to the `content/changelogs` directory, detailing the improvements and fixes in version 0.11.0. It specifically mentions fixing a focus issue with combobox items and ensuring the correct usage of published releases.

2. The `CHANGELOG.md` file in the `screenpipe-app-tauri/public` directory was updated to include these same updates. It replaces placeholder text with detailed notes on fixes and updates the changelog link to reflect comparisons from commit `f80f9` to `e8b71`. 

No other files were modified, and a binary file named `cn` was created, though its contents are not specified in the diff.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 2 (e8b719d02f217fde7090ef518c01da7044eb697d)</summary>

The commit with hash `e8b719d02f217fde7090ef518c01da7044eb697d`, authored by Louis Beaumont, includes a fix related to "focus." The changes involve updating the `Cargo.toml` file within the `screenpipe-app-tauri/src-tauri` directory. Specifically, the version number of the package `screenpipe-app` has been incremented from "0.10.9" to "0.11.0".
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 3 (c26cbf36a6b65970e88725a47d46cc9ad8effbfc)</summary>

The commit with hash `c26cbf36a6b65970e88725a47d46cc9ad8effbfc` addresses a bug fix related to the focusing issue of combobox items. The author, Yeonsu Bak, updated the dependency `@radix-ui/react-popover` from version `^1.1.1` to `^1.1.2` in the `package.json` file of the `screenpipe-app-tauri` project. This update aims to resolve the focusing problem by utilizing the improvements included in the newer version of the `@radix-ui/react-popover` library.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 4 (9650d040babe3840ced82cdc78cf0117afc404bc)</summary>

The commit made by Louis Beaumont updates documentation in the repository. Here is a summary of the changes:

1. **Modification of `_meta.js`:** 
   - Updated the meta file to add a new entry for "server" with the description "use screenpipe on a server".
   - Reordered the entries, moving "faq" to the end.

2. **Addition of `server.mdx`:** 
   - A new file named `server.mdx` was added.
   - The content includes instructions on how to use Screenpipe with Microsoft Remote Desktop on a server.
   - It describes that Screenpipe works by default, automatically selecting the monitor and remote audio, and it should function regardless of the client's operating system, including Linux servers.
   - The document also mentions the potential for custom business integrations and provides a link to more information.

These changes seem focused on enhancing and expanding the documentation regarding server usage and integrating Screenpipe into business environments.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 5 (07855c7623ad4205b51da03fd1157a75207a42f0)</summary>

The git commit with hash `07855c7623ad4205b51da03fd1157a75207a42f0` by author Louis Beaumont updates the `CONTRIBUTING.md` file. The change involves modifying a link in the "getting started" section. Previously, contributors were directed to the `README.md` for necessary dependencies; the update now points to a documentation page at `https://docs.screenpi.pe/`.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 6 (a8ed9a0ed48ba206c2fb8f50abad078ea624578f)</summary>

The commit with hash `a8ed9a0ed48ba206c2fb8f50abad078ea624578f`, authored by Louis Beaumont on November 15, 2024, deletes a file named `cn`. The file was previously a binary file with executable permissions (`100755`), and it has been completely removed from the repository.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 7 (7922064dda8c1882a3e52da0c30430fe4d641170)</summary>

The commit `7922064dda8c1882a3e52da0c30430fe4d641170` made by the author `ologbonowiwi` on November 13, 2024, involves a fix to a script located at `.github/scripts/generate_changelog_md.sh`. The change ensures that the script retrieves the most recent published release from the `screenpipe` project by modifying the command to select only those releases with the status "Published". Specifically, the `jq` command was updated to filter the release list by checking the "Published" status. This fix addresses an issue where the script might have otherwise selected a non-published release.
</details>

------------------------------------------------------------------------

<details>
<summary>summary for commit 8 (6c33a984bd1e19ab68ad5c242e5822daa83bcb2a)</summary>

The commit by Louis Beaumont on November 12, 2024, updated the `README.md` file. The change involved updating the `src` attribute of an image within an HTML `img` tag. The `src` URL was modified from `https://github.com/user-attachments/assets/bde07ed9-fb56-4f3c-86df-a58eb2bbc2b6` to a new URL `https://github.com/user-attachments/assets/6f4e80d9-89b6-4ed3-a82c-121abc849852`. The rest of the file remained unchanged.
</details>

------------------------------------------------------------------------

# Overall Summary of Changes

The provided git changes describe several updates and fixes across different parts of the repository:

1. **Changelog and Release Updates:**
   - A new changelog file `0.11.0.md` was added, documenting improvements and bug fixes, like resolving a focus issue with combobox items.
   - The `CHANGELOG.md` was updated to replace placeholder text with notes on fixes, highlighting changes from specific commit ranges.
   - The version number in `Cargo.toml` was updated from "0.10.9" to "0.11.0".

2. **Bug Fixes:**
   - A dependency (`@radix-ui/react-popover`) was updated in `package.json` to fix a focus issue with combobox items.

3. **Documentation Enhancements:**
   - Updates to documentation included modifying `_meta.js` and adding `server.mdx`, which provides usage instructions for Microsoft Remote Desktop and mentions custom business integration opportunities.
   - The `CONTRIBUTING.md` file's link for getting started was changed to a more direct documentation page.
   - An image URL in `README.md` was updated.

4. **Script Fixes:**
   - A script in `.github/scripts/generate_changelog_md.sh` was adjusted to ensure it retrieves only published releases using the `jq` command.

5. **File Deletion:**
   - A binary file named `cn` was removed from the repository.

These changes span release documentation, bug fixes, improvements to scripts, and enhancements in documentation, all contributing to the project's ongoing maintenance and usability.
