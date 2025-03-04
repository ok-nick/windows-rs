#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "Gaming_Input")]
pub mod Input;
#[cfg(feature = "Gaming_Preview")]
pub mod Preview;
#[cfg(feature = "Gaming_UI")]
pub mod UI;
#[cfg(feature = "Gaming_XboxLive")]
pub mod XboxLive;
