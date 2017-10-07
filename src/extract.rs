/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use read::read_index;
use std::io::{self, BufReader, ErrorKind, Read, Seek, SeekFrom};
use std::fs::{File, OpenOptions};
#[cfg(unix)] use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;

pub fn extract<P: AsRef<Path>>(path: P) -> io::Result<()> {
    let mut archive = BufReader::new(File::open(path)?);
    let index = read_index(&mut archive)?;

    for item in index {
        // Read data from the archive.
        archive.seek(SeekFrom::Start(item.offset as u64))?;
        let mut data = archive.by_ref().take(item.length as u64);

        // Write data to a file.
        let mut options = OpenOptions::new();
        options.write(true);
        options.create(true);
        #[cfg(unix)] {
            options.mode(item.flags);
        }
        let mut file = options.open(item.name)?; // XXX prevent directory traversal

        // Copy the data.
        let bytes_written = io::copy(&mut data, &mut file)?;
        if bytes_written != item.length as u64 {
            return Err(io::Error::new(ErrorKind::UnexpectedEof, ""))
        }
    }

    Ok(())
}
