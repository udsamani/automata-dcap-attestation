#![allow(unexpected_cfgs)]

use anchor_lang::prelude::*;

pub mod errors;
pub mod instructions;
pub mod state;
pub mod utils;

use errors::*;
use instructions::*;
use utils::*;
use zerocopy::AsBytes;

declare_id!("7obE3U8nYg7h1kaenZhkMwa8Dxzcfk5H8BRP6L4twwcx");

#[program]
pub mod automata_dcap_verifier {

    use dcap_rs::types::enclave_identity::{EnclaveIdentity, QuotingEnclaveIdentityAndSignature};
    use dcap_rs::types::quote::Quote;
    use anchor_lang::solana_program::sysvar::instructions::load_instruction_at_checked;
    use anchor_lang::solana_program::instruction::Instruction;

    use super::*;

    pub fn init_quote_buffer(
        ctx: Context<InitQuoteBuffer>,
        total_size: u32,
        num_chunks: u8,
    ) -> Result<()> {
        let data_buffer = &mut ctx.accounts.data_buffer;

        data_buffer.owner = *ctx.accounts.owner.key;
        data_buffer.total_size = total_size;
        data_buffer.num_chunks = num_chunks;
        data_buffer.chunks_received = 0;
        data_buffer.complete = false;
        data_buffer.data = vec![0; total_size as usize];

        msg!(
            "Quote buffer initialized with total size: {}, num chunks: {}",
            total_size,
            num_chunks
        );
        Ok(())
    }

    pub fn init_verified_output_account(
        ctx: Context<InitVerifiedOutput>,
    ) -> Result<()> {

        let verified_output = &mut ctx.accounts.verified_output;

        // Initialize the verified output account
        verified_output.owner = *ctx.accounts.owner.key;
        verified_output.quote_version = 0;
        verified_output.tee_type = 0;
        verified_output.tcb_status = String::new();
        verified_output.fmspc = [0; 6];
        verified_output.quote_body = Vec::new();
        verified_output.advisor_ids = None;
        verified_output.completed = false;

        msg!("Verified output account initialized");
        Ok(())
    }

    pub fn add_quote_chunk(
        ctx: Context<AddQuoteChunk>,
        chunk_index: u8,
        chunk_data: Vec<u8>,
        offset: u32,
    ) -> Result<()> {
        let data_buffer = &mut ctx.accounts.data_buffer;

        require!(
            data_buffer.owner == *ctx.accounts.owner.key,
            DcapVerifierError::InvalidOwner
        );
        require!(
            !data_buffer.complete,
            DcapVerifierError::BufferAlreadyComplete
        );
        require!(
            chunk_index < data_buffer.num_chunks,
            DcapVerifierError::InvalidChunkIndex
        );
        require!(
            (offset as usize + chunk_data.len()) as u32 <= data_buffer.total_size,
            DcapVerifierError::ChunkOutOfBounds
        );

        let start_index = offset as usize;
        let end_index = start_index + chunk_data.len();

        data_buffer.data[start_index..end_index].copy_from_slice(&chunk_data);
        data_buffer.chunks_received += 1;
        data_buffer.complete = data_buffer.chunks_received >= data_buffer.num_chunks;

        msg!(
            "Added chunk {} with offset {}, total received: {}",
            chunk_index,
            offset,
            data_buffer.chunks_received
        );
        Ok(())
    }

    pub fn verify_dcap_quote_integrity(ctx: Context<VerifyDcapQuoteIntegrity>) -> Result<()> {
        let data_buffer = &ctx.accounts.quote_data_buffer;
        let quote_data = &mut data_buffer.data.as_slice();

        let quote = Quote::read(quote_data).map_err(|e| {
            msg!("Error reading quote: {}", e);
            DcapVerifierError::InvalidQuote
        })?;

        let ix: Instruction = load_instruction_at_checked(0, &ctx.accounts.instructions_sysvar)?;
        verify_secp256r1_program_instruction_fields(&ix, &quote.signature.qe_report_body.as_bytes())?;

        quote.signature.verify_qe_report().map_err(|e| {
            msg!("Error verifying quote's qe report: {}", e);
            DcapVerifierError::InvalidQuote
        })?;

        Ok(())
    }

    pub fn verify_dcap_quote_isv_signature(ctx: Context<VerifyDcapQuoteIsvSignature>) -> Result<()> {
        let data_buffer = &ctx.accounts.quote_data_buffer;
        let quote_data = &mut data_buffer.data.as_slice();

        let quote = Quote::read(quote_data).map_err(|e| {
            msg!("Error reading quote: {}", e);
            DcapVerifierError::InvalidQuote
        })?;

        let header_bytes = quote.header.as_bytes();
        let body_bytes = quote.body.as_bytes();
        let mut data = Vec::with_capacity(header_bytes.len() + body_bytes.len());
        data.extend_from_slice(header_bytes);
        data.extend_from_slice(body_bytes);

        let ix: Instruction = load_instruction_at_checked(0, &ctx.accounts.instructions_sysvar)?;
        verify_secp256r1_program_instruction_fields(&ix, &data)?;

        Ok(())
    }

    pub fn verify_dcap_quote_enclave_source(
        ctx: Context<VerifyDcapQuoteEnclaveSource>,
        _qe_type: String,
        _version: u8,
    ) -> Result<()> {
        let data_buffer = &ctx.accounts.quote_data_buffer;
        let quote_data = &mut data_buffer.data.as_slice();

        let quote = Quote::read(quote_data).map_err(|e| {
            msg!("Error reading quote: {}", e);
            DcapVerifierError::InvalidQuote
        })?;

        let enclave_identity = &ctx.accounts.qe_identity_pda.data;
        let enclave_identity: QuotingEnclaveIdentityAndSignature = serde_json::from_slice(enclave_identity).map_err(|e| {
            msg!("Error deserializing enclave identity: {}", e);
            DcapVerifierError::InvalidQuote
        })?;

        let qe_identity = enclave_identity.get_enclave_identity_bytes();
        let qe_identity: EnclaveIdentity = serde_json::from_slice(&qe_identity).map_err(|e| {
            msg!("Error deserializing enclave identity: {}", e);
            DcapVerifierError::InvalidQuote
        })?;

        if qe_identity.mrsigner != quote.signature.qe_report_body.mr_signer {
            msg!(
                "invalid qe mrsigner, expected {} but got {}",
                hex::encode(qe_identity.mrsigner),
                hex::encode(quote.signature.qe_report_body.mr_signer)
            );
            return Err(DcapVerifierError::InvalidQuote.into());
        }

        // Compare the isv_prod_id values
        if qe_identity.isvprodid != quote.signature.qe_report_body.isv_prod_id.get() {
            msg!(
                "invalid qe isv_prod_id, expected {} but got {}",
                qe_identity.isvprodid,
                quote.signature.qe_report_body.isv_prod_id.get()
            );
            return Err(DcapVerifierError::InvalidQuote.into());
        }

        // Compare the attribute values
        let qe_report_attributes = quote.signature.qe_report_body.sgx_attributes;
        let calculated_mask = qe_identity
            .attributes_mask
            .iter()
            .zip(qe_report_attributes.iter())
            .map(|(&mask, &attribute)| mask & attribute);

        if calculated_mask
            .zip(qe_identity.attributes)
            .any(|(masked, identity)| masked != identity)
        {
            msg!("qe attrtibutes mismatch");
            return Err(DcapVerifierError::InvalidQuote.into());
        }

        // Compare misc_select values
        let misc_select = quote.signature.qe_report_body.misc_select;
        let calculated_mask = qe_identity
            .miscselect_mask
            .as_bytes()
            .iter()
            .zip(misc_select.as_bytes().iter())
            .map(|(&mask, &attribute)| mask & attribute);

        if calculated_mask
            .zip(qe_identity.miscselect.as_bytes().iter())
            .any(|(masked, &identity)| masked != identity)
        {
            msg!("qe misc_select mismatch");
            return Err(DcapVerifierError::InvalidQuote.into());
        }

        let qe_tcb_status = qe_identity.get_qe_tcb_status(quote.signature.qe_report_body.isv_svn.get());
        let qe_tcb_status_pda = &mut ctx.accounts.qe_tcb_status_pda;
        qe_tcb_status_pda.status = serde_json::to_string(&qe_tcb_status).map_err(|e| {
            msg!("Error serializing qe tcb status: {}", e);
            DcapVerifierError::InvalidQuote
        })?;

        Ok(())
    }

    pub fn update_verified_output_account(
        ctx: Context<UpdateVerifiedOutput>,
        tcb_status: String,
        advisory_ids: Vec<String>,
        fmspc: [u8; 6],
    ) -> Result<()> {
        let verified_output = &mut ctx.accounts.verified_output;

        let data_buffer = &ctx.accounts.quote_data_buffer;
        let quote = Quote::read(&mut data_buffer.data.as_slice()).map_err(|e| {
            msg!("Error reading quote: {}", e);
            DcapVerifierError::InvalidQuote
        })?;

        verified_output.tcb_status = tcb_status;
        verified_output.advisor_ids = Some(advisory_ids);
        verified_output.fmspc = fmspc;
        verified_output.quote_version = quote.header.version.get();
        verified_output.tee_type = quote.header.tee_type;
        verified_output.quote_body = quote.body.as_bytes().to_vec();
        verified_output.completed = true;

        Ok(())
    }

}
