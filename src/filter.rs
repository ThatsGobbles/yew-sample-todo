
use std::fmt::Display;

use serde_derive::Deserialize;
use serde_derive::Serialize;
use strum_macros::EnumIter;
use yew::Href;

use crate::Entry;

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Eq, Hash, EnumIter)]
pub enum Filter {
    All,
    Active,
    Done,
}

impl Display for Filter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            &Self::All => "All",
            &Self::Active => "Active",
            &Self::Done => "Done",
        };

        write!(f, "{}", s)
    }
}

impl Filter {
    pub fn passes(&self, entry: &Entry) -> bool {
        match self {
            &Self::All => true,
            &Self::Active => !entry.completed,
            &Self::Done => entry.completed,
        }
    }
}

impl Default for Filter {
    fn default() -> Self {
        Self::All
    }
}

impl Into<Href> for Filter {
    fn into(self) -> Href {
        match self {
            Filter::All => "#/".into(),
            Filter::Active => "#/active".into(),
            Filter::Done => "#/done".into(),
        }
    }
}
