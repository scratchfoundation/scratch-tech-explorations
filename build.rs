use std::{path::Path, fs::{self, DirEntry}, io, ffi::OsStr};

fn main() {
    copy_fonts();
}

fn copy_files<P: AsRef<Path>, Q: AsRef<Path>, R>(from_dir: P, to_dir: Q, predicate: R) -> io::Result<()>
    where R: Fn(&DirEntry) -> bool
{
    let dir_entries = fs::read_dir(from_dir.as_ref())?;
    for dir_entry in dir_entries.flatten() {
        let src_path = dir_entry.path();
        if let Some(file_name) = src_path.file_name() {
            if predicate(&dir_entry) {
                let dest_path = Path::new(to_dir.as_ref()).join(file_name);
                eprintln!("copy_files copying {0} to {1}", src_path.display(), dest_path.display());
                fs::copy(src_path, dest_path)?;
            }
        }
    }

    Ok(())
}

fn copy_fonts() {
    let font_extensions = vec![
        OsStr::new("ttf"),
        OsStr::new("otf"),
    ];
    let src_path = Path::new("node_modules/scratch-render-fonts/src");
    let dst_path = Path::new("assets/fonts");
    println!("cargo:rerun-if-changed={:?}/*.ttf", src_path);
    fs::create_dir_all(dst_path).expect("failed to create fonts directory");
    copy_files(src_path, dst_path, |dir_entry| {
        let src_path = dir_entry.path();
        eprintln!("copy_fonts checking {0}", src_path.display());
        if !src_path.is_file() {
            return false;
        }
        match src_path.extension() {
            Some(ext) => font_extensions.contains(&ext),
            None => false
        }
    }).expect("failed to copy fonts");
}
