use std::env;
use std::process::Command;
use std::str;
use regex::Regex;
use std::fs::File;
use std::io::Write;
use colored::*;
use futures::future::join_all;

fn run_git_command(repo_path: &str, git_command: &[String]) -> Result<String, String> {
    let mut command = Command::new(&git_command[0]);
    command.arg("-C").arg(repo_path);
    for arg in &git_command[1..] {
        command.arg(arg);
    }

    println!("Running command: {:?}", command); // Print the command

    let output = command.output().map_err(|e| format!("Failed to execute command: {}", e))?;
    println!("Git: {}", output.status); // Debug log
    if output.status.success() {
        println!("{}", "all good".green());
        let stdout = str::from_utf8(&output.stdout)
            .map_err(|e| format!("Failed to convert output to string: {}", e))?;
        Ok(stdout.to_string())
    } else {
        let stderr = str::from_utf8(&output.stderr)
            .map_err(|e| format!("Failed to convert error to string: {}", e))?;
        println!("Git command error: {}", stderr); // Debug log
        Err(stderr.to_string())
    }
}

async fn summarize_changes(changes: &str) -> Result<String, String> {
    let client = reqwest::Client::new();
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not found in .env file");
    let url = "https://api.openai.com/v1/chat/completions";

    let response = client.post(url)
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&serde_json::json!({
            "model": "gpt-4o",
            "messages": [
                {"role": "system", "content": "You are a helpful assistant."},
                {"role": "user", "content": format!("Summarize the following git changes:\n\n{}", changes)}
            ],
            "max_tokens": 150,
        }))
        .send()
        .await
        .map_err(|e| format!("Failed to send request: {}", e))?;

    let response_json: serde_json::Value = response.json().await.map_err(|e| format!("Failed to read response: {}", e))?;
    let content = response_json["choices"][0]["message"]["content"].as_str().ok_or("Failed to extract content")?;
    Ok(content.to_string())
}

fn open_md_in_preview(file_path: &str) {
    // Open the file in VS Code and show the preview
    Command::new("code")
        .arg(file_path)
        .arg("--command")
        .arg("markdown.showPreview")
        .spawn()
        .expect("Failed to open file in VS Code preview mode");
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <repo_path> <git_command> [<args>...]", args[0]);
        return;
    }

    let repo_path = &args[1];
    let git_command = &args[2..];

    match run_git_command(repo_path, git_command) {
        Ok(changes) => {
            if changes.trim().is_empty() {
                eprintln!("No changes found in the specified range.");
                return;
            }
            // println!("Git changes: {}", changes); // Debug log

            // Extract commit hashes
            let re = Regex::new(r"commit (\b[0-9a-f]{5,40}\b)").unwrap();
            let commit_hashes: Vec<&str> = re.captures_iter(&changes)
                .map(|cap| cap.get(1).unwrap().as_str())
                .collect();

            println!("-----------------------------------------------------------------------");
            println!("Once all commits are processed the script will open a summary text file");
            println!("_______________________________________________________________________");
            println!("Total number of commits: {}", commit_hashes.len());

            let current_dir = env::current_dir().expect("Failed to get current directory");
            let file_path = current_dir.join("git_commit_summaries.md");
            let file_path_str = file_path.to_str().expect("Failed to convert file path to string");
            let mut file = File::create(&file_path).expect("Failed to create file");

            writeln!(file, "# Git Commit Summaries\n").expect("Failed to write to file");

            writeln!(file, "-----------------------------------------------------------------------").expect("Failed to write to file");
            writeln!(file, "-----------------------------------------------------------------------").expect("Failed to write to file");
            writeln!(file, " ").expect("Failed to write to file");
            writeln!(file, "PRESS CMD+SHIFT+V TO VIEW IN MARKDOWN").expect("Failed to write to file");
            writeln!(file, " ").expect("Failed to write to file");
            writeln!(file, "_______________________________________________________________________").expect("Failed to write to file");
            writeln!(file, "-----------------------------------------------------------------------").expect("Failed to write to file");
            writeln!(file, "Total number of commits: {}\n", commit_hashes.len()).expect("Failed to write to file");

            let mut tasks = Vec::new();

            for (index, commit_hash) in commit_hashes.iter().enumerate() {
                let repo_path = repo_path.to_string();
                let commit_hash = commit_hash.to_string();
                let task = tokio::spawn(async move {
                    let git_show_command = vec!["git".to_string(), "show".to_string(), commit_hash.clone()];
                    match run_git_command(&repo_path, &git_show_command) {
                        Ok(commit_details) => {
                            match summarize_changes(&commit_details).await {
                                Ok(summary) => Some((index, commit_hash, summary)),
                                Err(e) => {
                                    eprintln!("Error summarizing changes for {}: {}", commit_hash, e);
                                    None
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("Error running git show for {}: {}", commit_hash, e);
                            None
                        }
                    }
                });
                tasks.push(task);
            }

            let results = join_all(tasks).await;

            let mut combined_changes = Vec::new();

            for result in results {
                if let Ok(Some((index, commit_hash, summary))) = result {
                    writeln!(file, "<details>\n<summary>Summary for commit {} ({})</summary>\n\n{}\n</details>\n", index + 1, commit_hash, summary)
                        .expect("Failed to write to file");
                    writeln!(file, "------------------------------------------------------------------------\n")
                        .expect("Failed to write to file");
                    combined_changes.push(summary);
                }
            }

            // Generate overall summary
            let overall_summary = summarize_changes(&combined_changes.join("\n")).await
                .unwrap_or_else(|e| format!("Error generating overall summary: {}", e));

            writeln!(file, "# Overall Summary of Changes\n\n{}", overall_summary)
                .expect("Failed to write overall summary to file");

            open_md_in_preview(file_path_str);
        }
        Err(e) => eprintln!("Error running git command: {}", e),
    }
}