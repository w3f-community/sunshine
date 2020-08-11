use ipld_block_builder::{derive_cache, Codec, IpldCache};
use libipld::store::Store;
use std::sync::Arc;
use substrate_subxt::balances::{AccountData, Balances};
use substrate_subxt::sp_runtime::traits::{IdentifyAccount, Verify};
use substrate_subxt::system::System;
use substrate_subxt::{extrinsic, sp_core, sp_runtime};
use sunshine_bounty_client::{
    bank::Bank, bounty::Bounty, donate::Donate, org::Org, vote::Vote, TextBlock,
};
use sunshine_client_utils::cid::CidBytes;
use sunshine_client_utils::client::{GenericClient, KeystoreImpl, OffchainStoreImpl};
use sunshine_client_utils::crypto::keychain::KeyType;
use sunshine_client_utils::crypto::sr25519;
use sunshine_client_utils::node::{
    ChainSpecError, Configuration, NodeConfig, RpcHandlers, ScServiceError, TaskManager,
};
use sunshine_faucet_client::Faucet;
use sunshine_identity_client::{Claim, Identity};

pub use sunshine_bounty_client::*;
pub use sunshine_client_utils as client;
pub use sunshine_faucet_client as faucet;
pub use sunshine_identity_client as identity;

pub type AccountId = <<sp_runtime::MultiSignature as Verify>::Signer as IdentifyAccount>::AccountId;
pub type Uid = u32;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Runtime;

impl System for Runtime {
    type Index = u32;
    type BlockNumber = u32;
    type Hash = sp_core::H256;
    type Hashing = sp_runtime::traits::BlakeTwo256;
    type AccountId = AccountId;
    type Address = AccountId;
    type Header = sp_runtime::generic::Header<Self::BlockNumber, Self::Hashing>;
    type Extrinsic = sp_runtime::OpaqueExtrinsic;
    type AccountData = AccountData<u128>;
}

impl Balances for Runtime {
    type Balance = u128;
}

impl Faucet for Runtime {}

impl Identity for Runtime {
    type Uid = Uid;
    type Cid = CidBytes;
    type Mask = [u8; 32];
    type Gen = u16;
    type IdAccountData = AccountData<<Self as Balances>::Balance>;
}

impl Org for Runtime {
    type IpfsReference = CidBytes;
    type OrgId = u64;
    type Shares = u64;
    type Constitution = TextBlock;
}

impl Vote for Runtime {
    type VoteId = u64;
    type Signal = u64;
    type Percent = sp_runtime::Permill;
    type VoteTopic = TextBlock;
    type VoterView = utils::vote::VoterView;
    type VoteJustification = TextBlock;
}

impl Donate for Runtime {}

impl Bank for Runtime {
    type BankId = u64;
    type SpendId = u64;
}

impl Bounty for Runtime {
    type IpfsReference = CidBytes;
    type BountyId = u64;
    type BountyPost = BountyBody;
    type SubmissionId = u64;
    type BountySubmission = BountyBody;
}

impl substrate_subxt::Runtime for Runtime {
    type Signature = sp_runtime::MultiSignature;
    type Extra = extrinsic::DefaultExtra<Self>;
}

pub struct OffchainClient<S> {
    claims: IpldCache<S, Codec, Claim>,
    bounties: IpldCache<S, Codec, BountyBody>,
    texts: IpldCache<S, Codec, TextBlock>,
}

impl<S: Store> OffchainClient<S> {
    pub fn new(store: S) -> Self {
        Self {
            claims: IpldCache::new(store.clone(), Codec::new(), 64),
            bounties: IpldCache::new(store.clone(), Codec::new(), 64),
            texts: IpldCache::new(store, Codec::new(), 64),
        }
    }
}

derive_cache!(OffchainClient, claims, Codec, Claim);
derive_cache!(OffchainClient, bounties, Codec, BountyBody);
derive_cache!(OffchainClient, texts, Codec, TextBlock);

impl<S: Store> From<S> for OffchainClient<S> {
    fn from(store: S) -> Self {
        Self::new(store)
    }
}

pub struct Node;

impl NodeConfig for Node {
    type ChainSpec = sunshine_node::chain_spec::ChainSpec;
    type Runtime = Runtime;

    fn impl_name() -> &'static str {
        sunshine_node::IMPL_NAME
    }

    fn impl_version() -> &'static str {
        sunshine_node::IMPL_VERSION
    }

    fn author() -> &'static str {
        sunshine_node::AUTHOR
    }

    fn copyright_start_year() -> i32 {
        sunshine_node::COPYRIGHT_START_YEAR
    }

    fn chain_spec_dev() -> Self::ChainSpec {
        sunshine_node::chain_spec::dev_chain_spec()
    }

    fn chain_spec_from_json_bytes(json: Vec<u8>) -> Result<Self::ChainSpec, ChainSpecError> {
        Self::ChainSpec::from_json_bytes(json).map_err(ChainSpecError)
    }

    fn new_light(config: Configuration) -> Result<(TaskManager, Arc<RpcHandlers>), ScServiceError> {
        Ok(sunshine_node::service::new_light(config)?)
    }

    fn new_full(config: Configuration) -> Result<(TaskManager, Arc<RpcHandlers>), ScServiceError> {
        Ok(sunshine_node::service::new_full(config)?)
    }
}

pub struct UserDevice;

impl KeyType for UserDevice {
    const KEY_TYPE: u8 = 0;
    type Pair = sr25519::Pair;
}

pub type Client =
    GenericClient<Node, UserDevice, KeystoreImpl<UserDevice>, OffchainClient<OffchainStoreImpl>>;

#[cfg(feature = "mock")]
pub mod mock {
    use super::*;
    use sunshine_client_utils::mock::{self, build_test_node, OffchainStoreImpl};
    pub use sunshine_client_utils::mock::{AccountKeyring, TempDir, TestNode};

    pub type Client = GenericClient<
        Node,
        UserDevice,
        mock::KeystoreImpl<UserDevice>,
        OffchainClient<OffchainStoreImpl>,
    >;

    pub type ClientWithKeystore = GenericClient<
        Node,
        UserDevice,
        KeystoreImpl<UserDevice>,
        OffchainClient<OffchainStoreImpl>,
    >;

    pub fn test_node() -> (TestNode, TempDir) {
        build_test_node::<Node>()
    }
}
