use crate::prelude::*;

#[derive(Debug, Default)]
pub struct InputFields {
    inputs: Vec<InputField>,
    active: Option<usize>,
}

impl InputFields {
    pub fn new(amount: usize) -> Self {
        let mut inputs = Vec::new();
        inputs.resize_with(amount, InputField::default);
        Self {
            inputs,
            active: None,
        }
    }

    pub fn with_active(mut self, active: usize) -> Result<Self> {
        self.select(active)?;
        Ok(self)
    }

    pub fn selected(&self) -> Option<&InputField> {
        if let Some(idx) = self.active {
            self.inputs.get(idx)
        } else {
            None
        }
    }

    pub fn selected_mut(&mut self) -> Option<&mut InputField> {
        if let Some(idx) = self.active {
            self.inputs.get_mut(idx)
        } else {
            None
        }
    }

    pub fn selected_idx(&self) -> Option<usize> {
        return self.active;
    }

    pub fn select(&mut self, idx: usize) -> Result<()> {
        if idx <= self.inputs.len() {
            self.active = Some(idx);
            Ok(())
        } else {
            bail!("Index {} is out of input bounds", idx)
        }
    }

    pub fn next(&mut self) {
        match &mut self.active {
            Some(idx) => {
                if *idx == self.inputs.len() - 1 {
                    *idx = 0;
                } else {
                    *idx += 1;
                }
            }
            None => self.active = Some(0),
        }
    }

    pub fn prev(&mut self) {
        match &mut self.active {
            Some(idx) => {
                if *idx == 0 {
                    *idx = self.inputs.len() - 1;
                } else {
                    *idx -= 1;
                }
            }
            None => self.active = Some(self.inputs.len() - 1),
        }
    }

    pub fn deselect(&mut self) {
        self.active = None;
    }

    pub fn get(&self, idx: usize) -> Result<&InputField> {
        self.inputs
            .get(idx)
            .ok_or(anyhow::Error::msg(format!("No input object for idx {}", idx)))
    }

    pub fn get_mut(&mut self, idx: usize) -> Result<&mut InputField> {
        self.inputs
            .get_mut(idx)
            .ok_or(anyhow::Error::msg(format!("No input object for idx {}", idx)))
    }
}

#[derive(Debug, Default)]
pub struct InputField {
    pub input: Input,
    pub blacklist: InputBlacklist,
}

#[derive(Debug, Default)]
pub enum InputBlacklist {
    #[default]
    None,
    Money,
    Alphabetic,
    AlphanumericNoSpace,
    Alphanumeric,
    NoSpace,
    Numeric,
}