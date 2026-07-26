use std::io::{Error as IoError, Result as IoResult};
use std::path::Path;

use super::super::cups_v1::{JobSheet, Media, Orientation, Quality, Sides, Task};

impl<'a> Task<'a> {
    // https://man7.org/linux/man-pages/man1/lpr.1.html
    pub fn command<P: AsRef<Path>>(&'a self, file: P) -> IoResult<String> {
        if self.copies() == 0 {
            return Err(IoError::other("empty job"));
        }
        let file = file.as_ref();
        let mut it = format!("lpr -T {} -#{} -r", self.name(), self.copies());

        if let Some(number_up) = self.number_up()
            && !number_up.is_empty()
        {
            let pages: Vec<String> = number_up.iter().map(|x| format!("{}", x)).collect();
            it = format!("{} -o number-up={}", it, pages.join("|"));
        }

        it = format!(
            "{} -o media={}",
            it,
            match self.media() {
                Media::A3 => Ok("a3"),
                Media::A4 => Ok("a4"),
                Media::Letter => Ok("letter"),
                _ => Err(IoError::other("unsupported media")),
            }?
        );
        it = format!(
            "{} -o job-sheets={}",
            it,
            match self.job_sheet() {
                JobSheet::Classified => Ok("classified"),
                JobSheet::Confidential => Ok("confidential"),
                JobSheet::Secret => Ok("secret"),
                JobSheet::Standard => Ok("standard"),
                JobSheet::TopSecret => Ok("topsecret"),
                JobSheet::Unclassified => Ok("unclassified"),
                _ => Err(IoError::other("unsupported job sheet")),
            }?
        );
        it = format!(
            "{} -o orientation-requested={}",
            it,
            match self.orientation() {
                Orientation::LandscapeCounterClockwise90 => Ok(4),
                Orientation::LandscapeClockwise90 => Ok(5),
                Orientation::ReversePortrait => Ok(6),
                _ => Err(IoError::other("unsupported orientation requested")),
            }?
        );
        it = format!(
            "{} -o print-quality={}",
            it,
            match self.quality() {
                Quality::Draft => Ok(3),
                Quality::Normal => Ok(4),
                Quality::Best => Ok(5),
                _ => Err(IoError::other("unsupported print quality")),
            }?
        );
        it = format!(
            "{} -o sides={}",
            it,
            match self.sides() {
                Sides::One => Ok("one-sided"),
                Sides::TwoLong => Ok("two-sided-long-edge"),
                Sides::TwoShort => Ok("two-sided-short-edge"),
                _ => Err(IoError::other("unsupported sides")),
            }?
        );

        Ok(format!("{} {}", it, file.display()))
    }
}
