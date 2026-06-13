use crate::app::App;
use ratatui::{
    Frame,
    layout::{Layout, Constraint, Direction, Alignment},
    widgets::{Block, Borders, Paragraph},
    text::{Span, Line},
    style::{Style, Color, Modifier},
    prelude::Stylize,
};




pub fn ui (frame:&mut Frame , app:&App) {

    let chunks = Layout::default()
    .direction(Direction::Vertical)
    .constraints([
        Constraint::Length(3),  // title bar — 3 rows tall
        Constraint::Min(0),     // middle — takes remaining space
        Constraint::Length(3),  // status bar — 3 rows tall
    ])
    .split(frame.area());

    
    let middle_chunk = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(chunks[1]);

    let title_line = Line::from(vec![
        Span::styled(" RoidOME " , Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
    ]);

    let title  = Paragraph::new(title_line)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL)).cyan(); 
    frame.render_widget(title , chunks[0]) ; 
    

    

    let labels = Paragraph::new(vec![ 
        Line::from(Span::styled("Temperature" , Style::default().fg(Color::White))),
        Line::from(Span::styled("Humidity" , Style::default().fg(Color::White))),
        Line::from(Span::styled("Gas Level" , Style::default().fg(Color::White))),
        Line::from(Span::styled("Motion" , Style::default().fg(Color::White))),
    ])
        .block(Block::default().title(" Sensors ").borders(Borders::ALL)).cyan() ; 



    frame.render_widget(labels,middle_chunk[0]) ; 

    
    
    

    let motion_color = if app.motion {Color::Red} else {Color::Green} ; 

    
    
    let values = Paragraph::new(vec! [ 

        Line::from(Span::styled(format!("{:.1}°C",app.temperature),Style::default().fg(Color::Red))), 
        Line::from(Span::styled(format!("{:.1}%",app.humidity),Style::default().fg(Color::Blue))), 
        Line::from(Span::styled(format!("{:.0} ADC",app.gas_level),Style::default().fg(Color::Yellow))), 
        Line::from(Span::styled(format!("{}",app.motion),Style::default().fg(motion_color))), 


    ])
        .block(Block::default().title(" Readings ").borders(Borders::ALL)).cyan() ; 

    frame.render_widget(values,middle_chunk[1]) ; 


    let status = Paragraph::new(vec![

        Line::from(Span::styled(format!(" Messages: {} - Press q to Quit",app.message_count),Style::default().fg(Color::DarkGray))),    

    ])
    .block(Block::default().title(" Status ").borders(Borders::ALL)).cyan(); 

    frame.render_widget(status,chunks[2]) ; 

}


