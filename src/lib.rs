use futures::TryStreamExt;
use mongodb::{
    self, Client, Collection, Database,
    bson::{self, doc, oid::ObjectId},
    options::ClientOptions,
};
use serde::{Deserialize, Serialize};


mod traits;
pub use traits::MongoObject;

mod structures;
pub use structures::{options::Options, credentials::Credentials}; 



pub struct SimpleMongo<T>
where
    T: for<'de> Deserialize<'de> + Sync + Send + Serialize + MongoObject + Clone,
{
    options: Options,
    collection: Collection<T>,
    database: Database,
}

impl<T> SimpleMongo<T>
where
    T: Send + Sync + for<'de> Deserialize<'de> + Serialize + MongoObject + Clone,
{
    pub fn parse_id(id: &str) -> String {
        ObjectId::parse_str(id)
            .expect(&format!("{id} is not a valid id"))
            .to_string()
    }

    pub fn add_credentials_to_url(url:&str,user:&str,password:&str)->String{

        let mut splitted = url.split("//");
    
        let mongo =splitted.next().unwrap();
    
        let rest = splitted.next().unwrap();
    
         format!("{mongo}//{user}:{password}@{rest}")
    }

    pub async fn connect(options: Options) -> SimpleMongo<T> {

        let mut parsed_options = options.clone();
        if let Some(ref x) = parsed_options.credentials{
            let new_uri = SimpleMongo::<T>::add_credentials_to_url(&parsed_options.uri, &x.username, &x.password);
            parsed_options.uri = new_uri;
        }
        
        let client =
            Client::with_options(ClientOptions::parse(&parsed_options.uri).await.unwrap()).map_err(|e| format!("Errore  connecting to  MongoDB: {}", e)).unwrap();
            // Test the connection by making a simple query
        
        let database: Database = client.database(&options.database_name);
        let collection = database.collection::<T>(&options.collection_path);
        collection.find(doc! {}).await.unwrap();
        return SimpleMongo {
            options:parsed_options,
            database,
            collection,
        };
    }

    pub async fn list_all(&self) -> Vec<T> {
        let cursor = self.collection.find(doc! {}).await.ok().unwrap();

        let res: Vec<T> = cursor.try_collect().await.unwrap();
        res
    }

    pub async fn find_one_by_id(&self, id: &str) -> Option<T> {
        let parsed_id: String = SimpleMongo::<T>::parse_id(id);
        self.collection
            .find_one(doc! {"id":parsed_id})
            .await
            .ok()
            .unwrap()
    }

    pub async fn insert_one(&self, item: T) -> Option<T> {
        if let Some(_) = self.collection.insert_one(&item).await.ok() {
            return self.find_one_by_id(&item.id()).await;
        } else {
            return None;
        }
    }

    pub async fn remove_one_by_id(&self, id: &str) -> Option<T> {
        let parsed_id: String = SimpleMongo::<T>::parse_id(id);
        if let Some(res) = self
            .collection
            .find_one_and_delete(doc! {"id":parsed_id})
            .await
            .ok()
        {
            return res;
        } else {
            return None;
        }
    }

    pub async fn clear(&self) -> bool {
        self.collection.delete_many(doc! {}).await.unwrap();
        self.collection.count_documents(doc! {}).await.unwrap() == 0
    }

    pub async fn update_one_by_id(&self, id: &str, update: T) -> Option<T> {
        let mut new_item: T = update.clone();
        new_item.set_id(id);

        let update_doc = bson::to_document(&new_item).unwrap(); // Convertiamo T in BSON
        let update_query = doc! { "$set": &update_doc }; // 

        let parsed_id: String = SimpleMongo::<T>::parse_id(id);
        if let Some(res) = self
            .collection
            .update_one(doc! {"id":parsed_id}, update_query)
            .await
            .ok()
        {
            if res.modified_count == 0 {
                return None;
            } else {
                return Some(new_item);
            }
        } else {
            None
        }
    }
}
