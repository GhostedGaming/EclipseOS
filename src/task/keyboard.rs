use crate::{print, println};
use crate::vga_buffer::{self, Writer};
use conquer_once::spin::OnceCell;
use core::{
    pin::Pin,
    task::{Context, Poll},
};
use crossbeam_queue::ArrayQueue;
use futures_util::{
    stream::{Stream, StreamExt},
    task::AtomicWaker,
};
use pc_keyboard::{layouts, DecodedKey, HandleControl, KeyCode, Keyboard, ScancodeSet1};

static SCANCODE_QUEUE: OnceCell<ArrayQueue<u8>> = OnceCell::uninit();
static WAKER: AtomicWaker = AtomicWaker::new();

/// Called by the keyboard interrupt handler
///
/// Must not block or allocate.
pub(crate) fn add_scancode(scancode: u8) {
    if let Ok(queue) = SCANCODE_QUEUE.try_get() {
        if let Err(_) = queue.push(scancode) {
            println!("WARNING: scancode queue full; dropping keyboard input");
        } else {
            WAKER.wake();
        }
    } else {
        println!("WARNING: scancode queue uninitialized");
    }
}

pub struct ScancodeStream {
    _private: (),
}

impl ScancodeStream {
    pub fn new() -> Self {
        SCANCODE_QUEUE
            .try_init_once(|| ArrayQueue::new(100))
            .expect("ScancodeStream::new should only be called once");
        ScancodeStream { _private: () }
    }
}

impl Stream for ScancodeStream {
    type Item = u8;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<u8>> {
        let queue = SCANCODE_QUEUE
            .try_get()
            .expect("scancode queue not initialized");

        // fast path
        if let Some(scancode) = queue.pop() {
            return Poll::Ready(Some(scancode));
        }

        WAKER.register(&cx.waker());
        match queue.pop() {
            Some(scancode) => {
                WAKER.take();
                Poll::Ready(Some(scancode))
            }
            None => Poll::Pending,
        }
    }
}

pub async fn print_keypresses() {
    let mut scancodes = ScancodeStream::new();
    let mut keyboard = Keyboard::new(
        ScancodeSet1::new(),
        layouts::Us104Key,
        HandleControl::Ignore,
    );

    while let Some(scancode) = scancodes.next().await {
        if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
            if let Some(key) = keyboard.process_keyevent(key_event) {
                match key {
                    DecodedKey::Unicode(character) => {
                        // Don't print backspace character
                        if character != '\u{0008}' && character != '\u{007F}' { 
                            print!("{}", character);
                        } else {
                            // Call backspace function instead
                            crate::vga_buffer::backspace();
                        }
                        if character == '\u{001B}'{

                        }
                    },
                    DecodedKey::RawKey(key) => {
                        match key {
                            KeyCode::Backspace => {
                                // Call backspace function
                                crate::vga_buffer::backspace();
                            },
                            KeyCode::Delete => {
                                // Call backspace function
                                crate::vga_buffer::backspace();
                            },
                            KeyCode::Tab => {
                                print!("    ");
                            },
                            KeyCode::LShift => {},
                            KeyCode::RShift => {},
                            KeyCode::LControl => {},
                            KeyCode::RControl => {},
                            KeyCode::LAlt => {},
                            KeyCode::RAltGr => {},
                            KeyCode::ArrowUp => {},
                            KeyCode::ArrowDown => {},
                            KeyCode::ArrowLeft => {},
                            KeyCode::ArrowRight => {},
                            KeyCode::Escape => {},
                            KeyCode::Home => {},
                            KeyCode::PageUp => {},
                            KeyCode::PageDown => {},
                            KeyCode::CapsLock => {},

                            _ => print!("{:?}", key),
                        }
                    }
                }
            }
        }
    }
}
