use {
    crate::support,
    std::{
        fs::{self, DirEntry},
        path::Path,
    },
    anyhow::Result,
    fehler::throws,
};

// one possible implementation of walking a directory only visiting files
#[throws]
fn visit_files(dir: &Path, cb: &mut dyn for<'r> FnMut(&'r DirEntry) -> Result<()>) {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_files(&path, cb)?;
            } else {
                cb(&entry)?;
            }
        }
    }
}
