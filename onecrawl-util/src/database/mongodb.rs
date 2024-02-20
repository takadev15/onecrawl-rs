use std::ops::Deref;

use mongodb::{
    bson::{doc, oid::ObjectId, Bson},
    results::{InsertManyResult, InsertOneResult},
};
use serde::{
    de::DeserializeOwned,
    Serialize,
};

pub mod crawling {
    pub mod model;
}

pub mod page_information {
    pub mod model;
}

pub mod page_images {
    pub mod model;
}

pub mod page_linking {
    pub mod model;
}

pub mod page_list {
    pub mod model;
}

pub mod page_form {
    pub mod model;
}

pub mod page_scripts {
    pub mod model;
}

pub mod page_styles {
    pub mod model;
}

pub mod page_tables {
    pub mod model;
}

#[derive(Debug, Clone)]
pub struct MongoDB(pub mongodb::Client);

pub fn init_db(client: mongodb::Client) -> MongoDB {
    return MongoDB(client);
}

impl Deref for MongoDB {
    type Target = mongodb::Client;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// Common Function Across All Table
impl MongoDB {
    pub fn db(&self) -> mongodb::Database {
        self.database("crawler")
    }

    pub fn coll<T>(&self, collection: &str) -> mongodb::Collection<T> {
        self.db().collection(collection)
    }

    /// Insert a single data to monggodb's collection
    ///
    /// # Parameters
    ///
    /// - `collection`: Collection's name
    /// - `document` : Serialized document that wanted to be inserted
    ///
    /// # Returns
    ///
    /// - `InsertOneResult`: Result
    /// - `Error`: Error
    pub async fn insert_once<T: Serialize>(
        &self,
        collection: &'static str,
        document: T,
    ) -> Result<InsertOneResult, mongodb::error::Error> {
        self.coll::<T>(collection).insert_one(document, None).await
    }

    /// Insert multiple data to monggodb's collection
    ///
    /// # Parameters
    ///
    /// - `collection`: Collection's name
    /// - `document` : Serialized array of documents that wanted to be inserted
    pub async fn insert_bulk<T: Serialize>(
        &self,
        collection: &'static str,
        documents: Vec<T>,
    ) -> Result<InsertManyResult, mongodb::error::Error> {
        self.coll::<T>(collection).insert_many(documents, None).await
    }

    pub fn get_inserted_id(&self, bson: Bson) -> Option<String> {
        if let Bson::ObjectId(object_id) = bson {
            Some(object_id.to_hex())
        } else {
            None
        }
    }

    pub async fn check_value<T: DeserializeOwned>(
        &self,
        collection: &str,
        field: &str,
        value: String,
    ) -> bool {
        let query = doc! {
            field : value
        };
        if self
            .coll::<T>(collection)
            .count_documents(query, None)
            .await
            .unwrap()
            > 0
        {
            true
        } else {
            false
        }
    }

    pub async fn count_data<T: DeserializeOwned>(
        &self,
        collection: &str,
        field: &str,
        value: String,
    ) -> u64 {
        let query = doc! {
            field : value
        };
        self.coll::<T>(collection)
            .count_documents(query, None)
            .await
            .unwrap()
    }
}
