use std::process::Command;
use std::ffi::OsStr;

pub struct LoggedCommand {
    inner: Command,
}

impl LoggedCommand {
    pub fn new(command: &str) -> Self {
        Self {
            inner: Command::new(command),
        }
    }

    pub fn arg<S: AsRef<OsStr>>(&mut self, arg: S)-> &mut Self {
        self.inner.arg(arg.as_ref());
        self
    }

    pub fn status(&mut self) -> std::io::Result<()> {
        // Format the command to print without quotes
        let full_command = format!(
            "{} {}",
            self.inner.get_program().to_string_lossy(),
            self.inner.get_args().map(|arg| arg.to_string_lossy()).collect::<Vec<_>>().join(" ")
        );
        println!("[INFO] Running command: {}", full_command);

        // Execute the command
        match self.inner.status() {
            Ok(status) => {
                if status.success() {
                    println!("[INFO] Command executed successfully.");
                } else {
                    eprintln!("[WARN] Command exited with status: {}", status);
                }
            }
            Err(e) => {
                eprintln!("[ERROR] Failed to execute command: {}", e);
            }
        }
        Ok(())
    }
}
