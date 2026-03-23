#[cfg(feature = "chrono")]
mod chrono;

mod macros;
pub(crate) mod bind;

use crate::impl_sql_for_character;
impl_sql_for_character!(char);
