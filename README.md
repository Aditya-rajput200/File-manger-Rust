🗂 File Organizer CLI

A blazing-fast Rust-based file organizer that automatically sorts your files into categorized folders — complete with:

✅ Progress bar

🎨 Colored output

🧾 Log report (report.txt)

🔁 Recursive directory support

🧠 Dry-run mode (preview without moving files)

⚡ Optional multi-threading (coming soon)



📸 Demo (Preview)

🚀 Starting File Organizer
📂 Folder: E:/test | Mode: Move Files | Recursive: true

[████████████████████████████████████████] 15/15 file.mp4 (2.30 MB)
✅ Done!

📦 Organization Completed!
📁 Report saved at: E:\test\report.txt


---

🧰 Features

Feature	Description

🟢 Auto-categorization	Detects file type (Images, Videos, Docs, Archives, etc.)
🎨 Colorful output	Uses the colored crate for better CLI readability
📋 Logging	Generates a detailed report.txt with timestamps
🧾 Progress bar	Real-time progress tracking via indicatif
🧠 Dry-run mode	Simulates the process without modifying files
🔁 Recursive option	Organizes files inside all nested directories
🪶 Lightweight	Built entirely in safe, idiomatic Rust



---

🦀 Tech Stack

Language: Rust

CLI Framework: clap

Progress Bar: indicatif

Directory Walker: walkdir

Color Output: colored

Timestamps: chrono



---

⚙ Installation

1️⃣ Clone the repository

git clone
cd file-organizer-cli

2️⃣ Install dependencies

cargo build

3️⃣ Run the CLI

cargo run -- --path "E:/test"


---

🧠 Usage

Basic

file-organizer --path "E:/Downloads"

Dry Run (Preview)

file-organizer --path "E:/Downloads" --dry-run

Recursive Mode

file-organizer --path "E:/Downloads" --recursive

Short Flags

file-organizer -p "E:/Downloads" -r -d


---

📄 Generated Report Example

🧾 File Organizer Report - 2025-10-16 21:10:32

Moved: "cat.jpg" → "E:/test/Images/cat.jpg" (512.0 KB)
Moved: "movie.mkv" → "E:/test/Videos/movie.mkv" (10.4 MB)

📊 Summary:
  • Files processed: 15
  • Total size: 55.23 MB
  • Mode: Moved files
  • Recursive: true


---

🧩 Folder Structure

file-organizer-cli/
├── src/
│   └── main.rs
├── Cargo.toml
├── Cargo.lock
└── README.md


---

🚧 Upcoming Features

⚡ Parallel file moving using rayon

🧮 Smart conflict handling (rename duplicates safely)

🧱 Config file support (.organizer.toml)

🌐 Cross-platform builds for Linux, Windows, macOS




---

📜 License

Licensed under the MIT License — free for personal and commercial use.
Feel free to fork, modify, and share.


---

👨‍💻 Author

Aditya Raj
Rust Developer & Automation Enthusiast
