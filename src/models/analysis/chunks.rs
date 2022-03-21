use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Chunk {
    pub included_in_block_hash: String,
    pub chunk_hash: String,
    pub shard_id: BigDecimal,
    pub signature: String,
    pub gas_limit: BigDecimal,
    pub gas_used: BigDecimal,
    pub author_account_id: String,
}

impl Chunk {
    pub fn from_chunk_view(
        chunk_view: &near_indexer::IndexerChunkView,
        block_hash: &near_indexer::near_primitives::hash::CryptoHash,
    ) -> Self {
        Self {
            included_in_block_hash: block_hash.to_string(),
            chunk_hash: chunk_view.header.chunk_hash.to_string(),
            shard_id: chunk_view.header.shard_id.into(),
            signature: chunk_view.header.signature.to_string(),
            gas_limit: chunk_view.header.gas_limit.into(),
            gas_used: chunk_view.header.gas_used.into(),
            author_account_id: chunk_view.author.to_string(),
        }
    }

    pub fn from(
        shards: &[near_indexer::IndexerShard],
        block_hash: &near_indexer::near_primitives::hash::CryptoHash,
    ) -> Vec<Chunk> {
        if shards.is_empty() {
            return vec![];
        }
        let chunk_models: Vec<Chunk> = shards
            .iter()
            .filter_map(|shard| shard.chunk.as_ref())
            .map(|chunk| Chunk::from_chunk_view(chunk, block_hash))
            .collect();

        if chunk_models.is_empty() {
            return vec![];
        }
        chunk_models
    }
}
