//! A kernel shell implementation.

use crate::{print, println};
use crate::interrupts::serialkbd::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1, KeyCode};
use futures_util::StreamExt;
use alloc::collections::VecDeque;
use alloc::vec::Vec;
use alloc::string::String;
use super::keyboard::{ScancodeStream};

mod util;
mod fs;

/// The Kernel Shell takes full control of the serial keyboard driver.
/// 
/// Support for SPMC channels for this driver are TODO.
/// 
/// Also note that the ksh CAN cause kernel panics as all commands
/// are executed in kernel mode.
pub async fn ksh_main() {
    let mut scancodes = ScancodeStream::new();
    let mut keyboard = Keyboard::new(layouts::Us104Key, ScancodeSet1,
        HandleControl::Ignore);

    loop {
        print!("ksh > ");
        let s = get_line_display(&mut scancodes, &mut keyboard).await;
        let s : Vec<&str> = s.split(' ').collect(); 

        match s[0] {
            "echo" => util::echo(s),
            "echoparam" => util::echoparam(s),
            "fizz" | "fizzbuzz" => util::fizzbuzz(s),
            "ls" | "dir" => fs::ls(s),
            "run" | "exec" => fs::run(s),
            "print" | "show" => fs::print(s),
            "help" => util::help(),
            _ => println!("Unknown command. Type 'help' for a list of commands."),
        }
    }
}

/// Retrieve a single line, terminated by a newline character, from the serial keyboard.
/// Will not allow deleting past the column it started in.
async fn get_line_display(scancodes: &mut ScancodeStream, keyboard: &mut Keyboard<layouts::Us104Key, ScancodeSet1>) -> String {
    let mut out : String = String::with_capacity(80);
    'stringbuilder: loop {
        match getchar(scancodes, keyboard).await {
            DecodedKey::Unicode(character) => match character {
                '\n' => {
                    print!("\n");
                    break 'stringbuilder;
                },
                '\x08' => {
                    // Only delete characters we've processed.
                    if let Some(_) = out.pop() {
                        print!("{}", '\x08');
                    }
                }
                _ => {
                    out.push(character);
                    print!("{}", character);
                }
            },
            DecodedKey::RawKey(_) => {},
        }
    }
    out
}

/// Retrieve a single character from the serial keyboard.
#[inline]
async fn getchar(scancodes: &mut ScancodeStream, keyboard: &mut Keyboard<layouts::Us104Key, ScancodeSet1>) -> DecodedKey {
    loop {
        if let Some(scancode) = scancodes.next().await {
            if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
                if let Some(key) = keyboard.process_keyevent(key_event) {
                    return key;
                }
            }
        }
    }
}