// Copyright 2020-2024 Manta Network.
// This file is part of Manta.
//
// Manta is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Manta is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Manta.  If not, see <http://www.gnu.org/licenses/>.

//! Chain-Specific Command Line Interfaces

use crate::{
    chain_specs,
    cli::{Cli, RelayChainCli, Subcommand},
    rpc::{create_calamari_full, create_manta_full},
    service::new_partial,
};
use cumulus_primitives_core::ParaId;
use frame_benchmarking_cli::{BenchmarkCmd, SUBSTRATE_REFERENCE_HARDWARE};
use log::info;
use manta_primitives::types::Header;
use sc_cli::{
    ChainSpec, CliConfiguration, DefaultConfigurationValues, ImportParams, KeystoreParams,
    NetworkParams, SharedParams, SubstrateCli,
};
use sc_service::config::{BasePath, PrometheusConfig};
use sp_runtime::{generic, traits::AccountIdConversion, OpaqueExtrinsic};
use std::net::SocketAddr;

use calamari_runtime::RuntimeApi as CalamariRuntimeApi;
use manta_runtime::RuntimeApi as MantaRuntimeApi;

pub use sc_cli::Error;

/// Result Type Alias with default [`Error`] Type
pub type Result<T = (), E = Error> = core::result::Result<T, E>;

/// Block Type
pub type Block = generic::Block<Header, OpaqueExtrinsic>;

/// Manta Parachain ID
pub const MANTA_PARACHAIN_ID: u32 = 2104;

/// Calamari Parachain ID
pub const CALAMARI_PARACHAIN_ID: u32 = 2084;

trait IdentifyChain {
    fn is_manta(&self) -> bool;
    fn is_calamari(&self) -> bool;
    fn is_localdev(&self) -> bool;
}

impl IdentifyChain for dyn sc_service::ChainSpec {
    fn is_manta(&self) -> bool {
        self.id().starts_with("manta")
    }
    fn is_calamari(&self) -> bool {
        self.id().starts_with("calamari")
    }
    fn is_localdev(&self) -> bool {
        self.id().ends_with("localdev")
    }
}

impl<T: sc_service::ChainSpec + 'static> IdentifyChain for T {
    fn is_manta(&self) -> bool {
        <dyn sc_service::ChainSpec>::is_manta(self)
    }
    fn is_calamari(&self) -> bool {
        <dyn sc_service::ChainSpec>::is_calamari(self)
    }
    fn is_localdev(&self) -> bool {
        <dyn sc_service::ChainSpec>::is_localdev(self)
    }
}

fn load_spec(id: &str) -> Result<Box<dyn sc_service::ChainSpec>, String> {
    match id {
        // manta chainspec
        "manta-dev" => Ok(Box::new(chain_specs::manta_development_config())),
        "manta-local" => Ok(Box::new(chain_specs::manta_local_config(false))),
        "manta-localdev" => Ok(Box::new(chain_specs::manta_local_config(true))),
        "manta-testnet" => Ok(Box::new(chain_specs::manta_testnet_config())),
        "manta" => Ok(Box::new(chain_specs::manta_mainnet_config()?)),
        // calamari chainspec
        "calamari-dev" => Ok(Box::new(chain_specs::calamari_development_config())),
        "calamari-local" => Ok(Box::new(chain_specs::calamari_local_config(false))),
        "calamari-localdev" => Ok(Box::new(chain_specs::calamari_local_config(true))),
        "calamari-testnet" => Ok(Box::new(chain_specs::calamari_testnet_config()?)),
        "calamari" => Ok(Box::new(chain_specs::calamari_config()?)),
        path => {
            let chain_spec = chain_specs::ChainSpec::from_json_file(path.into())?;
            if chain_spec.is_manta() {
                Ok(Box::new(chain_specs::MantaChainSpec::from_json_file(
                    path.into(),
                )?))
            } else if chain_spec.is_calamari() {
                Ok(Box::new(chain_specs::CalamariChainSpec::from_json_file(
                    path.into(),
                )?))
            } else {
                Err("Please input a file name starting with manta, calamari.".into())
            }
        }
    }
}

impl SubstrateCli for Cli {
    fn impl_name() -> String {
        "Manta/Calamari Collator".into()
    }

    fn impl_version() -> String {
        env!("SUBSTRATE_CLI_IMPL_VERSION").into()
    }

    fn description() -> String {
        format!(
            "Manta/Calamari Collator\n\nThe command-line arguments provided first will be \
		passed to the parachain node, while the arguments provided after -- will be passed \
		to the relaychain node.\n\n\
		{} [parachain-args] -- [relaychain-args]",
            Self::executable_name()
        )
    }

    fn author() -> String {
        env!("CARGO_PKG_AUTHORS").into()
    }

    fn support_url() -> String {
        "https://github.com/Manta-Network/Manta/issues/new".into()
    }

    fn copyright_start_year() -> i32 {
        2020
    }

    fn load_spec(&self, id: &str) -> Result<Box<dyn sc_service::ChainSpec>, String> {
        load_spec(id)
    }
}

impl SubstrateCli for RelayChainCli {
    fn impl_name() -> String {
        "Manta/Calamari Collator".into()
    }

    fn impl_version() -> String {
        env!("SUBSTRATE_CLI_IMPL_VERSION").into()
    }

    fn description() -> String {
        format!(
            "Manta/Calamari collator\n\nThe command-line arguments provided first will be \
		passed to the parachain node, while the arguments provided after -- will be passed \
		to the relaychain node.\n\n\
		{} [parachain-args] -- [relaychain-args]",
            Self::executable_name()
        )
    }

    fn author() -> String {
        env!("CARGO_PKG_AUTHORS").into()
    }

    fn support_url() -> String {
        "https://github.com/Manta-Network/Manta/issues/new".into()
    }

    fn copyright_start_year() -> i32 {
        2020
    }

    fn load_spec(&self, id: &str) -> Result<Box<dyn sc_service::ChainSpec>, String> {
        polkadot_cli::Cli::from_iter([RelayChainCli::executable_name()].iter()).load_spec(id)
    }
}

/// Creates partial components for the runtimes that are supported by the benchmarks.
macro_rules! construct_benchmark_partials {
    ($config:expr, |$partials:ident| $code:expr) => {
        if $config.chain_spec.is_manta() {
            let $partials =
                new_partial::<MantaRuntimeApi>(&$config, $config.chain_spec.is_localdev())?;
            $code
        } else if $config.chain_spec.is_calamari() {
            let $partials =
                new_partial::<CalamariRuntimeApi>(&$config, $config.chain_spec.is_localdev())?;
            $code
        } else {
            Err("The chain is not supported".into())
        }
    };
}

macro_rules! construct_async_run {
    (|$components:ident, $cli:ident, $cmd:ident, $config:ident| $( $code:tt )* ) => {{
        let runner = $cli.create_runner($cmd)?;
            if runner.config().chain_spec.is_manta() {
                runner.async_run(|$config| {
                    let $components = crate::service::new_partial::<MantaRuntimeApi>(
                        &$config,
                        $config.chain_spec.is_localdev(),
                    )?;
                    let task_manager = $components.task_manager;
                    { $( $code )* }.map(|v| (v, task_manager))
                })
            } else if runner.config().chain_spec.is_calamari() {
                runner.async_run(|$config| {
                    let $components = new_partial::<CalamariRuntimeApi>(
                        &$config,
                        $config.chain_spec.is_localdev(),
                    )?;
                    let task_manager = $components.task_manager;
                    { $( $code )* }.map(|v| (v, task_manager))
                })
            } else {
                panic!("wrong chain spec, must be one of manta, calamari chain specs");
            }
    }}
}

/// Parse command line arguments into service configuration.
pub fn run_with(cli: Cli) -> Result {
    match &cli.subcommand {
        Some(Subcommand::BuildSpec(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.sync_run(|config| cmd.run(config.chain_spec, config.network))
        }
        Some(Subcommand::CheckBlock(cmd)) => {
            construct_async_run!(|components, cli, cmd, config| {
                Ok(cmd.run(components.client, components.import_queue))
            })
        }
        Some(Subcommand::ExportBlocks(cmd)) => {
            construct_async_run!(|components, cli, cmd, config| {
                Ok(cmd.run(components.client, config.database))
            })
        }
        Some(Subcommand::ExportState(cmd)) => {
            construct_async_run!(|components, cli, cmd, config| {
                Ok(cmd.run(components.client, config.chain_spec))
            })
        }
        Some(Subcommand::ImportBlocks(cmd)) => {
            construct_async_run!(|components, cli, cmd, config| {
                Ok(cmd.run(components.client, components.import_queue))
            })
        }
        Some(Subcommand::Revert(cmd)) => {
            construct_async_run!(|components, cli, cmd, config| {
                Ok(cmd.run(components.client, components.backend, None))
            })
        }
        Some(Subcommand::PurgeChain(cmd)) => {
            let runner = cli.create_runner(cmd)?;

            runner.sync_run(|config| {
                let polkadot_cli = RelayChainCli::new(
                    &config,
                    [RelayChainCli::executable_name()]
                        .iter()
                        .chain(cli.relaychain_args.iter()),
                );

                let polkadot_config = SubstrateCli::create_configuration(
                    &polkadot_cli,
                    &polkadot_cli,
                    config.tokio_handle.clone(),
                )
                .map_err(|err| format!("Relay chain argument error: {err}"))?;

                cmd.run(config, polkadot_config)
            })
        }
        Some(Subcommand::ExportGenesisHead(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.sync_run(|config| {
                if config.chain_spec.is_manta() {
                    let partials =
                        new_partial::<MantaRuntimeApi>(&config, config.chain_spec.is_localdev())?;
                    cmd.run(partials.client)
                } else if config.chain_spec.is_calamari() {
                    let partials = new_partial::<CalamariRuntimeApi>(
                        &config,
                        config.chain_spec.is_localdev(),
                    )?;
                    cmd.run(partials.client)
                } else {
                    Err("Must be either calamari or manta runtime".into())
                }
            })
        }
        Some(Subcommand::ExportGenesisWasm(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.sync_run(|_config| {
                let spec = cli.load_spec(&cmd.shared_params.chain.clone().unwrap_or_default())?;
                cmd.run(&*spec)
            })
        }
        Some(Subcommand::Benchmark(cmd)) => {
            let runner = cli.create_runner(cmd)?;

            // Switch on the concrete benchmark sub-command
            match cmd {
                BenchmarkCmd::Pallet(cmd) => {
                    if cfg!(feature = "runtime-benchmarks") {
                        runner.sync_run(|config| cmd.run::<Block, ()>(config))
                    } else {
                        Err("Benchmarking wasn't enabled when building the node. \
				You can enable it with `--features runtime-benchmarks`."
                            .into())
                    }
                }
                BenchmarkCmd::Block(cmd) => runner.sync_run(|config| {
                    construct_benchmark_partials!(config, |partials| cmd.run(partials.client))
                }),
                #[cfg(not(feature = "runtime-benchmarks"))]
                BenchmarkCmd::Storage(_) => Err(
                    "Storage benchmarking can be enabled with `--features runtime-benchmarks`."
                        .into(),
                ),
                #[cfg(feature = "runtime-benchmarks")]
                BenchmarkCmd::Storage(cmd) => runner.sync_run(|config| {
                    construct_benchmark_partials!(config, |partials| {
                        let db = partials.backend.expose_db();
                        let storage = partials.backend.expose_storage();

                        cmd.run(config, partials.client.clone(), db, storage)
                    })
                }),
                BenchmarkCmd::Extrinsic(_) => Err("Unsupported benchmarking command".into()),
                BenchmarkCmd::Overhead(_) => Err("Unsupported benchmarking command".into()),
                BenchmarkCmd::Machine(cmd) => {
                    runner.sync_run(|config| cmd.run(&config, SUBSTRATE_REFERENCE_HARDWARE.clone()))
                }
            }
        }
        #[cfg(feature = "try-runtime")]
        Some(Subcommand::TryRuntime(_)) => Err(try_runtime_cli::DEPRECATION_NOTICE.into()),
        #[cfg(not(feature = "try-runtime"))]
        Some(Subcommand::TryRuntime) => Err("Try-runtime wasn't enabled when building the node. \
		You can enable it with `--features try-runtime`."
            .into()),
        None => {
            let runner = cli.create_runner(&cli.run.normalize())?;
            let chain_spec = &runner.config().chain_spec;
            info!("id:{}", chain_spec.id());
            let collator_options = cli.run.collator_options();

            runner.run_node_until_exit(|config| async move {
                let para_id = crate::chain_specs::Extensions::try_get(&*config.chain_spec)
                    .map(|e| e.para_id)
                    .ok_or("Could not find parachain extension in chain-spec.")?;

                let polkadot_cli = RelayChainCli::new(
                    &config,
                    [RelayChainCli::executable_name()]
                        .iter()
                        .chain(cli.relaychain_args.iter()),
                );

                let id = ParaId::from(para_id);

                let parachain_account =
                    AccountIdConversion::<polkadot_primitives::AccountId>::into_account_truncating(
                        &id,
                    );

                let tokio_handle = config.tokio_handle.clone();

                info!("Parachain id: {:?}", id);
                info!("Parachain Account: {}", parachain_account);
                info!(
                    "Is collating: {}",
                    if config.role.is_authority() {
                        "yes"
                    } else {
                        "no"
                    }
                );

                // hard code async backing off
                let async_backing = false;
                let is_localdev = config.chain_spec.is_localdev();

                if config.chain_spec.is_manta() {
                    if is_localdev {
                        crate::service::start_dev_node::<MantaRuntimeApi, _>(
                            config,
                            create_manta_full,
                        )
                        .await
                        .map(|r| r.0)
                        .map_err(Into::into)
                    } else {
                        let polkadot_config = SubstrateCli::create_configuration(
                            &polkadot_cli,
                            &polkadot_cli,
                            tokio_handle,
                        )
                        .map_err(|err| format!("Relay chain argument error: {err}"))?;

                        crate::service::start_parachain_node::<MantaRuntimeApi, _>(
                            config,
                            polkadot_config,
                            collator_options,
                            id,
                            create_manta_full,
                            cli.block_authoring_duration,
                            async_backing,
                        )
                        .await
                        .map(|r| r.0)
                        .map_err(Into::into)
                    }
                } else if config.chain_spec.is_calamari() {
                    if is_localdev {
                        crate::service::start_dev_node::<CalamariRuntimeApi, _>(
                            config,
                            create_calamari_full,
                        )
                        .await
                        .map(|r| r.0)
                        .map_err(Into::into)
                    } else {
                        let polkadot_config = SubstrateCli::create_configuration(
                            &polkadot_cli,
                            &polkadot_cli,
                            tokio_handle,
                        )
                        .map_err(|err| format!("Relay chain argument error: {err}"))?;

                        crate::service::start_parachain_node::<CalamariRuntimeApi, _>(
                            config,
                            polkadot_config,
                            collator_options,
                            id,
                            create_calamari_full,
                            cli.block_authoring_duration,
                            async_backing,
                        )
                        .await
                        .map(|r| r.0)
                        .map_err(Into::into)
                    }
                } else {
                    Err("chain spec error: must be one of manta or calamari chain specs".into())
                }
            })
        }
    }
}

/// Parse command line arguments into service configuration.
pub fn run() -> Result {
    run_with(Cli::from_args())
}

impl DefaultConfigurationValues for RelayChainCli {
    fn p2p_listen_port() -> u16 {
        30334
    }

    fn rpc_listen_port() -> u16 {
        9945
    }

    fn prometheus_listen_port() -> u16 {
        9616
    }
}

impl CliConfiguration<Self> for RelayChainCli {
    fn shared_params(&self) -> &SharedParams {
        self.base.base.shared_params()
    }

    fn import_params(&self) -> Option<&ImportParams> {
        self.base.base.import_params()
    }

    fn network_params(&self) -> Option<&NetworkParams> {
        self.base.base.network_params()
    }

    fn keystore_params(&self) -> Option<&KeystoreParams> {
        self.base.base.keystore_params()
    }

    fn base_path(&self) -> Result<Option<BasePath>> {
        Ok(self
            .shared_params()
            .base_path()?
            .or_else(|| self.base_path.clone().map(Into::into)))
    }

    fn rpc_addr(&self, default_listen_port: u16) -> Result<Option<SocketAddr>> {
        self.base.base.rpc_addr(default_listen_port)
    }

    fn prometheus_config(
        &self,
        default_listen_port: u16,
        chain_spec: &Box<dyn ChainSpec>,
    ) -> Result<Option<PrometheusConfig>> {
        self.base
            .base
            .prometheus_config(default_listen_port, chain_spec)
    }

    fn init<F>(
        &self,
        _support_url: &String,
        _impl_version: &String,
        _logger_hook: F,
        _config: &sc_service::Configuration,
    ) -> Result<()>
    where
        F: FnOnce(&mut sc_cli::LoggerBuilder, &sc_service::Configuration),
    {
        unreachable!("PolkadotCli is never initialized; qed");
    }

    fn chain_id(&self, is_dev: bool) -> Result<String> {
        let chain_id = self.base.base.chain_id(is_dev)?;

        Ok(if chain_id.is_empty() {
            self.chain_id.clone().unwrap_or_default()
        } else {
            chain_id
        })
    }

    fn role(&self, is_dev: bool) -> Result<sc_service::Role> {
        self.base.base.role(is_dev)
    }

    fn transaction_pool(&self, is_dev: bool) -> Result<sc_service::config::TransactionPoolOptions> {
        self.base.base.transaction_pool(is_dev)
    }

    fn rpc_methods(&self) -> Result<sc_service::config::RpcMethods> {
        self.base.base.rpc_methods()
    }

    fn rpc_max_connections(&self) -> Result<u32> {
        self.base.base.rpc_max_connections()
    }

    fn rpc_cors(&self, is_dev: bool) -> Result<Option<Vec<String>>> {
        self.base.base.rpc_cors(is_dev)
    }

    fn default_heap_pages(&self) -> Result<Option<u64>> {
        self.base.base.default_heap_pages()
    }

    fn force_authoring(&self) -> Result<bool> {
        self.base.base.force_authoring()
    }

    fn disable_grandpa(&self) -> Result<bool> {
        self.base.base.disable_grandpa()
    }

    fn max_runtime_instances(&self) -> Result<Option<usize>> {
        self.base.base.max_runtime_instances()
    }

    fn announce_block(&self) -> Result<bool> {
        self.base.base.announce_block()
    }

    fn telemetry_endpoints(
        &self,
        chain_spec: &Box<dyn ChainSpec>,
    ) -> Result<Option<sc_telemetry::TelemetryEndpoints>> {
        self.base.base.telemetry_endpoints(chain_spec)
    }
}
