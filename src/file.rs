use fs_extra::dir::copy;
use fs_extra::dir::CopyOptions;
use std::error::Error;
use std::fs;

use crate::date::EndDate;

pub fn rename_file(date_struct: EndDate, dir: String, flag: bool) -> Result<(), Box<dyn Error>> {
    let options = CopyOptions::new();

    copy("WE_Template", &dir, &options)?;

    let form1 = format!("{}\\WE_Template", &dir);
    let form2 = date_struct.format_week();

    fs::rename(form1, form2)?;

    Ok(())
}
