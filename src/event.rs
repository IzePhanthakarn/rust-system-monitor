use std::time::Duration;

use crossterm::event::{self, Event as CEvent, KeyEvent};

/// Event ที่เราจะใช้ในแอป
/// - Tick: เกิดตามเวลา (เพื่อ refresh)
/// - Key: เกิดเมื่อ user กดปุ่ม
pub enum Event {
    Tick,
    Key(KeyEvent),
}

pub struct EventHandler {
    tick_rate: Duration,
}

impl EventHandler {
    /// tick_ms เช่น 500 = รีเฟรช 2 ครั้ง/วินาที
    pub fn new(tick_ms: u64) -> Self {
        Self {
            tick_rate: Duration::from_millis(tick_ms),
        }
    }

    /// รอ event ถัดไป
    pub fn next(&mut self) -> std::io::Result<Event> {
        // ---------------------------
        // event::poll(timeout):
        // - ถ้ามี input ภายใน timeout จะอ่าน key ได้ทันที
        // - ถ้าไม่มี input ครบเวลา จะปล่อย Tick ออกไป
        //
        // นี่คือ pattern สุดคลาสสิกของ TUI:
        // "render + handle key + tick refresh"
        // ---------------------------
        if event::poll(self.tick_rate)? {
            match event::read()? {
                CEvent::Key(k) => Ok(Event::Key(k)),
                _ => Ok(Event::Tick), // event อื่น ๆ ไม่สนใจ ก็ถือว่า tick
            }
        } else {
            Ok(Event::Tick)
        }
    }
}