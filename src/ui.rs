use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    prelude::*,
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{
        Axis, Block, Borders, Chart, Dataset, GraphType, List, ListItem, ListState, Paragraph,
        Tabs, Wrap,
    },
    Frame, Terminal,
};
use std::{io, time::Duration};
use tui_input::Input;
use unicode_width::UnicodeWidthStr;

use crate::client::SpaceTradersClient;

#[derive(Clone, Copy)]
pub enum AppState {
    Dashboard,
    Ships,
    ShipDetail,
    Systems,
    SystemDetail,
    Markets,
    WaypointDetail,
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
    pub selected_ship_index: Option<usize>,
    pub selected_system_index: Option<usize>,
    pub waypoints_state: ListState,
    pub selected_waypoint_index: Option<usize>,
}

impl App {
    pub fn new(client: SpaceTradersClient) -> Self {
        let mut ships_state = ListState::default();
        ships_state.select(Some(0));
        let mut systems_state = ListState::default();
        systems_state.select(Some(0));
        let mut waypoints_state = ListState::default();
        waypoints_state.select(Some(0));

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
            selected_ship_index: None,
            selected_system_index: None,
            waypoints_state,
            selected_waypoint_index: None,
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
            AppState::ShipDetail => AppState::Ships,
            AppState::SystemDetail => AppState::Systems,
            AppState::WaypointDetail => AppState::SystemDetail,
        };
    }

    pub fn previous_tab(&mut self) {
        self.state = match self.state {
            AppState::Dashboard => AppState::Markets,
            AppState::Ships => AppState::Dashboard,
            AppState::Systems => AppState::Ships,
            AppState::Markets => AppState::Systems,
            AppState::ShipDetail => AppState::Ships,
            AppState::SystemDetail => AppState::Systems,
            AppState::WaypointDetail => AppState::SystemDetail,
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
            AppState::SystemDetail => {
                if let Some(systems) = &self.systems {
                    if let Some(system_index) = self.selected_system_index {
                        if system_index < systems.len() {
                            let system = &systems[system_index];
                            if system.waypoints.is_empty() {
                                return;
                            }

                            let i = match self.waypoints_state.selected() {
                                Some(i) => {
                                    if i >= system.waypoints.len() - 1 {
                                        0
                                    } else {
                                        i + 1
                                    }
                                }
                                None => 0,
                            };
                            self.waypoints_state.select(Some(i));
                        }
                    }
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
            AppState::SystemDetail => {
                if let Some(systems) = &self.systems {
                    if let Some(system_index) = self.selected_system_index {
                        if system_index < systems.len() {
                            let system = &systems[system_index];
                            if system.waypoints.is_empty() {
                                return;
                            }

                            let i = match self.waypoints_state.selected() {
                                Some(i) => {
                                    if i == 0 {
                                        system.waypoints.len() - 1
                                    } else {
                                        i - 1
                                    }
                                }
                                None => 0,
                            };
                            self.waypoints_state.select(Some(i));
                        }
                    }
                }
            }
            _ => {}
        }
    }

    pub fn toggle_help(&mut self) {
        self.help_visible = !self.help_visible;
    }

    pub fn view_selected_ship_detail(&mut self) {
        if let Some(ships) = &self.ships {
            if !ships.is_empty() {
                if let Some(i) = self.ships_state.selected() {
                    self.selected_ship_index = Some(i);
                    self.state = AppState::ShipDetail;
                }
            }
        }
    }

    pub fn view_selected_system_detail(&mut self) {
        if let Some(systems) = &self.systems {
            if !systems.is_empty() {
                if let Some(i) = self.systems_state.selected() {
                    self.selected_system_index = Some(i);
                    self.state = AppState::SystemDetail;
                }
            }
        }
    }

    pub fn view_selected_waypoint_detail(&mut self) {
        if let Some(systems) = &self.systems {
            if let Some(system_index) = self.selected_system_index {
                if system_index < systems.len() {
                    let system = &systems[system_index];
                    if !system.waypoints.is_empty() {
                        if let Some(i) = self.waypoints_state.selected() {
                            if i < system.waypoints.len() {
                                self.selected_waypoint_index = Some(i);
                                self.state = AppState::WaypointDetail;
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn back_from_detail(&mut self) {
        self.state = match self.state {
            AppState::ShipDetail => AppState::Ships,
            AppState::SystemDetail => AppState::Systems,
            AppState::WaypointDetail => AppState::SystemDetail,
            _ => self.state,
        };
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
                        KeyCode::Enter => match app.state {
                            AppState::Ships => app.view_selected_ship_detail(),
                            AppState::Systems => app.view_selected_system_detail(),
                            AppState::SystemDetail => app.view_selected_waypoint_detail(),
                            AppState::ShipDetail => app.back_from_detail(),
                            AppState::WaypointDetail => app.back_from_detail(),
                            AppState::Dashboard => {}
                            AppState::Markets => {}
                        },
                        KeyCode::Esc => match app.state {
                            AppState::ShipDetail => app.back_from_detail(),
                            AppState::SystemDetail => app.back_from_detail(),
                            AppState::WaypointDetail => app.back_from_detail(),
                            _ => {}
                        },
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
            AppState::Ships | AppState::ShipDetail => 1,
            AppState::Systems | AppState::SystemDetail | AppState::WaypointDetail => 2,
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
        AppState::ShipDetail => render_ship_detail(f, app, chunks[1]),
        AppState::SystemDetail => render_system_detail(f, app, chunks[1]),
        AppState::WaypointDetail => render_waypoint_detail(f, app, chunks[1]),
    }

    // Status bar / Help screen with updated instructions
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
                Span::styled("Enter", Style::default().fg(Color::Cyan)),
                Span::raw(": View details  "),
                Span::styled("Esc", Style::default().fg(Color::Cyan)),
                Span::raw(": Back from details  "),
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
        let status_text = "Press h for help | q to quit | r to refresh | Enter to view details";
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
    // Split the area into two chunks: top half for systems list, bottom half for systems plot
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(area);

    let systems_list_area = chunks[0];
    let systems_plot_area = chunks[1];

    // Render systems list
    let systems_block = Block::default().borders(Borders::ALL).title("Systems");

    match app.systems.as_ref() {
        None => {
            let loading_text = Text::from("Loading systems...");
            let loading_paragraph = Paragraph::new(loading_text)
                .block(systems_block)
                .wrap(Wrap { trim: true });
            f.render_widget(loading_paragraph, systems_list_area);
        }
        Some(systems) if systems.is_empty() => {
            let empty_text = Text::from("No systems found.");
            let empty_paragraph = Paragraph::new(empty_text)
                .block(systems_block)
                .wrap(Wrap { trim: true });
            f.render_widget(empty_paragraph, systems_list_area);
        }
        Some(systems) => {
            let systems_items: Vec<ListItem> = systems
                .iter()
                .map(|s| {
                    let system_type = s.r#type.to_string();
                    let color = get_system_type_color(&system_type);

                    ListItem::new(Line::from(vec![
                        Span::styled(
                            format!("[{:^12}] ", system_type),
                            Style::default().fg(color),
                        ),
                        Span::raw(format!("{} ", s.symbol)),
                        Span::styled(
                            format!("(x:{}, y:{})", s.x, s.y),
                            Style::default().fg(Color::DarkGray),
                        ),
                    ]))
                })
                .collect();

            let systems_list = List::new(systems_items)
                .block(systems_block)
                .highlight_style(
                    Style::default()
                        .bg(Color::DarkGray)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol("> ");

            f.render_stateful_widget(systems_list, systems_list_area, &mut app.systems_state);

            // Render systems plot
            render_system_plot(f, app, systems_plot_area, systems);
        }
    }
}

fn render_system_plot(
    f: &mut Frame,
    app: &App,
    area: Rect,
    systems: &[spacetraders_api::models::System],
) {
    if systems.is_empty() {
        return;
    }

    // Find the min and max coordinates to set the boundaries
    let (min_x, max_x, min_y, max_y) = systems.iter().fold(
        (i32::MAX, i32::MIN, i32::MAX, i32::MIN),
        |(min_x, max_x, min_y, max_y), system| {
            (
                min_x.min(system.x),
                max_x.max(system.x),
                min_y.min(system.y),
                max_y.max(system.y),
            )
        },
    );

    // Create the plot block
    let block = Block::default().borders(Borders::ALL).title("System Map");

    let inner_area = block.inner(area);
    f.render_widget(block, area);

    let plot_width = inner_area.width as usize;
    let plot_height = inner_area.height as usize;

    // Add buffer space for plotting points
    let x_range = (max_x - min_x + 1) as f64;
    let y_range = (max_y - min_y + 1) as f64;

    // Create a canvas to plot points on
    let mut canvas = vec![vec![(' ', Color::Reset); plot_width]; plot_height];

    // Plot systems
    for system in systems {
        // Map system coordinates to plot coordinates
        let plot_x = ((system.x - min_x) as f64 / x_range * (plot_width as f64 - 1.0)) as usize;
        let plot_y = ((max_y - system.y) as f64 / y_range * (plot_height as f64 - 1.0)) as usize;

        if plot_x < plot_width && plot_y < plot_height {
            let system_type = system.r#type.to_string();
            let color = get_system_type_color(&system_type);
            canvas[plot_y][plot_x] = ('●', color);
        }
    }

    // Highlight selected system
    if let Some(selected_idx) = app.systems_state.selected() {
        if selected_idx < systems.len() {
            let system = &systems[selected_idx];
            let plot_x = ((system.x - min_x) as f64 / x_range * (plot_width as f64 - 1.0)) as usize;
            let plot_y =
                ((max_y - system.y) as f64 / y_range * (plot_height as f64 - 1.0)) as usize;

            if plot_x < plot_width && plot_y < plot_height {
                canvas[plot_y][plot_x] = ('★', Color::White);
            }
        }
    }

    // Convert canvas to text
    let mut lines = Vec::new();
    for row in canvas {
        let mut spans = Vec::new();
        for (ch, color) in row {
            spans.push(Span::styled(ch.to_string(), Style::default().fg(color)));
        }
        lines.push(Line::from(spans));
    }

    let text = Text::from(lines);
    let paragraph = Paragraph::new(text);
    f.render_widget(paragraph, inner_area);
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

fn render_ship_detail(f: &mut Frame, app: &mut App, area: Rect) {
    let ship_index = match app.selected_ship_index {
        Some(index) => index,
        None => {
            // If no ship is selected, display an error message
            let message = Paragraph::new("No ship selected")
                .block(Block::default().borders(Borders::ALL).title("Error"))
                .wrap(Wrap { trim: true });
            f.render_widget(message, area);
            return;
        }
    };

    if let Some(ships) = &app.ships {
        if ship_index >= ships.len() {
            // If the selected index is out of bounds, display an error message
            let message = Paragraph::new("Invalid ship selection")
                .block(Block::default().borders(Borders::ALL).title("Error"))
                .wrap(Wrap { trim: true });
            f.render_widget(message, area);
            return;
        }

        let ship = &ships[ship_index];
        let ship_role = ship.registration.role.to_string();
        let role_color = get_ship_role_color(&ship_role);

        // Split the screen into sections
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Header
                Constraint::Length(8), // Basic info
                Constraint::Min(0),    // Additional info
            ])
            .split(area);

        // Render header
        let header = Paragraph::new(vec![Line::from(vec![Span::styled(
            &ship.symbol,
            Style::default()
                .fg(Color::Blue)
                .add_modifier(Modifier::BOLD),
        )])])
        .block(Block::default().borders(Borders::ALL).title("Ship Detail"))
        .alignment(Alignment::Center);
        f.render_widget(header, chunks[0]);

        // Render basic info
        let basic_info = vec![
            Line::from(vec![
                Span::styled("Role: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::styled(&ship_role, Style::default().fg(role_color)),
            ]),
            Line::from(vec![
                Span::styled(
                    "Registration: ",
                    Style::default().add_modifier(Modifier::BOLD),
                ),
                Span::raw(format!(
                    "{} ({})",
                    ship.registration.name, ship.registration.faction_symbol
                )),
            ]),
            Line::from(vec![
                Span::styled("Frame: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(format!("{:?}", ship.frame.symbol)),
            ]),
            Line::from(vec![
                Span::styled("Engine: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(format!("{:?}", ship.engine.symbol)),
            ]),
            Line::from(vec![
                Span::styled("Fuel: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(format!("{}/{}", ship.fuel.current, ship.fuel.capacity)),
            ]),
        ];

        let basic_info_widget = Paragraph::new(basic_info)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Ship Information"),
            )
            .wrap(Wrap { trim: true });
        f.render_widget(basic_info_widget, chunks[1]);

        // Additional ship details: cargo, nav, etc.
        let additonal_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(chunks[2]);

        // Left side: Cargo and modules
        let cargo_capacity = ship.cargo.capacity;
        let cargo_units = ship.cargo.units;
        let cargo_info = vec![
            Line::from(vec![
                Span::styled("Cargo: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(format!("{}/{}", cargo_units, cargo_capacity)),
            ]),
            Line::from(""),
            Line::from(vec![Span::styled(
                "Modules: ",
                Style::default().add_modifier(Modifier::BOLD),
            )]),
        ];
        let mut modules_info = cargo_info;
        for module in &ship.modules {
            modules_info.push(Line::from(vec![
                Span::raw(" - "),
                Span::styled(
                    format!("{:?}", module.symbol),
                    Style::default().fg(Color::Cyan),
                ),
            ]));
        }

        let cargo_widget = Paragraph::new(modules_info)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Cargo & Modules"),
            )
            .wrap(Wrap { trim: true });
        f.render_widget(cargo_widget, additonal_chunks[0]);

        // Right side: Navigation
        let mut nav_info = vec![
            Line::from(vec![
                Span::styled("Status: ", Style::default().add_modifier(Modifier::BOLD)),
                {
                    let status_str = ship.nav.status.to_string();
                    let status_color = match status_str.as_str() {
                        "IN_TRANSIT" => Color::Yellow,
                        "DOCKED" => Color::Green,
                        "IN_ORBIT" => Color::Blue,
                        _ => Color::White,
                    };
                    Span::styled(status_str, Style::default().fg(status_color))
                },
            ]),
            Line::from(vec![
                Span::styled("Location: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(&ship.nav.waypoint_symbol),
            ]),
            Line::from(vec![Span::styled(
                "Route: ",
                Style::default().add_modifier(Modifier::BOLD),
            )]),
        ];

        // Add route information
        nav_info.push(Line::from(vec![
            Span::raw("  From: "),
            Span::styled(
                &ship.nav.route.origin.symbol,
                Style::default().fg(Color::Blue),
            ),
        ]));
        nav_info.push(Line::from(vec![
            Span::raw("  To: "),
            Span::styled(
                &ship.nav.route.destination.symbol,
                Style::default().fg(Color::Green),
            ),
        ]));
        nav_info.push(Line::from(vec![
            Span::raw("  Arrival: "),
            Span::styled(&ship.nav.route.arrival, Style::default().fg(Color::Yellow)),
        ]));

        let nav_widget = Paragraph::new(nav_info)
            .block(Block::default().borders(Borders::ALL).title("Navigation"))
            .wrap(Wrap { trim: true });
        f.render_widget(nav_widget, additonal_chunks[1]);
    } else {
        // If ships data is not loaded, display a loading message
        let message = Paragraph::new("Loading ship data...")
            .block(Block::default().borders(Borders::ALL).title("Loading"))
            .wrap(Wrap { trim: true });
        f.render_widget(message, area);
    }
}

fn render_system_detail(f: &mut Frame, app: &mut App, area: Rect) {
    let system_index = match app.selected_system_index {
        Some(index) => index,
        None => {
            // If no system is selected, display an error message
            let message = Paragraph::new("No system selected")
                .block(Block::default().borders(Borders::ALL).title("Error"))
                .wrap(Wrap { trim: true });
            f.render_widget(message, area);
            return;
        }
    };

    if let Some(systems) = &app.systems {
        if system_index >= systems.len() {
            // If the selected index is out of bounds, display an error message
            let message = Paragraph::new("Invalid system selection")
                .block(Block::default().borders(Borders::ALL).title("Error"))
                .wrap(Wrap { trim: true });
            f.render_widget(message, area);
            return;
        }

        let system = &systems[system_index];
        let system_type = system.r#type.to_string();
        let system_color = get_system_type_color(&system_type);

        // Split the screen into sections
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Header
                Constraint::Length(7), // Basic info
                Constraint::Min(0),    // Waypoints section
            ])
            .split(area);

        // Render system header
        let header_text = Text::from(vec![Line::from(vec![
            Span::raw("System: "),
            Span::styled(&system.symbol, Style::default().fg(Color::Green)),
            Span::raw(" - Type: "),
            Span::styled(&system_type, Style::default().fg(system_color)),
        ])]);
        let header = Paragraph::new(header_text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("System Detail"),
            )
            .wrap(Wrap { trim: true });
        f.render_widget(header, chunks[0]);

        // Render basic info
        let basic_info = vec![
            Line::from(vec![
                Span::styled("Type: ", Style::default().add_modifier(Modifier::BOLD)),
                {
                    let system_type_str = system.r#type.to_string();
                    let type_color = get_system_type_color(&system_type_str);
                    Span::styled(system_type_str, Style::default().fg(type_color))
                },
            ]),
            Line::from(vec![
                Span::styled("Position: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(format!("X: {}, Y: {}", system.x, system.y)),
            ]),
            Line::from(vec![
                Span::styled("Waypoints: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(system.waypoints.len().to_string()),
            ]),
            Line::from(vec![
                Span::styled("Factions: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(system.factions.len().to_string()),
            ]),
        ];

        let basic_info_widget = Paragraph::new(basic_info)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("System Information"),
            )
            .wrap(Wrap { trim: true });
        f.render_widget(basic_info_widget, chunks[1]);

        // Split waypoints section: list on left, map on right
        let waypoints_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(chunks[2]);

        let waypoints_list_area = waypoints_chunks[0];
        let waypoints_plot_area = waypoints_chunks[1];

        // Render waypoints list
        if system.waypoints.is_empty() {
            let no_waypoints = Paragraph::new("No waypoints in this system")
                .block(Block::default().borders(Borders::ALL).title("Waypoints"))
                .wrap(Wrap { trim: true });
            f.render_widget(no_waypoints, waypoints_list_area);
        } else {
            let waypoint_items: Vec<ListItem> = system
                .waypoints
                .iter()
                .map(|waypoint| {
                    let waypoint_type = waypoint.r#type.to_string();
                    let waypoint_color = match waypoint_type.as_str() {
                        "PLANET" => Color::Green,
                        "GAS_GIANT" => Color::LightRed,
                        "MOON" => Color::White,
                        "ORBITAL_STATION" => Color::Blue,
                        "JUMP_GATE" => Color::Magenta,
                        "ASTEROID_FIELD" => Color::Yellow,
                        "NEBULA" => Color::LightMagenta,
                        "DEBRIS_FIELD" => Color::DarkGray,
                        _ => Color::Gray,
                    };

                    let lines = vec![Line::from(vec![
                        Span::styled(&waypoint.symbol, Style::default().fg(Color::Cyan)),
                        Span::raw(" - "),
                        Span::styled(waypoint_type.clone(), Style::default().fg(waypoint_color)),
                        Span::raw(format!(" ({}, {})", waypoint.x, waypoint.y)),
                    ])];

                    ListItem::new(lines)
                })
                .collect();

            let waypoints_list = List::new(waypoint_items)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Waypoints (press Enter for details)"),
                )
                .highlight_style(Style::default().bg(Color::DarkGray))
                .highlight_symbol(">> ");

            // Using app's waypoints_state for selection
            f.render_stateful_widget(
                waypoints_list,
                waypoints_list_area,
                &mut app.waypoints_state,
            );

            // Render waypoints plot
            render_waypoints_plot(f, app, waypoints_plot_area, &system.waypoints);
        }
    } else {
        // If systems data is not loaded, display a loading message
        let message = Paragraph::new("Loading system data...")
            .block(Block::default().borders(Borders::ALL).title("Loading"))
            .wrap(Wrap { trim: true });
        f.render_widget(message, area);
    }
}

fn render_waypoints_plot(
    f: &mut Frame,
    app: &App,
    area: Rect,
    waypoints: &[spacetraders_api::models::SystemWaypoint],
) {
    if waypoints.is_empty() {
        return;
    }

    // Find the min and max coordinates to set the boundaries
    let (min_x, max_x, min_y, max_y) = waypoints.iter().fold(
        (i32::MAX, i32::MIN, i32::MAX, i32::MIN),
        |(min_x, max_x, min_y, max_y), waypoint| {
            (
                min_x.min(waypoint.x),
                max_x.max(waypoint.x),
                min_y.min(waypoint.y),
                max_y.max(waypoint.y),
            )
        },
    );

    // Create the plot block
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Waypoints Map");

    let inner_area = block.inner(area);
    f.render_widget(block, area);

    let plot_width = inner_area.width as usize;
    let plot_height = inner_area.height as usize;

    // Add buffer space for plotting points
    let x_range = (max_x - min_x + 1).max(2) as f64; // Ensure range is at least 2
    let y_range = (max_y - min_y + 1).max(2) as f64; // Ensure range is at least 2

    // Create a canvas to plot points on
    let mut canvas = vec![vec![(' ', Color::Reset); plot_width]; plot_height];

    // Plot the star at center (0,0)
    let star_x = ((0 - min_x) as f64 / x_range * (plot_width as f64 - 1.0)) as usize;
    let star_y = ((max_y - 0) as f64 / y_range * (plot_height as f64 - 1.0)) as usize;

    if star_x < plot_width && star_y < plot_height {
        canvas[star_y][star_x] = ('★', Color::Yellow);
    }

    // Plot waypoints
    for waypoint in waypoints {
        // Map waypoint coordinates to plot coordinates
        let plot_x = ((waypoint.x - min_x) as f64 / x_range * (plot_width as f64 - 1.0)) as usize;
        let plot_y = ((max_y - waypoint.y) as f64 / y_range * (plot_height as f64 - 1.0)) as usize;

        if plot_x < plot_width && plot_y < plot_height {
            let waypoint_type = waypoint.r#type.to_string();
            let waypoint_char = match waypoint_type.as_str() {
                "PLANET" => 'P',
                "GAS_GIANT" => 'G',
                "MOON" => 'm',
                "ORBITAL_STATION" => 'S',
                "JUMP_GATE" => 'J',
                "ASTEROID_FIELD" => '∗',
                "NEBULA" => '≈',
                "DEBRIS_FIELD" => '⦿',
                _ => '•',
            };

            let waypoint_color = match waypoint_type.as_str() {
                "PLANET" => Color::Green,
                "GAS_GIANT" => Color::LightRed,
                "MOON" => Color::White,
                "ORBITAL_STATION" => Color::Blue,
                "JUMP_GATE" => Color::Magenta,
                "ASTEROID_FIELD" => Color::Yellow,
                "NEBULA" => Color::LightMagenta,
                "DEBRIS_FIELD" => Color::DarkGray,
                _ => Color::Gray,
            };

            canvas[plot_y][plot_x] = (waypoint_char, waypoint_color);
        }
    }

    // Highlight selected waypoint
    if let Some(selected_idx) = app.waypoints_state.selected() {
        if selected_idx < waypoints.len() {
            let waypoint = &waypoints[selected_idx];
            let plot_x =
                ((waypoint.x - min_x) as f64 / x_range * (plot_width as f64 - 1.0)) as usize;
            let plot_y =
                ((max_y - waypoint.y) as f64 / y_range * (plot_height as f64 - 1.0)) as usize;

            if plot_x < plot_width && plot_y < plot_height {
                // Keep the waypoint type character but make it bright white and boldface
                let waypoint_type = waypoint.r#type.to_string();
                let waypoint_char = match waypoint_type.as_str() {
                    "PLANET" => 'P',
                    "GAS_GIANT" => 'G',
                    "MOON" => 'm',
                    "ORBITAL_STATION" => 'S',
                    "JUMP_GATE" => 'J',
                    "ASTEROID_FIELD" => '∗',
                    "NEBULA" => '≈',
                    "DEBRIS_FIELD" => '⦿',
                    _ => '•',
                };
                canvas[plot_y][plot_x] = (waypoint_char, Color::White);
            }
        }
    }

    // Convert canvas to text
    let mut lines = Vec::new();
    for row in canvas {
        let mut spans = Vec::new();
        for (ch, color) in row {
            spans.push(Span::styled(ch.to_string(), Style::default().fg(color)));
        }
        lines.push(Line::from(spans));
    }

    let text = Text::from(lines);
    let paragraph = Paragraph::new(text);
    f.render_widget(paragraph, inner_area);
}

fn render_waypoint_detail(f: &mut Frame, app: &mut App, area: Rect) {
    let waypoint_index = match app.selected_waypoint_index {
        Some(index) => index,
        None => {
            // If no waypoint is selected, display an error message
            let message = Paragraph::new("No waypoint selected")
                .block(Block::default().borders(Borders::ALL).title("Error"))
                .wrap(Wrap { trim: true });
            f.render_widget(message, area);
            return;
        }
    };

    if let Some(systems) = &app.systems {
        if let Some(system_index) = app.selected_system_index {
            if system_index < systems.len() {
                let system = &systems[system_index];
                if waypoint_index >= system.waypoints.len() {
                    // If the selected index is out of bounds, display an error message
                    let message = Paragraph::new("Invalid waypoint selection")
                        .block(Block::default().borders(Borders::ALL).title("Error"))
                        .wrap(Wrap { trim: true });
                    f.render_widget(message, area);
                    return;
                }

                let waypoint = &system.waypoints[waypoint_index];
                let waypoint_type = waypoint.r#type.to_string();
                let waypoint_color = match waypoint_type.as_str() {
                    "PLANET" => Color::Green,
                    "GAS_GIANT" => Color::LightRed,
                    "MOON" => Color::White,
                    "ORBITAL_STATION" => Color::Blue,
                    "JUMP_GATE" => Color::Magenta,
                    "ASTEROID_FIELD" => Color::Yellow,
                    "NEBULA" => Color::LightMagenta,
                    "DEBRIS_FIELD" => Color::DarkGray,
                    _ => Color::Gray,
                };

                // Split the screen into sections
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Length(3), // Header
                        Constraint::Length(8), // Basic info
                        Constraint::Min(0),    // Traits and other details
                    ])
                    .split(area);

                // Render header
                let header = Paragraph::new(vec![
                    Line::from(vec![Span::styled(
                        &waypoint.symbol,
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                    )]),
                    Line::from(vec![Span::styled(
                        format!("System: {}", system.symbol),
                        Style::default().fg(Color::Gray),
                    )]),
                ])
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Waypoint Detail"),
                )
                .alignment(Alignment::Center);
                f.render_widget(header, chunks[0]);

                // Render basic info
                let basic_info = vec![
                    Line::from(vec![
                        Span::styled("Type: ", Style::default().add_modifier(Modifier::BOLD)),
                        Span::styled(&waypoint_type, Style::default().fg(waypoint_color)),
                    ]),
                    Line::from(vec![
                        Span::styled("Position: ", Style::default().add_modifier(Modifier::BOLD)),
                        Span::raw(format!("X: {}, Y: {}", waypoint.x, waypoint.y)),
                    ]),
                    Line::from(vec![
                        Span::styled("Orbits: ", Style::default().add_modifier(Modifier::BOLD)),
                        Span::raw(match &waypoint.orbits {
                            Some(orbits) => orbits,
                            None => "None",
                        }),
                    ]),
                    Line::from(vec![
                        Span::styled("Orbitals: ", Style::default().add_modifier(Modifier::BOLD)),
                        Span::raw(format!("{} objects", waypoint.orbitals.len())),
                    ]),
                ];

                let basic_info_widget = Paragraph::new(basic_info)
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .title("Waypoint Information"),
                    )
                    .wrap(Wrap { trim: true });
                f.render_widget(basic_info_widget, chunks[1]);

                // Split bottom area for orbitals and other details
                let bottom_chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                    .split(chunks[2]);

                // Render orbitals on the left
                let orbitals_info = if waypoint.orbitals.is_empty() {
                    vec![Line::from("No orbital bodies")]
                } else {
                    waypoint
                        .orbitals
                        .iter()
                        .map(|orbital| {
                            Line::from(vec![
                                Span::styled("• ", Style::default().fg(Color::Blue)),
                                Span::styled(
                                    &orbital.symbol,
                                    Style::default()
                                        .fg(Color::Cyan)
                                        .add_modifier(Modifier::BOLD),
                                ),
                            ])
                        })
                        .collect()
                };

                let orbitals_widget = Paragraph::new(orbitals_info)
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .title("Orbital Bodies"),
                    )
                    .wrap(Wrap { trim: true });
                f.render_widget(orbitals_widget, bottom_chunks[0]);

                // Render system information on the right
                let system_info = vec![
                    Line::from(vec![
                        Span::styled("System: ", Style::default().add_modifier(Modifier::BOLD)),
                        Span::styled(system.symbol.clone(), Style::default().fg(Color::Green)),
                    ]),
                    Line::from(vec![
                        Span::styled("Type: ", Style::default().add_modifier(Modifier::BOLD)),
                        {
                            let system_type_str = system.r#type.to_string();
                            let type_color = get_system_type_color(&system_type_str);
                            Span::styled(system_type_str, Style::default().fg(type_color))
                        },
                    ]),
                    Line::from(vec![
                        Span::styled("Position: ", Style::default().add_modifier(Modifier::BOLD)),
                        Span::raw(format!("X: {}, Y: {}", system.x, system.y)),
                    ]),
                    Line::from(vec![
                        Span::styled("Waypoints: ", Style::default().add_modifier(Modifier::BOLD)),
                        Span::raw(format!("{} total", system.waypoints.len())),
                    ]),
                ];

                let system_widget = Paragraph::new(system_info)
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .title("System Information"),
                    )
                    .wrap(Wrap { trim: true });
                f.render_widget(system_widget, bottom_chunks[1]);
            }
        }
    } else {
        // If systems data is not loaded, display a loading message
        let message = Paragraph::new("Loading system data...")
            .block(Block::default().borders(Borders::ALL).title("Loading"))
            .wrap(Wrap { trim: true });
        f.render_widget(message, area);
    }
}
