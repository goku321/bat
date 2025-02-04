use std::collections::HashSet;
use std::str::FromStr;

use crate::errors::*;

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub enum OutputComponent {
    Auto,
    Changes,
    Grid,
    Header,
    Numbers,
    Snip,
    Full,
    Plain,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub enum OutputWrap {
    Character,
    None,
}

impl Default for OutputWrap {
    fn default() -> Self {
        OutputWrap::None
    }
}

impl OutputComponent {
    pub fn components(self, interactive_terminal: bool) -> &'static [OutputComponent] {
        match self {
            OutputComponent::Auto => {
                if interactive_terminal {
                    OutputComponent::Full.components(interactive_terminal)
                } else {
                    OutputComponent::Plain.components(interactive_terminal)
                }
            }
            OutputComponent::Changes => &[OutputComponent::Changes],
            OutputComponent::Grid => &[OutputComponent::Grid],
            OutputComponent::Header => &[OutputComponent::Header],
            OutputComponent::Numbers => &[OutputComponent::Numbers],
            OutputComponent::Snip => &[OutputComponent::Snip],
            OutputComponent::Full => &[
                OutputComponent::Changes,
                OutputComponent::Grid,
                OutputComponent::Header,
                OutputComponent::Numbers,
                OutputComponent::Snip,
            ],
            OutputComponent::Plain => &[],
        }
    }
}

impl FromStr for OutputComponent {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "auto" => Ok(OutputComponent::Auto),
            "changes" => Ok(OutputComponent::Changes),
            "grid" => Ok(OutputComponent::Grid),
            "header" => Ok(OutputComponent::Header),
            "numbers" => Ok(OutputComponent::Numbers),
            "snip" => Ok(OutputComponent::Snip),
            "full" => Ok(OutputComponent::Full),
            "plain" => Ok(OutputComponent::Plain),
            _ => Err(format!("Unknown style '{}'", s).into()),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct OutputComponents(pub HashSet<OutputComponent>);

impl OutputComponents {
    pub fn changes(&self) -> bool {
        self.0.contains(&OutputComponent::Changes)
    }

    pub fn grid(&self) -> bool {
        self.0.contains(&OutputComponent::Grid)
    }

    pub fn header(&self) -> bool {
        self.0.contains(&OutputComponent::Header)
    }

    pub fn numbers(&self) -> bool {
        self.0.contains(&OutputComponent::Numbers)
    }

    pub fn snip(&self) -> bool {
        self.0.contains(&OutputComponent::Snip)
    }

    pub fn plain(&self) -> bool {
        self.0.iter().all(|c| c == &OutputComponent::Plain)
    }
}
