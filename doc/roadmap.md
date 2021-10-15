# Rust-OS Roadmap

The final aim of this project is to have a kernel that can load programs into userspace, and provide sytem call capabilities to them.
Furthermore, it will be able to concurrently execute multiple programs using preemptive scheduling.


The Kernel will be a hybrid kernel, with the following split:

- Kernel
    - Memory Management
    - Interrupt and Peripheral Management
    - Multitasking
    - Message Passing
    - Kernel Threads
    - Process Loading

- Userspace
    - STDLib/LibC
    - Process Threads
    - I/O Drivers
    - Filesystem Drivers (Via FUSE)
    - Message Passing (See Below)


This requires multiple sections:

## Memory Management

- X Paging, with frame allocation and 
  - huge page support.
- X A heap allocator 
  - X with deallocation support.
- X TSS stack switching for interrupt handlers 
  - and userspace programs.

## Interrupts

- Kernel should handle CPU exceptions properly, including OOM Killing tasks
  and guard page support.
- Kernel should provide a set of system calls to enable the other features.

## Peripherals

- Kernel should be able to give programs a CLI buffer to use, either
  in terminal mode or x,y mode as appropriate.
- Pixel buffer mode stretch goal.
- Kernel should provide serial system calls to get input from a serial
  keyboard. 
- USB keyboard driver stretch goal.

## Filesystems

- A FUSE-like interface on the Kernel Side. The Kernel will provide
  the filesystem interface abstraction with all userspace FS drivers
  must implement.
- A Ramdisk driver (FAT?) for boot-time code that can be compiled
  separately.

## Multitasking

- Preemptive multitasking with a simple priority round-robin queue.
- Message passing via kernel and filesystem methods, but also an 
  opt-in parent-child direct messaging capability (experimental).
- Kernel should use cooperative multitasking (rust's Async/Await).

## Message Passing

- Kernel facilitates message passing via filesystem I/O (files)
  and system calls to PID or parent.
- Processes may opt-in to a trusted "direct" shared memory approach
  to message passing that does not involve the kernel (experimental)

## Process Loading

- Kernel should boot into a kernel shell (ksh, program loaded 
  from ramdisk). From there, the user may run the programs in
  the ramdisk, or run *init* which will spawn the userspace.
- Programs can run fork() and then exec() to spawn new programs
  into the process tree, but only in userspace.

## LibC

- Kernel-provided LIBC will be a modified MUSL libc.

## I/O and Filesystems

- Hardware access given to Userspace drivers through
  system calls.
- Filesystem drivers will implement the FUSE API.