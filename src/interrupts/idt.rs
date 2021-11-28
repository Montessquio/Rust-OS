//! Contains structures and methods for working with an x86 IDT.

use bit_field::BitField;
use core::marker::PhantomData;
use x86_64::structures::idt::{InterruptStackFrame, PageFaultErrorCode};

#[derive(Clone, Debug)]
#[repr(C)]
#[repr(align(16))]
pub struct InterruptDescriptorTable {
    pub divide_by_zero: Entry<Handler>,
    pub debug: Entry<Handler>,
    pub non_maskable_interrupt: Entry<Handler>,
    pub breakpoint: Entry<Handler>,
    pub overflow: Entry<Handler>,
    pub bound_range_exceeded: Entry<Handler>,
    pub invalid_opcode: Entry<Handler>,
    pub device_not_available: Entry<Handler>,
    pub double_fault: Entry<DivergingHandlerWithErrCode>,
    coprocessor_segment_overrun: Entry<Handler>, // Corprocessor Segment Overrun
    pub invalid_tss: Entry<HandlerWithErrCode>,
    pub segment_not_present: Entry<HandlerWithErrCode>,
    pub stack_segment_fault: Entry<HandlerWithErrCode>,
    pub general_protection_fault: Entry<HandlerWithErrCode>,
    pub page_fault: Entry<PageFaultHandler>,
    reserved_1: Entry<Handler>,
    pub x87_floating_point: Entry<Handler>,
    pub alignment_check: Entry<HandlerWithErrCode>,
    pub machine_check: Entry<DivergingHandler>,
    pub simd_floating_point: Entry<Handler>,
    pub virtualization: Entry<Handler>,
    reserved_2: [Entry<Handler>; 9], // 9 CPU reserved entries.
    pub security_exception: Entry<HandlerWithErrCode>,
    reserved_3: Entry<Handler>,
    user_interrupts: [Entry<Handler>; 224]
}

impl InterruptDescriptorTable {
    /// Create a new IDT with all empty entries.
    #[inline]
    pub const fn new() -> InterruptDescriptorTable {
        InterruptDescriptorTable {
            divide_by_zero: Entry::empty(),
            debug: Entry::empty(),
            non_maskable_interrupt: Entry::empty(),
            breakpoint: Entry::empty(),
            overflow: Entry::empty(),
            bound_range_exceeded: Entry::empty(),
            invalid_opcode: Entry::empty(),
            device_not_available: Entry::empty(),
            double_fault: Entry::empty(),
            coprocessor_segment_overrun: Entry::empty(),
            invalid_tss: Entry::empty(),
            segment_not_present: Entry::empty(),
            stack_segment_fault: Entry::empty(),
            general_protection_fault: Entry::empty(),
            page_fault: Entry::empty(),
            reserved_1: Entry::empty(),
            x87_floating_point: Entry::empty(),
            alignment_check: Entry::empty(),
            machine_check: Entry::empty(),
            simd_floating_point: Entry::empty(),
            virtualization: Entry::empty(),
            reserved_2: [Entry::empty(); 9],
            security_exception: Entry::empty(),
            reserved_3: Entry::empty(),
            user_interrupts: [Entry::empty(); 224],
        }
    }

    /// Sets all entries to empty in place.
    #[inline]
    pub fn clear(&mut self) {
        *self = Self::new();
    }

    /// Uses the "lidt" command.
    /// This is safe because the IDT must be
    /// bounded over the static lifetime.
    #[inline]
    pub fn load(&'static self) {
        unsafe { self.load_unsafe() }
    }

    #[inline]
    pub unsafe fn load_unsafe(&self) {
        use x86_64::instructions::tables::lidt;
        lidt(&self.pointer());
    }

    // Return a smart pointer to ourselves.
    fn pointer(&self) -> x86_64::structures::DescriptorTablePointer {
        use core::mem::size_of;
        x86_64::structures::DescriptorTablePointer {
            base: x86_64::VirtAddr::new(self as *const _ as u64),
            limit: (size_of::<Self>() - 1) as u16,
        }
    }
}

impl core::ops::Index<usize> for InterruptDescriptorTable {
    type Output = Entry<Handler>;

    /// Returns the IDT entry with the specified index.
    ///
    /// Panics if index is outside the IDT (i.e. greater than 255) or if the entry is an
    /// exception that pushes an error code (use the struct fields for accessing these entries).
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.divide_by_zero,
            1 => &self.debug,
            2 => &self.non_maskable_interrupt,
            3 => &self.breakpoint,
            4 => &self.overflow,
            5 => &self.bound_range_exceeded,
            6 => &self.invalid_opcode,
            7 => &self.device_not_available,
            9 => &self.coprocessor_segment_overrun,
            16 => &self.x87_floating_point,
            19 => &self.simd_floating_point,
            20 => &self.virtualization,
            i @ 32..=255 => &self.user_interrupts[i - 32],
            i @ 15 | i @ 31 | i @ 21..=29 => panic!("entry {} is reserved", i),
            i @ 8 | i @ 10..=14 | i @ 17 | i @ 30 => {
                panic!("entry {} is an exception with error code", i)
            }
            i @ 18 => panic!("entry {} is an diverging exception (must not return)", i),
            i => panic!("no entry with index {}", i),
        }
    }
}

impl core::ops::IndexMut<usize> for InterruptDescriptorTable {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.divide_by_zero,
            1 => &mut self.debug,
            2 => &mut self.non_maskable_interrupt,
            3 => &mut self.breakpoint,
            4 => &mut self.overflow,
            5 => &mut self.bound_range_exceeded,
            6 => &mut self.invalid_opcode,
            7 => &mut self.device_not_available,
            9 => &mut self.coprocessor_segment_overrun,
            16 => &mut self.x87_floating_point,
            19 => &mut self.simd_floating_point,
            20 => &mut self.virtualization,
            i @ 32..=255 => &mut self.user_interrupts[i - 32],
            i @ 15 | i @ 31 | i @ 21..=29 => panic!("entry {} is reserved", i),
            i @ 8 | i @ 10..=14 | i @ 17 | i @ 30 => {
                panic!("entry {} is an exception with error code", i)
            }
            i @ 18 => panic!("entry {} is an diverging exception (must not return)", i),
            i => panic!("no entry with index {}", i),
        }
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Entry<F> {
    ptr_lo: u16,
    gdt_selector: u16,
    options: EntryOptions,
    ptr_md: u16,
    ptr_hi:  u32,
    reserved: u32,
    phantom: core::marker::PhantomData<F>,
}

impl <T> core::fmt::Debug for Entry<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Entry")
            .field("handler_addr", &format_args!("{:#x}", self.handler_addr()))
            .field("gdt_selector", &self.gdt_selector)
            .field("options", &self.options)
            .finish()
    }
}

impl<T> PartialEq for Entry<T> {
    fn eq(&self, other: &Self) -> bool {
        self.ptr_lo == other.ptr_lo
            && self.gdt_selector == other.gdt_selector
            && self.options == other.options
            && self.ptr_md == other.ptr_md
            && self.ptr_hi == other.ptr_hi
            && self.reserved == other.reserved
    }
}

pub type Handler = extern "x86-interrupt" fn(InterruptStackFrame);
pub type HandlerWithErrCode = extern "x86-interrupt" fn(InterruptStackFrame, error_code: u64);
pub type PageFaultHandler = extern "x86-interrupt" fn(InterruptStackFrame, error_code: PageFaultErrorCode);
pub type DivergingHandler = extern "x86-interrupt" fn(InterruptStackFrame) -> !;
pub type DivergingHandlerWithErrCode = extern "x86-interrupt" fn(InterruptStackFrame, error_code: u64) -> !;

impl<F> Entry<F> {
    #[inline]
    pub const fn empty() -> Self {
        Entry { ptr_lo: 0, gdt_selector: 0, options: EntryOptions::minimal(), ptr_md: 0, ptr_hi: 0, reserved: 0, phantom: PhantomData }
    }

    #[inline]
    pub unsafe fn set_handler_addr(&mut self, addr: x86_64::VirtAddr) -> &mut EntryOptions {
        let addr = addr.as_u64();

        self.ptr_lo = addr as u16;
        self.ptr_md = (addr >> 16) as u16;
        self.ptr_hi = (addr >> 32) as u32;

        use x86_64::instructions::segmentation::Segment;
        self.gdt_selector = x86_64::instructions::segmentation::CS::get_reg().0;

        self.options.set_present(true);
        &mut self.options
    }

    #[inline]
    fn handler_addr(&self) -> u64 {
        self.ptr_lo as u64
            | (self.ptr_md as u64) << 16
            | (self.ptr_hi as u64) << 32
    }
}

// Macro copied unchanged from x86_64 crate.
macro_rules! impl_set_handler_fn {
    ($h:ty) => {
        impl Entry<$h> {
            #[inline]
            pub fn set_handler_fn(&mut self, handler: $h) -> &mut EntryOptions {
                let handler = x86_64::VirtAddr::new(handler as u64);
                unsafe { self.set_handler_addr(handler) }
            }
        }
    };
}

impl_set_handler_fn!(Handler);
impl_set_handler_fn!(HandlerWithErrCode);
impl_set_handler_fn!(PageFaultHandler);
impl_set_handler_fn!(DivergingHandler);
impl_set_handler_fn!(DivergingHandlerWithErrCode);

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq)]
pub struct EntryOptions(u16);


impl core::fmt::Debug for EntryOptions {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_tuple("EntryOptions")
            .field(&format_args!("{:#06x}", self.0))
            .finish()
    }
}

impl EntryOptions {
    #[inline]
    const fn minimal() -> Self {
        EntryOptions(0b1110_0000_0000)
    }

    #[inline]
    pub fn set_present(&mut self, present: bool) -> &mut Self {
        self.0.set_bit(15, present);
        self
    }

    #[inline]
    pub fn disable_interrupts(&mut self, disable: bool) -> &mut Self {
        self.0.set_bit(8, !disable);
        self
    }

    #[inline]
    pub fn set_privilege_level(&mut self, dpl: x86_64::PrivilegeLevel) -> &mut Self {
        self.0.set_bits(13..15, dpl as u16);
        self
    }

    #[inline]
    pub unsafe fn set_stack_index(&mut self, index: u16) -> &mut Self {
        // The hardware IST index starts at 1, but our software IST index
        // starts at 0. Therefore we need to add 1 here.
        self.0.set_bits(0..3, index + 1);
        self
    }
}