use i386::instructions::port::*;
use i386::structures::idt::ExceptionStackFrame;
use print::Printer;
use core::fmt::Write;

fn acknowledge_irq(irq: u8) {
    unsafe {
        // TODO: this is probably very unsafe. Maybe. I don't really know.
        outb(0x20, 0x20);
        if irq >= 8 {
            outb(0xA0, 0x20);
        }
    }
}

macro_rules! irq_handler {
    ($irq:expr, $name:ident) => {{
        extern "x86-interrupt" fn $name(stack_frame: &mut ExceptionStackFrame) {
            // TODO: Reroute this into a generic interrupt system?
            acknowledge_irq($irq);
        }
        $name
    }}
}

pub const IRQ_HANDLERS : [extern "x86-interrupt" fn(stack_frame: &mut ExceptionStackFrame); 16] = [
    irq_handler!(0, timer_handler),
    keyboard_handler,
    //irq_handler!(1, keyboard_handler);
    irq_handler!(2, cascade_handler),
    irq_handler!(3, serial2_handler),
    irq_handler!(4, serial1_handler),
    irq_handler!(5, sound_handler),
    irq_handler!(6, floppy_handler),
    irq_handler!(7, parallel1_handler),
    irq_handler!(8, rtc_handler),
    irq_handler!(9, acpi_handler),
    irq_handler!(10, irq10_handler),
    irq_handler!(11, irq11_handler),
    irq_handler!(12, mouse_handler),
    irq_handler!(13, irq13_handler),
    irq_handler!(14, primary_ata_handler),
    irq_handler!(15, secondary_ata_handler),
];


extern "x86-interrupt" fn keyboard_handler(stack_frame: &mut ExceptionStackFrame) {
    const KEYBOARD_MAP : [char; 59] = [
        '\x00', '\x1b', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '-', '=', '\x08',
        '\t', 'q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p', '[', ']', '\n',
        '\x00', 'a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', ';', '\'', '`', '\x00',
        '\\', 'z', 'x', 'c', 'v', 'b', 'n', 'm', ',', '.', '/', '\x00', '*',
        '\x00', ' ', '\x00'
    ];
    //writeln!(Printer, "Keyboard! {:#?}", stack_frame);
    unsafe {
        let status = inb(0x64);
        if status & 0x01 != 0 {
            let keycode = inb(0x60);
            if (keycode as usize) < KEYBOARD_MAP.len() && KEYBOARD_MAP[keycode as usize] != '\x00' {
                write!(Printer, "{}", KEYBOARD_MAP[keycode as usize]);
            }
        }
    }
    acknowledge_irq(1);
}
