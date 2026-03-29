use std::{io, path::PathBuf};

pub fn is_executable(de: &PathBuf) -> Result<bool, io::Error> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let file_metadata = std::fs::metadata(de)?;
        Ok(file_metadata.permissions().mode() & 0o100 != 0)
    }

    #[cfg(windows)]
    {
        todo!()
    }
}
