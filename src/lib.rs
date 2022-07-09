use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, LookupSet, UnorderedMap, UnorderedSet};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::AccountId;
use near_sdk::{
    env, near_bindgen, setup_alloc, Balance, BorshStorageKey, CryptoHash, PanicOnDefault, Promise,
    Timestamp,
};

setup_alloc!();

use crate::product::*;
use crate::project::*;
use crate::order::*;

mod actions_of_project;
mod page;
mod project;

// Ecom modules
mod product;
mod order;

pub type ProjectId = String;

#[derive(BorshStorageKey, BorshSerialize)]
pub(crate) enum StorageKey {
    Websities,
    // Pages { project_id: ProjectId },
    ProjectsOwner,
    ProjectsOwnerInner { owner: AccountId },
    DeployQueue,
    UsedDomains,

    // Ecom
    ProductsBySite,
    ProductsBySiteInner { site_id: String },
    Products,

    // Order
    Orders,
    OrdersBySite,
    OrdersBySiteInner { site_id: String },
    OrderByUser,
    OrderByUserInner { user_id: AccountId },
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct DwixContract {
    pub owner_id: AccountId,
    pub websities: UnorderedMap<ProjectId, Project>,
    pub projects_owner: LookupMap<AccountId, UnorderedSet<ProjectId>>,
    pub deploy_queue: UnorderedMap<ProjectId, Timestamp>,
    pub last_deploy_request: Timestamp,
    pub used_domains: UnorderedSet<String>,

    // Ecom
    pub products_by_site: LookupMap<ProjectId, UnorderedSet<String>>,
    pub products: LookupMap<String, Product>,

    pub orders: LookupMap<String /*OrderId*/, Order>,
    pub orders_by_site: LookupMap<String, UnorderedSet<String>>,
    pub orders_by_user: LookupMap<AccountId, UnorderedSet<String>>,
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
            used_domains: UnorderedSet::new(StorageKey::UsedDomains),

            products_by_site: LookupMap::new(StorageKey::ProductsBySite),
            products: LookupMap::new(StorageKey::Products),
            orders: LookupMap::new(StorageKey::Orders),
            orders_by_site: LookupMap::new(StorageKey::OrdersBySite),
            orders_by_user: LookupMap::new(StorageKey::OrderByUser)
        }
    }
}
