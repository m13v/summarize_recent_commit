import os
import subprocess
import re
import asyncio
import aiohttp
from dialoguer import Input
from termcolor import colored

async def run_git_command(repo_path, git_command):
    command = f"cd {repo_path} && {git_command}"
    print(colored(f"Running git command: {git_command}", "cyan"))

    process = subprocess.Popen(command, shell=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    stdout, stderr = process.communicate()

    if process.returncode == 0:
        print(colored("Git command executed successfully", "green"))
        return stdout.decode('utf-8')
    else:
        print(colored(f"Git command error: {stderr.decode('utf-8')}", "red"))
        return None

async def summarize_changes(changes):
    api_key = os.getenv("OPENAI_API_KEY")
    if not api_key:
        raise EnvironmentError("OPENAI_API_KEY not found in environment variables")

    url = "https://api.openai.com/v1/chat/completions"
    headers = {
        "Authorization": f"Bearer {api_key}",
        "Content-Type": "application/json"
    }
    payload = {
        "model": "gpt-4o",
        "messages": [
            {"role": "system", "content": "You are a helpful assistant."},
            {"role": "user", "content": f"Summarize the following git changes:\n\n{changes}"}
        ],
        "max_tokens": 1000
    }

    async with aiohttp.ClientSession() as session:
        async with session.post(url, headers=headers, json=payload) as response:
            if response.status == 200:
                response_json = await response.json()
                return response_json["choices"][0]["message"]["content"]
            else:
                raise Exception(f"Failed to send request: {response.status}")

def open_md_in_preview(file_path):
    command = f"code {file_path} --command markdown.showPreview"
    subprocess.Popen(command, shell=True)

async def main():
    args = os.sys.argv
    repo_path = args[1] if len(args) > 1 else Input().with_prompt("Enter the repository absolute path").interact_text()
    git_command = args[2] if len(args) > 2 else Input().with_prompt("Enter the full git command").interact_text()

    changes = await run_git_command(repo_path, git_command)
    if not changes or not changes.strip():
        print(colored("No changes found in the specified range.", "yellow"))
        return

    commit_hashes = re.findall(r"commit (\b[0-9a-f]{5,40}\b)", changes)
    print("-----------------------------------------------------------------------")
    print("Once all commits are processed the script will open a summary text file")
    print("_______________________________________________________________________")
    print(f"Total number of commits: {len(commit_hashes)}")

    current_dir = os.getcwd()
    file_path = os.path.join(current_dir, "git_commit_summaries.md")

    with open(file_path, 'w') as file:
        file.write("# Git Commit Summaries\n")
        file.write("-----------------------------------------------------------------------\n")
        file.write("-----------------------------------------------------------------------\n")
        file.write(" \n")
        file.write("PRESS CMD+SHIFT+V TO VIEW IN MARKDOWN\n")
        file.write(" \n")
        file.write("_______________________________________________________________________\n")
        file.write("-----------------------------------------------------------------------\n")
        file.write(f"Total number of commits: {len(commit_hashes)}\n")

    tasks = []
    for index, commit_hash in enumerate(commit_hashes):
        task = asyncio.create_task(process_commit(repo_path, commit_hash, index, len(commit_hashes), file_path))
        tasks.append(task)

    await asyncio.gather(*tasks)

    print(colored("Generating overall summary...", "cyan"))
    combined_changes = "\n".join([task.result() for task in tasks if task.result()])
    overall_summary = await summarize_changes(combined_changes)

    with open(file_path, 'a') as file:
        file.write(f"# Overall Summary of Changes\n\n{overall_summary}")

    open_md_in_preview(file_path)
    print(colored("Job finished successfully!", "green"))
    print(colored(f"Summary file created at: {file_path}", "green"))

async def process_commit(repo_path, commit_hash, index, total_commits, file_path):
    git_show_command = f"git show {commit_hash}"
    commit_details = await run_git_command(repo_path, git_show_command)
    if commit_details:
        summary = await summarize_changes(commit_details)
        if summary:
            print(colored(f"Processing commit {index + 1} of {total_commits}: {commit_hash}", "cyan"))
            with open(file_path, 'a') as file:
                file.write(f"<details>\n<summary>Summary for commit {index + 1} ({commit_hash})</summary>\n\n{summary}\n</details>\n")
                file.write("------------------------------------------------------------------------\n")
            return summary
    return None

if __name__ == "__main__":
    asyncio.run(main())