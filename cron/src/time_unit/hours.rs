use std::{borrow::Cow, sync::LazyLock};

use crate::{
    ordinal::{Ordinal, OrdinalSet},
    time_unit::TimeUnitField,
};

static ALL: LazyLock<OrdinalSet> = LazyLock::new(Hours::supported_ordinals);

#[derive(Clone, Debug, Eq)]
pub struct Hours {
    ordinals: Option<OrdinalSet>,
}

impl TimeUnitField for Hours {
    fn from_optional_ordinal_set(ordinal_set: Option<OrdinalSet>) -> Self {
        Hours {
            ordinals: ordinal_set,
        }
    }
    fn name() -> Cow<'static, str> {
        Cow::from("Hours")
    }
    fn inclusive_min() -> Ordinal {
        0
    }
    fn inclusive_max() -> Ordinal {
        23
    }
    fn ordinals(&self) -> &OrdinalSet {
        match &self.ordinals {
            Some(ordinal_set) => ordinal_set,
            None => &ALL,
        }
    }
}

impl PartialEq for Hours {
    fn eq(&self, other: &Hours) -> bool {
        self.ordinals() == other.ordinals()
    }
}
