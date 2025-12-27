use crossterm::{execute, queue, QueueableCommand};
use crossterm::style::{
    SetForegroundColor,
    Color,
    Print
};
use crossterm::cursor;
use crossterm::terminal;

use std::io::Result as IOResult;
use std::io::{Stdout};


use std::time::{Duration, Instant};


/// Makes an object drawable on a screen
/// 
pub trait Draw {
    fn draw(&self, stdout: &mut Stdout) -> IOResult<()>;
}



/// A very flappy birb
/// 
#[derive(Debug, Clone)]
pub struct Birb {
    pos : (u16, u16),   // position
    vel : f32,          // Velocity
}


impl Birb {

    /// Gives a new birb
    /// 
    pub fn new() -> IOResult<Self> {
        let (_, stdout_y) = terminal::size()?;

        let x = 10;
        let y = stdout_y / 2;

        let res = Self { pos: (x, y), vel: 0.0 };

        Ok(res)
    }

    /// Gives the current position of the birb
    /// 
    pub fn pos(&self) -> (u16, u16) {
        self.pos
    }


    /// Gets the vertical velocity of the birb
    /// 
    pub fn velocity(&self) -> f32 {
        self.vel
    }


    /// Makes the birb go jump
    /// 
    pub fn jump(&mut self) {
        self.vel = -2.2;

    }


    /// Time tick:
    /// Updates position and velocity
    /// 
    pub fn update(&mut self) -> IOResult<()> {
        let stdout_y = terminal::size()?.1;
        let to_apply = self.vel.abs() as u16;

        if self.vel < 0.0 && self.pos.1 > to_apply{
            self.pos.1 -= to_apply
        } else if self.pos.1 + to_apply < stdout_y {
            self.pos.1 += to_apply
        }

        if self.vel < 1.2 {
            self.vel += 0.4;
        }

        Ok(())
    }
}


impl Draw for Birb {
    fn draw(&self, stdout: &mut Stdout) -> IOResult<()> {
        execute!(
            stdout,
            SetForegroundColor(Color::Yellow),
            cursor::MoveTo(self.pos.0, self.pos.1),
            Print("██"),
        )
    }
}




/// A very pipe
/// 
#[derive(Debug, Clone)]
pub struct Pipe {
    pos             : u16,  // X-position of the pipe
    split_height    : u16   // The height at which the pipe is split
}


impl Pipe {

    /// Gives a new instance of the pipe
    /// 
    pub fn new() -> IOResult<Self> {
        let (stdout_x, stdout_y) = terminal::size()?;
        
        let pos = stdout_x - 4;
        let split_height = rand::random_range(0..stdout_y-4);

        let res = Self {
            pos,
            split_height
        };

        Ok(res)
    }

    /// Gives the current x-position of the pipe
    /// 
    pub fn pos(&self) -> u16 {
        self.pos
    }


    /// Updates position, returns the new x-position of the pipe
    /// 
    pub fn update(&mut self) -> u16 {
        if self.pos > 0 {
            self.pos -= 1;
        } else {
            self.pos = 0;
        }

        self.pos
    }
}


impl Draw for Pipe {
    fn draw(&self, stdout: &mut Stdout) -> IOResult<()> {
        let (_, stdout_y) = terminal::size()?;
        let mut curr_row = 0;
        let mut s = String::new();

        // Move to where to start drawing and set color to green
        queue!(
            stdout,
            SetForegroundColor(Color::Green),
            cursor::MoveTo(self.pos, 0)
        )?;

        // Print the top part of the pipe
        while curr_row < self.split_height {
            s.push_str("████\n");
            curr_row += 1;
        }

        // Print the gap
        for _ in 0..7 {
            s.push_str("\n");
        }

        // Print the bottom
        while curr_row < stdout_y -1 {
            s.push_str("████\n");
            curr_row += 1;
        }
        s.push_str("████");

        stdout.queue(PrintLines(&s))?;

        Ok(())
    }
}


/// Contains multiple pipes, makes it easier to do logic with all pipes at once
/// 
#[derive(Debug, Clone)]
pub struct PipeVec {
    // Contains all pipes
    pipes: Vec<Pipe>,

    // Time when last pipe was spawned
    last_pipe: Instant,


    // Spwan rate of pipes
    spawn_rate: Duration
}


impl PipeVec {
    /// Creates a new instance with a single pipe
    /// 
    pub fn new() -> IOResult<Self> {
        let pipe = Pipe::new()?;
        let pipes = vec![pipe];
        let last_pipe = Instant::now();
        let spawn_rate = Duration::from_secs(5);

        let res = Self { pipes, last_pipe, spawn_rate };

        Ok(res)
    }


    /// Updates all pipes. 
    /// Potentially spawns a new pipe if elapsed time since last spawn is long enough.
    /// 
    pub fn update(&mut self) -> IOResult<()> {
        let mut remove_first = false;

        for pipe in &mut self.pipes {
            if pipe.update() == 0 {remove_first = true}
        }

        if remove_first {
            self.pipes.remove(0);
        }

        if self.last_pipe.elapsed() >= self.spawn_rate {
            let new_pipe = Pipe::new()?;
            self.pipes.push(new_pipe);

            self.last_pipe = Instant::now();
        }

        Ok(())
    }


    /// Changes spawn rate of pipes
    /// 
    pub fn set_spawn_rate(&mut self, rate: Duration) {
        self.spawn_rate = rate;
    }
}


impl Draw for PipeVec {
    fn draw(&self, stdout: &mut Stdout) -> IOResult<()> {
        for pipe in &self.pipes {
            pipe.draw(stdout)?;
        }

        Ok(())
    }
}






/// Queues the print command to print a string, but it handles newline (\n) better
/// 
pub struct PrintLines<'a>(&'a str);

use crossterm::Command;
impl<'a> Command for PrintLines<'a> {
    fn write_ansi(&self, f: &mut impl core::fmt::Write) -> core::fmt::Result {
        cursor::SavePosition.write_ansi(f)?;

        for (i, line) in self.0.lines().enumerate() {
            Print(line).write_ansi(f)?;
            cursor::RestorePosition.write_ansi(f)?;
            cursor::MoveDown((i+1) as u16).write_ansi(f)?;
        }
        
        Ok(())
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> IOResult<()> {
        Ok(())
    }
}