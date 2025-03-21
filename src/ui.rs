use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    prelude::*,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Tabs, Wrap},
    Terminal,
};
use std::{io, time::Duration};
use tui_input::Input;

use crate::client::SpaceTradersClient;

pub enum AppState {
    Dashboard,
    Ships,
    Systems,
    Markets,
}

pub struct App {
    pub state: AppState,
    pub client: SpaceTradersClient,
    pub should_quit: bool,
    pub input: Input,
    pub agent: Option<spacetraders_api::models::Agent>,
    pub ships: Option<Vec<spacetraders_api::models::Ship>>,
    pub systems: Option<Vec<spacetraders_api::models::System>>,
    pub status_message: String,
    pub ships_state: ListState,
    pub systems_state: ListState,
    pub help_visible: bool,
}

impl App {
    pub fn new(client: SpaceTradersClient) -> Self {
        let mut ships_state = ListState::default();
        ships_state.select(Some(0));
        let mut systems_state = ListState::default();
        systems_state.select(Some(0));

        Self {
            state: AppState::Dashboard,
            client,
            should_quit: false,
            input: Input::default(),
            agent: None,
            ships: None,
            systems: None,
            status_message: String::from("Welcome to Space Traders"),
            ships_state,
            systems_state,
            help_visible: false,
        }
    }

    pub async fn refresh_data(&mut self) -> Result<()> {
        match self.state {
            AppState::Dashboard => {
                self.agent = Some(self.client.get_my_agent().await?);
                let status = self.client.get_status().await?;
                self.status_message = format!(
                    "Status: {}\nVersion: {}\nReset Date: {}",
                    status.status, status.version, status.reset_date
                );
            }
            AppState::Ships => {
                self.ships = Some(self.client.get_my_ships().await?);
            }
            AppState::Systems => {
                self.systems = Some(self.client.get_systems(Some(1), Some(20)).await?);
            }
            _ => {}
        }
        Ok(())
    }

    pub fn next_tab(&mut self) {
        self.state = match self.state {
            AppState::Dashboard => AppState::Ships,
            AppState::Ships => AppState::Systems,
            AppState::Systems => AppState::Markets,
            AppState::Markets => AppState::Dashboard,
        };
    }

    pub fn previous_tab(&mut self) {
        self.state = match self.state {
            AppState::Dashboard => AppState::Markets,
            AppState::Ships => AppState::Dashboard,
            AppState::Systems => AppState::Ships,
            AppState::Markets => AppState::Systems,
        };
    }

    pub fn next_item(&mut self) {
        match self.state {
            AppState::Ships => {
                if let Some(ships) = &self.ships {
                    if ships.is_empty() {
                        return;
                    }

                    let i = match self.ships_state.selected() {
                        Some(i) => {
                            if i >= ships.len() - 1 {
                                0
                            } else {
                                i + 1
                            }
                        }
                        None => 0,
                    };
                    self.ships_state.select(Some(i));
                }
            }
            AppState::Systems => {
                if let Some(systems) = &self.systems {
                    if systems.is_empty() {
                        return;
                    }

                    let i = match self.systems_state.selected() {
                        Some(i) => {
                            if i >= systems.len() - 1 {
                                0
                            } else {
                                i + 1
                            }
                        }
                        None => 0,
                    };
                    self.systems_state.select(Some(i));
                }
            }
            _ => {}
        }
    }

    pub fn previous_item(&mut self) {
        match self.state {
            AppState::Ships => {
                if let Some(ships) = &self.ships {
                    if ships.is_empty() {
                        return;
                    }

                    let i = match self.ships_state.selected() {
                        Some(i) => {
                            if i == 0 {
                                ships.len() - 1
                            } else {
                                i - 1
                            }
                        }
                        None => 0,
                    };
                    self.ships_state.select(Some(i));
                }
            }
            AppState::Systems => {
                if let Some(systems) = &self.systems {
                    if systems.is_empty() {
                        return;
                    }

                    let i = match self.systems_state.selected() {
                        Some(i) => {
                            if i == 0 {
                                systems.len() - 1
                            } else {
                                i - 1
                            }
                        }
                        None => 0,
                    };
                    self.systems_state.select(Some(i));
                }
            }
            _ => {}
        }
    }

    pub fn toggle_help(&mut self) {
        self.help_visible = !self.help_visible;
    }
}

pub async fn run_app(app: &mut App) -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Refresh initial data
    app.refresh_data().await?;

    // Main loop
    while !app.should_quit {
        terminal.draw(|f| ui(f, app))?;

        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => app.should_quit = true,
                        KeyCode::Char('r') => {
                            let _ = app.refresh_data().await;
                        }
                        KeyCode::Char('h') => app.toggle_help(),
                        KeyCode::Tab => {
                            app.next_tab();
                            let _ = app.refresh_data().await;
                        }
                        KeyCode::BackTab => {
                            app.previous_tab();
                            let _ = app.refresh_data().await;
                        }
                        KeyCode::Down | KeyCode::Char('j') => app.next_item(),
                        KeyCode::Up | KeyCode::Char('k') => app.previous_item(),
                        KeyCode::Char('1') => {
                            app.state = AppState::Dashboard;
                            let _ = app.refresh_data().await;
                        }
                        KeyCode::Char('2') => {
                            app.state = AppState::Ships;
                            let _ = app.refresh_data().await;
                        }
                        KeyCode::Char('3') => {
                            app.state = AppState::Systems;
                            let _ = app.refresh_data().await;
                        }
                        KeyCode::Char('4') => {
                            app.state = AppState::Markets;
                            let _ = app.refresh_data().await;
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn ui(f: &mut Frame, app: &mut App) {
    // Create layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),                                    // Tabs
            Constraint::Min(0),                                       // Content
            Constraint::Length(if app.help_visible { 9 } else { 3 }), // Status bar
        ])
        .split(f.size());

    // Tabs
    let titles = vec!["Dashboard", "Ships", "Systems", "Markets"];
    let titles = titles
        .iter()
        .map(|t| {
            let (first, rest) = t.split_at(1);
            Line::from(vec![
                Span::styled(first, Style::default().fg(Color::Yellow)),
                Span::styled(rest, Style::default()),
            ])
        })
        .collect();

    let tabs = Tabs::new(titles)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Space Traders"),
        )
        .select(match app.state {
            AppState::Dashboard => 0,
            AppState::Ships => 1,
            AppState::Systems => 2,
            AppState::Markets => 3,
        })
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Yellow));

    f.render_widget(tabs, chunks[0]);

    // Content
    match app.state {
        AppState::Dashboard => render_dashboard(f, app, chunks[1]),
        AppState::Ships => render_ships(f, app, chunks[1]),
        AppState::Systems => render_systems(f, app, chunks[1]),
        AppState::Markets => render_markets(f, app, chunks[1]),
    }

    // Status bar / Help screen
    if app.help_visible {
        let help_text = vec![
            Line::from(Span::styled(
                "Keyboard Navigation",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from(vec![
                Span::styled("q", Style::default().fg(Color::Cyan)),
                Span::raw(": Quit  "),
                Span::styled("r", Style::default().fg(Color::Cyan)),
                Span::raw(": Refresh  "),
                Span::styled("h", Style::default().fg(Color::Cyan)),
                Span::raw(": Toggle Help  "),
            ]),
            Line::from(vec![
                Span::styled("Tab", Style::default().fg(Color::Cyan)),
                Span::raw(": Next tab  "),
                Span::styled("Shift+Tab", Style::default().fg(Color::Cyan)),
                Span::raw(": Previous tab  "),
            ]),
            Line::from(vec![
                Span::styled("j/Down", Style::default().fg(Color::Cyan)),
                Span::raw(": Next item  "),
                Span::styled("k/Up", Style::default().fg(Color::Cyan)),
                Span::raw(": Previous item  "),
            ]),
            Line::from(vec![
                Span::styled("1-4", Style::default().fg(Color::Cyan)),
                Span::raw(": Switch tabs directly (1=Dashboard, 2=Ships, etc.)"),
            ]),
        ];

        let help = Paragraph::new(help_text)
            .block(Block::default().borders(Borders::ALL).title("Help"))
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true });

        f.render_widget(help, chunks[2]);
    } else {
        let status_text = "Press h for help | q to quit | r to refresh";
        let status = Paragraph::new(status_text)
            .block(Block::default().borders(Borders::ALL))
            .wrap(Wrap { trim: true });

        f.render_widget(status, chunks[2]);
    }
}

fn render_dashboard(f: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    // Agent info
    let agent_block = Block::default().borders(Borders::ALL).title("Agent Info");

    if let Some(agent) = &app.agent {
        // Create styled lines for the agent info
        let lines = vec![
            Line::from(vec![
                Span::raw("Agent: "),
                Span::styled(
                    &agent.symbol,
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(vec![
                Span::raw("Credits: "),
                Span::styled(
                    agent.credits.to_string(),
                    Style::default().fg(Color::Yellow),
                ),
            ]),
            Line::from(vec![
                Span::raw("HQ: "),
                Span::styled(&agent.headquarters, Style::default().fg(Color::Cyan)),
            ]),
            Line::from(vec![
                Span::raw("Faction: "),
                Span::styled(&agent.starting_faction, Style::default().fg(Color::Magenta)),
            ]),
            Line::from(vec![
                Span::raw("Ships: "),
                Span::styled(
                    agent.ship_count.to_string(),
                    Style::default().fg(Color::Blue),
                ),
            ]),
        ];

        let agent_info = Paragraph::new(lines)
            .block(agent_block)
            .wrap(Wrap { trim: true });

        f.render_widget(agent_info, chunks[0]);
    } else {
        let loading = Paragraph::new("Loading agent data...")
            .block(agent_block)
            .wrap(Wrap { trim: true });
        f.render_widget(loading, chunks[0]);
    }

    // Status info
    let status_block = Block::default().borders(Borders::ALL).title("Game Status");

    // Parse the status message to apply colors
    let status_lines = app
        .status_message
        .split('\n')
        .map(|line| {
            if line.starts_with("Status:") {
                Line::from(vec![
                    Span::raw("Status: "),
                    Span::styled(
                        line.trim_start_matches("Status: "),
                        Style::default().fg(Color::Green),
                    ),
                ])
            } else if line.starts_with("Version:") {
                Line::from(vec![
                    Span::raw("Version: "),
                    Span::styled(
                        line.trim_start_matches("Version: "),
                        Style::default().fg(Color::Blue),
                    ),
                ])
            } else if line.starts_with("Reset Date:") {
                Line::from(vec![
                    Span::raw("Reset Date: "),
                    Span::styled(
                        line.trim_start_matches("Reset Date: "),
                        Style::default().fg(Color::Yellow),
                    ),
                ])
            } else {
                Line::from(line)
            }
        })
        .collect::<Vec<Line>>();

    let status = Paragraph::new(status_lines)
        .block(status_block)
        .wrap(Wrap { trim: true });
    f.render_widget(status, chunks[1]);
}

fn get_ship_role_color(ship_role: &str) -> Color {
    match ship_role {
        "COMMAND" => Color::Yellow,
        "EXCAVATOR" => Color::LightRed,
        "HAULER" => Color::Green,
        "INTERCEPTOR" => Color::Red,
        "EXPLORER" => Color::Blue,
        "TRANSPORT" => Color::Cyan,
        "CARRIER" => Color::Magenta,
        "PATROL" => Color::LightBlue,
        "SATELLITE" => Color::LightCyan,
        "SURVEYOR" => Color::LightGreen,
        _ => Color::Gray,
    }
}

fn render_ships(f: &mut Frame, app: &mut App, area: Rect) {
    let ships_block = Block::default().borders(Borders::ALL).title("Ships");

    if let Some(ships) = &app.ships {
        if ships.is_empty() {
            let no_ships = Paragraph::new("No ships found")
                .block(ships_block)
                .wrap(Wrap { trim: true });
            f.render_widget(no_ships, area);
        } else {
            let items: Vec<ListItem> = ships
                .iter()
                .map(|ship| {
                    let ship_role = ship.registration.role.to_string();
                    let role_color = get_ship_role_color(&ship_role);

                    // Create a simple location string - nav might be a complex type
                    let location = "Current Location";

                    // Create styled lines for the ship info
                    let lines = vec![
                        Line::from(vec![
                            Span::raw("Ship: "),
                            Span::styled(&ship.symbol, Style::default().fg(Color::Blue)),
                            Span::raw(" - "),
                            Span::styled(ship_role, Style::default().fg(role_color)),
                        ]),
                        Line::from(vec![
                            Span::raw("Location: "),
                            Span::styled(location, Style::default().fg(Color::Yellow)),
                        ]),
                        Line::from(""),
                    ];

                    ListItem::new(lines)
                })
                .collect();

            let list = List::new(items)
                .block(ships_block)
                .highlight_style(Style::default().bg(Color::DarkGray))
                .highlight_symbol(">> ");

            f.render_stateful_widget(list, area, &mut app.ships_state);
        }
    } else {
        let loading = Paragraph::new("Loading ships data...")
            .block(ships_block)
            .wrap(Wrap { trim: true });
        f.render_widget(loading, area);
    }
}

fn get_system_type_color(system_type: &str) -> Color {
    match system_type {
        "NEUTRON_STAR" => Color::Cyan,
        "RED_STAR" => Color::Red,
        "ORANGE_STAR" => Color::LightRed,
        "BLUE_STAR" => Color::Blue,
        "YOUNG_STAR" => Color::Yellow,
        "WHITE_DWARF" => Color::White,
        "BLACK_HOLE" => Color::DarkGray,
        "HYPERGIANT" => Color::Magenta,
        "NEBULA" => Color::LightMagenta,
        "UNSTABLE" => Color::LightYellow,
        _ => Color::Gray,
    }
}

fn render_systems(f: &mut Frame, app: &mut App, area: Rect) {
    let systems_block = Block::default().borders(Borders::ALL).title("Systems");

    if let Some(systems) = &app.systems {
        if systems.is_empty() {
            let no_systems = Paragraph::new("No systems found")
                .block(systems_block)
                .wrap(Wrap { trim: true });
            f.render_widget(no_systems, area);
        } else {
            let items: Vec<ListItem> = systems
                .iter()
                .map(|system| {
                    let system_type = system.r#type.to_string();
                    let system_color = get_system_type_color(&system_type);

                    // Create styled lines for the system info
                    let lines = vec![
                        Line::from(vec![
                            Span::raw("System: "),
                            Span::styled(&system.symbol, Style::default().fg(Color::Green)),
                        ]),
                        Line::from(vec![
                            Span::raw("Type: "),
                            Span::styled(system_type, Style::default().fg(system_color)),
                            Span::raw(format!(" - Waypoints: {}", system.waypoints.len())),
                        ]),
                        Line::from(""),
                    ];

                    ListItem::new(lines)
                })
                .collect();

            let list = List::new(items)
                .block(systems_block)
                .highlight_style(Style::default().bg(Color::DarkGray))
                .highlight_symbol(">> ");

            f.render_stateful_widget(list, area, &mut app.systems_state);
        }
    } else {
        let loading = Paragraph::new("Loading systems data...")
            .block(systems_block)
            .wrap(Wrap { trim: true });
        f.render_widget(loading, area);
    }
}

fn render_markets(f: &mut Frame, _app: &mut App, area: Rect) {
    let markets_block = Block::default().borders(Borders::ALL).title("Markets");

    // Create a more visually appealing "coming soon" message
    let lines = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("✨ ", Style::default().fg(Color::Yellow)),
            Span::styled(
                "COMING SOON",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" ✨", Style::default().fg(Color::Yellow)),
        ]),
        Line::from(""),
        Line::from(vec![Span::raw(
            "The Markets feature is under development and will be available in a future update.",
        )]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Stay tuned for trading capabilities!",
            Style::default().fg(Color::Green),
        )]),
    ];

    let coming_soon = Paragraph::new(lines)
        .block(markets_block)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    f.render_widget(coming_soon, area);
}
