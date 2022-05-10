use crate::*;

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Page {
    pub pathname: String,
    pub build_data: CryptoHash,
}
