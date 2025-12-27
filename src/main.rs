use crossterm::queue;
use flappy_birb::*;


use crossterm::{QueueableCommand, terminal, cursor};
use crossterm::terminal::ClearType;


use std::io::Result as IOResult;
use std::io::Write;


fn main() -> IOResult<()> {
    let mut stdout = std::io::stdout();
    terminal::enable_raw_mode()?;
    queue!(
        stdout, 
        cursor::Hide,
        terminal::Clear(ClearType::All),
    )?;

    let pipe = Pipe::new()?;

    let last_draw = std::time::Instant::now();

    loop {
        if last_draw.elapsed() > std::time::Duration::from_millis(1000) / 60 {
            pipe.draw(&mut stdout)?;
            stdout.flush()?;
        }
    }
}