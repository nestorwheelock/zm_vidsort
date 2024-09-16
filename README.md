# zm_vidsort

**zm_vidsort** is a Rust-based tool designed to organize ZoneMinder video downloads by moving video files into a single directory and removing unnecessary files. It helps clean up directories by deleting extraneous files and placing all relevant video files in one location.

## Features

- **File Cleanup**: Removes all `.txt`, `.html`, and `.js` files from the current directory.
- **Video File Organization**: Moves video files from subdirectories into the current directory.
- **Directory Cleanup**: Removes subdirectories after moving the video files.

## Usage

To use **vidsort**, simply run the program in a directory containing ZoneMinder downloads. It will automatically:

1. Remove any extraneous files (`.txt`, `.html`, `.js`).
2. Move video files (with the format `*-video.mp4`) from subdirectories into the current directory.
3. Remove the now-empty subdirectories.

### Example:

1. Run the program in the directory containing the ZoneMinder download directories:

   ```bash
   ./target/release/vidsort
   ```

2. The program will clean up and organize the directory, moving the video files and deleting unnecessary files.

### Example Output:

```bash
Removed extraneous file: example.txt
Moved video file: 1234-video.mp4
Removed directory: 1234
All operations completed successfully.
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.
