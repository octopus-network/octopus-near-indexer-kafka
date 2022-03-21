use crate::models::analysis::blocks::Block;
use crate::models::analysis::chunks::Chunk;

use serde::{Deserialize, Serialize};

pub mod blocks;
pub mod chunks;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Analysis {
    pub analysis_blocks: Block,
    pub analysis_chunks: Vec<Chunk>,
}
