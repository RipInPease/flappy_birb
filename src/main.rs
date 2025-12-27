use crossterm::{queue, execute};
use flappy_birb::*;


use crossterm::{
    QueueableCommand, 
    terminal::{self, ClearType}, 
    cursor, 
    event::{self, Event, KeyEvent, KeyCode, KeyModifiers}
};


use std::io::Result as IOResult;
use std::io::Write;


use std::time::{Instant, Duration};


fn main() -> IOResult<()> {
    let mut stdout = std::io::stdout();
    terminal::enable_raw_mode()?;
    queue!(
        stdout, 
        cursor::Hide,
        terminal::Clear(ClearType::All),
    )?;
    
    let mut pipes = PipeVec::new()?;
    let mut birb = Birb::new()?;
    
    let mut last_draw = Instant::now();
    let mut last_pipe_update = Instant::now();
    let mut last_birb_update = Instant::now();
    
    loop {
        while event::poll(Duration::ZERO)? {
            match event::read()? {
                Event::Key(key) => handle_input(key, &mut birb),
                _ => ()
            }
        }

        // Only draw after every 60fps
        if last_draw.elapsed() > Duration::from_millis(1000) / 60 {
            execute!(stdout, terminal::BeginSynchronizedUpdate)?;

            stdout.queue(terminal::Clear(ClearType::All))?;
            pipes.draw(&mut stdout)?;
            birb.draw(&mut stdout)?;
            stdout.flush()?;

            execute!(stdout, terminal::EndSynchronizedUpdate)?;

            last_draw = Instant::now();
        }


        // Only move pipes every x milliseconds
        if last_pipe_update.elapsed() > Duration::from_millis(1000) / 10 {
            pipes.update()?;

            last_pipe_update = Instant::now();
        }

        // Only update birb after x milliseconds
        if last_birb_update.elapsed() > Duration::from_millis(1000) / 10 {
            birb.update()?;

            last_birb_update = Instant::now();
        }
    }
}


/// Handle key inputs
/// 
fn handle_input(event: KeyEvent, birb: &mut Birb) {

    // Ctrl+C to touch grass
    if event.modifiers == KeyModifiers::CONTROL && event.code == KeyCode::Char('c') {
        panic!()
    }

    else if event.code == KeyCode::Char(' ') {
        birb.jump();
    }
}