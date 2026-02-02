use std::time::Duration;
use std::io::stdout;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};

pub struct BenchResult {
    pub name: String,
    pub status: String,
    pub duration: Option<Duration>,
    pub description : Option<String>
}

pub fn run_tui<F>(run_bench_fn: F) -> Result<(), Box<dyn std::error::Error>>
where
    F: FnMut() -> Vec<BenchResult>,
{
    // TUI setup
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut results = Vec::new();

    let res = run_app(&mut terminal, &mut results, run_bench_fn);

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("{:?}", err);
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend, F>(
    terminal: &mut Terminal<B>,
    results: &mut Vec<BenchResult>,
    mut run_bench_fn: F,
) -> Result<(), Box<dyn std::error::Error>>
where
    <B as ratatui::backend::Backend>::Error: std::error::Error + 'static,
    F: FnMut() -> Vec<BenchResult>,
{
    *results = run_bench_fn();
    loop {
        terminal.draw(|f| {
            let size = f.area();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Min(0), Constraint::Length(3)].as_ref())
                .split(size);

            let mut display_text = String::new();
            if results.is_empty() {
                display_text.push_str("No results yet. Press 'r' to run benchmarks.");
            } else {
                display_text.push_str(&format!(
                    "{:<35} | {:<10} | {:<18} | {:<60}\n",
                    "Benchmark Name", "Status", "Duration", "Description"
                ));
                display_text.push_str(&"-".repeat(130));
                display_text.push('\n');

                for res in results.iter() {
                    let dur_str = match res.duration {
                        Some(d) => format!(" {:?}", d),
                        None => "".to_string(),
                    };
                    let description_str = match res.description.as_ref() {
                        Some(e) => e,
                        None => "",
                    };
                    display_text.push_str(&format!(
                        "{:<35} | {:<10} | {:<18} | {:<60}\n",
                        res.name, res.status, dur_str, description_str
                    ));
                }
            }

            let paragraph = Paragraph::new(display_text)
                .block(Block::default().title("Benchmark Results").borders(Borders::ALL));
            f.render_widget(paragraph, chunks[0]);

            let help = Paragraph::new("Press 'q' to quit, 'r' to run benchmarks")
                .block(Block::default().title("Help").borders(Borders::ALL));
            f.render_widget(help, chunks[1]);
        }).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('r') => {
                        *results = run_bench_fn();
                    }
                    _ => {}
                }
            }
        }
    }
}
