use crate::*;
use near_sdk::json_types::WrappedBalance;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
#[serde(tag = "type")]
pub enum ProductStatus {
    Pending,
    Confirmed,
    Completed,
    Cancelled,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub creator: AccountId,
    pub description: String,
    pub status: ProductStatus,
    pub media: String,
    pub price: Balance,
}

impl Product {
    pub fn from(v: WrappedProduct) -> Product {
        Product {
            id: v.id,
            name: v.name,
            creator: v.creator,
            description: v.description,
            status: v.status,
            media: v.media,
            price: v.price.into(),
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct WrappedProduct {
    pub id: String,
    pub name: String,
    pub creator: AccountId,
    pub description: String,
    pub status: ProductStatus,
    pub media: String,
    pub price: WrappedBalance,
}

impl WrappedProduct {
    pub fn from(v: Product) -> WrappedProduct {
        WrappedProduct {
            id: v.id,
            name: v.name,
            creator: v.creator,
            description: v.description,
            status: v.status,
            media: v.media,
            price: v.price.into(),
        }
    }
}

#[near_bindgen]
impl DwixContract {
    pub fn new_product(&mut self, mut product: WrappedProduct, site_id: ProjectId) {
        assert!(self.websities.get(&site_id).is_some(), "Website not found");
        let block_timestamp = env::block_timestamp() / 1_000_000_000;
        let id: String = block_timestamp.to_string() + "_" + &env::predecessor_account_id();

        product.id = id.clone();
        product.creator = env::predecessor_account_id();
        self.products.insert(&id, &Product::from(product));

        let mut products = self.products_by_site.get(&site_id).unwrap_or_else(|| {
            UnorderedSet::new(StorageKey::ProductsBySiteInner {
                site_id: site_id.clone(),
            })
        });
        products.insert(&id);
        self.products_by_site.insert(&site_id, &products);
    }

    pub fn update_product(&mut self, product: WrappedProduct) {
        let mut internal_product = self.products.get(&product.id).expect("Product not found");
        assert!(env::predecessor_account_id() == internal_product.creator, "You don't own this product");

        // TODO: Move into update product internal fn
        internal_product.name = product.name;
        internal_product.description = product.description;
        internal_product.media = product.media;
        internal_product.price = product.price.into();

        self.products.insert(&internal_product.id, &internal_product);
    }

    pub fn product_by_id(&self, product_id: String) -> WrappedProduct {
        WrappedProduct::from(self.products.get(&product_id).expect("Product not found"))
    }

    // pub fn get_products_by_site(&self, site_id: ProjectId) -> Vec<WrappedProduct> {
    //     if let product_ids = self.products_by_site.get(&site_id) {
    //         product_ids
    //             .into_iter()
    //             .map(|id| {
    //                 // let product = self.products.get(&id).expect("Product not found");
    //                 // WrappedProduct::from(product)
    //                 id
    //             })
    //             .collect::<Vec<WrappedProduct>>()
    //     } else {
    //         vec![]
    //     }
    // }
}
