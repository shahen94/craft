use fs_extra::dir::CopyOptions;
use std::fs;
use std::path::Path;

pub fn copy_dir(from: &Path, to: &Path) -> Result<(), Box<dyn std::error::Error>> {
    if to.exists() {
        fs::remove_dir_all(to).unwrap();
    }

    fs::create_dir_all(to).unwrap();

    let options = CopyOptions::new().overwrite(true);

    copy_recursive(from, to, &options)
}

fn copy_recursive(
    from: &std::path::Path,
    to: &std::path::Path,
    options: &CopyOptions,
) -> Result<(), Box<dyn std::error::Error>> {
    let from_meta = fs::metadata(from)?;

    if from_meta.is_dir() {
        let dir_entries = fs::read_dir(from)?;

        for entry in dir_entries {
            let entry = entry?;
            let entry_path = entry.path();
            let entry_name = entry.file_name().into_string().unwrap(); // Convert OsString to String

            let dest_path = to.join(&entry_name);

            if entry_path.is_dir() {
                fs::create_dir_all(&dest_path)?;
                copy_recursive(&entry_path, &dest_path, options)?;
            } else {
                fs_extra::copy_items(&[entry_path], to, options)?;
            }
        }
    }

    Ok(())
}
