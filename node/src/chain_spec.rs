use hex_literal::hex;
use sc_service::{ChainType, config::MultiaddrWithPeerId};
//use sc_telemetry::TelemetryEndpoints;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::crypto::UncheckedInto;
use sp_core::{sr25519, Pair, Public};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::traits::{IdentifyAccount, Verify};
use std::path::PathBuf;
use std::str::FromStr;
use sunshine_runtime::{
    AccountId, AuraConfig, BalancesConfig, GenesisConfig, GrandpaConfig, Signature, SystemConfig,
    WASM_BINARY,
};

//const STAGING_TELEMETRY_URL: &str = "wss://telemetry.getsunshine.com/submit/";

pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

fn seed_to_public<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
    TPublic::Pair::from_string(&seed, None).unwrap().public()
}

fn seed_to_account_id<TPublic: Public>(seed: &str) -> AccountId
where
    <TPublic::Pair as Pair>::Public: Into<<Signature as Verify>::Signer>,
{
    seed_to_public::<TPublic>(seed).into().into_account()
}

fn seed_to_authority_keys(seed: &str) -> (AuraId, GrandpaId) {
    (
        seed_to_public::<AuraId>(seed),
        seed_to_public::<GrandpaId>(seed),
    )
}

#[derive(Clone, Debug)]
pub enum Chain {
    Dev,
    Local,
    Staging,
    Json(PathBuf),
}

impl Chain {
    pub fn to_chain_spec(self) -> Result<ChainSpec, String> {
        Ok(match self {
            Self::Dev => dev_chain_spec(),
            Self::Local => local_chain_spec(),
            Self::Staging => staging_chain_spec(),
            Self::Json(path) => ChainSpec::from_json_file(path)?,
        })
    }
}

impl FromStr for Chain {
    type Err = String;

    fn from_str(chain: &str) -> Result<Self, Self::Err> {
        Ok(match chain {
            "dev" => Chain::Dev,
            "local" => Chain::Local,
            "" | "staging" => Chain::Staging,
            path => Chain::Json(PathBuf::from(path)),
        })
    }
}

pub fn dev_chain_spec() -> ChainSpec {
    ChainSpec::from_genesis(
        "Development",
        "dev",
        ChainType::Development,
        || {
            testnet_genesis(
                &[seed_to_authority_keys("//Alice")],
                &[
                    seed_to_account_id::<sr25519::Public>("//Alice"),
                    seed_to_account_id::<sr25519::Public>("//Alice//stash"),
                ],
            )
        },
        vec![],
        None,
        None,
        None,
        None,
    )
}

pub fn local_chain_spec() -> ChainSpec {
    ChainSpec::from_genesis(
        "Local Testnet",
        "local-testnet",
        ChainType::Local,
        || {
            testnet_genesis(
                &[
                    seed_to_authority_keys("//Alice"),
                    seed_to_authority_keys("//Bob"),
                ],
                &[
                    seed_to_account_id::<sr25519::Public>("//Alice"),
                    seed_to_account_id::<sr25519::Public>("//Alice//stash"),
                    seed_to_account_id::<sr25519::Public>("//Bob"),
                    seed_to_account_id::<sr25519::Public>("//Bob//stash"),
                ],
            )
        },
        vec![],
        None,
        None,
        None,
        None,
    )
}

fn staging_chain_spec_genesis() -> GenesisConfig {
    // subkey inspect "$secret"/sunshine/1-3/controller
    let controller = &[
        // 5FCsDTobbck9vtfqGnyYbqbFw5YrtWyK6FaVZ1Sq2KWCVLit
        hex!["8aee2acc755ee0a3e161db53b03fd988b0c8e00d8c09dbc4edd22b4523eeb868"],
        // 5EjhaW3GzQ3d8JrbWv5iZMq6RDMssPxQLJZ3qQTPSgKF8Jxb
        hex!["7636196df2d9e3d998ee88b665b1b5d6997f9d26a6bbe1e4ee594ca984ac0c0b"],
        // 5F2AHzWa1V2R2X3Pk6Sq7RG9dqH5AV1FrZ15RS5P4m6ZAYmt
        hex!["82c3f52d3eb6ce05233343f4a0c9096b03ba59d0826751095e3cdce00ff8cb41"],
    ];

    // subkey inspect "$secret"/sunshine/1-3/stash
    let _stash = &[
        // 5FW5PEWgMtwGwnTPgDs5dC2a3b2cMPDhLX5P79kjs7aL7VxW
        hex!["980e4d3fb16f722e5f9af556fd4d0570d8012203db5de2b2c106b21590b26358"],
        // 5CJ6TDnS9uGDZ3MuDdZkFDGgFuRWiWAifvnhUEuChzXHR9wv
        hex!["0a364117f31bc06fa0493539021da6e7322e011741712f0cc1f52600281e8603"],
        // 5EUhjzQZ2wJ7u31kMTuvzcbu2gN9xgvE9a7CPqxBUD5NyB6r
        hex!["6ac5f81125be4b074e3566cb00ae99890a8c9d5dbeef74305a6f7680d49ba923"],
    ];

    // subkey --ed25519 inspect "$secret"//sunshine//1-3/session
    let session = &[
        // 5DUiDNXtr9WWQ6Sg9cpd5XDfbLRMXV9RQ3SfyEMsMj4yeB4Q
        hex!["3e8b532432f03543a7bd6ceaccc6469cdcce0d996728a2f84b9b76cec3ec66b9"],
        // 5EX52w4Rzi66uPeXnz9kFVL7zva3bX3byM2MRs6exJTwNxXn
        hex!["6c9422aca5f4cbbc8b38bae94d01a43443a3152b1c3e87c0fbde2ad3a473de35"],
        // 5CZp81EdMJLjCyEVccuF2DDfXfb6vUgdQTfT5fQvdP9XjyF9
        hex!["163334629ed454020ca7068329cf35064bab7e8cf4a60f76beff637c0817b5bd"],
    ];

    testnet_genesis(
        &[
            (controller[0].unchecked_into(), session[0].unchecked_into()),
            (controller[1].unchecked_into(), session[1].unchecked_into()),
            (controller[2].unchecked_into(), session[2].unchecked_into()),
        ],
        &[],
    )
}

pub fn staging_chain_spec() -> ChainSpec {
    let boot_nodes: Vec<_> = [
        "12D3KooWAhftS4ujcxgJDoEaJ8hFaQTuc4Vk3jsthP2fBbh9tc8f",
        "12D3KooWK2b6aJsBMkg3JRn4PbCZBXaGcB9mA1YtqQ7ZWpqg3cmv",
        "12D3KooWRCioHfKYchRJAhd5ZEaZwVMYTNuNG7JDCHjGa3ozxS4M",
    ]
    .iter()
    .map(|peer_id| {
        MultiaddrWithPeerId {
            peer_id: peer_id.parse().unwrap(),
            multiaddr: "/ip4/127.0.0.1".parse().unwrap(),
        }
    })
    .collect();
    ChainSpec::from_genesis(
        "Staging Testnet",
        "staging-testnet",
        ChainType::Live,
        staging_chain_spec_genesis,
        boot_nodes,
        //Some(TelemetryEndpoints::new(vec![(STAGING_TELEMETRY_URL.to_string(), 0)]).unwrap()),
        None,
        None,
        None,
        None,
    )
}

fn testnet_genesis(
    initial_authorities: &[(AuraId, GrandpaId)],
    endowed_accounts: &[AccountId],
) -> GenesisConfig {
    GenesisConfig {
        frame_system: Some(SystemConfig {
            code: WASM_BINARY.to_vec(),
            changes_trie_config: Default::default(),
        }),
        pallet_balances: Some(BalancesConfig {
            balances: endowed_accounts
                .iter()
                .cloned()
                .map(|k| (k, 1 << 60))
                .collect(),
        }),
        pallet_aura: Some(AuraConfig {
            authorities: initial_authorities.iter().map(|x| (x.0.clone())).collect(),
        }),
        pallet_grandpa: Some(GrandpaConfig {
            authorities: initial_authorities
                .iter()
                .map(|x| (x.1.clone(), 1))
                .collect(),
        }),
    }
}
