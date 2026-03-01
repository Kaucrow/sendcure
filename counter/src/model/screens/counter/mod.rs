pub mod recv_pkg;
pub mod send_pkg;

use crate::{
    prelude::*,
    model::{Popup, input::*}
};

#[derive(Debug)]
pub enum Tab {
    Received(recv_pkg::State),
    Send(send_pkg::State),
}

#[derive(Debug)]
pub enum TabId {
    Received,
    Send,
}

#[derive(Debug, Default)]
pub struct Tabs {
    tabs: Vec<Tab>,
}

impl Tabs {
    pub fn new(tabs: Vec<Tab>) -> Self {
        Self {
            tabs,
        }
    }

    pub fn get(&self, idx: usize) -> Result<&Tab> {
        self.tabs
            .get(idx)
            .ok_or(anyhow::Error::msg(format!("No tab for idx {}", idx)))
    }

    pub fn get_mut(&mut self, idx: usize) -> Result<&mut Tab> {
        self.tabs
            .get_mut(idx)
            .ok_or(anyhow::Error::msg(format!("No tab for idx {}", idx)))
    }
}

#[derive(Debug, Default)]
pub struct TimedHelpText {
    text: String,
    expires_at: Option<Instant>,
}

impl TimedHelpText {
    pub fn set(&mut self, text: impl Into<String>, duration: Duration) {
        self.text = text.into();
        self.expires_at = Some(Instant::now() + duration);
    }

    pub fn get(&self) -> Option<&str> {
        if let Some(expiry) = self.expires_at {
            if Instant::now() < expiry {
                return Some(&self.text);
            }
        }
        None
    }

    pub fn clear(&mut self) {
        self.expires_at = None;
        self.text.clear();
    }
}

#[derive(Debug)]
pub struct Package {
    pub package_id: i32,
    pub package_desc: String,
    pub package_weight: f64,
    pub package_width: i32,
    pub package_length: i32,
    pub package_height: i32,
}

#[derive(Debug, Default)]
pub struct State {
    pub active_popup: Option<Popup>,
    pub tabs: Tabs,
    pub inputs: InputFields,
    pub action_sel: Option<u8>,
    pub sidebar_state: ListState,
    pub client: Option<u32>,
    pub help_text: String,
    pub temp_help_text: TimedHelpText,
}