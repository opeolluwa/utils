// use entity::store::Entity as StoreEntity;
// use anyhow::Result;
// use std::collections::HashMap;
// use uuid::Uuid;

// /// the table trait allows the operation of curd endpoint on tables
// pub trait Store {
//     /// create an instance of the store
//     fn new(key:&str, value:&str ) -> Self;
//     /// find one by id
//     fn find_one(&self,id:&str) -> Result<Self, ()> where Self: Sized;
//     /// find all records in the table
//     fn find_all() -> Result<Vec<Self>, ()> where Self: Sized;
//     ///delete one record in the table
//     fn delete_one(&self,id:&str) -> Result<(), ()> where Self: Sized;
//     /// delete all records in the table
//     fn delete_all() -> Result<(), ()> where Self: Sized;
//     /// update a record in the table
//     fn update(&self, fields: HashMap<String, String>) -> Result<(), ()> where Self: Sized;

// }

// impl Default for StoreEntity {
//     fn default() -> Self {
//         StoreEntity::
//     }
// }
// impl Store for StoreEntity {
//     fn new(key:&str, value:&str) -> Self{
//         Self {
//             id:
//         }
//     }

//     fn find_one(&self, id: &str) -> Result<Self, ()> where Self: Sized {
//         // implementation goes here
//         todo!()

//     }

//     fn find_all() -> Result<Vec<Self>, ()> where Self: Sized {
//         // implementation goes here
//         todo!()

//     }

//     fn delete_one(&self, id: &str) -> Result<(), ()> where Self: Sized {
//         // implementation goes here
//         todo!()

//     }

//     fn delete_all() -> Result<(), ()> where Self: Sized {
//         // implementation goes here
//         todo!()

//     }

//     fn update(&self, fields: HashMap<String, String>) -> Result<(), ()> where Self: Sized {
//         // implementation goes here
//         todo!()

//     }

// }
