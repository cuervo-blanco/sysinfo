use clap::Parser;
use walkdir::WalkDir;
use std::{
    fs::{self, File},
    os::unix::fs::MetadataExt,
    process::Command,
    path::{Path, PathBuf},
    collections::HashMap,
    env,
};
use serde::Serialize;
use serde_json::to_writer_pretty;
use rayon::prelude::*;


#[derive(Parser)]
struct Args {
    #[arg(default_value = ".")]
    path: String
}

#[derive(Serialize)]
struct FileInfo {
    path: String,
    size: u64,
    file_type: String,
    owner: u32
}

#[derive(serde::Serialize)]
struct Report {
    total_size: u64,
    file_types: HashMap<String, u64>,
    ownership: HashMap<u32, u64>,
    files: Vec<FileInfo>,
}

fn collect_data(path: &str) -> Report {
    let entries: Vec<_> = WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .collect();

    let local_results: Vec<_> = entries.par_iter().map(|entry| {
        if let Ok(metadata) = fs::symlink_metadata(entry.path()) {
            let file_size = metadata.len();
            let file_type = get_file_type(entry.path());
            let owner = metadata.uid();

            Some((
                file_size,
                file_type.clone(),
                owner,
                FileInfo {
                    path: entry.path().display().to_string(),
                    size: file_size,
                    file_type: file_type.clone(),
                    owner,
                },
            ))
        } else {
            None
        }
    }).collect();

    let mut total_size = 0u64;
    let mut file_types = HashMap::new();
    let mut ownership = HashMap::new();
    let mut files_info = Vec::new();

    for result in local_results.into_iter().flatten() {
        let (file_size, file_type, owner, file_info) = result;

        total_size += file_size;
        *file_types.entry(file_type).or_insert(0u64) += 1;
        *ownership.entry(owner).or_insert(0u64) += 1;
        files_info.push(file_info);
    }

    Report {
        total_size,
        file_types,
        ownership,
        files: files_info,
    }
}

fn get_file_type(path: &Path) -> String {
    if let Some(extension) = path.extension() {
        extension.to_string_lossy().to_string()
    } else {
        "unknown".to_string()
    }
}

fn save_report(report: &Report, output_path: &str) {
    let file = File::create(output_path).unwrap();
    to_writer_pretty(file, report).unwrap();
}

fn main() {
    let args = Args::parse();
    let path = args.path;

    let report = collect_data(&path);
    let output_path = format!("{}/sysinfo_report.json", path);
    save_report(&report, &output_path);

    let mut script_path = PathBuf::from("scripts");
    script_path.push("generate_visuals.py");

    // let script_path_str = script_path.to_str().expect("Failed to convert script path to string");

    let output_path = format!("sysinfo_report.json");
    let python_interpreter = if cfg!(target_os = "windows") {
        "python" } else { "python3" };

    let status = Command::new(python_interpreter)
        .arg(script_path) // Path to the python script
        .arg(&output_path) // Path to the sysinfo_report
        .status()
        .expect("Failed to execute Python script");

    if !status.success() {
        eprintln!("Python script exited with an error");
    }
}
