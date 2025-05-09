use crate::pccs::get_signer;
use sdk::EnclaveIdentityType;

#[tokio::test]
async fn test_enclave_identity_upsert() {
    let enclave_identity_data = include_bytes!("../../data/qe_identity.json").to_vec();


    let client = sdk::PccsClient::new(get_signer()).unwrap();
    let num_chunks = sdk::get_num_chunks(enclave_identity_data.len(), 512);
    let data_buffer_pubkey = client.init_data_buffer(enclave_identity_data.len() as u32, num_chunks).await.unwrap();
    client.upload_chunks(data_buffer_pubkey, &enclave_identity_data, 512).await.unwrap();

    let _tx = client.upsert_enclave_identity(
        EnclaveIdentityType::TdQe,
        2,
        data_buffer_pubkey,
    ).await.unwrap();

    let enclave_identity = client.get_enclave_identity(
        EnclaveIdentityType::TdQe,
        2,
    ).await.unwrap();

    let actual_identity_type: EnclaveIdentityType = enclave_identity.identity_type.into();
    let expected_identity_type = EnclaveIdentityType::TdQe;
    assert_eq!(actual_identity_type, expected_identity_type);
    assert_eq!(enclave_identity.version, 2);
    assert_eq!(enclave_identity.data, enclave_identity_data);
}
