use std::path::{Path,PathBuf};

/// declare what dir you want and we do next for you
pub fn declare_dir() {
    unimplemented!();
}

/// help function to check is two dir/file eq
pub fn dir_eq<PA:AsRef<Path>,PB:AsRef<Path>>(path_a:PA,path_b:PB) -> Result<bool,failure::Error> {
    let path_a = path_a.as_ref().to_path_buf();
    let path_b = path_b.as_ref().to_path_buf();

    println!("{:?} {:?}",path_a,path_b);
    unimplemented!();
}

pub fn dir_eq_inside(path_a:PathBuf,path_b:PathBuf) ->Result<bool,failure::Error> {
    unimplemented!();
}

/// given a dir we will zip it
pub fn zip_dir(path:PathBuf,zip_path:PathBuf) ->Result<(),failure::Error> {
    unimplemented!();
}

/// given a zip we will unzip it into you expect dir
/// let dir_a = "~/some_dir";
/// let zip_path ="~/some_zip.zip"
/// let dir_b ="~/temp/some_dir"
/// dir_eq(unzip(zip(dir_a,zip_path)?,dir_b)?,dir_a)? == true;
pub fn unzip(path:PathBuf,zip_path:PathBuf) ->Result<(),failure::Error> {
    unimplemented!();
}

#[cfg(test)]
mod tests {
   use super::*;
   #[test]
   fn test_dir_eq() {
       let ret = dir_eq(r#"C:\Users\developer\work\saas\win_driver"#,r#"C:\Users\developer\work\saas\a\win_driver"#).unwrap();
       assert_eq!(ret,true);
   }
}