use std::{str::FromStr, sync::Arc};

use anchor_client::{
    solana_client::rpc_client::RpcClient, solana_sdk::{
        commitment_config::CommitmentConfig,
        pubkey::Pubkey,
        signature::{read_keypair_file, Keypair}, signer::Signer,
    }, Client, Cluster, Program
};
use automata_on_chain_pccs::state::{CertificateAuthority, EnclaveIdentityType, TcbType};

#[cfg(test)]
mod test_pck_certificate;

#[cfg(test)]
mod test_pcs_certificate;

#[cfg(test)]
mod test_enclave_identity;

#[cfg(test)]
mod test_tcb_info;

pub struct PccsTestConfig {
    pub program_id: String,
    pub rpc_url: String,
}

impl Default for PccsTestConfig {
    fn default() -> Self {
        Self {
            program_id: "7UuiyphTDFxz4BTBA8MhwwEHVAt4ttREXEhqdRicaVpA".to_string(),
            rpc_url: "http://localhost:8899".to_string(),
        }
    }
}

pub struct PccsTestHarness {
    program: Program<Arc<Keypair>>,
    _config: PccsTestConfig,
    _rpc_client: RpcClient,
}

impl PccsTestHarness {
    pub fn new(config: PccsTestConfig) -> Self {
        let anchor_wallet =
            std::env::var("ANCHOR_WALLET").expect("ANCHOR_WALLET environment variable not set");
        let payer = read_keypair_file(&anchor_wallet).expect("Failed to read keypair file");

        let payer = Arc::new(payer);

        let client = Client::new_with_options(
            Cluster::Custom(config.rpc_url.clone(), config.rpc_url.clone()),
            payer,
            CommitmentConfig::confirmed(),
        );

        let program_id = Pubkey::from_str(&config.program_id).expect("Invalid program ID");

        let program = client
            .program(program_id)
            .expect("Failed to create program client");

        let rpc_client =
            RpcClient::new_with_commitment(config.rpc_url.clone(), CommitmentConfig::confirmed());

        Self {
            program,
            _config: config,
            _rpc_client: rpc_client,
        }
    }

    pub fn upsert_pck_certificate(
        &self,
        qe_id: String,
        pce_id: String,
        tcbm: String,
        data_buffer_pubkey: Pubkey,
    ) -> anyhow::Result<()> {

        let pck_certificate_pda = Pubkey::find_program_address(
            &[b"pck_cert", &qe_id.as_bytes()[..8], &pce_id.as_bytes()[..2], &tcbm.as_bytes()[..8]],
            &self.program.id()
        );
        let tx = self
            .program
            .request()
            .accounts(automata_on_chain_pccs::accounts::UpsertPckCertificate {
                authority: self.program.payer(),
                pck_certificate: pck_certificate_pda.0,
                data_buffer: data_buffer_pubkey,
                system_program: anchor_client::solana_sdk::system_program::ID,
            })
            .args(automata_on_chain_pccs::instruction::UpsertPckCertificate {
                qe_id,
                pce_id,
                tcbm,
            })
            .send()
            .expect("Failed to upsert PCK certificate");

        println!("Transaction signature: {}", tx);
        Ok(())
    }

    pub fn upsert_pcs_certificate(
        &self,
        ca_type: CertificateAuthority,
        data_buffer_pubkey: Pubkey,
    ) -> anyhow::Result<()> {
        let pcs_certificate_pda = Pubkey::find_program_address(
            &[b"pcs_cert", ca_type.common_name().as_bytes()],
            &self.program.id()
        );
        let tx = self
            .program
            .request()
            .accounts(automata_on_chain_pccs::accounts::UpsertPcsCertificate {
                authority: self.program.payer(),
                pcs_certificate: pcs_certificate_pda.0,
                data_buffer: data_buffer_pubkey,
                system_program: anchor_client::solana_sdk::system_program::ID,
            })
            .args(automata_on_chain_pccs::instruction::UpsertPcsCertificate {
                ca_type,
            })
            .send()
            .expect("Failed to upsert PCS certificate");

        println!("Transaction signature: {}", tx);
        Ok(())
    }

    pub fn upsert_enclave_identity(
        &self,
        id: EnclaveIdentityType,
        version: u8,
        data_buffer_pubkey: Pubkey,
    ) -> anyhow::Result<()> {
        let enclave_identity_pda = Pubkey::find_program_address(
            &[b"enclave_identity", id.common_name().as_bytes(), &version.to_le_bytes()[..1]],
            &self.program.id()
        );
        let tx = self
            .program
            .request()
            .accounts(automata_on_chain_pccs::accounts::UpsertEnclaveIdentity {
                authority: self.program.payer(),
                enclave_identity: enclave_identity_pda.0,
                data_buffer: data_buffer_pubkey,
                system_program: anchor_client::solana_sdk::system_program::ID,
            })
            .args(automata_on_chain_pccs::instruction::UpsertEnclaveIdentity {
                id,
                version,
            })
            .send()
            .expect("Failed to upsert enclave identity");

        println!("Transaction signature: {}", tx);
        Ok(())
    }

    pub fn upsert_tcb_info(
        &self,
        tcb_type: TcbType,
        version: u8,
        fmspc: String,
        data_buffer_pubkey: Pubkey,
    ) -> anyhow::Result<()> {
        let tcb_info_pda = Pubkey::find_program_address(
            &[b"tcb_info", tcb_type.common_name().as_bytes(), &version.to_le_bytes()[..1], &fmspc.as_bytes()[..8]],
            &self.program.id()
        );
        let tx = self
            .program
            .request()
            .accounts(automata_on_chain_pccs::accounts::UpsertTcbInfo {
                authority: self.program.payer(),
                tcb_info: tcb_info_pda.0,
                data_buffer: data_buffer_pubkey,
                system_program: anchor_client::solana_sdk::system_program::ID,
            })
            .args(automata_on_chain_pccs::instruction::UpsertTcbInfo {
                tcb_type,
                version,
                fmspc,
            })
            .send()
            .expect("Failed to upsert TCB info");

        println!("Transaction signature: {}", tx);
        Ok(())
    }



    pub fn get_pck_certificate(
        &self,
        qe_id: String,
        pce_id: String,
        tcbm: String,
    ) -> anyhow::Result<automata_on_chain_pccs::state::PckCertificate> {
         // Derive the same PDA as used when upserting
        let (pck_certificate_pda, _) = Pubkey::find_program_address(
            &[
                b"pck_cert",
                &qe_id.as_bytes()[..8],
                &pce_id.as_bytes()[..2],
                &tcbm.as_bytes()[..8]
            ],
            &self.program.id()
        );

        // Fetch the account data
        let account = self.program
            .account::<automata_on_chain_pccs::state::PckCertificate>(pck_certificate_pda)?;

        println!("Found PCK certificate for QE_ID: {}, PCE_ID: {}, TCBM: {}",
            qe_id, pce_id, tcbm);

        Ok(account)
    }

    pub fn get_pcs_certificate(
        &self,
        ca_type: CertificateAuthority,
    ) -> anyhow::Result<automata_on_chain_pccs::state::PcsCertificate> {
        let pcs_certificate_pda = Pubkey::find_program_address(
            &[b"pcs_cert", ca_type.common_name().as_bytes()],
            &self.program.id()
        );
        let account = self.program
            .account::<automata_on_chain_pccs::state::PcsCertificate>(pcs_certificate_pda.0)?;
        Ok(account)
    }

    pub fn get_enclave_identity(
        &self,
        id: EnclaveIdentityType,
        version: u8,
    ) -> anyhow::Result<automata_on_chain_pccs::state::EnclaveIdentity> {
        let enclave_identity_pda = Pubkey::find_program_address(
            &[b"enclave_identity", id.common_name().as_bytes(), &version.to_le_bytes()[..1]],
            &self.program.id()
        );
        let account = self.program
            .account::<automata_on_chain_pccs::state::EnclaveIdentity>(enclave_identity_pda.0)?;
        Ok(account)
    }

    pub fn get_tcb_info(
        &self,
        tcb_type: TcbType,
        version: u8,
        fmspc: String,
    ) -> anyhow::Result<automata_on_chain_pccs::state::TcbInfo> {
        let tcb_info_pda = Pubkey::find_program_address(
            &[b"tcb_info", tcb_type.common_name().as_bytes(), &version.to_le_bytes()[..1], &fmspc.as_bytes()[..8]],
            &self.program.id()
        );
        let account = self.program
            .account::<automata_on_chain_pccs::state::TcbInfo>(tcb_info_pda.0)?;
        Ok(account)
    }

    pub fn init_data_buffer(
        &self,
        total_size: u32,
        num_chunks: u8,
    ) -> anyhow::Result<Pubkey> {
        let data_buffer_keypair = Keypair::new();
        let data_buffer_pubkey = data_buffer_keypair.pubkey();

        let tx = self
            .program
            .request()
            .accounts(automata_on_chain_pccs::accounts::InitDataBuffer {
                owner: self.program.payer(),
                data_buffer: data_buffer_pubkey,
                system_program: anchor_client::solana_sdk::system_program::ID,
            })
            .args(automata_on_chain_pccs::instruction::InitDataBuffer {
                total_size,
                num_chunks,
            })
            .signer(&data_buffer_keypair)
            .send()
            .expect("Failed to initialize data buffer");

        println!("Transaction signature: {}", tx);
        Ok(data_buffer_pubkey)
    }

    pub fn upload_chunks(
        &self,
        data_buffer_pubkey: Pubkey,
        data: &[u8],
        chunk_size: usize,
    ) -> anyhow::Result<()> {
        for (i, chunk) in data.chunks(chunk_size).enumerate() {
            let chunk_index = i as u8;
            let offset = i as u32 * chunk_size as u32;
            let chunk_data = chunk.to_vec();

            let tx = self
                .program
                .request()
                .accounts(automata_on_chain_pccs::accounts::AddDataChunk {
                    owner: self.program.payer(),
                    data_buffer: data_buffer_pubkey,
                })
                .args(automata_on_chain_pccs::instruction::AddDataChunk {
                    chunk_index,
                    offset,
                    chunk_data,
                })
                .send()
                .expect("Failed to add data chunk");

            println!("Transaction signature: {}", tx)
        }
        Ok(())
    }

    pub fn get_num_chunks(data_len: usize, chunk_size: usize) -> u8 {
        ((data_len as f64 / chunk_size as f64).ceil()) as u8
    }
}
