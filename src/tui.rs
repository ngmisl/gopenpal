//! Terminal User Interface module.
//!
//! This module provides an interactive TUI using ratatui for managing
//! water intake, reminders, and AI chat in a visual dashboard.

use chrono::Local;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, List, ListItem, Paragraph, Tabs},
    Frame, Terminal,
};
use std::io;
use tokio::sync::mpsc;
use tracing::info;

use crate::chat::ChatSession;
use crate::db::Database;
use crate::error::Result;
use crate::openrouter::OpenRouterClient;

/// TUI application state.
struct App {
    /// Current selected tab
    current_tab: usize,
    /// Tab titles
    tabs: Vec<&'static str>,
    /// Water input buffer
    water_input: String,
    /// Chat input buffer
    chat_input: String,
    /// Chat messages display
    chat_messages: Vec<String>,
    /// Whether we're in input mode
    input_mode: InputMode,
    /// Scroll position for chat
    chat_scroll: u16,
}

#[derive(PartialEq)]
enum InputMode {
    Normal,
    EditingWater,
    EditingChat,
}

impl Default for App {
    fn default() -> Self {
        Self {
            current_tab: 0,
            tabs: vec!["Dashboard", "Water Tracking", "Chat", "Settings"],
            water_input: String::new(),
            chat_input: String::new(),
            chat_messages: Vec::new(),
            input_mode: InputMode::Normal,
            chat_scroll: 0,
        }
    }
}

/// Run the TUI application.
///
/// # Arguments
///
/// * `db` - Database connection
/// * `api_key` - Optional OpenRouter API key for chat functionality
///
/// # Errors
///
/// Returns error if terminal operations or database queries fail
pub async fn run_tui(db: Database, api_key: Option<String>) -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::default();
    let (tx, mut rx) = mpsc::channel::<String>(100);

    // Run the app
    let result = run_app(&mut terminal, &mut app, &db, api_key, tx, &mut rx).await;

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    result
}

/// Main application loop.
async fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
    db: &Database,
    api_key: Option<String>,
    tx: mpsc::Sender<String>,
    rx: &mut mpsc::Receiver<String>,
) -> Result<()> {
    loop {
        terminal.draw(|f| ui(f, app, db))?;

        // Handle async chat responses
        if let Ok(msg) = rx.try_recv() {
            app.chat_messages.push(format!("Assistant: {}", msg));
        }

        // Poll for events with timeout
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                // Only process key press events, not release
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                match app.input_mode {
                    InputMode::Normal => match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Left => {
                            app.current_tab = app.current_tab.saturating_sub(1);
                        }
                        KeyCode::Right => {
                            app.current_tab = (app.current_tab + 1).min(app.tabs.len() - 1);
                        }
                        KeyCode::Char('w') if app.current_tab == 1 => {
                            app.input_mode = InputMode::EditingWater;
                        }
                        KeyCode::Char('c') if app.current_tab == 2 => {
                            app.input_mode = InputMode::EditingChat;
                        }
                        KeyCode::Up if app.current_tab == 2 => {
                            app.chat_scroll = app.chat_scroll.saturating_sub(1);
                        }
                        KeyCode::Down if app.current_tab == 2 => {
                            app.chat_scroll = app.chat_scroll.saturating_add(1);
                        }
                        _ => {}
                    },
                    InputMode::EditingWater => match key.code {
                        KeyCode::Enter => {
                            if let Ok(amount) = app.water_input.parse::<i64>() {
                                if let Err(e) = db.record_water_intake(amount, None).await {
                                    info!("Failed to log water: {}", e);
                                }
                                app.water_input.clear();
                            }
                            app.input_mode = InputMode::Normal;
                        }
                        KeyCode::Char(c) => {
                            app.water_input.push(c);
                        }
                        KeyCode::Backspace => {
                            app.water_input.pop();
                        }
                        KeyCode::Esc => {
                            app.input_mode = InputMode::Normal;
                            app.water_input.clear();
                        }
                        _ => {}
                    },
                    InputMode::EditingChat => match key.code {
                        KeyCode::Enter => {
                            if !app.chat_input.is_empty() && api_key.is_some() {
                                let input = app.chat_input.clone();
                                app.chat_messages.push(format!("You: {}", input));
                                app.chat_input.clear();

                                // Send chat message asynchronously
                                let api_key_clone = api_key.clone().unwrap();
                                let db_clone = db.clone();
                                let tx_clone = tx.clone();

                                tokio::spawn(async move {
                                    let client = OpenRouterClient::new(api_key_clone);
                                    let session = ChatSession::new(
                                        db_clone,
                                        client,
                                        "anthropic/claude-3.5-sonnet".to_string(),
                                    );

                                    if let Ok(response) = session.send_message(&input).await {
                                        let _ = tx_clone.send(response).await;
                                    }
                                });
                            }
                            app.input_mode = InputMode::Normal;
                        }
                        KeyCode::Char(c) => {
                            app.chat_input.push(c);
                        }
                        KeyCode::Backspace => {
                            app.chat_input.pop();
                        }
                        KeyCode::Esc => {
                            app.input_mode = InputMode::Normal;
                            app.chat_input.clear();
                        }
                        _ => {}
                    },
                }
            }
        }
    }
}

/// Render the UI.
///
/// This function is called on each frame to render the current state
/// of the application. It uses blocking calls to fetch data, which is
/// acceptable here as rendering should be fast.
fn ui(f: &mut Frame, app: &App, db: &Database) {
    let size = f.area();

    // Create the layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(size);

    // Render tabs
    let titles: Vec<Line> = app.tabs.iter().map(|t| Line::from(*t)).collect();

    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title("GopenPal"))
        .select(app.current_tab)
        .style(Style::default().fg(Color::Cyan))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .bg(Color::DarkGray),
        );

    f.render_widget(tabs, chunks[0]);

    // Render content based on selected tab
    match app.current_tab {
        0 => render_dashboard(f, chunks[1], db),
        1 => render_water_tracking(f, chunks[1], app, db),
        2 => render_chat(f, chunks[1], app),
        3 => render_settings(f, chunks[1], db),
        _ => {}
    }
}

/// Render the dashboard tab.
fn render_dashboard(f: &mut Frame, area: Rect, db: &Database) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(8),
            Constraint::Min(0),
        ])
        .split(area);

    // Welcome message
    let welcome = Paragraph::new(format!(
        "Welcome to GopenPal - {}",
        Local::now().format("%A, %B %d, %Y")
    ))
    .style(Style::default().fg(Color::Green))
    .alignment(Alignment::Center)
    .block(Block::default().borders(Borders::ALL));
    f.render_widget(welcome, chunks[0]);

    // Water intake stats
    let handle = tokio::runtime::Handle::current();
    let total_ml = handle.block_on(async { db.get_today_total_ml().await.unwrap_or(0) });

    let daily_goal = 2000; // 2 liters
    let progress = (total_ml as f64 / daily_goal as f64).min(1.0);

    let gauge = Gauge::default()
        .block(
            Block::default()
                .title("Today's Water Intake")
                .borders(Borders::ALL),
        )
        .gauge_style(Style::default().fg(Color::Cyan).bg(Color::Black))
        .percent((progress * 100.0) as u16)
        .label(format!(
            "{} / {} ml ({:.0}%)",
            total_ml,
            daily_goal,
            progress * 100.0
        ));
    f.render_widget(gauge, chunks[1]);

    // Quick stats
    let stats_text = vec![
        Line::from(vec![
            Span::styled("Press ", Style::default()),
            Span::styled("←/→", Style::default().fg(Color::Yellow)),
            Span::styled(" to navigate tabs", Style::default()),
        ]),
        Line::from(vec![
            Span::styled("Press ", Style::default()),
            Span::styled("q", Style::default().fg(Color::Red)),
            Span::styled(" to quit", Style::default()),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "Navigate to other tabs for more features!",
            Style::default().fg(Color::Gray),
        )),
    ];

    let help = Paragraph::new(stats_text)
        .block(Block::default().title("Quick Help").borders(Borders::ALL))
        .alignment(Alignment::Left);
    f.render_widget(help, chunks[2]);
}

/// Render the water tracking tab.
fn render_water_tracking(f: &mut Frame, area: Rect, app: &App, db: &Database) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(area);

    // Input area
    let input_style = if app.input_mode == InputMode::EditingWater {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    };

    let input = Paragraph::new(app.water_input.as_str())
        .style(input_style)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Log Water (ml) - Press 'w' to edit, Enter to submit, Esc to cancel"),
        );
    f.render_widget(input, chunks[0]);

    // Recent entries
    let handle = tokio::runtime::Handle::current();
    let entries = handle.block_on(async { db.get_today_water_intake().await.unwrap_or_default() });

    let items: Vec<ListItem> = entries
        .iter()
        .map(|entry| {
            let time = entry.timestamp.format("%H:%M").to_string();
            let content = format!(
                "{} - {} ml {}",
                time,
                entry.amount_ml,
                entry.notes.as_deref().unwrap_or("")
            );
            ListItem::new(content)
        })
        .collect();

    let total = handle.block_on(async { db.get_today_total_ml().await.unwrap_or(0) });

    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .title(format!("Today's Entries (Total: {} ml)", total)),
    );
    f.render_widget(list, chunks[1]);
}

/// Render the chat tab.
fn render_chat(f: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(area);

    // Chat messages
    let messages: Vec<ListItem> = app
        .chat_messages
        .iter()
        .map(|m| ListItem::new(m.clone()))
        .collect();

    let messages_list = List::new(messages).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Chat History (↑/↓ to scroll)"),
    );
    f.render_widget(messages_list, chunks[0]);

    // Input area
    let input_style = if app.input_mode == InputMode::EditingChat {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    };

    let input = Paragraph::new(app.chat_input.as_str())
        .style(input_style)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Message - Press 'c' to chat, Enter to send, Esc to cancel"),
        );
    f.render_widget(input, chunks[1]);
}

/// Render the settings tab.
fn render_settings(f: &mut Frame, area: Rect, db: &Database) {
    let handle = tokio::runtime::Handle::current();
    let settings = handle.block_on(async { db.get_reminder_settings().await.ok() });

    let text = if let Some(s) = settings {
        vec![
            Line::from(Span::styled(
                "Reminder Settings",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from(vec![
                Span::raw("Status: "),
                Span::styled(
                    if s.enabled { "Enabled" } else { "Disabled" },
                    Style::default().fg(if s.enabled { Color::Green } else { Color::Red }),
                ),
            ]),
            Line::from(format!("Interval: {} minutes", s.interval_minutes)),
            Line::from(format!(
                "Work Hours: {} - {}",
                s.work_hours_start, s.work_hours_end
            )),
            Line::from(format!("Work Days: {}", s.work_days)),
            Line::from(""),
            Line::from(Span::styled(
                "Use CLI commands to modify settings:",
                Style::default().fg(Color::Gray),
            )),
            Line::from(Span::raw("  gopenpal reminder config --interval 45")),
            Line::from(Span::raw("  gopenpal reminder config --enabled false")),
        ]
    } else {
        vec![Line::from("Failed to load settings")]
    };

    let paragraph = Paragraph::new(text)
        .block(Block::default().title("Settings").borders(Borders::ALL))
        .alignment(Alignment::Left);
    f.render_widget(paragraph, area);
}
