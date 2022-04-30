use std::{
    collections::hash_map::DefaultHasher,
    fs::{create_dir, metadata, remove_dir_all, File},
    hash::{Hash, Hasher},
    path::{Path, PathBuf},
};

use anyhow::Context;

use crate::osm::MessageInfo;

const TEMP_DIR_PATH: &str = "/tmp/osm";

/// Returns the path of the created file.
pub fn create_msg_tmp_file(
    message_info: &MessageInfo,
    contents: String,
) -> std::io::Result<String> {
    let path = get_tmp_file_path(message_info);
    std::fs::write(&path, contents)?;
    Ok(path)
}

fn get_tmp_file_path(message_info: &MessageInfo) -> String {
    let mut hasher = DefaultHasher::new();
    message_info.hash(&mut hasher);
    let hash = hasher.finish();
    format!("{}/{}.html", TEMP_DIR_PATH, hash)
}

pub fn init_tmp_dir() -> anyhow::Result<()> {
    match metadata(TEMP_DIR_PATH) {
        Ok(metadata) => {
            if metadata.is_dir() {
                remove_dir_all(TEMP_DIR_PATH).context(format!(
                    "failed to remove directory \"{}\" and all of its contents",
                    TEMP_DIR_PATH
                ))?;
            } else {
                return Err(anyhow::Error::msg(format!("there is a file called \"{}\" which prevents the program from using it's tmp directory", TEMP_DIR_PATH)));
            }
        }
        Err(err) => match err.kind() {
            std::io::ErrorKind::NotFound => {}
            _ => {
                return Err(err).context(format!(
                    "failed to get information about the path \"{}\"",
                    TEMP_DIR_PATH
                ))
            }
        },
    }

    // after removing the directory and its contents, create a new empty directory.
    create_dir(TEMP_DIR_PATH).context(format!("failed to create directory {}", TEMP_DIR_PATH))?;

    Ok(())
}
