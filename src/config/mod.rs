use crate::{
    core::{
        create_genesis_block,
        encoding::{json_decoder::JsonDecoder, json_encoder::JsonEncoder},
        mem_storage::MemStorage,
        BlockHasher, BlockchainConfig, DefaultBlockValidator, DynBlockValidator, DynStorage,
        TxHasher,
    },
    crypto::PrivateKey,
    prelude::*,
};

#[derive(Debug, Clone)]
pub struct Config {
    pub encoding: EncodingConfig,
    pub hashers: HasherConfig,
    pub storage: DynStorage,
    pub block_validator: DynBlockValidator,
    pub genesis_block: Block,
    pub block_time_ms: u64,
}

impl Default for Config {
    fn default() -> Self {
        let encoding = EncodingConfig {
            encoder: Box::new(JsonEncoder),
            decoder: Box::new(JsonDecoder),
        };

        let hashers = HasherConfig {
            tx_hasher: Box::new(TxHasher),
            block_hasher: Box::new(BlockHasher::new(encoding.encoder.clone())),
        };

        let storage = Box::new(MemStorage::new());
        let block_validator = Box::new(DefaultBlockValidator {});
        let genesis_block = create_genesis_block();
        let block_time_ms = 1000;

        Self {
            encoding,
            hashers,
            storage,
            block_validator,
            genesis_block,
            block_time_ms,
        }
    }
}

impl Config {
    pub fn node_config(&self) -> NodeConfig {
        NodeConfig {
            encoding: self.encoding.clone(),
            hashers: self.hashers.clone(),
        }
    }

    pub fn blockchain_config(&self) -> BlockchainConfig {
        BlockchainConfig {
            encoding: self.encoding.clone(),
            hashers: self.hashers.clone(),
            storage: self.storage.clone(),
            block_validator: self.block_validator.clone(),
            genesis_block: self.genesis_block.clone(),
        }
    }

    pub fn validator_config(&self, private_key: PrivateKey) -> ValidatorConfig {
        ValidatorConfig {
            encoding: self.encoding.clone(),
            hashers: self.hashers.clone(),
            private_key,
            block_time_ms: self.block_time_ms,
        }
    }

    pub fn encoding_config(&self) -> EncodingConfig {
        self.encoding.clone()
    }
    pub fn hasher_config(&self) -> HasherConfig {
        self.hashers.clone()
    }
}

#[derive(Debug, Clone)]
pub struct ValidatorConfig {
    pub private_key: PrivateKey,
    pub block_time_ms: u64,
    pub encoding: EncodingConfig,
    pub hashers: HasherConfig,
}

#[derive(Debug, Clone)]
pub struct HasherConfig {
    pub tx_hasher: DynHasher<Transaction>,
    pub block_hasher: DynHasher<Block>,
}

#[derive(Debug, Clone)]
pub struct EncodingConfig {
    pub encoder: DynEncoder,
    pub decoder: DynDecoder,
}

#[derive(Debug, Clone)]
pub struct NodeConfig {
    pub encoding: EncodingConfig,
    pub hashers: HasherConfig,
}
