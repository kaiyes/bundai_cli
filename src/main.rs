use std::io::{self, stdout};

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};

use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::widgets::Block;
use ratatui::widgets::*;
use ratatui::Frame;
use ratatui::Terminal;

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut should_quit = false;
    while !should_quit {
        terminal.draw(ui)?;
        should_quit = handle_events()?;
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn handle_events() -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

fn ui(frame: &mut Frame) {
    let main_layout = Layout::new(
        Direction::Vertical,
        [
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ],
    )
    .split(frame.size());
    frame.render_widget(
        Block::new().borders(Borders::TOP).title("Bundai"),
        main_layout[0],
    );

    let inner_layout = Layout::new(
        Direction::Horizontal,
        [Constraint::Percentage(30), Constraint::Percentage(70)],
    )
    .split(main_layout[1]);

    // Create a vertical layout for smaller blocks in the left area
    let left_block_layout = Layout::new(
        Direction::Vertical,
        [
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
        ],
    )
    .split(inner_layout[0]);

    // Render smaller blocks in the left area
    frame.render_widget(
        Block::default().borders(Borders::ALL).title("Level N1"),
        left_block_layout[0],
    );
    frame.render_widget(
        Block::default().borders(Borders::ALL).title("Level N2"),
        left_block_layout[1],
    );
    frame.render_widget(
        Block::default().borders(Borders::ALL).title("Level N3"),
        left_block_layout[2],
    );
    frame.render_widget(
        Block::default().borders(Borders::ALL).title("Level N4"),
        left_block_layout[3],
    );
    frame.render_widget(
        Block::default().borders(Borders::ALL).title("Level N5"),
        left_block_layout[4],
    );

    // Create a grid layout for the right block
    let grid_layout = Layout::new(
        Direction::Vertical,
        (0..5)
            .map(|_| Constraint::Percentage(20))
            .collect::<Vec<_>>(), // 5 rows
    )
    .split(inner_layout[1]);

    for (i, row) in grid_layout.iter().enumerate() {
        let row_layout = Layout::new(
            Direction::Horizontal,
            (0..5)
                .map(|_| Constraint::Percentage(20))
                .collect::<Vec<_>>(), // 5 columns
        )
        .split(*row);

        for (j, cell) in row_layout.iter().enumerate() {
            let title_text = format!("Kanji {}", i * 5 + j + 1);
            frame.render_widget(
                Block::default().borders(Borders::ALL).title(title_text),
                *cell,
            );
        }
    }
}
