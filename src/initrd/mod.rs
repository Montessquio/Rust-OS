//! This file contains the initrd system that
//! loads extra drivers necessary for boot, such as
//! file system and hard drive drivers.

use core::convert::TryInto;
use lazy_static::lazy_static;
use alloc::{borrow::ToOwned, string::String, collections::BTreeMap, string::ToString};

lazy_static! {
    pub static ref INITRD: USTAR = USTAR::new(include_bytes!("initrd.tar"));
}

#[derive(Debug)]
pub struct USTAR {
    headers: BTreeMap<String, USTARHeader>,
    blob: &'static [u8],
}

impl USTAR {
    pub fn new(buf: &'static [u8]) -> Self {
        let buf_iter = buf.iter();
        let mut headers: BTreeMap<String, USTARHeader> = alloc::collections::BTreeMap::new();
        let mut skip = 0;

        // Iterate over each block in the TAR archive.
        for (i, block) in buf.chunks_exact(512).enumerate() {
            // Skip file contents. This prevent us from peeking into
            // sub-archives embedded in the main archive.
            if skip != 0 {
                // Catch files smaller than 1 block.
                if skip < 512 { skip = 512; }
                skip = skip - 512;
                continue;
            }
            
            // Try to create a header from the current block.
            if let Ok(header) = USTARHeader::from_block(block, i) {
                let name = header.name.trim_matches('.');
                let typeflag = header.typeflag.clone();
                let size = header.size.clone();

                headers.insert(name.to_string(), header.clone());

                // Set up variable to skip
                // file contents.
                use USTARFileType::*;
                match typeflag {
                    REGTYPE|AREGTYPE => skip = size,
                    _ => skip = 0,
                }
            }
        }

        USTAR{headers, blob: buf}
    }

    // Returns Some if the file exists and None if the file
    // does not exist.
    pub fn open(&self, name: String) -> Option<&[u8]> {
        match self.headers.get(&name) {
            Some(header) => { 
                let lower = header.offset+512;
                let upper = header.offset+512+(header.size as usize);
                Some(&self.blob[lower..upper])
            },
            None => None,
        }
    }

    pub fn entries(&self) -> alloc::vec::Vec<USTARHeader> {
        let mut v: alloc::vec::Vec<USTARHeader> = alloc::vec::Vec::new();
        for val in self.headers.values() {
            v.push(val.clone());
        }
        v
    }
}

#[derive(Clone, Debug)]
pub struct USTARHeader {
    pub name:       String, // Offset 0, Len 100
    pub size:       u128,   // Offset 124, Len 12
    pub typeflag:   USTARFileType, // O 156, L1
    pub offset:     usize, // Contains this header's offset from the start of the file in bytes.

}

impl USTARHeader {
    /// Parses a single header from a data block.
    /// Returns Ok(USTARHeader) if it is a valid
    /// header. Returns an error if it is not a header.
    pub fn from_block(block: &[u8], offset: usize) -> Result<USTARHeader, &'static str> {
        // Check the magic header value
        // for the value `ustar`.
        if block[257..263].to_owned() != [117, 115, 116, 97, 114, 0] {
            return Err("format is not USTAR!");
        }

        // We only care about three fields in the archive.
        let name: String = block[0..100].iter().map(|c| { *c as char }).collect::<String>().trim_matches(char::from(0)).to_owned().trim_start_matches('.').to_owned();
        let size: u128 = u128::from_str_radix(&block[124..135].iter().map(|c| { *c as char }).collect::<String>(), 8).expect("Size was not valid octal!");
        let typeflag = USTARFileType::from_char(block[156] as char);
        let offset = offset * 512;
        Ok(USTARHeader{name, size, typeflag, offset})
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum USTARFileType {
    REGTYPE,
    AREGTYPE,
    LNKTYPE,
    SYMTYPE,
    CHRTYPE,
    BLKTYPE,
    DIRTYPE,
    FIFOTYPE,
    CONTTYPE,
}

impl USTARFileType {
    pub fn from_char(i: char) -> Self {
        use USTARFileType::*;
        match i {
            '0' => REGTYPE,
            '\0'=> AREGTYPE,
            '1' => LNKTYPE,
            '2' => SYMTYPE,
            '3' => CHRTYPE,
            '4' => BLKTYPE,
            '5' => DIRTYPE,
            '6' => FIFOTYPE,
            '7' => CONTTYPE,
            _ => REGTYPE,
        }
    }
}