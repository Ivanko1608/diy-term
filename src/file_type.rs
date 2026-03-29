use std::{fs::DirEntry, io, path::PathBuf};

pub fn is_executable(de: &PathBuf) -> Result<bool, io::Error> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let file_metadata = std::fs::metadata(de)?;
        file_metadata.permissions().mode() & 0o100 != 0;
    }

    Ok(false)
}
