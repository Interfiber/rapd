#![allow(clippy::if_same_then_else)]

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use rapd::{metadata_for_file, RapdMetadata, RapdPlayerTime, RapdServer};
use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Spans, Text},
    widgets::{Block, Borders, Clear, Gauge, List, ListItem, ListState, Paragraph},
    Frame, Terminal,
};

mod rapd;

#[derive(PartialEq)]
enum Mode {
    Database,
    Playlists,
    ViewingPlaylist,
    AddFileToPlaylist,
}

pub struct App<'a> {
    db_state: ListState,
    db_files: Vec<ListItem<'a>>,
    playlist_state: ListState,
    playlists: Vec<ListItem<'a>>,
    files: Vec<String>,
    length: RapdPlayerTime,
    time: RapdPlayerTime,
    metadata: Option<RapdMetadata>,
    keybinds_open: bool,
    playlist_open: bool,
    mode: Mode,
    playlist_names: Vec<String>,
    playlist_files: Vec<ListItem<'a>>,
    playlist_files_state: ListState,
    playlist_desc: String,
    playlist_paths: Vec<String>,
    viewing_playlist: bool,
    playlist_name: String,
}

impl<'a> App<'a> {
    pub fn new() -> App<'a> {
        let mut l = App {
            db_state: ListState::default(),
            db_files: vec![],
            length: RapdPlayerTime {
                hour: 1,
                min: 1,
                second: 1,
            },
            time: RapdPlayerTime {
                hour: 0,
                min: 0,
                second: 0,
            },
            files: vec![],
            metadata: Default::default(),
            keybinds_open: false,
            mode: Mode::Database,
            playlist_state: ListState::default(),
            playlists: vec![],
            playlist_names: vec![],
            playlist_open: false,
            playlist_files: vec![],
            playlist_files_state: ListState::default(),
            playlist_desc: String::from("No description!"),
            playlist_paths: vec![],
            viewing_playlist: false,
            playlist_name: String::from(""),
        };

        l.db_state.select(Some(0));

        l
    }

    pub fn load_database(&mut self) {
        let db = rapd::database_files();

        for metadata in db.iter() {
            let line = format!("{} - {}", metadata.artist, metadata.title);
            self.db_files.push(ListItem::new(line));
            self.files.push(metadata.file.clone());
        }
    }

    pub fn load_playlists(&mut self) {
        self.playlist_names.clear();
        self.playlists.clear();
        self.playlist_paths.clear();
        self.playlist_files_state.select(Some(0));

        let playlists = rapd::playlists();

        for playlist in playlists.iter() {
            self.playlists
                .push(ListItem::new(playlist.playlist_name.to_owned()));
            self.playlist_names.push(playlist.playlist_name.to_owned());
        }
    }

    pub fn update(&mut self) {
        self.length = rapd::get_length();
        self.time = rapd::get_time();
    }

    pub fn get_time(&self) -> String {
        let mut sec = format!("{}", self.length.second);
        let mut min = format!("{}", self.length.min);

        let mut sec1 = format!("{}", self.time.second);
        let mut min1 = format!("{}", self.time.min);

        if self.length.second < 9 {
            sec = format!("0{}", self.length.second);
        }

        if self.length.min < 9 {
            min = format!("0{}", self.length.min);
        }

        if self.time.second < 9 {
            sec1 = format!("0{}", self.time.second);
        }

        if self.time.min < 9 {
            min1 = format!("0{}", self.time.min);
        }

        format!(
            "{}:{}:{} / {}:{}:{}",
            self.time.hour, min1, sec1, self.length.hour, min, sec
        )
    }

    pub fn get_progress(&self) -> u64 {
        let min_perc = 0_f64;
        let millis_len =
            ((((self.length.hour * 60) * 60) + (self.length.min * 60) + self.length.second) * 1000)
                as u32;
        let millis_time = ((((self.time.hour * 60) * 60) + (self.time.min * 60) + self.time.second)
            * 1000) as u32;
        let track_progress = std::cmp::min(millis_time, millis_len);
        let track_perc = (track_progress as f64 / f64::from(millis_len)) * 100_f64;

        min_perc.max(track_perc) as u64
    }

    pub fn get_metadata(&self) -> String {
        let metadata = self.metadata.as_ref();

        if let Some(metadata) = metadata {
            format!(
                "Author: {}\nAlbum: {}\nSong: {}\n",
                metadata.artist, metadata.album, metadata.title
            )
        } else {
            String::new()
        }
    }

    pub fn update_metadata(&mut self, file: String) {
        let mut server = RapdServer::new();
        server.connect();

        let meta = metadata_for_file(&mut server, file);
        server.close();

        self.metadata = Some(meta);
    }
}

impl<'a> Default for App<'a> {
    fn default() -> App<'a> {
        Self::new()
    }
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let mut app = App::new();
    app.load_database();
    app.load_playlists();
    let res = run_app(&mut terminal, &mut app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_secs(1);

    loop {
        if last_tick.elapsed() >= tick_rate {
            app.update();
            last_tick = Instant::now();
        }

        terminal.draw(|f| ui(f, app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    return Ok(());
                }

                if let KeyCode::F(1) = key.code {
                    // play file
                    if app.mode == Mode::Database {
                        let selected = app.db_state.selected().unwrap_or(0);
                        rapd::play_file(app.files[selected].to_owned(), false);
                    } else if app.mode == Mode::ViewingPlaylist {
                        let selected = app.playlist_files_state.selected().unwrap_or(0);
                        rapd::play_file(app.playlist_paths[selected].to_owned(), false);
                    }
                }

                if let KeyCode::F(2) = key.code {
                    // loop file
                    if app.mode == Mode::Database {
                        let selected = app.db_state.selected().unwrap_or(0);
                        rapd::play_file(app.files[selected].to_owned(), true);
                    } else if app.mode == Mode::ViewingPlaylist {
                        let selected = app.playlist_files_state.selected().unwrap_or(0);
                        rapd::play_file(app.playlist_paths[selected].to_owned(), true);
                    }
                }

                if let KeyCode::F(3) = key.code {
                    // stop player
                    rapd::stop();
                }

                if let KeyCode::F(4) = key.code {
                    // pause player
                    rapd::pause();
                }

                if let KeyCode::F(7) = key.code {
                    // add item to selected playlist
                    if app.mode == Mode::Database {
                        app.mode = Mode::AddFileToPlaylist;
                        app.viewing_playlist = true;
                    } else if app.mode == Mode::AddFileToPlaylist {
                        app.mode = Mode::Playlists;
                        app.viewing_playlist = false;
                    }
                }

                if let KeyCode::Enter = key.code {
                    // toggle selected view
                    if app.mode == Mode::Database {
                        app.mode = Mode::Playlists;
                        app.playlist_state.select(Some(0));
                    } else if app.mode == Mode::AddFileToPlaylist {
                        // ignore
                    } else {
                        app.mode = Mode::Database;
                    }
                }

                if let KeyCode::Char('d') = key.code {
                    if app.mode == Mode::Playlists {
                        // remove playlist
                        let selected = app.playlist_state.selected().unwrap_or(0);
                        let name = app.playlist_names[selected].to_owned();

                        rapd::remove_playlist(name);
                        app.load_playlists();
                    }
                }

                if let KeyCode::F(5) = key.code {
                    // open keybinds
                    if app.mode != Mode::ViewingPlaylist {
                        app.keybinds_open = !app.keybinds_open;
                    }
                }

                if let KeyCode::F(6) = key.code {
                    // expand playlist
                    if app.mode == Mode::ViewingPlaylist {
                        app.playlist_open = false;
                        app.mode = Mode::Playlists;
                    } else if app.mode == Mode::Playlists {
                        let selected = app.playlist_state.selected().unwrap_or(0);
                        let playlist_name = &app.playlist_names[selected];

                        // get playlist info
                        let mut server = RapdServer::new();
                        server.connect();

                        let info = rapd::playlist_info(String::from(playlist_name), &mut server);

                        app.playlist_files.clear();
                        app.playlist_paths.clear();

                        for file in info.files {
                            let metadata = rapd::metadata_for_file(&mut server, file.to_owned());
                            app.playlist_files.push(ListItem::new(format!(
                                "{} - {}",
                                metadata.artist, metadata.title
                            )));
                            app.playlist_paths.push(file);
                        }

                        app.playlist_desc = info.playlist_desc;

                        server.close();

                        app.playlist_open = true;
                        app.mode = Mode::ViewingPlaylist;
                        app.playlist_files_state.select(Some(1));
                    }
                }
                if let KeyCode::Down = key.code {
                    if app.mode == Mode::Database {
                        let i = match app.db_state.selected() {
                            Some(i) => {
                                if i >= app.db_files.len() - 1 {
                                    0
                                } else {
                                    i + 1
                                }
                            }
                            None => 0,
                        };

                        app.db_state.select(Some(i));
                    } else if app.mode == Mode::ViewingPlaylist {
                        let i = match app.playlist_files_state.selected() {
                            Some(i) => {
                                if i >= app.playlist_files.len() - 1 {
                                    0
                                } else {
                                    i + 1
                                }
                            }
                            None => 0,
                        };

                        app.playlist_files_state.select(Some(i));
                    } else {
                        let i = match app.playlist_state.selected() {
                            Some(i) => {
                                if app.playlists.is_empty() {
                                    0
                                } else if i >= app.playlists.len() - 1 {
                                    0
                                } else {
                                    i + 1
                                }
                            }
                            None => 0,
                        };

                        app.playlist_state.select(Some(i));
                    }
                }

                if app.mode == Mode::AddFileToPlaylist {
                    match key.code {
                        KeyCode::Char(c) => {
                            app.playlist_name.push(c);
                        }
                        KeyCode::Backspace => {
                            app.playlist_name.pop();
                        }
                        KeyCode::Enter => {
                            // get selected file
                            let selected = app.db_state.selected().unwrap_or(0);
                            let file = &app.files[selected];
                            rapd::add_file_to_playlist(
                                app.playlist_name.to_owned(),
                                String::from(file),
                            );
                            app.mode = Mode::Playlists;
                            app.viewing_playlist = false;
                            app.playlist_name = String::from("");
                        }
                        _ => {}
                    }
                }

                if let KeyCode::Up = key.code {
                    if app.mode == Mode::Database {
                        let i = match app.db_state.selected() {
                            Some(i) => {
                                if i == 0 {
                                    app.db_files.len() - 1
                                } else {
                                    i - 1
                                }
                            }
                            None => 0,
                        };
                        app.db_state.select(Some(i));
                    } else if app.mode == Mode::ViewingPlaylist {
                        let i = match app.playlist_files_state.selected() {
                            Some(i) => {
                                if i == 0 {
                                    app.playlist_paths.len() - 1
                                } else {
                                    i - 1
                                }
                            }
                            None => 0,
                        };
                        app.playlist_files_state.select(Some(i));
                    } else {
                        let i = match app.playlist_state.selected() {
                            Some(i) => {
                                if app.playlists.is_empty() {
                                    0
                                } else if i == 0 {
                                    app.playlists.len() - 1
                                } else {
                                    i - 1
                                }
                            }
                            None => 0,
                        };
                        app.playlist_state.select(Some(i));
                    }
                }
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(5)
        .constraints(
            [
                Constraint::Percentage(60),
                Constraint::Percentage(40),
                Constraint::Percentage(90),
            ]
            .as_ref(),
        )
        .split(f.size());

    let mut title = "Music";
    if app.mode == Mode::Database {
        title = "Music [SELECTED]";
    }
    let list = List::new(app.db_files.to_vec())
        .block(Block::default().title(title).borders(Borders::ALL))
        .style(tui::style::Style::default().fg(Color::White))
        .highlight_style(
            tui::style::Style::default()
                .add_modifier(Modifier::BOLD)
                .add_modifier(Modifier::ITALIC),
        )
        .highlight_symbol(">> ");
    f.render_stateful_widget(list, chunks[0], &mut app.db_state);

    let mut title = "Playlists";
    if app.mode == Mode::Playlists {
        title = "Playlists [SELECTED]";
    }

    let playlists = List::new(app.playlists.to_vec())
        .block(Block::default().title(title).borders(Borders::ALL))
        .style(tui::style::Style::default().fg(Color::White))
        .highlight_style(
            tui::style::Style::default()
                .add_modifier(Modifier::BOLD)
                .add_modifier(Modifier::ITALIC),
        )
        .highlight_symbol(">> ");
    f.render_stateful_widget(playlists, chunks[1], &mut app.playlist_state);

    let progress = Gauge::default()
        .block(
            Block::default()
                .title("Playback progress")
                .borders(Borders::ALL),
        )
        .gauge_style(Style::default().fg(Color::White).bg(Color::Black))
        .label(app.get_time())
        .percent(app.get_progress().try_into().unwrap());

    let chunks_2 = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(90),
                Constraint::Percentage(10),
                Constraint::Percentage(0),
            ]
            .as_ref(),
        )
        .split(f.size());
    f.render_widget(progress, chunks_2[1]);

    if app.keybinds_open {
        let keybinds = Paragraph::new("[F5] toggle keybinds\n[F1] play selected file\n[F2] loop selected file\n[F3] stop player\n[F4] pause player\n[ENTER] toggle to playlists\n[F6] expand playlist").block(Block::default().borders(Borders::ALL).title("Keybinds"));
        let area = centered_rect(60, 24, f.size());

        f.render_widget(Clear, area); // clear background
        f.render_widget(keybinds, area);
    } else if app.playlist_open {
        let files = List::new(app.playlist_files.to_vec())
            .block(
                Block::default()
                    .title(format!("Playlist - {}", app.playlist_desc))
                    .borders(Borders::ALL),
            )
            .style(tui::style::Style::default().fg(Color::White))
            .highlight_style(
                tui::style::Style::default()
                    .add_modifier(Modifier::BOLD)
                    .add_modifier(Modifier::ITALIC),
            )
            .highlight_symbol(">> ");

        let area = centered_rect(60, 25, f.size());

        f.render_widget(Clear, area);
        f.render_stateful_widget(files, area, &mut app.playlist_files_state);
    } else if app.viewing_playlist {
        let style = Style::default().add_modifier(Modifier::RAPID_BLINK);

        let mut text = Text::from(Spans::from(app.playlist_name.clone()));
        text.patch_style(style);

        let input = Paragraph::new(app.playlist_name.as_ref()).block(
            Block::default()
                .borders(Borders::ALL)
                .title("Enter playlist name, press ENTER to confirm"),
        );

        let area = centered_rect(50, 10, f.size());

        f.render_widget(Clear, area);
        f.render_widget(input, area);
    }
}
