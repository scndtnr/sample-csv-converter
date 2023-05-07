use anyhow::Result;

use crate::commands::{config::load_config, convert_to_path};

pub(crate) fn generate_csv(src: String, dst: String, dry_run: bool) -> Result<()> {
    let cfg = load_config();
    let src_path = convert_to_path(&src);
    let dst_path = convert_to_path(&dst);
    todo!()
}
