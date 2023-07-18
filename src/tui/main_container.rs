//! module that holds `MainContainer` struct.

use std::io::Stdout;

use tuirealm::{
    command::{Cmd, CmdResult},
    event::Event,
    props::{AttrValue, Attribute, Color, Props, Style},
    tui::{
        backend::CrosstermBackend,
        layout::Rect,
        symbols::DOT,
        text::Spans,
        widgets::Tabs,
        Frame,
    },
    Component,
    MockComponent,
    State,
    StateValue,
};

use crate::tui::{Msg, UserEvent};

/// Holds all the widgets for the TUI.
pub(super) struct MainContainer {
    /// The tabs.
    tabs:   Tabs<'static>,
    /// The properties of the component.
    props:  Props,
    /// The states of the component.
    states: MainContainerStates,
}

impl MainContainer {
    /// Create a new instance of `MainContainer`.
    pub(super) fn new() -> Self {
        let tab_names = ["General", "Guilds", "Database", "Logs"]
            .into_iter()
            .map(Spans::from)
            .collect();

        Self {
            tabs:   Tabs::new(tab_names)
                .style(Style::default().fg(Color::White))
                .highlight_style(Style::default().fg(Color::Yellow))
                .divider(DOT),
            props:  Props::default(),
            states: MainContainerStates::default(),
        }
    }
}

impl MockComponent for MainContainer {
    fn view(&mut self, frame: &mut Frame<'_, CrosstermBackend<Stdout>>, area: Rect) {
        if self.props.get_or(Attribute::Display, AttrValue::Flag(true)) == AttrValue::Flag(true) {
            frame.render_widget(self.tabs.clone(), area);
        }
    }

    fn query(&self, attr: Attribute) -> Option<AttrValue> { self.props.get(attr) }

    fn attr(&mut self, attr: Attribute, value: AttrValue) { self.props.set(attr, value); }

    fn state(&self) -> State { State::One(StateValue::Usize(self.states.selected_tab)) }

    fn perform(&mut self, _cmd: Cmd) -> CmdResult {
        // TODO: Handle the Cmds.
        CmdResult::None
    }
}

impl Component<Msg, UserEvent> for MainContainer {
    fn on(&mut self, _ev: Event<UserEvent>) -> Option<Msg> { None }
}

/// The states of the [`MainContainer`].
#[derive(Copy, Clone, Default)]
struct MainContainerStates {
    /// Index of the selected tab.
    selected_tab: usize,
}
