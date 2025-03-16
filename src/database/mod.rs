use crate::entitiy::Item;

pub struct Database {}

impl Database {
    fn connect() -> () {
        todo!()
    }

    fn get_all_items() -> Vec<Item> {}

    fn get_item(item_id: &i8) -> Item {}

    fn update_item(item_id: &i8, new_item: Item) -> () {}

    fn remove_item(item_id: &i8) -> () {}

    fn add_item(item: Item) -> () {}
}
