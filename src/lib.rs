use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap, UnorderedSet, LookupSet};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::AccountId;
use near_sdk::{
    env, near_bindgen, setup_alloc, Balance, BorshStorageKey, CryptoHash, PanicOnDefault, Promise,
    Timestamp,
};

setup_alloc!();

use crate::project::*;
use crate::product::*;

mod actions_of_project;
mod page;
mod project;

// Ecom modules
mod product;

pub type ProjectId = String;

#[derive(BorshStorageKey, BorshSerialize)]
pub(crate) enum StorageKey {
    Websities,
    // Pages { project_id: ProjectId },
    ProjectsOwner,
    ProjectsOwnerInner { owner: AccountId },
    DeployQueue,

    // Ecom
    ProductsBySite,
    ProductsBySiteInner { site_id: String },
    Products,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct DwixContract {
    pub owner_id: AccountId,
    pub websities: UnorderedMap<ProjectId, Project>,
    pub projects_owner: LookupMap<AccountId, UnorderedSet<ProjectId>>,
    pub deploy_queue: UnorderedMap<ProjectId, Timestamp>,
    pub last_deploy_request: Timestamp,

    // Ecom
    pub products_by_site: LookupMap<ProjectId, UnorderedSet<String>>,
    pub products: LookupMap<String, Product>,
}

#[near_bindgen]
impl DwixContract {
    #[init]
    pub fn new() -> Self {
        Self {
            owner_id: env::predecessor_account_id(),
            websities: UnorderedMap::new(StorageKey::Websities),
            projects_owner: LookupMap::new(StorageKey::ProjectsOwner),
            deploy_queue: UnorderedMap::new(StorageKey::DeployQueue),
            last_deploy_request: 0,

            products_by_site: LookupMap::new(StorageKey::ProductsBySite),
            products: LookupMap::new(StorageKey::Products)
        }
    }
}
