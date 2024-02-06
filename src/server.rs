// use std::sync::Arc;
// use std::collections::{HashMap, HashSet};
// use std::sync::Mutex;
// use rand::Rng;
// use serde::Serialize;

// #[derive(Clone)]
// pub struct RestaurantApp {
//     orders: Arc<Mutex<HashMap<u32, HashSet<Order>>>>,
// }

// #[derive(Hash, Eq, PartialEq, Debug, Clone, Serialize)]
// pub struct Order {
//     item: String,
//     table_number: u32,
//     cook_time: u32,
// }

// impl RestaurantApp {
//     pub fn new() -> Self {
//         RestaurantApp {
//             orders: Arc::new(Mutex::new(HashMap::new())),
//         }
//     }

//     pub fn add_order(&self, item: String, table_number: u32) {
//         println!("Adding order");
//         let cook_time = rand::thread_rng().gen_range(5..=15);
//         let order = Order {
//             item,
//             table_number,
//             cook_time,
//         };

//         let mut orders = self.orders.lock().unwrap();
//         orders
//             .entry(table_number)
//             .or_insert_with(HashSet::new)
//             .insert(order);
//     }

//     pub fn remove_order(&self, item: &str, table_number: u32) {
//         let mut orders = self.orders.lock().unwrap();
//         if let Some(table_orders) = orders.get_mut(&table_number) {
//             table_orders.retain(|order| order.item != item);
//             if table_orders.is_empty() {
//                 orders.remove(&table_number);
//             }
//         }
//     }

//     pub fn query_all_orders(&self, table_number: u32) -> Vec<Order> {
//         let orders = self.orders.lock().unwrap();
//         orders.get(&table_number).cloned().unwrap_or_default().into_iter().collect()
//     }

//     pub fn get_order(&self, item: &str, table_number: u32) -> Option<Order> {
//         let orders = self.orders.lock().unwrap();
//         orders
//             .get(&table_number)
//             .and_then(|table_orders| table_orders.iter().find(|order| order.item == item).cloned())
//     }
// }
