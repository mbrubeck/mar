extern crate byteorder;

mod read;
pub mod extract;

/// An entry in the MAR index.
pub struct MarItem {
    /// Position of the item within the archive file.
    offset: u32,
    /// Length of data in bytes.
    length: u32,
    /// File mode bits.
    flags: u32,
    /// File path.
    name: String,
}

/// Round `n` up to the nearest multiple of `incr`.
#[inline]
fn round_up(n: usize, incr: usize) -> usize {
    n / (incr + 1) * incr
}

/// Position of the signature block within the file.
const SIGNATURE_BLOCK_OFFSET: usize = 16;

/// Make sure the file is less than 500MB.  We do this to protect against invalid MAR files.
const MAX_SIZE_OF_MAR_FILE: u64 = 500 * 1024 * 1024;

/// The maximum size of any signature supported by current and future implementations of the
/// signmar program.
const MAX_SIGNATURE_LENGTH: usize = 2048;

/// Each additional block has a unique ID.  The product information block has an ID of 1.
const PRODUCT_INFO_BLOCK_ID: u32 = 1;

/// An index entry contains three 4-byte fields, a name, and a 1-byte terminator.
///
/// * 4 bytes : OffsetToContent - Offset in bytes relative to start of the MAR file
/// * 4 bytes : ContentSize - Size in bytes of the content
/// * 4 bytes : Flags - File permission bits (in standard unix-style format).
/// * M bytes : FileName - File name (byte array)
/// * 1 byte  : null terminator
#[inline]
fn mar_item_size(name_len: usize) -> usize {
    3 * 4 + name_len + 1
}

struct ProductInformationBlock {
    MARChannelID: Vec<u8>,
    productVersion: Vec<u8>,
}

// Product Information Block (PIB) constants:
const PIB_MAX_MAR_CHANNEL_ID_SIZE: usize = 63;
const PIB_MAX_PRODUCT_VERSION_SIZE: usize = 31;
