use std::io::{BufRead, BufReader, Error};
use std::os::unix::process::ExitStatusExt;
use std::process::{Command, ExitStatus};
use std::thread::sleep;
use std::time::Duration;
mod cmd;
mod constants;
use cmd::{
    attach_session, create_session, create_window, list_sessions, list_windows, rename_window, run_module, set_env
};
use constants::*;

use crate::config::WINDOWS;

pub struct Session {
    pub name: String,
}

impl Session {
    pub fn new(name: String) -> Session {
        Session {
            name,
        }
    }

    pub fn build(&self, debug: bool) -> Result<ExitStatus, Error> {
        // session
        self.create_session()?;
        self.sleep_till_session_creation();
        self.set_env("ZXC", "1");
        if debug {
            self.set_env("ZXC_DEBUG", "1");
        }
        // windows
        self.create_windows()?;
        self.sleep_till_windows_creation();
        self.start_modules()
    }

    /* Gets tmux command for the session
     *
     * Returns:
     *      tmux -L session_name
     */

    pub fn cmd(&self) -> Command {
        let mut cmd = Command::new(TMUX);
        cmd.arg(SOCKET_FLAG).arg(&self.name);
        cmd
    }

    /* Command:
     *        tmux -L session_name -f $HOME/config/tmux.conf new-session -d
     */

    #[inline]
    pub fn create_session(&self) -> Result<ExitStatus, Error> {
        let mut cmd = self.cmd();
        create_session(&mut cmd)?;
        cmd.status()
    }

    /* Steps:
     *      1. Build command
     *          tmux -L session_name ls
     *
     *      2. If command succeed, session is created break else sleep for
     *          500 ms
     */

    pub fn sleep_till_session_creation(&self) {
        let mut cmd = self.cmd();
        list_sessions(&mut cmd);
        loop {
            if let Ok(result) = cmd.status() {
                if result.success() {
                    break;
                }
                // sleep for 500 ms
                sleep(Duration::from_millis(500));
            }
        }
    }

    /* Description:
     *      Create and rename windows.
     *
     * Steps:
     *       1. Iterate through MODULES
     *
     *       2. Create Window,
     *              tmux -L session_name new-window
     *
     *       3. Rename Window
     *              tmux -L session_name rename-window -t $windows_id $module
     */

    pub fn create_windows(&self) -> Result<ExitStatus, Error> {
        for (index, (_, name, _)) in WINDOWS.iter().enumerate() {
            if index != 0 {
                let mut cmd = self.cmd();
                create_window(&mut cmd);
                cmd.status()?;
            }
            let mut cmd = self.cmd();
            rename_window(&mut cmd, index, name);
            cmd.status()?;
        }
        Ok(ExitStatus::from_raw(0))
    }

    /* Steps:
     *      1. Build command
     *              tmux -L session_name list-windows -F "#{window_index}"
     *
     *      2. If number of windows created is 5, break else
     *          sleep for 500 ms
     */

    pub fn sleep_till_windows_creation(&self) {
        let mut cmd = self.cmd();
        list_windows(&mut cmd);

        loop {
            if let Ok(result) = cmd.output() {
                if BufReader::new(result.stdout.as_slice())
                    .lines()
                    .count()
                    == 4
                {
                    break;
                }
                sleep(Duration::from_millis(500));
            }
        }
    }

    /* Steps:
     *       1. Iterate through MODULES
     *       2. Start Module
     *          tmux -L session_name send-keys -t module.target() module Enter
     */

    pub fn start_modules(&self) -> Result<ExitStatus, Error> {
        for (_, name, target) in WINDOWS.iter() {
            let mut cmd = self.cmd();
            run_module(&mut cmd, target, name);
            cmd.status()?;
        }
        Ok(ExitStatus::from_raw(0))
    }

    /* Command:
     *      tmux -L session_name setenv env_var env_value
     */

    pub fn set_env(&self, name: &str, value: &str) {
        let mut cmd = self.cmd();
        set_env(&mut cmd, name, value);
        if let Err(e) = cmd.status() {
            eprintln!("Failed to set env var|{} |{} |{}", name, value, e);
        };
    }

    /* Command:
     *      tmux -L session_name attach -t :1
     *      By default, tmux attaches to the history window (:1).
     */

    pub fn attach_session(&self) -> Result<ExitStatus, Error> {
        let mut cmd = self.cmd();
        attach_session(&mut cmd);
        cmd.status()
    }
}

#[cfg(test)]
mod holas {
    use std::ffi::OsStr;

    use super::*;

    #[test]
    fn hola_cmd() {
        let session = Session::new("hola".to_string());
        let binding = session.cmd();
        let args: Vec<&OsStr> = binding.get_args().collect();
        let verify = vec!["-L", "hola"];
        assert_eq!(verify, args);
    }
}
