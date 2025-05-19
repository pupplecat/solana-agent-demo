use std::cell::RefCell;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use color_eyre::Result;
use crossterm::event::{self, KeyCode, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout, Position},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, List, ListItem, ListState, Paragraph},
};
use rig::agent::{Agent, PromptRequest};
use rig::providers::gemini::completion::CompletionModel;
use textwrap::wrap;

/// Application state.
pub struct App {
    /// Current input value.
    input: String,
    /// Cursor position in the input field.
    character_index: usize,
    /// Current input mode.
    input_mode: InputMode,
    /// Message history with timestamps and roles.
    messages: Vec<Message>,
    /// Scroll state for message list.
    messages_state: ListState,
    /// Application state (e.g., processing).
    state: AppState,
    /// Gemini agent.
    agent: Agent<CompletionModel>,
    /// Spinner animation state.
    spinner: RefCell<Spinner>,
    /// Start time of the current processing task.
    processing_start: Option<Instant>,
    /// Progress bar animation state.
    progress: RefCell<Progress>,
}

/// Input mode for the application.
#[derive(PartialEq)]
enum InputMode {
    Normal,
    Editing,
}

/// Application state.
#[derive(PartialEq)]
enum AppState {
    Idle,
    Processing,
    Cancelling,
}

/// Message with role and timestamp.
#[derive(Clone)]
struct Message {
    role: MessageRole,
    content: String,
    timestamp: u64,
}

/// Role of the message sender.
#[derive(Clone, PartialEq)]
enum MessageRole {
    User,
    Assistant,
    System,
}

/// Spinner animation state.
struct Spinner {
    frames: &'static [&'static str],
    index: usize,
}

impl Spinner {
    fn new() -> Self {
        Self {
            frames: &["|", "/", "-", "\\"],
            index: 0,
        }
    }

    fn next(&mut self) -> &'static str {
        let frame = self.frames[self.index];
        self.index = (self.index + 1) % self.frames.len();
        frame
    }
}

/// Progress bar animation state.
struct Progress {
    position: usize,
    direction: i8,
    width: usize,
}

impl Progress {
    fn new(width: usize) -> Self {
        Self {
            position: 0,
            direction: 1,
            width,
        }
    }

    fn next(&mut self) -> String {
        let bar_width = self.width / 2;
        let mut bar = vec!['-'; self.width];
        let start = self.position;
        let end = (self.position + bar_width).min(self.width);
        for i in start..end {
            bar[i] = '#';
        }

        self.position = (self.position as i32 + self.direction as i32) as usize;
        if self.position + bar_width >= self.width || self.position == 0 {
            self.direction *= -1;
        }

        format!("[{}]", bar.iter().collect::<String>())
    }
}

impl App {
    pub fn new(agent: Agent<CompletionModel>) -> Self {
        Self {
            input: String::new(),
            input_mode: InputMode::Normal,
            messages: Vec::new(),
            character_index: 0,
            messages_state: ListState::default(),
            state: AppState::Idle,
            agent,
            spinner: RefCell::new(Spinner::new()),
            processing_start: None,
            progress: RefCell::new(Progress::new(10)),
        }
    }

    fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.character_index.saturating_sub(1);
        self.character_index = self.clamp_cursor(cursor_moved_left);
    }

    fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.character_index.saturating_add(1);
        self.character_index = self.clamp_cursor(cursor_moved_right);
    }

    fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.input.insert(index, new_char);
        self.move_cursor_right();
    }

    fn byte_index(&self) -> usize {
        self.input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.character_index)
            .unwrap_or(self.input.len())
    }

    fn delete_char(&mut self) {
        if self.character_index != 0 {
            let current_index = self.character_index;
            let from_left_to_current_index = current_index - 1;
            let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
            let after_char_to_delete = self.input.chars().skip(current_index);
            self.input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.chars().count())
    }

    fn reset_cursor(&mut self) {
        self.character_index = 0;
    }

    fn submit_message(&mut self) {
        if self.state != AppState::Idle || self.input.trim().is_empty() {
            return;
        }

        let prompt = self.input.clone();
        self.messages.push(Message {
            role: MessageRole::User,
            content: prompt,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        });
        self.messages.push(Message {
            role: MessageRole::System,
            content: "Waiting for response...".to_string(),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        });
        self.input.clear();
        self.reset_cursor();
        self.state = AppState::Processing;
        self.processing_start = Some(Instant::now());
        self.scroll_to_bottom();
    }

    fn cancel_processing(&mut self) {
        if self.state == AppState::Processing {
            self.state = AppState::Cancelling;
            self.messages.push(Message {
                role: MessageRole::System,
                content: "Request cancelled.".to_string(),
                timestamp: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            });
            self.scroll_to_bottom();
        }
    }

    fn scroll_to_bottom(&mut self) {
        if !self.messages.is_empty() {
            self.messages_state.select(Some(self.messages.len() - 1));
        }
    }

    pub async fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        let tick_rate = Duration::from_millis(250);
        let mut last_tick = Instant::now();

        loop {
            terminal.draw(|frame| self.render(frame))?;

            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or(Duration::from_secs(0));

            if event::poll(timeout)? {
                if let Some(key) = event::read()?.as_key_press_event() {
                    match self.input_mode {
                        InputMode::Normal => match key.code {
                            KeyCode::Char('e') => self.input_mode = InputMode::Editing,
                            KeyCode::Char('q') => return Ok(()),
                            KeyCode::Up => {
                                let selected = self.messages_state.selected().unwrap_or(0);
                                if selected > 0 {
                                    self.messages_state.select(Some(selected - 1));
                                }
                            }
                            KeyCode::Down => {
                                let selected = self.messages_state.selected().unwrap_or(0);
                                if selected < self.messages.len() - 1 {
                                    self.messages_state.select(Some(selected + 1));
                                }
                            }
                            _ => {}
                        },
                        InputMode::Editing if key.kind == KeyEventKind::Press => match key.code {
                            KeyCode::Enter => self.submit_message(),
                            KeyCode::Char(to_insert) => self.enter_char(to_insert),
                            KeyCode::Backspace => self.delete_char(),
                            KeyCode::Left => self.move_cursor_left(),
                            KeyCode::Right => self.move_cursor_right(),
                            KeyCode::Esc => {
                                if self.state == AppState::Processing {
                                    self.cancel_processing();
                                } else {
                                    self.input_mode = InputMode::Normal;
                                }
                            }
                            _ => {}
                        },
                        InputMode::Editing => {}
                    }
                }
            }

            if last_tick.elapsed() >= tick_rate {
                if self.state == AppState::Processing {
                    let prompt = self
                        .messages
                        .iter()
                        .rev()
                        .find(|m| m.role == MessageRole::User)
                        .map(|m| m.content.clone())
                        .unwrap_or_default();

                    tokio::select! {
                        result = PromptRequest::new(&self.agent, prompt).multi_turn(5) => {
                            match result {
                                Ok(response) => {
                                    // Remove the "Waiting for response..." message
                                    self.messages.retain(|m| m.role != MessageRole::System || m.content != "Waiting for response...");
                                    self.messages.push(Message {
                                        role: MessageRole::Assistant,
                                        content: response,
                                        timestamp: SystemTime::now()
                                            .duration_since(UNIX_EPOCH)
                                            .unwrap()
                                            .as_secs(),
                                    });
                                    self.state = AppState::Idle;
                                    self.processing_start = None;
                                    self.scroll_to_bottom();
                                }
                                Err(e) => {
                                    self.messages.retain(|m| m.role != MessageRole::System || m.content != "Waiting for response...");
                                    self.messages.push(Message {
                                        role: MessageRole::Assistant,
                                        content: format!("Error: {}", e),
                                        timestamp: SystemTime::now()
                                            .duration_since(UNIX_EPOCH)
                                            .unwrap()
                                            .as_secs(),
                                    });
                                    self.state = AppState::Idle;
                                    self.processing_start = None;
                                    self.scroll_to_bottom();
                                }
                            }
                        }
                        _ = async {
                            while self.state == AppState::Processing {
                                tokio::time::sleep(Duration::from_millis(100)).await;
                            }
                        } => {}
                    }
                } else if self.state == AppState::Cancelling {
                    self.messages.retain(|m| {
                        m.role != MessageRole::System || m.content != "Waiting for response..."
                    });
                    self.state = AppState::Idle;
                    self.processing_start = None;
                }
                last_tick = Instant::now();
            }
        }
    }

    fn render(&self, frame: &mut Frame) {
        let vertical = Layout::vertical([
            Constraint::Length(1), // Help area (top)
            Constraint::Min(1),    // Messages area (remaining space)
            Constraint::Length(3), // Input area (bottom, fixed height)
        ]);
        let [help_area, messages_area, input_area] = vertical.areas(frame.area());

        let (msg, style) = match self.input_mode {
            InputMode::Normal => (
                vec![
                    "Press ".into(),
                    "q".bold(),
                    " to exit, ".into(),
                    "e".bold(),
                    " to start editing, ".into(),
                    "↑/↓".bold(),
                    " to scroll.".into(),
                ],
                Style::default().add_modifier(Modifier::RAPID_BLINK),
            ),
            InputMode::Editing => (
                vec![
                    "Press ".into(),
                    "Esc".bold(),
                    match self.state {
                        AppState::Idle => " to stop editing, ",
                        _ => " to cancel, ",
                    }
                    .into(),
                    "Enter".bold(),
                    " to send message.".into(),
                ],
                Style::default(),
            ),
        };
        let text = Text::from(Line::from(msg)).patch_style(style);
        let help_message = Paragraph::new(text);
        frame.render_widget(help_message, help_area);

        let elapsed = self
            .processing_start
            .map(|start| start.elapsed().as_secs_f32())
            .map(|secs| format!(" ({:.1}s)", secs))
            .unwrap_or_default();
        let title = match self.state {
            AppState::Idle => "Input".to_string(),
            AppState::Processing => {
                let frame = self.spinner.borrow_mut().next();
                format!("Processing {} {}", frame, elapsed)
            }
            AppState::Cancelling => "Cancelling...".to_string(),
        };
        let input = Paragraph::new(self.input.as_str())
            .style(match self.input_mode {
                InputMode::Normal => Style::default(),
                InputMode::Editing => Style::default().fg(Color::Yellow),
            })
            .block(Block::bordered().title(title));
        frame.render_widget(input, input_area); // Render input in the bottom area

        if self.input_mode == InputMode::Editing {
            frame.set_cursor_position(Position::new(
                input_area.x + self.character_index as u16 + 1,
                input_area.y + 1,
            ));
        }

        // Calculate available width for messages (subtract 2 for borders)
        let available_width = messages_area.width.saturating_sub(2) as usize;

        let messages: Vec<ListItem> = self
            .messages
            .iter()
            .enumerate()
            .map(|(_, m)| {
                let prefix = match m.role {
                    MessageRole::User => "User",
                    MessageRole::Assistant => "Assistant",
                    MessageRole::System => "System",
                };
                let timestamp =
                    chrono::DateTime::<chrono::Utc>::from_timestamp(m.timestamp as i64, 0)
                        .map(|dt| dt.format("%H:%M:%S").to_string())
                        .unwrap_or("Unknown".to_string());

                // Calculate prefix width (e.g., "[12:34:56] User: ")
                let prefix_str = format!("[{}] {}: ", timestamp, prefix);
                let prefix_width = prefix_str.len();
                let content_width = available_width.saturating_sub(prefix_width);

                // Wrap the content
                let wrapped_content = wrap(&m.content, content_width)
                    .into_iter()
                    .map(|cow| cow.to_string())
                    .collect::<Vec<String>>();

                // Create spans for each line
                let mut spans = Vec::new();
                let prefix_span = Span::styled(
                    prefix_str,
                    Style::default().fg(match m.role {
                        MessageRole::User => Color::Green,
                        MessageRole::Assistant => Color::Cyan,
                        MessageRole::System => Color::Magenta,
                    }),
                );

                // First line includes the prefix
                if let Some(first_line) = wrapped_content.first() {
                    let mut first_line_spans = vec![prefix_span];
                    first_line_spans.push(Span::raw(first_line.to_string()));
                    spans.push(Line::from(first_line_spans));
                }

                // Subsequent lines are indented to align with the content
                let indent = " ".repeat(prefix_width);
                for line in wrapped_content.iter().skip(1) {
                    spans.push(Line::from(vec![
                        Span::raw(indent.clone()),
                        Span::raw(line.to_string()),
                    ]));
                }

                // Add progress bar if applicable
                if m.role == MessageRole::System
                    && m.content == "Waiting for response..."
                    && self.state == AppState::Processing
                {
                    let progress_line = Line::from(vec![
                        Span::raw(indent),
                        Span::styled(
                            self.progress.borrow_mut().next(),
                            Style::default().fg(Color::Magenta),
                        ),
                    ]);
                    spans.push(progress_line);
                }

                ListItem::new(Text::from(spans))
            })
            .collect();
        let messages = List::new(messages)
            .block(Block::bordered().title("Messages"))
            .highlight_style(Style::default().fg(Color::Yellow));
        frame.render_stateful_widget(messages, messages_area, &mut self.messages_state.clone());
    }
}
