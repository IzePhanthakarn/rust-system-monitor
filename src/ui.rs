use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::*,
    widgets::{Block, Borders, Paragraph, Row, Table, Wrap},
};

use crate::app::App;

/// วาด UI ทั้งหน้าจอ
pub fn draw(f: &mut Frame, app: &App) {
    let size = f.size();

    // แบ่งพื้นที่ทั้งหน้าจอเป็น 3 ส่วน: header / body / footer
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // header
            Constraint::Min(0),    // body
            Constraint::Length(2), // footer
        ])
        .split(size);

    draw_header(f, chunks[0], app);
    draw_body(f, chunks[1], app);
    draw_footer(f, chunks[2], app);
}

fn draw_header(f: &mut Frame, area: Rect, app: &App) {
    let s = &app.stats;
    let text = format!(
        "CPU: {:.1}% | Cores: {} | Load: {:.2} {:.2} {:.2} | RAM: {:.0}% ({:.2}/{:.2} GB)",
        s.cpu_pct, s.cores, s.load_1, s.load_5, s.load_15, s.ram_pct, s.ram_used_gb, s.ram_total_gb
    );

    let w = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).title("Server Monitor"))
        .wrap(Wrap { trim: true });

    f.render_widget(w, area);
}

fn draw_body(f: &mut Frame, area: Rect, app: &App) {
    // body แบ่งเป็นซ้าย(ดิสก์) / ขวา(top process)
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(area);

    draw_disks(f, cols[0], app);
    draw_processes(f, cols[1], app);
}

fn draw_disks(f: &mut Frame, area: Rect, app: &App) {
    let rows: Vec<Row> = app
        .stats
        .disks
        .iter()
        .map(|d| {
            Row::new(vec![
                d.mount.clone(),
                format!("{:.1} GB", d.used_gb),
                format!("{:.1} GB", d.total_gb),
                format!("{:.0}%", d.pct),
            ])
        })
        .collect();

    let table = Table::new(
        rows,
        [
            Constraint::Percentage(45),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(15),
        ],
    )
    .header(Row::new(vec!["Mount", "Used", "Total", "%"]))
    .block(Block::default().borders(Borders::ALL).title("Disks"));

    f.render_widget(table, area);
}

fn draw_processes(f: &mut Frame, area: Rect, app: &App) {
    let rows: Vec<Row> = app
        .stats
        .top_procs
        .iter()
        .map(|p| {
            Row::new(vec![
                p.pid.clone(),
                format!("{:.1}", p.mem_mb),
                format!("{:.1}", p.cpu_pct),
                p.name.clone(),
            ])
        })
        .collect();

    let title = format!("Top Processes (top_n = {})", app.top_n);
    let table = Table::new(
        rows,
        [
            Constraint::Length(8),
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Min(10),
        ],
    )
    .header(Row::new(vec!["PID", "MEM(MB)", "CPU(%)", "NAME"]))
    .block(Block::default().borders(Borders::ALL).title(title));

    f.render_widget(table, area);
}

fn draw_footer(f: &mut Frame, area: Rect, app: &App) {
    let text = format!("Keys: q/Esc = quit | +/- = change top_n (current {})", app.top_n);
    let w = Paragraph::new(text).block(Block::default().borders(Borders::ALL));
    f.render_widget(w, area);
}