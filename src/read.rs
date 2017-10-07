use byteorder::{BigEndian, ReadBytesExt};
use MarItem;
use std::io;
use std::io::{BufRead, ErrorKind, Read, Seek, SeekFrom};

/// Magic bytes found at the start of a MAR file.
const MAR_ID: &[u8; MAR_ID_SIZE] = b"MAR1";
const MAR_ID_SIZE: usize = 4;

/// Read the index from a MAR file.
pub fn read_index<R>(mut archive: R) -> io::Result<Vec<MarItem>>
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

    let name = String::from_utf8(name)
        .or(Err(io::Error::new(ErrorKind::InvalidData, "Filename is not UTF-8")))?;
    
    Ok(MarItem { offset, length, flags, name })
}
