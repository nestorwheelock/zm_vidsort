use std::fs;
use std::path::Path;

fn main() -> std::io::Result<()> {
    // Remove all .txt, .html, and .js files in the current directory
    for entry in fs::read_dir(".")? {
        let entry = entry?;
        let path = entry.path();
        if let Some(ext) = path.extension() {
            if ext == "txt" || ext == "html" || ext == "js" {
                fs::remove_file(&path)?;
            }
        }
    }

    // Iterate over all directories in the current directory
    for entry in fs::read_dir(".")? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            // Construct the video file path
            let video_file_path = path.join(format!(
                "{}-video.mp4",
                path.file_name().unwrap().to_string_lossy()
            ));

            // Move the video file to the current directory
            if video_file_path.exists() {
                fs::rename(&video_file_path, Path::new("./").join(video_file_path.file_name().unwrap()))?;
            }

            // Remove the directory
            fs::remove_dir(&path)?;
        }
    }

    Ok(())
}
