use std::{borrow::Cow, sync::LazyLock};

use crate::{
    ordinal::{Ordinal, OrdinalSet},
    time_unit::TimeUnitField,
};

static ALL: LazyLock<OrdinalSet> = LazyLock::new(Minutes::supported_ordinals);

#[derive(Clone, Debug, Eq)]
pub struct Minutes {
    ordinals: Option<OrdinalSet>,
}

impl TimeUnitField for Minutes {
    fn from_optional_ordinal_set(ordinal_set: Option<OrdinalSet>) -> Self {
        Minutes {
            ordinals: ordinal_set,
        }
    }
    fn name() -> Cow<'static, str> {
        Cow::from("Minutes")
    }
    fn inclusive_min() -> Ordinal {
        0
    }
    fn inclusive_max() -> Ordinal {
        59
    }
    fn ordinals(&self) -> &OrdinalSet {
        match &self.ordinals {
            Some(ordinal_set) => ordinal_set,
            None => &ALL,
        }
    }
}

impl PartialEq for Minutes {
    fn eq(&self, other: &Minutes) -> bool {
        self.ordinals() == other.ordinals()
    }
}
