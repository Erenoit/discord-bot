//! Module for TUI

use std::{process::Command, time::Duration};

use anyhow::Result;
use tuirealm::{
    application::{Application, PollStrategy},
    listener::EventListenerCfg,
    terminal::TerminalBridge,
    tui::layout::{Constraint, Direction, Layout},
    Update,
};

use crate::tui::main_container::MainContainer;

mod main_container;

/// This struct is responsible for the whole TUI.
pub struct Tui {
    /// The `Application` that is used for the TUI.
    app:      Application<Id, Msg, UserEvent>,
    /// Terminal bridge.
    terminal: TerminalBridge,
    /// Whether to redraw the TUI.
    redraw:   bool,
}

impl Tui {
    /// Create a new instance of [`Tui`].
    ///
    /// # Errors
    ///
    /// This function can return an error if the terminal cannot be initialized.
    /// It can also return an error if the `Application` cannot be initialized.
    pub fn new() -> Result<Self> {
        Ok(Self {
            app:      Self::app_init()?,
            terminal: TerminalBridge::new()?,
            redraw:   true,
        })
    }

    /// Initializes the [`Application`].
    ///
    /// [`Application`]: tuirealm::Application
    fn app_init() -> Result<Application<Id, Msg, UserEvent>> {
        let mut app = Application::init(
            EventListenerCfg::default()
                .default_input_listener(Duration::from_millis(20))
                .poll_timeout(Duration::from_millis(10))
                .tick_interval(Duration::from_secs(1)),
        );

        app.mount(
            Id::MainContainer,
            Box::new(MainContainer::new()),
            Vec::default(),
        )?;

        app.active(&Id::MainContainer)?;

        Ok(app)
    }

    /// Run the TUI.
    ///
    /// # Errors
    ///
    /// This function can return an error if the terminal cannot be initialized.
    pub fn run(&mut self) -> Result<()> {
        self.terminal.enter_alternate_screen()?;
        self.terminal.enable_raw_mode()?;

        loop {
            match self.app.tick(PollStrategy::Once) {
                Ok(messages) if !messages.is_empty() => {
                    self.redraw = true;

                    for msg in messages {
                        let mut msg = Some(msg);
                        while msg.is_some() {
                            msg = self.update(msg);
                        }
                    }
                },
                Err(_) => {
                    // self.app.dispatch(Msg::AppClose);
                    break;
                },
                _ => {},
            }

            if self.redraw {
                Command::new("notify-send").arg("Redraw").spawn()?;
                // let frame = &mut self.terminal.raw_mut().get_frame();
                // self.app.view(&Id::MainContainer, frame, frame.size());

                self.terminal.raw_mut().draw(|frame| {
                    let layout = Layout::default()
                        .direction(Direction::Vertical)
                        .margin(0)
                        .constraints([Constraint::Percentage(100)].as_ref())
                        .split(frame.size());

                    self.app.view(&Id::MainContainer, frame, layout[0]);
                })?;
                self.redraw = false;
            }
        }

        Ok(())
    }

    /// Restores the terminal after the TUI has been closed.
    ///
    /// # Errors
    ///
    /// This function can return an error if the terminal cannot be restored.
    pub fn clear(&mut self) -> Result<()> {
        self.terminal.leave_alternate_screen()?;
        self.terminal.disable_raw_mode()?;
        self.terminal.clear_screen()?;

        Ok(())
    }
}

impl Update<Msg> for Tui {
    fn update(&mut self, msg: Option<Msg>) -> Option<Msg> {
        if let Some(msg) = msg {
            match msg {
                Msg::AppClose => {
                    return None;
                },
            }
        }

        None
    }
}

/// Ids for the `Application`.
#[derive(Copy, Clone, Eq, Hash, PartialEq)]
enum Id {
    /// Main container.
    MainContainer,
}

/// Messages that can be sent to the `Application`.
#[derive(Hash, PartialEq)]
enum Msg {
    /// Close the application.
    AppClose,
}

#[derive(Copy, Clone, Eq, Hash, PartialEq, PartialOrd, Ord)]
enum UserEvent {
    /// Close the application.
    AppClose,
}
