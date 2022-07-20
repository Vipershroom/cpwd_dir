use fs_extra::dir::copy;
use fs_extra::dir::CopyOptions;
use std::error::Error;
use std::fs;

use crate::date::EndDate;

pub fn rename_file(date_struct: EndDate, dir: String, flag: bool) -> Result<(), Box<dyn Error>> {
    let options = CopyOptions::new();

    copy("WE_Template", &dir, &options)?;

    Ok(())
}
