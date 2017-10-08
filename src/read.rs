/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

 //! Reading MAR files.

use byteorder::{BigEndian, ReadBytesExt};
use super::{MarFileInfo, MarItem};
use std::io::{self, BufRead, ErrorKind, Read, Seek, SeekFrom};
use std::u32;

/// Magic bytes found at the start of a MAR file.
const MAR_ID: &[u8; MAR_ID_SIZE] = b"MAR1";
const MAR_ID_SIZE: usize = 4;

/// Position of the signature block within the file, directly after the 16-byte header.
const SIGNATURE_BLOCK_OFFSET: u64 = 16;

/// Read metadata from a MAR file.
pub fn get_info<R>(mut archive: R) -> io::Result<MarFileInfo>
    where R: Read + Seek
{
    // Read the header.
    let mut id = [0; MAR_ID_SIZE];
    archive.read_exact(&mut id)?;
    if id != *MAR_ID {
        return Err(io::Error::new( ErrorKind::InvalidData,
            "Not a MAR file (invalid bytes at start of file)."))
    }
    let offset_to_index = archive.read_u32::<BigEndian>()?;
    let num_signatures = archive.read_u32::<BigEndian>()?;

    // Seek to the index and read the first offset to content field.
    archive.seek(SeekFrom::Start(offset_to_index as u64))?;
    let offset_to_content = archive.read_u32::<BigEndian>()?;

    // In an old-style MAR file with no signature block, the content will start right after the
    // magic bytes and the 4-byte index offset.
    let has_signature_block = offset_to_content as usize != MAR_ID_SIZE + 4;

    // Seek to the signature block and skip past all the signatures.
    archive.seek(SeekFrom::Start(SIGNATURE_BLOCK_OFFSET))?;
    for _ in 0..num_signatures {
        archive.seek(SeekFrom::Current(4))?;
        let signature_len = archive.read_u32::<BigEndian>()?;
        archive.seek(SeekFrom::Current(signature_len as i64))?;
    }

    // Check for additional blocks.
    let pos = archive.seek(SeekFrom::Current(0))?;
    if pos > u32::MAX as u64 {
        return Err(io::Error::new(ErrorKind::InvalidData, "Signature block size overflow"))
    }
    let offset_additional_blocks = pos as u32;
    let has_additional_blocks = offset_additional_blocks == offset_to_content;
    let num_additional_blocks = if has_additional_blocks {
        archive.read_u32::<BigEndian>()?
    } else {
        0
    };

    Ok(MarFileInfo {
        has_signature_block,
        num_signatures,
        offset_additional_blocks,
        has_additional_blocks,
        num_additional_blocks
    })
}

/// Read the index from a MAR file.
///
/// TODO: Return an iterator?
pub(crate) fn read_index<R>(mut archive: R) -> io::Result<Vec<MarItem>>
    where R: Read + Seek
{
    // Verify the magic bytes.
    let mut id = [0; MAR_ID_SIZE];
    archive.read_exact(&mut id)?;
    if id != *MAR_ID {
        return Err(io::Error::new( ErrorKind::InvalidData,
            "Not a MAR file (invalid bytes at start of file)."))
    }

    // Seek to the index.
    let offset_to_index = archive.read_u32::<BigEndian>()?;
    archive.seek(SeekFrom::Start(offset_to_index as u64))?;

    // Read the index into memory.
    let size_of_index = archive.read_u32::<BigEndian>()?;
    let mut buf = vec![0; size_of_index as usize];
    archive.read_exact(&mut buf)?;

    // Reach each item from the index.
    let mut items = vec![];
    let mut buf = &buf[..];
    while !buf.is_empty() {
        items.push(read_next_item(&mut buf)?);
    }
    Ok(items)
}

/// Read a single entry from the index.
fn read_next_item<R: BufRead>(mut index: R) -> io::Result<MarItem> {
    let offset = index.read_u32::<BigEndian>()?;
    let length = index.read_u32::<BigEndian>()?;
    let flags = index.read_u32::<BigEndian>()?;
    
    let mut name = Vec::new();
    index.read_until(0, &mut name)?;
    name.pop(); // Remove the trailing NUL.

    let name = String::from_utf8(name)
        .or(Err(io::Error::new(ErrorKind::InvalidData, "Filename is not UTF-8")))?;
    
    Ok(MarItem { offset, length, flags, name })
}
