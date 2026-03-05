use crossterm::event::{KeyCode, KeyEvent};

use crate::sys::{MachineStats, SysReader};

/// App คือ state ของโปรแกรม TUI
/// - เก็บข้อมูลล่าสุดที่ดึงได้จากเครื่อง
/// - เก็บ setting ต่าง ๆ เช่นจำนวน process ที่แสดง
pub struct App {
    pub reader: SysReader,
    pub stats: MachineStats,
    pub top_n: usize,
}

impl App {
    pub fn new() -> Self {
        let mut reader = SysReader::new();
        let stats = reader.read_all(10);

        Self {
            reader,
            stats,
            top_n: 10,
        }
    }

    /// refresh ข้อมูลล่าสุดจากระบบ
    pub fn refresh(&mut self) {
        self.stats = self.reader.read_all(self.top_n);
    }

    /// handle คีย์จากผู้ใช้
    /// คืนค่า true = ให้ออกจากโปรแกรม
    pub fn on_key(&mut self, key: KeyEvent) -> bool {
        match key.code {
            // q หรือ Esc = ออก
            KeyCode::Char('q') | KeyCode::Esc => true,

            // เพิ่ม/ลดจำนวน process ที่โชว์
            // นี่เป็น pattern ที่ใช้บ่อยใน TUI: key -> เปลี่ยน state -> render ใหม่
            KeyCode::Char('+') | KeyCode::Char('=') => {
                self.top_n = (self.top_n + 5).min(50);
                self.refresh();
                false
            }
            KeyCode::Char('-') => {
                self.top_n = self.top_n.saturating_sub(5).max(5);
                self.refresh();
                false
            }

            _ => false,
        }
    }
}