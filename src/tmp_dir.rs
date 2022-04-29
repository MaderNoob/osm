use std::{
    collections::hash_map::DefaultHasher,
    fs::{create_dir, metadata, remove_dir_all, File},
    hash::{Hash, Hasher},
    path::{Path, PathBuf},
};

use anyhow::Context;

use crate::osm::MessageInfo;

/// Returns the path of the created file.
pub fn create_msg_tmp_file(message_info: &MessageInfo, contents: String)->std::io::Result<String>{
    let path = get_tmp_file_path(message_info);
    std::fs::write(&path, contents)?;
    Ok(path)
}

fn get_tmp_file_path(message_info: &MessageInfo) -> String {
    let mut hasher = DefaultHasher::new();
    message_info.hash(&mut hasher);
    let hash = hasher.finish();
    format!("tmp/{}.html", hash)
}

pub fn init_tmp_dir() -> anyhow::Result<()> {
    match metadata("tmp") {
        Ok(metadata) => {
            if metadata.is_dir() {
                remove_dir_all("tmp")
                    .context("failed to remove tmp directory and all of its contents")?;
            } else {
                return Err(anyhow::Error::msg(format!("there is a file called \"tmp\" which prevents the program from using it's tmp directory")));
            }
        }
        Err(err) => match err.kind() {
            std::io::ErrorKind::NotFound => {}
            _ => return Err(err).context("failed to get information about the path \"tmp\""),
        },
    }

    // after removing the directory and its contents, create a new empty directory.
    create_dir("tmp").context("failed to create tmp directory")?;

    Ok(())
}
