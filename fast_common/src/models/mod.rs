use std::error::Error;
use anyhow::anyhow;
use sea_orm::sea_query::any;

use serde::{Serialize};
use serde::de::DeserializeOwned;

pub mod profile_info;
pub mod dept_info;
