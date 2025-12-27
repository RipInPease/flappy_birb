use crossterm::{queue, QueueableCommand};
use crossterm::style::{
    SetBackgroundColor,
    SetForegroundColor,
    Color,
    Print
};
use crossterm::cursor;
use crossterm::terminal;

use std::io::Result as IOResult;
use std::io::{Stdout};


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
    vel : u16,          // Velocity
}


impl Birb {

    /// Gives the current position of the birb
    /// 
    pub fn pos(&self) -> (u16, u16) {
        self.pos
    }


    /// Gets the vertical velocity of the birb
    /// 
    pub fn velocity(&self) -> u16 {
        self.vel
    }


    /// Makes the birb go jump
    /// 
    pub fn jump(&mut self) {
        self.vel = 10;
    }


    /// Time tick:
    /// Updates position and velocity
    /// 
    pub fn update(&mut self) {
        self.pos.1 += self.vel;
        self.vel -= 1;
    }
}


impl Draw for Birb {
    fn draw(&self, stdout: &mut Stdout) -> IOResult<()> {
        queue!(
            stdout,
            SetForegroundColor(Color::Yellow),
            cursor::MoveTo(self.pos.0, self.pos.1),
            PrintLines("██\n██"),
        )
    }
}




/// A very pipe
/// 
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


        // Move to where to start drawing and set color to green
        queue!(
            stdout,
            SetForegroundColor(Color::Green),
            cursor::MoveTo(self.pos, 0)
        )?;


        // Print the top part of the pipe
        for _ in 0..self.split_height {
            stdout.queue(PrintLines("████\n"))?;
        }

        stdout.queue(cursor::MoveDown(7))?; 

        while cursor::position()?.1 < stdout_y -1 {
            stdout.queue(PrintLines("████\n"))?;
        }
        stdout.queue(Print("████"))?;

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