use crate::app::App;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Gauge, Paragraph},
};
use ratatui_image::{Resize, StatefulImage};

// ── Palette ──────────────────────────────────────────────────────────────────
const ACCENT: Color    = Color::Rgb(0, 200, 180);
const DIM: Color       = Color::Rgb(80, 90, 100);
const SURFACE: Color   = Color::Rgb(18, 22, 28);
const TEXT: Color      = Color::Rgb(210, 215, 220);
const TEMP_COL: Color  = Color::Rgb(255, 100, 80);
const HUM_COL: Color   = Color::Rgb(80, 160, 255);
const GAS_COL: Color   = Color::Rgb(255, 200, 60);
const OK_COL: Color    = Color::Rgb(60, 210, 120);
const ALERT_COL: Color = Color::Rgb(255, 80, 80);

fn label_style() -> Style { Style::default().fg(DIM) }
fn value_style(color: Color) -> Style {
    Style::default().fg(color).add_modifier(Modifier::BOLD)
}
fn accent_block(title: &str) -> Block<'_> {
    Block::default()
        .title(Span::styled(
            format!(" {} ", title),
            Style::default().fg(ACCENT).add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(DIM))
}

fn temp_pct(t: f32) -> u16 { ((t.clamp(-10.0, 60.0) + 10.0) / 70.0 * 100.0) as u16 }
fn hum_pct(h: f32) -> u16  { h.clamp(0.0, 100.0) as u16 }
fn gas_pct(g: f32) -> u16  { (g.clamp(0.0, 4095.0) / 4095.0 * 100.0) as u16 }

pub fn ui(frame: &mut Frame, app: &mut App) {
    let area = frame.area();
    let root = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(area);
    render_header(frame, root[0], app);
    render_body(frame, root[1], app);
    render_footer(frame, root[2], app);
}

fn render_header(frame: &mut Frame, area: Rect, app: &App) {
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Fill(1), Constraint::Length(28)])
        .split(area);

    frame.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled("ROID", Style::default().fg(ACCENT).add_modifier(Modifier::BOLD)),
            Span::styled("OME", Style::default().fg(TEXT).add_modifier(Modifier::BOLD)),
            Span::styled("  /  Smart Home OS", Style::default().fg(DIM)),
        ]))
        .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).border_style(Style::default().fg(DIM)))
        .alignment(Alignment::Left),
        cols[0],
    );

    let status_color = if app.connected { OK_COL } else { ALERT_COL };
    let status_label = if app.connected { "● LIVE" } else { "○ CONNECTING" };
    frame.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled(status_label, Style::default().fg(status_color).add_modifier(Modifier::BOLD)),
            Span::styled(format!("  msgs {}", app.message_count), Style::default().fg(DIM)),
        ]))
        .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).border_style(Style::default().fg(DIM)))
        .alignment(Alignment::Center),
        cols[1],
    );
}

fn render_body(frame: &mut Frame, area: Rect, app: &mut App) {
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(45),
            Constraint::Percentage(25),
            Constraint::Percentage(30),
        ])
        .split(area);

    render_sensors(frame, cols[0], app);
    render_side(frame, cols[1], app);
    render_camera(frame, cols[2], app);
}

fn render_sensors(frame: &mut Frame, area: Rect, app: &App) {
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
        ])
        .split(area);

    render_sensor_row(frame, rows[0], "TEMPERATURE",
        format!("{:.1} °C", app.temperature), TEMP_COL,
        temp_pct(app.temperature),
        format!("range  −10 → 60 °C   /   device  {}", app.device_id));
    render_sensor_row(frame, rows[1], "HUMIDITY",
        format!("{:.1} %", app.humidity), HUM_COL,
        hum_pct(app.humidity),
        format!("range  0 → 100 %   /   device  {}", app.device_id));
    render_sensor_row(frame, rows[2], "GAS LEVEL",
        format!("{:.0} ADC", app.gas_level), GAS_COL,
        gas_pct(app.gas_level),
        "range  0 → 4095 ADC   /   raw 12-bit read".to_string());
}

fn render_sensor_row(
    frame: &mut Frame, area: Rect, title: &str,
    value: String, color: Color, pct: u16, footnote: String,
) {
    let block = accent_block(title);
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Length(1)])
        .split(inner);

    frame.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled(value, value_style(color)),
            Span::raw("  "),
            Span::styled(footnote, label_style()),
        ])),
        rows[0],
    );
    frame.render_widget(
        Gauge::default()
            .gauge_style(Style::default().fg(color).bg(SURFACE))
            .percent(pct)
            .label(Span::styled(
                format!("{}%", pct),
                Style::default().fg(Color::Black).add_modifier(Modifier::BOLD),
            )),
        rows[1],
    );
}

fn render_side(frame: &mut Frame, area: Rect, app: &App) {
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(55), Constraint::Percentage(45)])
        .split(area);
    render_motion(frame, rows[0], app);
    render_device(frame, rows[1], app);
}

fn render_motion(frame: &mut Frame, area: Rect, app: &App) {
    let (state_label, state_color, detail) = if app.motion {
        ("DETECTED", ALERT_COL, "PIR triggered — snapshot queued")
    } else {
        ("CLEAR", OK_COL, "No movement in sensor range")
    };

    let block = accent_block("MOTION");
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Length(1), Constraint::Min(0)])
        .split(inner);

    frame.render_widget(
        Paragraph::new(Span::styled(state_label,
            Style::default().fg(state_color).add_modifier(Modifier::BOLD))),
        rows[0],
    );
    frame.render_widget(
        Paragraph::new(Span::styled(detail, label_style())),
        rows[1],
    );
    let indicator_char = if app.motion { "▐▌" } else { "  " };
    frame.render_widget(
        Paragraph::new(vec![
            Line::from(""),
            Line::from(Span::styled(indicator_char,
                Style::default().fg(state_color).add_modifier(Modifier::SLOW_BLINK))),
        ]).alignment(Alignment::Center),
        rows[2],
    );
}



fn render_device(frame: &mut Frame, area: Rect, app: &App) {
    let block = accent_block("NODES");
    let inner = block.inner(area);
    frame.render_widget(block, area);

    if app.devices.is_empty() {
        frame.render_widget(
            Paragraph::new(Span::styled("no heartbeats yet", label_style())),
            inner,
        );
        return;
    }

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let rows_constraints: Vec<Constraint> = app.devices
        .keys()
        .map(|_| Constraint::Length(3))
        .collect();

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints(rows_constraints)
        .split(inner);

    for (i, (id, status)) in app.devices.iter().enumerate() {
        if i >= rows.len() { break; }

        let age = now.saturating_sub(status.last_seen);
        let health_color = if age < 60 { OK_COL } else { ALERT_COL };
        let health_label = if age < 60 { "● LIVE" } else { "○ STALE" };

        let node_rows = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Length(1), Constraint::Length(1)])
            .split(rows[i]);

        frame.render_widget(
            Paragraph::new(Line::from(vec![
                Span::styled(health_label, Style::default().fg(health_color).add_modifier(Modifier::BOLD)),
                Span::raw("  "),
                Span::styled(id.as_str(), Style::default().fg(TEXT)),
            ])),
            node_rows[0],
        );
        frame.render_widget(
            Paragraph::new(Span::styled(
                format!("heap {}KB  rssi {}dBm", status.free_heap / 1024, status.rssi),
                label_style(),
            )),
            node_rows[1],
        );
        frame.render_widget(
            Paragraph::new(Span::styled(
                format!("uptime {}s  seen {}s ago", status.uptime / 1000, age),
                label_style(),
            )),
            node_rows[2],
        );
    }
}





fn render_camera(frame: &mut Frame, area: Rect, app: &mut App) {
    let block = accent_block("CAMERA");
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(2)])
        .split(inner);

    match app.image_state {
        None => {
            frame.render_widget(
                Paragraph::new(Line::from(
                    Span::styled("waiting for snapshot...", label_style())
                )).alignment(Alignment::Center),
                rows[0],
            );
        }
        Some(ref mut state) => {
            let image_widget = StatefulImage::new().resize(Resize::Fit(None));
            frame.render_stateful_widget(image_widget, rows[0], state);
        }
    }

    let meta = match &app.latest_frame_path {
        None => "no frame yet".to_string(),
        Some(path) => format!("frame #{} — {}", app.frame_count, path),
    };
    frame.render_widget(
        Paragraph::new(Span::styled(meta, label_style())),
        rows[1],
    );
}

fn render_footer(frame: &mut Frame, area: Rect, _app: &App) {
    frame.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled(" q ", Style::default().fg(Color::Black).bg(DIM)),
            Span::styled("  quit", label_style()),
            Span::styled("     RoidOME v0.1  —  Distributed Smart Home OS", label_style()),
        ]))
        .block(Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(DIM))),
        area,
    );
}
