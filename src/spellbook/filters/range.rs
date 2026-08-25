use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use tsify::Tsify;

use crate::spell::{Distance, RangeType};

#[derive(Tsify, Serialize, Deserialize)]
pub struct RangeFilter {
    range_types: HashSet<RangeType>,
    distance: Option<Distance>,
}
