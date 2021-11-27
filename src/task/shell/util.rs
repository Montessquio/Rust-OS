// The echo command simply repeats its arguments back to the user.
// The echoparam command does so with debug output.
use alloc::vec::Vec;
use crate::{print, println};

pub fn help() {
    println!(r#"Kernel Shell (ksh) version 0.0.1

    echo <params...>:       Shell test utility. Display a line of text.
    echoparam <params...>:  Shell test utility. 
                            Display given arguments in debug format.
    fizzbuzz <u32>:         Shell demo routine. 
                            Performs the fizzbuzz challenge with the
                            given number of iterations.
    ls  <path>:             List directories and files in the initramfs.
    exec <path>:            Run executables in the initramfs.
    print [-a] <path>:      Print the hex values of files in the initramfs.
                            The -a flag prints the files as ASCII."#);
}

pub fn echo(mut argv: Vec<&str>) {
    // Remove command from the args before printing.
    argv.remove(0);
    for arg in argv {
        print!("{} ", arg);
    }
    print!("\n");
}

pub fn echoparam(mut argv: Vec<&str>) {
    if let Some(last) = argv.pop() {
        print!("[");
        for arg in argv {
            print!("\"{}\", ", arg);
        }
        println!("\"{}\"]", last);
    } else {
        println!("None");
    }
}

pub fn fizzbuzz(mut argv: Vec<&str>) {
    if let Some(asciinum) = argv.get(1) {
        if let Ok(n) = asciinum.parse::<u32>() {
            for i in 1..n {
                match (i%3, i%5) {
                    (0, 0) => println!("FizzBuzz"),
                    (0, _) => println!("Fizz"),
                    (_, 0) => println!("Buzz"),
                    (_, _) => println!("{}", i)
                }
            }
        }
    } else {
        println!("Usage: fizzbuzz <iterations: integer>");
    }
}