#!/usr/bin/env python3

import json
import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns
import os
import sys


def main(report_path):
    if not os.path.exists(report_path):
        print(f"Report file {report_path} does not exist.")
        sys.exit(1)

    with open(report_path, 'r') as f:
        report_data = json.load(f)

    file_types = report_data.get('file_types', {})
    ownership = report_data.get('ownership', {})
    files = report_data.get('files', [])

    # Convert files list to DataFrame
    files_df = pd.DataFrame(files)

    output_dir = os.path.dirname(report_path)
    if not os.path.exists(output_dir):
        raise FileNotFoundError(
            f"Output directory {output_dir} does not exist.")

    # File Size Distribution
    plt.figure(figsize=(10, 6))
    sns.histplot(files_df['size'], bins=50, log_scale=(False, True))
    plt.title('File Size Distribution')
    plt.xlabel('Size (bytes)')
    plt.ylabel('Count')
    plt.tight_layout()
    plt.savefig(os.path.join(output_dir, 'file_size_distribution.png'))
    plt.close()

    # File Type Distribution
    file_type_series = pd.Series(file_types)
    plt.figure(figsize=(8, 8))
    file_type_series.plot.pie(autopct='%1.1f%%')
    plt.title('File Type Distribution')
    plt.ylabel('')
    plt.tight_layout()
    plt.savefig(os.path.join(output_dir, 'file_type_distribution.png'))
    plt.close()

    # Files per User
    ownership_series = pd.Series(ownership)
    plt.figure(figsize=(10, 6))
    ownership_series.plot.bar()
    plt.title('Files per User')
    plt.xlabel('User ID')
    plt.ylabel('Number of Files')
    plt.tight_layout()
    plt.savefig(os.path.join(output_dir, 'files_per_user.png'))
    plt.close()

    generate_summary_report(report_data, output_dir)

    print(f"Visualizations saved to {output_dir}")


def generate_summary_report(report_data, output_dir):
    total_size = report_data.get('total_size', 0)
    num_files = len(report_data.get('files', []))

    summary = f"""# System Report Summary

- **Total Size**: {total_size} bytes
- **Total Files**: {num_files}

## File Type Distribution

![File Type Distribution](file_type_distribution.png)

## File Size Distribution

![File Size Distribution](file_size_distribution.png)

## Files per User

![Files per User](files_per_user.png)
"""

    summary_path = os.path.join(output_dir, 'summary_report.md')
    with open(summary_path, 'w') as f:
        f.write(summary)


if __name__ == '__main__':
    if len(sys.argv) != 2:
        print(
            "Usage: python3 generate_visuals.py <path_so_sysinfo_report.json>")
        sys.exit(1)

    report_file_path = sys.argv[1]
    main(report_file_path)
