use crate::app::App;






use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Gauge},
};

// ── Palette ──────────────────────────────────────────────────────────────────
const ACCENT: Color     = Color::Rgb(0, 200, 180);   // teal — RoidOME brand
const DIM: Color        = Color::Rgb(80, 90, 100);   // muted borders / labels
const SURFACE: Color    = Color::Rgb(18, 22, 28);    // dark background hint
const TEXT: Color       = Color::Rgb(210, 215, 220); // primary text
const TEMP_COL: Color   = Color::Rgb(255, 100, 80);  // warm red — heat
const HUM_COL: Color    = Color::Rgb(80, 160, 255);  // cool blue — moisture
const GAS_COL: Color    = Color::Rgb(255, 200, 60);  // amber — caution
const OK_COL: Color     = Color::Rgb(60, 210, 120);  // green — safe / clear
const ALERT_COL: Color  = Color::Rgb(255, 80, 80);   // red — motion / alert

// ── Style helpers ─────────────────────────────────────────────────────────────
fn label_style() -> Style {
    Style::default().fg(DIM)
}
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

// ── Gauge normalisation ───────────────────────────────────────────────────────
fn temp_pct(t: f32) -> u16 {
    ((t.clamp(-10.0, 60.0) + 10.0) / 70.0 * 100.0) as u16
}
fn hum_pct(h: f32) -> u16 {
    h.clamp(0.0, 100.0) as u16
}
fn gas_pct(g: f32) -> u16 {
    (g.clamp(0.0, 4095.0) / 4095.0 * 100.0) as u16
}

// ── Main entry ───────────────────────────────────────────────────────────────
pub fn ui(frame: &mut Frame, app: &App) {
    let area = frame.area();

    // Root: header | body | footer
    let root = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),   // header
            Constraint::Min(0),      // body
            Constraint::Length(3),   // footer
        ])
        .split(area);

    render_header(frame, root[0], app);
    render_body(frame, root[1], app);
    render_footer(frame, root[2], app);
}

// ── Header ───────────────────────────────────────────────────────────────────
fn render_header(frame: &mut Frame, area: Rect, app: &App) {
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Fill(1),      // brand
            Constraint::Length(28),   // connection status
        ])
        .split(area);

    // Brand
    let brand = Paragraph::new(Line::from(vec![
        Span::styled("ROID", Style::default().fg(ACCENT).add_modifier(Modifier::BOLD)),
        Span::styled("OME", Style::default().fg(TEXT).add_modifier(Modifier::BOLD)),
        Span::styled("  /  Smart Home OS", Style::default().fg(DIM)),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(DIM)),
    )
    .alignment(Alignment::Left);
    frame.render_widget(brand, cols[0]);

    // Connection badge
    let status_color = if app.connected { OK_COL } else { ALERT_COL };
    let status_label = if app.connected { "● LIVE" } else { "○ CONNECTING" };
    let status = Paragraph::new(Line::from(vec![
        Span::styled(status_label, Style::default().fg(status_color).add_modifier(Modifier::BOLD)),
        Span::styled(
            format!("  msgs {}", app.message_count),
            Style::default().fg(DIM),
        ),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(DIM)),
    )
    .alignment(Alignment::Center);
    frame.render_widget(status, cols[1]);
}

// ── Body ─────────────────────────────────────────────────────────────────────
fn render_body(frame: &mut Frame, area: Rect, app: &App) {
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(45),  // sensor gauges
            Constraint::Percentage(25),// motion + device info
            Constraint::Percentage(30),// camera 
                                                    
        ])
        .split(area);

    render_sensors(frame, cols[0], app);
    render_side(frame, cols[1], app);
    render_camera(frame, cols[2], app);
}

// ── Sensor panel (left) ───────────────────────────────────────────────────────
fn render_sensors(frame: &mut Frame, area: Rect, app: &App) {
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
        ])
        .split(area);

    // Temperature
    render_sensor_row(
        frame, rows[0],
        "TEMPERATURE",
        format!("{:.1} °C", app.temperature),
        TEMP_COL,
        temp_pct(app.temperature),
        format!("range  −10 → 60 °C   /   device  {}", app.device_id),
    );

    // Humidity
    render_sensor_row(
        frame, rows[1],
        "HUMIDITY",
        format!("{:.1} %", app.humidity),
        HUM_COL,
        hum_pct(app.humidity),
        format!("range  0 → 100 %   /   device  {}", app.device_id),
    );

    // Gas
    render_sensor_row(
        frame, rows[2],
        "GAS LEVEL",
        format!("{:.0} ADC", app.gas_level),
        GAS_COL,
        gas_pct(app.gas_level),
        "range  0 → 4095 ADC   /   raw 12-bit read".to_string(),
    );
}

fn render_sensor_row(
    frame: &mut Frame,
    area: Rect,
    title: &str,
    value: String,
    color: Color,
    pct: u16,
    footnote: String,
) {
    let block = accent_block(title);
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),  // value + footnote
            Constraint::Length(1),  // gauge
        ])
        .split(inner);

    // Value row
    let value_line = Paragraph::new(Line::from(vec![
        Span::styled(value, value_style(color)),
        Span::raw("  "),
        Span::styled(footnote, label_style()),
    ]));
    frame.render_widget(value_line, rows[0]);

    // Gauge
    let gauge = Gauge::default()
        .gauge_style(Style::default().fg(color).bg(SURFACE))
        .percent(pct)
        .label(Span::styled(
            format!("{}%", pct),
            Style::default().fg(Color::Black).add_modifier(Modifier::BOLD),
        ));
    frame.render_widget(gauge, rows[1]);
}

// ── Side panel (right) ────────────────────────────────────────────────────────
fn render_side(frame: &mut Frame, area: Rect, app: &App) {
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(55),  // motion
            Constraint::Percentage(45),  // device info
        ])
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
        .constraints([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Min(0),
        ])
        .split(inner);

    frame.render_widget(
        Paragraph::new(Span::styled(
            state_label,
            Style::default()
                .fg(state_color)
                .add_modifier(Modifier::BOLD),
        )),
        rows[0],
    );

    frame.render_widget(
        Paragraph::new(Span::styled(detail, label_style())),
        rows[1],
    );

    // Big indicator block
    let indicator_char = if app.motion { "▐▌" } else { "  " };
    let indicator = Paragraph::new(
        vec![
            Line::from(""),
            Line::from(Span::styled(
                indicator_char,
                Style::default()
                    .fg(state_color)
                    .add_modifier(Modifier::SLOW_BLINK),
            )),
        ]
    )
    .alignment(Alignment::Center);
    frame.render_widget(indicator, rows[2]);
}

fn render_device(frame: &mut Frame, area: Rect, app: &App) {
    let block = accent_block("NODE");
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Min(0),
        ])
        .split(inner);

    let fields: &[(&str, &str)] = &[
        ("id    ", &app.device_id),
        ("broker", "localhost:1883"),
        ("topic ", "home/#"),
    ];

    for (i, (label, value)) in fields.iter().enumerate() {
        if i >= rows.len() { break; }
        frame.render_widget(
            Paragraph::new(Line::from(vec![
                Span::styled(*label, label_style()),
                Span::raw("  "),
                Span::styled(*value, Style::default().fg(TEXT)),
            ])),
            rows[i],
        );
    }
}

// ── Footer ────────────────────────────────────────────────────────────────────
fn render_footer(frame: &mut Frame, area: Rect, _app: &App) {
    let footer = Paragraph::new(Line::from(vec![
        Span::styled(" q ", Style::default().fg(Color::Black).bg(DIM)),
        Span::styled("  quit", label_style()),
        Span::styled("     RoidOME v0.1  —  Distributed Smart Home OS", label_style()),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(DIM)),
    );
    frame.render_widget(footer, area);
}



fn render_camera(frame: &mut Frame, area: Rect, app: &App) {
    let block = accent_block("CAMERA");
    let inner = block.inner(area);
    frame.render_widget(block, area);

    match &app.latest_frame_path {
        None => {
            let waiting = Paragraph::new(Line::from(
                Span::styled("waiting for snapshot...", label_style())
            ));
            frame.render_widget(waiting, inner);
        }
        Some(path) => {
            let w = inner.width;
            let h = inner.height.saturating_sub(2);

        let lines: Vec<Line> = render_with_chafa(path, w, h)
            .into_iter()
            .map(|l| Line::from(Span::raw(l)))
        .collect();
        
        frame.render_widget(Paragraph::new(lines), inner);

        } 
    }
}

fn render_with_chafa(path: &str, width: u16, height: u16) -> Vec<String> {
    let output = std::process::Command::new("chafa")
        .args([
            "--size", &format!("{}x{}", width, height),
            "--format", "symbols",
            "--colors", "256",
            "--stretch",
            path,
        ])
        .output();

    match output {
        Ok(out) => {
            let raw = String::from_utf8_lossy(&out.stdout);
            // strip ANSI escape codes — keep only printable chars
            raw.lines()
                .map(|line| strip_ansi(line))
                .collect()
        }
        Err(_) => vec!["[ chafa not found — brew install chafa ]".to_string()],
    }
}

fn strip_ansi(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\x1b' {
            // skip escape sequence until we hit a letter
            while let Some(&next) = chars.peek() {
                chars.next();
                if next.is_ascii_alphabetic() { break; }
            }
        } else {
            result.push(c);
        }
    }
    result
}
