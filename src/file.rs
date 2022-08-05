use crate::date::EndDate;
use fs_extra::dir::copy;
use fs_extra::dir::CopyOptions;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

#[allow(unused_assignments)]
pub fn rename_file(date_struct: EndDate, dir: PathBuf, flag: bool) -> Result<(), Box<dyn Error>> {
    let options = CopyOptions::new();

    let mut copy_dir = PathBuf::new();

    if flag {
        copy_dir.push("./templates/WE_Template_prog/");
    } else {
        copy_dir.push("./templates/WE_Templater_web/");
    }
    copy(&copy_dir, &dir, &options)?;

    let copy_slice = &copy_dir.to_str().unwrap()[12..];

    let form1 = dir.join(PathBuf::from(copy_slice));
    let form2 = dir.join(date_struct.format_week());

    fs::rename(form1, form2)?;

    Ok(())
}
