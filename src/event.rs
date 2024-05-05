use crate::{profile, term_disable_stdio_buffer, term_read_char, term_reenable_stdio_buffer};

#[derive(Debug)]
pub enum Event {
    CharEvent(char),
    // MouseButton(Mouse, Action),
    // MouseMove(i32, i32),
    // Scroll(i32, i32),
    Resize(i64, i64),
}

pub(crate) struct EventHandler {
    width: i64,
    height: i64,
}

impl EventHandler {
    pub fn new() -> EventHandler {
        let (width, height) = if let Some((w, h)) = term_size::dimensions() {
            (w as i64, h as i64)
        } else {
            (0, 0)
        };

        unsafe { term_disable_stdio_buffer() }; // My first ever unsafe code ^^

        EventHandler { width, height }
    }

    fn char_event_get(&self) -> Vec<Event> {
        profile!();
        let mut input: Vec<u8> = vec![];

        let mut c = unsafe { term_read_char() };
        if c == 0 {
            return vec![];
        }
        input.push(c);

        loop {
            c = unsafe { term_read_char() };
            if c == 0 {
                break;
            }
            input.push(c);
        }

        String::from_utf8(input)
            .expect("read() read non utf-8 byte")
            .chars()
            .filter(|c| *c != 1 as char)
            .map(|c| Event::CharEvent(c))
            .collect()
    }

    fn resize_event_get(&mut self) -> Option<Event> {
        if let Some((w, h)) = term_size::dimensions() {
            let w = w as i64;
            let h = h as i64;
            if self.width != w || self.height != h {
                self.width = w;
                self.height = h;
                Some(Event::Resize(w, h))
            } else {
                None
            }
        } else {
        None
        }
    }

    pub(crate) fn get_events(&mut self) -> Vec<Event> {
        let mut events = self.char_event_get();

        if let Some(e) = self.resize_event_get() {
            events.push(e);
        }

        events
    }
}

impl Drop for EventHandler {
    fn drop(&mut self) {
        unsafe { term_reenable_stdio_buffer() };
    }
}
