use near_sdk::json_types::WrappedBalance;

use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
#[serde(tag = "type")]
pub enum OrderStatus {
    Pending,
    Confirmed,
    Completed,
    Cancelled,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Order {
    pub id: String,
    pub owner: AccountId,
    pub address: String,
    pub status: OrderStatus,
    pub product_ids: Vec<String>,
    pub price: WrappedBalance,
}

#[near_bindgen]
impl DwixContract {
    #[payable]
    pub fn new_order(&mut self, address: String, product_ids: Vec<String>, site_id: ProjectId) {
        let products_by_site = self.products_by_site.get(&site_id).expect("Site not found");

        for id in product_ids.clone() {
            assert!(
                products_by_site.contains(&id),
                "Proudct is not from this site"
            );
        }

        let price: Balance = product_ids
            .iter()
            .map(|v| self.products.get(v).unwrap().price)
            .sum();
        env::log(format!("price {:#?}, attached_deposit {:#?}", price, env::attached_deposit()).as_bytes());

        assert!(env::attached_deposit() == price, "Not enough money");

        let block_timestamp = env::block_timestamp() / 1_000_000_000;
        let order_id: String = block_timestamp.to_string() + "_" + &env::predecessor_account_id();

        self.orders.insert(
            &order_id.clone(),
            &Order {
                id: order_id.clone(),
                owner: env::predecessor_account_id(),
                address,
                status: OrderStatus::Pending,
                product_ids,
                price: price.into(),
            },
        );

        let mut orders_by_site = self.orders_by_site.get(&site_id).unwrap_or_else(|| {
            UnorderedSet::new(StorageKey::OrdersBySiteInner {
                site_id: site_id.clone(),
            })
        });
        orders_by_site.insert(&order_id.clone());
        self.orders_by_site.insert(&site_id, &orders_by_site);

        let mut orders_by_user = self.orders_by_user.get(&env::predecessor_account_id()).unwrap_or_else(|| {
            UnorderedSet::new(StorageKey::OrderByUserInner {
                user_id: env::predecessor_account_id(),
            })
        });
        orders_by_user.insert(&order_id);
        self.orders_by_user
            .insert(&env::predecessor_account_id(), &orders_by_user);
    }

    // Owner call this function
    pub fn confirm_order(&mut self, site_id: ProjectId, order_id: String) {
        //TODO: Only owner of this order/this site can confirm
        let mut order = self.orders.get(&order_id).expect("Order not found");
        let products_by_site = self
            .products_by_site
            .get(&site_id)
            .expect("Products not found");

        for id in order.product_ids.clone() {
            assert!(
                products_by_site.contains(&id),
                "Product is not from this site"
            );
        }

        order.status = OrderStatus::Confirmed;
        self.orders.insert(&order_id, &order);
    }

    pub fn cancel_order(&mut self, order_id: String) {
        //TODO: Only buyer can cancel this order
        let mut order = self.orders.get(&order_id).expect("Order not found");
        assert!(matches!(order.status, OrderStatus::Pending), "Can not cancel this order");

        order.status = OrderStatus::Cancelled;
        self.orders.insert(&order_id, &order);

        Promise::new(env::predecessor_account_id()).transfer(order.price.into());
    }

    // Owner call this function
    pub fn completed_order(&mut self, order_id: String, site_id: ProjectId) {
        //TODO: Only owner of this order/this site can confirm
        let mut order = self.orders.get(&order_id).expect("Order not found");
        // let products_by_site = self
        //     .products_by_site
        //     .get(&site_id)
        //     .expect("Products not found");

        // for id in order.product_ids.clone() {
        //     assert!(
        //         products_by_site.contains(&id),
        //         "Product is not from this site"
        //     );
        // }

        order.status = OrderStatus::Completed;
        self.orders.insert(&order_id, &order);

        Promise::new(env::predecessor_account_id()).transfer(order.price.into());
    }

    pub fn get_order_by_id(&self, order_id: String) -> Order {
        self.orders.get(&order_id).expect("Order not found")
    }

    pub fn get_orders_by_site(&self, site_id: ProjectId) -> Vec<Order> {
        if let Some(orders) = self.orders_by_site.get(&site_id) {
            orders
                .iter()
                .map(|v| self.orders.get(&v).unwrap())
                .collect()
        } else {
            vec![]
        }
    }

    pub fn get_orders_by_user(&self, user_id: AccountId) -> Vec<Order> {
        if let Some(orders) = self.orders_by_user.get(&user_id) {
            orders
                .iter()
                .map(|v| self.orders.get(&v).unwrap())
                .collect()
        } else {
            vec![]
        }
    }
}
