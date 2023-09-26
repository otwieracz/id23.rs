use std::num::TryFromIntError;

use chrono::DateTime;
use chrono::Datelike;
use chrono::Utc;
use thiserror::Error;

const CHARSPACE: [char; 33] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0',
];


#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    IntOutOfBound(#[from] TryFromIntError),
    #[error("out of bounds when converting date or sequence number to charspace")]
    OutOfBounds
}

fn as_id23_char(n: usize) -> Result<char, Error> {
    if n < CHARSPACE.len() {
        return Ok(CHARSPACE[n])
    } else {
        return Err(Error::OutOfBounds)
    }
}

pub fn id_v1(dt: DateTime<Utc>) -> Result<String, Error> {
    let year = as_id23_char((dt.year() - 2023).try_into()?)?;
    let month = as_id23_char(dt.month().try_into()?)?;
    let day = as_id23_char(dt.day().try_into()?)?;

    Ok(format!("{}{}{}", year, month, day))
}
