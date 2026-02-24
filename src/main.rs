use arboard::Clipboard;
use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind, KeyModifiers,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use harper_core::{Dictionary, Document, LintSet, Linter};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};
use std::io::{self, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

struct App {
    content: String,
    cursor_pos: (u16, u16),
    file_name: Option<String>,
    file_modified: bool,
    dialect: String,
    lint_results: Vec<LintResult>,
    suggestions: Vec<Suggestion>,
    last_lint_time: Instant,
    dictionary: Dictionary,
    lint_set: LintSet,
    lint_triggered: Arc<AtomicBool>,
}

#[derive(Clone, Debug)]
struct LintResult {
    line: usize,
    start_col: usize,
    end_col: usize,
    message: String,
}

#[derive(Clone, Debug)]
struct Suggestion {
    line: usize,
    col: usize,
    original: String,
    replacement: String,
    message: String,
}

impl App {
    fn new() -> Self {
        let dictionary = Dictionary::new();
        let mut lint_set = LintSet::new();
        lint_set.add_standard(dictionary.clone());

        Self {
            content: String::new(),
            cursor_pos: (0, 0),
            file_name: None,
            file_modified: false,
            dialect: "US".to_string(),
            lint_results: Vec::new(),
            suggestions: Vec::new(),
            last_lint_time: Instant::now(),
            dictionary,
            lint_set,
            lint_triggered: Arc::new(AtomicBool::new(false)),
        }
    }

    fn load_file(&mut self, path: &str) -> Result<(), String> {
        let metadata = std::fs::metadata(path).map_err(|e| e.to_string())?;

        if metadata.len() > 10 * 1024 * 1024 {
            return Err("File too large (max 10MB)".to_string());
        }

        let content = std::fs::read_to_string(path).map_err(|e| e.to_string())?;

        self.content = content;
        self.file_name = Some(path.to_string());
        self.file_modified = false;
        self.cursor_pos = (0, 0);

        self.last_lint_time = Instant::now();
        self.lint_triggered.store(true, Ordering::SeqCst);

        Ok(())
    }

    fn save_file(&mut self) -> Result<(), String> {
        if let Some(ref path) = self.file_name {
            std::fs::write(path, &self.content).map_err(|e| e.to_string())?;
            self.file_modified = false;
            Ok(())
        } else {
            Err("No file loaded".to_string())
        }
    }

    fn paste_from_clipboard(&mut self) -> Result<(), String> {
        let mut clipboard = Clipboard::new().map_err(|e| e.to_string())?;
        if let Ok(text) = clipboard.get_text() {
            self.content.push_str(&text);
            self.file_modified = true;
            self.last_lint_time = Instant::now();
            self.lint_triggered.store(true, Ordering::SeqCst);
            Ok(())
        } else {
            Err("No text in clipboard".to_string())
        }
    }

    fn run_linter(&mut self) {
        let content = self.content.clone();

        let doc = Document::new_plain_english(&content);
        let mut lints = self.lint_set.lint(&doc);

        self.lint_results.clear();
        self.suggestions.clear();

        for lint in lints.drain(..) {
            let span = lint.span;
            let (line, start_col) = offset_to_line_col(&content, span.start);
            let end_col = start_col + span.len().min(1);
            let lint_message = lint.message.clone();

            self.lint_results.push(LintResult {
                line,
                start_col,
                end_col,
                message: lint_message.clone(),
            });

            for suggestion in lint.suggestions {
                self.suggestions.push(Suggestion {
                    line,
                    col: start_col,
                    original: content
                        .lines()
                        .nth(line)
                        .unwrap_or("")
                        .chars()
                        .skip(start_col)
                        .take(span.len())
                        .collect(),
                    replacement: suggestion.to_string(),
                    message: lint_message.clone(),
                });
            }
        }

        self.last_lint_time = Instant::now();
    }
}

fn offset_to_line_col(content: &str, offset: usize) -> (usize, usize) {
    let mut line = 0;
    let mut col = 0;
    let mut current_offset = 0;

    for ch in content.chars() {
        if current_offset >= offset {
            break;
        }
        if ch == '\n' {
            line += 1;
            col = 0;
        } else {
            col += 1;
        }
        current_offset += 1;
    }

    (line, col)
}

fn run_app<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    let mut app = App::new();

    loop {
        terminal.draw(|f| ui(f, &app))?;

        if event::poll(Duration::from_millis(10))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('o') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            if let Ok(current_dir) = std::env::current_dir() {
                                let file_path = current_dir.join("test.txt");
                                if let Err(e) =
                                    app.load_file(file_path.to_str().unwrap_or("test.txt"))
                                {
                                    eprintln!("Error loading file: {}", e);
                                }
                            }
                        }
                        KeyCode::Char('v')
                            if key.modifiers.contains(KeyModifiers::CONTROL)
                                && key.modifiers.contains(KeyModifiers::SHIFT) =>
                        {
                            if let Err(e) = app.paste_from_clipboard() {
                                eprintln!("Error pasting: {}", e);
                            }
                        }
                        KeyCode::Char('v')
                            if key.modifiers.contains(KeyModifiers::SUPER)
                                && key.modifiers.contains(KeyModifiers::SHIFT) =>
                        {
                            if let Err(e) = app.paste_from_clipboard() {
                                eprintln!("Error pasting: {}", e);
                            }
                        }
                        KeyCode::Char(c) => {
                            app.content.push(c);
                            app.file_modified = true;
                            app.last_lint_time = Instant::now();
                            app.lint_triggered.store(true, Ordering::SeqCst);
                        }
                        KeyCode::Backspace => {
                            app.content.pop();
                            app.file_modified = true;
                            app.last_lint_time = Instant::now();
                            app.lint_triggered.store(true, Ordering::SeqCst);
                        }
                        KeyCode::Enter => {
                            app.content.push('\n');
                            app.file_modified = true;
                            app.last_lint_time = Instant::now();
                            app.lint_triggered.store(true, Ordering::SeqCst);
                        }
                        KeyCode::Esc => {
                            return Ok(());
                        }
                        _ => {}
                    }
                }
            }
        }

        if app.lint_triggered.load(Ordering::SeqCst)
            && app.last_lint_time.elapsed() >= Duration::from_millis(10)
        {
            app.run_linter();
            app.lint_triggered.store(false, Ordering::SeqCst);

            if app.file_modified && app.file_name.is_some() {
                if let Err(e) = app.save_file() {
                    eprintln!("Error saving file: {}", e);
                }
            }
        }
    }
}

fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(3),
            Constraint::Length(3),
            Constraint::Length(1),
        ])
        .split(f.area());

    let editor_area = chunks[0];
    let suggestions_area = chunks[1];
    let status_bar = chunks[2];

    let lines: Vec<&str> = app.content.lines().collect();
    let line_count = if app.content.is_empty() || !app.content.ends_with('\n') {
        lines.len().max(1)
    } else {
        lines.len()
    };

    let mut editor_content = String::new();
    for i in 0..line_count {
        let line_num = format!("{:>4} ", i + 1);
        let line_content = lines.get(i).unwrap_or(&"");
        editor_content.push_str(&line_num);
        editor_content.push_str(line_content);
        editor_content.push('\n');
    }

    let editor_block = Block::default()
        .title("Editor")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White));

    let editor_widget = Paragraph::new(editor_content)
        .block(editor_block)
        .style(Style::default().fg(Color::Reset));

    f.render_widget(editor_widget, editor_area);

    let suggestions_block = Block::default()
        .title("Suggestions")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White));

    let suggestions_content = if app.suggestions.is_empty() {
        " No suggestions ".to_string()
    } else {
        app.suggestions
            .iter()
            .enumerate()
            .map(|(i, s)| {
                format!(
                    "{}. {} -> {} ({})",
                    i + 1,
                    s.original,
                    s.replacement,
                    s.message
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    };

    let suggestions_widget = Paragraph::new(suggestions_content)
        .block(suggestions_block)
        .style(Style::default().fg(Color::Yellow));

    f.render_widget(suggestions_widget, suggestions_area);

    let file_display = app.file_name.as_deref().unwrap_or("[No File]");
    let modified_indicator = if app.file_modified { "*" } else { "" };
    let cursor_info = format!(
        " {}{} | Ln {}, Col {} | Dialect: {} | Errors: {} ",
        file_display,
        modified_indicator,
        app.cursor_pos.1 + 1,
        app.cursor_pos.0 + 1,
        app.dialect,
        app.lint_results.len()
    );

    let status_block = Block::default()
        .borders(Borders::NONE)
        .style(Style::default().bg(Color::DarkGray).fg(Color::White));

    let status_widget = Paragraph::new(cursor_info)
        .block(status_block)
        .style(Style::default().bg(Color::DarkGray).fg(Color::White));

    f.render_widget(status_widget, status_bar);
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("Error: {:?}", err);
    }

    Ok(())
}
