use crate::database::Database;
use crate::entitiy::Item;

pub trait StoreRepository {
    fn get() -> Vec<Item>;
    fn remove_item(item_id: &i8) -> &i8;
    fn add_item(desc: &String) -> String;
    fn check_item(item_id: &i8) -> (&i8, bool);
}

pub struct Store {
    database: Database
}

impl Store {
    fn new(database: Database) -> Self {
        Store {
            database
        }
    }
}

impl StoreRepository for Store {
    fn get() -> Vec<Item> {
        todo!()
    }

    fn remove_item(item_id: &i8) -> &i8 {
        todo!()
    }

    fn add_item(desc: &String) -> String {
        todo!()
    }

    fn check_item(item_id: &i8) -> (&i8, bool) {
        todo!()
    }
}

