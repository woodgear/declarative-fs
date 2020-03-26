use std::path::{Path, PathBuf};
use std::fs;
use std::io;

pub enum FsItem {
    Dir(String),
    StringFile(String, String),
    BinFile(String, Vec<u8>),
}

/// declare what dir you want and we do next for you
pub fn declare_dir(root: PathBuf, fs: Vec<FsItem>) -> Result<(), failure::Error> {
    if !root.exists() {
        std::fs::create_dir_all(&root)?;
    }
    for item in fs {
        match item {
            FsItem::Dir(path) => {
                let path = root.join(path);
                std::fs::create_dir_all(path)?;
            }
            FsItem::StringFile(path, data) => {
                let path = root.join(path);
                std::fs::write(path, &data.as_bytes())?;
            }
            FsItem::BinFile(path, data) => {
                let path = root.join(path);
                std::fs::write(path, &data)?;
            }
        }
    }
    Ok(())
}

/// help function to check is two dir/file eq
pub fn dir_eq<PA: AsRef<Path>, PB: AsRef<Path>>(
    path_a: PA,
    path_b: PB,
) -> Result<bool, failure::Error> {
    let path_a = path_a.as_ref().to_path_buf();
    let path_b = path_b.as_ref().to_path_buf();
    fn file_eq(left: &Path, right: &Path) -> Result<bool, failure::Error> {
        let left_content = std::fs::read(left)?;
        let right_content = std::fs::read(right)?;
        return Ok(left_content == right_content);
    }

    fn all_left_is_same_in_right(left: &Path, right: &Path) -> Result<bool, failure::Error> {
        use walkdir::WalkDir;
        for entry in WalkDir::new(left).into_iter().filter_map(|e| e.ok()) {
            let entry_path = entry.path();
            let correspond_entry_path = right.join(entry_path.strip_prefix(left)?);
            if entry_path.is_dir() {
                if !(correspond_entry_path.exists() && correspond_entry_path.is_dir()) {
                    return Ok(false);
                }
            }
            if entry_path.is_file() {
                if !(correspond_entry_path.exists()
                    && correspond_entry_path.is_file()
                    && file_eq(&entry_path, &correspond_entry_path)?)
                {
                    return Ok(false);
                }
            }
        }
        return Ok(true);
    }

    if !(path_a.is_dir() && path_a.is_dir()) {
        return Err(failure::format_err!("{}", "must be dir"));
    }

    if path_a.file_name() != path_a.file_name() {
        return Err(failure::format_err!("{}", "last name must be same"));
    }
    return Ok(all_left_is_same_in_right(&path_a, &path_b)?
        && all_left_is_same_in_right(&path_b, &path_a)?);
}

pub fn zip_dir<S: AsRef<Path>, Z: AsRef<Path>>(
    src_dir: S,
    zip_path: Z,
) -> Result<(), failure::Error> {
    use std::io::prelude::*;
    use std::io::Write;
    use zip::write::FileOptions;

    use std::fs::File;
    use walkdir::WalkDir;
    let zip_path = zip_path.as_ref().to_path_buf();
    let src_dir = src_dir.as_ref().to_path_buf();
    let zip_file = File::create(&zip_path)?;
    let mut zip = zip::ZipWriter::new(zip_file);

    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Bzip2)
        .unix_permissions(0o755);

    let mut buffer = Vec::new();
    let walkdir = WalkDir::new(&src_dir);
    let it = walkdir.into_iter();

    for entry in it.filter_map(|e| e.ok()) {
        let path = entry.path();
        let name = path.strip_prefix(Path::new(&src_dir))?;

        // Write file or directory explicitly
        // Some unzip tools unzip files with directory paths correctly, some do not!
        if path.is_file() {
            zip.start_file_from_path(name, options)?;
            let mut f = File::open(path)?;

            f.read_to_end(&mut buffer)?;
            zip.write_all(&*buffer)?;
            buffer.clear();
        } else if name.as_os_str().len() != 0 {
            // Only if not root! Avoids path spec / warning
            // and mapname conversion failed error on unzip
            zip.add_directory_from_path(name, options)?;
        }
    }
    zip.finish()?;

    Ok(())
}


pub fn unzip<Z: AsRef<Path>, D: AsRef<Path>>(
    zip_path: Z,
    dst_dir: D,
) -> Result<(), failure::Error> {
    let zip_path = zip_path.as_ref().to_path_buf();
    let dst_dir = dst_dir.as_ref().to_path_buf();

    let file = fs::File::open(&zip_path)?;
    let mut archive = zip::ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = dst_dir.join(file.sanitized_name());
        if (&*file.name()).ends_with('/') {
            fs::create_dir_all(&outpath)?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p)?;
                }
            }
            let mut outfile = fs::File::create(&outpath)?;
            io::copy(&mut file, &mut outfile)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempdir::TempDir;
    #[test]
    fn test_declare_fs() {
        let tmp_dir = TempDir::new("example").unwrap();
        let root = tmp_dir.path();
        declare_dir(
            root.join("a"),
            vec![
                FsItem::Dir("1".to_string()),
                FsItem::StringFile("1.txt".to_string(), "ssssss".to_string()),
            ],
        )
        .unwrap();
        println!("{:?}", root);
    }
    #[test]
    fn test_zip() {
        let tmp_dir = TempDir::new("example").unwrap();
        let home = tmp_dir.path();
        let root = home.join("a");
        declare_dir(
            root.clone(),
            vec![
                FsItem::Dir("1".to_string()),
                FsItem::StringFile("1.txt".to_string(), "ssssss".to_string()),
            ],
        )
        .unwrap();
        let unzip_dir = home.join("a-1/a");
        let zip_path = home.join("a.zip");
        zip_dir(root.clone(), zip_path.clone()).unwrap();
        unzip(zip_path.clone(), unzip_dir.clone()).unwrap();
        let ret = dir_eq(unzip_dir, root).unwrap();
        assert_eq!(ret, true);
    }
    #[test]
    fn test_dir_eq() {
        let ret = dir_eq(
            r#"C:\Users\developer\work\saas\win_driver"#,
            r#"C:\Users\developer\work\saas\a\win_driver"#,
        )
        .unwrap();

        assert_eq!(ret, true);
    }
}
