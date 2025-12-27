use crossterm::{queue, execute};
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
    
    let mut pipes = PipeVec::new()?;
    let birb = Birb::new()?;
    
    let mut last_draw = std::time::Instant::now();
    let mut last_update = std::time::Instant::now();
    
    loop {
        if last_draw.elapsed() > std::time::Duration::from_millis(1000) / 60 {
            execute!(stdout, terminal::BeginSynchronizedUpdate)?;

            stdout.queue(terminal::Clear(ClearType::All))?;
            pipes.draw(&mut stdout)?;
            birb.draw(&mut stdout)?;
            stdout.flush()?;

            execute!(stdout, terminal::EndSynchronizedUpdate)?;

            last_draw = std::time::Instant::now();
        }


        if last_update.elapsed() > std::time::Duration::from_millis(1000) / 10 {
            pipes.update()?;

            last_update = std::time::Instant::now();
        }
    }
}