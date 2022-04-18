// External imports
use bson::{doc, Document};
use mongodb::{options::FindOptions};
use mongodb::{Collection};
use serde::{Deserialize, Serialize};
// External constructors
extern crate serde;
extern crate serde_json;

// Estructure data for DB
#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub title: String,
    pub author: String,
}

// Reference colection clone
#[derive(Clone)]
pub struct ApiService {
    collection: Collection,
}

// Functions with quieries to Mongo
impl ApiService {
    pub fn new(collection: Collection) -> ApiService {
        ApiService { collection }
    }



    // Get all documents
    pub fn get_json(&self) -> std::result::Result<std::vec::Vec<bson::ordered::OrderedDocument>, mongodb::error::Error> {
        //Sort id es para poner el orden( 1 para ascendente, -1 para descendente)
        let find_options = FindOptions:: builder().sort(doc! {"id": -1}).limit(30).build();
        let cursor = self.collection.find(None, find_options).ok().expect("Failed to execute find.");
        let docs: Vec<_> = cursor.map(|doc| doc.unwrap()).collect();
        Ok(docs)
    }

    // Get all documents
    pub fn get_json_all(&self) -> std::result::Result<std::vec::Vec<bson::ordered::OrderedDocument>, mongodb::error::Error> {
        let cursor = self.collection.find(None, None).ok().expect("Failed to execute find.");
        let docs: Vec<_> = cursor.map(|doc| doc.unwrap()).collect();
        Ok(docs)
    }

  /*pub fn get_json_top(&self) -> std::result::Result<std::vec::Vec<bson::ordered::OrderedDocument>, mongodb::error::Error> {
    let cursor = self.collection.find(doc! { "game_name": "Random" }, None).ok().expect("Failed to execute find.");
        let docs: Vec<_> = cursor.map(|doc| doc.unwrap()).collect();
        Ok(docs)
    }*/

    // Get documents with quiery
    pub fn get_by(&self, param: &String) -> std::result::Result<std::vec::Vec<bson::ordered::OrderedDocument>, mongodb::error::Error> {
        let cursor = self.collection.find(doc! { "author": { "$regex": param } }, None).ok().expect("Failed to execute find.");
        let docs: Vec<_> = cursor.map(|doc| doc.unwrap()).collect();
        let _serialized = serde_json::to_string(&docs).unwrap();
        Ok(docs)
    }
}
