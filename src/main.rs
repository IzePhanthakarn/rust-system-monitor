mod app;
mod event;
mod sys;
mod ui;

use std::io;

use app::App;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use event::{Event, EventHandler};
use ratatui::{backend::CrosstermBackend, Terminal};

fn main() -> io::Result<()> {
    // ---------------------------
    // โหมด Raw mode:
    // - ทำให้เราอ่านคีย์บอร์ดได้ "ทันที" (ไม่ต้องรอ Enter)
    // - เป็นสิ่งที่ไม่ค่อยเจอใน CLI ปกติ แต่จำเป็นมากสำหรับ TUI
    // ---------------------------
    enable_raw_mode()?;

    // ---------------------------
    // EnterAlternateScreen:
    // - สลับเข้า "หน้าจอพิเศษ" (เหมือน htop)
    // - ออกจากโปรแกรมแล้วจะกลับสู่หน้าจอเดิม
    // ---------------------------
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // state แอป (สิ่งที่จะแสดง)
    let mut app = App::new();

    // event loop handler: รวม input + tick (รีเฟรช)
    let mut events = EventHandler::new(500); // tick ทุก 500ms

    // เตรียมข้อมูลครั้งแรก
    app.refresh();

    // loop หลักของ TUI
    loop {
        terminal.draw(|f| ui::draw(f, &app))?;

        match events.next()? {
            Event::Tick => {
                // ทุก tick จะ refresh ข้อมูล (CPU/MEM/DISK/PROCESS)
                app.refresh();
            }
            Event::Key(key) => {
                // ส่ง key ไปให้ app ตัดสินใจ
                if app.on_key(key) {
                    break; // true = ออกจากโปรแกรม
                }
            }
        }
    }

    // ออกโปรแกรมแบบ "คืนสภาพ terminal"
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}