//! Filesystem manipulation commands.
use alloc::vec::Vec;
use crate::print;
use crate::alloc::borrow::ToOwned;

pub fn ls(mut argv: Vec<&str>) {
    // Remove command from the args before printing.
    if argv.len() != 2 {
        print!("{} requires exactly one argument!\n", argv[0]);
        return;
    }

    argv.remove(0);

    use crate::initrd::USTARFileType;
    for entry in crate::initrd::INITRD.entries() {
        if entry.name == argv[0] && entry.typeflag == USTARFileType::DIRTYPE {
            print!("Index of {}:\n", entry.name);
            for entry in crate::initrd::INITRD.entries() {
                if entry.name.starts_with(argv[0]) && entry.name != argv[0] {
                    match entry.typeflag {
                        USTARFileType::REGTYPE|
                        USTARFileType::AREGTYPE => print!("F {} {}\n", entry.size, entry.name),
                        USTARFileType::DIRTYPE =>  print!("D {} {}\n", entry.size, entry.name),
                        _ => print!("? {}\n", entry.name),
                    }
                }
            }
            return;
        }
    }

    print!("Directory not found.\n")
}

pub fn run(mut argv: Vec<&str>) {
    // Remove command from the args before printing.
    argv.remove(0);
    print!("Unimplemented!\n");
}

pub fn print(mut argv: Vec<&str>) {
    if argv.len() != 2 && argv.len() != 3 {
        print!("{} requires exactly one argument!\n", argv[0]);
        return;
    }

    // Remove command from the args before printing.
    argv.remove(0);

    if argv[0] == "-a" {
        if let Some(blob) = crate::initrd::INITRD.open(argv[1].to_owned()) {
            for byte in blob {
                if *byte as char != '\0' {
                    print!("{}", *byte as char);
                }
            }
    
            print!("\n");
            return
        }
    
        print!("File not found.\n");
    } else {
        if let Some(blob) = crate::initrd::INITRD.open(argv[0].to_owned()) {
            for (i, byte) in blob.iter().enumerate() {
                if i % 8 == 0 {
                    print!("\n");
                }
                print!("{:X?} ", byte);
            }
    
            print!("\n");
            return
        }
    
        print!("File not found.\n");
    }
}