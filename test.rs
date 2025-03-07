use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use simple_mongo::{Credentials, MongoObject, Options, SimpleMongo};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct User {
    id: String,
    name: String,
}

impl MongoObject for User {
    fn new(name: &str) -> User {
        User {
            id: ObjectId::new().to_string(),
            name: name.to_string(),
        }
    }

    fn id(&self) -> String {
        self.id.clone()
    }

    fn set_id(&mut self, id: &str) {
        self.id = id.to_string();
    }
}


#[tokio::main]
async fn main() {
    let options = Options::new(
        "mongodb://127.0.0.1:27017".to_string(),
        "db".to_string(),
        "collezione".to_string(),
        Credentials::new("paolo", "1234"),
    );

    let conn = SimpleMongo::<User>::connect(options).await;

    let inserted = conn.insert_one(User::new("paolo")).await;
    println!("created: {:?}",inserted);
    // let id = inserted.unwrap().id().clone();

    // let found = conn.find_one_by_id(&id).await;
    // println!("found: {found:?}");
    // let id = found.unwrap().id();
    // conn.remove_one_by_id(&id).await;

    // let updated = conn.update_one_by_id(&id,User::new("giovanni")).await;
    // println!("updated {updated:?}");

    // let found2 = conn.find_one_by_id(&id).await;
    // println!("found again: {found2:?}");

    // let lista = conn.list_all().await;
    // println!("{lista:?}");
    // println!("{:?}",conn.clear().await);

    // let lista = conn.list_all().await;
    // println!("{lista:?}");
    // println!("{:?}",conn.remove_one_by_id(&x.unwrap().id()).await);
}
