use std::{
    path::Path,
    process::{Child, Command},
};

use anyhow::Result;

pub struct VideoPlayer {
    command: Command,
    child: Option<Child>,
}

impl VideoPlayer {
    pub fn new(player_path: impl AsRef<Path>, video_path: impl AsRef<Path>) -> Self {
        let mut command = Command::new(player_path.as_ref());
        command.arg(video_path.as_ref());
        Self {
            command,
            child: None,
        }
    }
}

impl super::VideoPlayer for VideoPlayer {
    fn start(&mut self) -> Result<()> {
        if self.child.is_none() {
            self.child.replace(self.command.spawn()?);
        }
        Ok(())
    }

    fn stop(&mut self) -> Result<()> {
        if let Some(mut child) = self.child.take() {
            child.kill()?;
        }
        Ok(())
    }
}
