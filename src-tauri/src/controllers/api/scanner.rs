use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::utils::fs::scan_medias;

use super::{ApiResult, FromArgs, Response};

#[derive(Deserialize, Serialize)]
struct IndexArgs {
    root_path: String,
    allowed_mimes: Vec<String>,
}
impl FromArgs for IndexArgs {}
pub fn index(args: Value) -> ApiResult {
    let args = IndexArgs::from_args(args)?;
    Response::ok(scan_medias(args.root_path, args.allowed_mimes))
}
