use crossterm::{cursor, terminal, QueueableCommand};
use std::{
    io::{stdout, Stdout, Write},
    thread, time,
};

use super::Maze;

use crate::render::Renderer;

pub struct DefaultRenderer {
    stdout: Stdout,
}

impl DefaultRenderer {
    pub fn new(stdout: Stdout) -> Self {
        Self { stdout }
    }
}

impl Default for DefaultRenderer {
    fn default() -> Self {
        Self { stdout: stdout() }
    }
}

impl<const N: usize> Renderer<Maze<N>> for DefaultRenderer {
    fn setup(&mut self) {
        self.stdout.flush().expect("Failed to flush stdout");
    }

    fn teardown(&mut self) {
        self.stdout.flush().expect("Failed to flush stdout");
    }

    fn render(&mut self, environment: &Maze<N>) {
        // // Save the current cursor position
        self.stdout.queue(cursor::SavePosition).unwrap();

        // Clear the terminal from the saved cursor position upwards
        self.stdout
            .queue(terminal::Clear(terminal::ClearType::FromCursorUp))
            .unwrap();

        // Restore the cursor position
        self.stdout.queue(cursor::RestorePosition).unwrap();

        // // Write the new maze content
        self.stdout
            .write_all(format!("{}\n", environment).as_bytes())
            .unwrap();

        // Flush the output to ensure it's displayed
        self.stdout.flush().unwrap();

        // // Optionally sleep for a smooth rendering delay
        thread::sleep(time::Duration::from_millis(100));
    }
}
