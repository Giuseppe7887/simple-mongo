# SimpleMongo

A Rust library designed to simplify interactions with MongoDB. It provides an intuitive interface for common CRUD operations.

## Features

Simplified MongoDB connection
CRUD operations (Create, Read, Update, Delete)
Support for custom data types via Serde
MongoDB ID handling with validation

<br/>

## Dependencies

```toml
[dependencies]
futures = "0.3"
mongodb = "2.4"
serde = { version = "1.0", features = ["derive"] }
```

<br/>

## Usage

#### Define a Data Model

```rust
// models/user.rs

use serde::{Deserialize, Serialize};
use simple_mongo::MongoObject;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct User {
    id: String, // id is required
    name: String,
    email: String,
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

```

<br/>

## Connect to database

```rust
// main.rs

use simple_mongo::{SimpleMongo, Options};
use tokio;


#[tokio::main]
async fn main(){
    let options = Options::new(
        "mongodb://localhost:27017".to_string(),
        "my_database".to_string(),
        "users".to_string(),
        Credentials::new("user","password"); // or use None to use it without authentication
    );

    // define mongo connection passing your Data Structure as Generic
    let db = SimpleMongo::<User>::connect(options).await;

}
```

<br/>

## CRUD Operations

### Insert a document

```rust
let user = User::new("Jhon");
let user_creared = conn.insert_one(user).await;
println!("user created: {:?}",user_creared);
```

### Find and remove a document

```rust
let found_user = conn.find_one_by_id(&id).await;
println!("found: {:?}",found_user);

let id =  found_user.unwrap().id();
let removed_user = conn.remove_one_by_id(&id).await;
println!("removed {:?}",removed_user);
```


### List all documents
```rust
let all_users =  conn.list_all().await;
println!("all all users {:?}",all_users);
```


### Update a document
```rust
let user_to_update = conn.find_one_by_id(&id).await;
   
let id =  found_user.unwrap().id();

let updated_user = conn.update_one_by_id(&id,User::new("Mark")).await;
```

## Drop current Collection

```rust
  conn.clear().await;
```

<br/>
<br/>

# API Reference

## Traits

### `MongoObject`

A trait that must be implemented by types that will be stored in MongoDB.

```rust
pub trait MongoObject {
    fn new(name: &str) -> Self;
    fn id(&self) -> String;
    fn set_id(&mut self, id: &str);
}
```

#### Methods

| Method | Signature | Description |
|--------|-----------|-------------|
| `new` | `fn new(name: &str) -> Self` | Creates a new instance of the object with the specified name |
| `id` | `fn id(&self) -> String` | Returns the object's ID as a String |
| `set_id` | `fn set_id(&mut self, id: &str)` | Sets the object's ID using the provided string |

## Structs

### `Credentials`

Represents authentication credentials for MongoDB.

```rust
#[derive(Clone)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}
```

#### Fields

| Field | Type | Description |
|-------|------|-------------|
| `username` | `String` | The username for MongoDB authentication |
| `password` | `String` | The password for MongoDB authentication |

#### Methods

| Method | Signature | Description |
|--------|-----------|-------------|
| `new` | `fn new(username: &str, password: &str) -> Option<Credentials>` | Creates a new Credentials instance with the specified username and password |

### `Options`

Contains the connection options for MongoDB.

```rust
#[derive(Clone)]
pub struct Options {
    uri: String,
    database_name: String,
    collection_path: String,
    credentials: Option<Credentials>,
}
```

#### Fields

| Field | Type | Description |
|-------|------|-------------|
| `uri` | `String` | MongoDB connection URI |
| `database_name` | `String` | Name of the database to connect to |
| `collection_path` | `String` | Path to the collection within the database |
| `credentials` | `Option<Credentials>` | Optional authentication credentials |

#### Methods

| Method | Signature | Description |
|--------|-----------|-------------|
| `new` | `fn new(uri: String, database_name: String, collection_path: String, credentials: Option<Credentials>) -> Options` | Creates a new Options instance with the specified parameters |

### `SimpleMongo<T>`

The main structure that provides the interface for interacting with MongoDB. The type parameter `T` must implement `Deserialize`, `Serialize`, `MongoObject`, `Clone`, `Send`, and `Sync`.

```rust
pub struct SimpleMongo<T>
where
    T: for<'de> Deserialize<'de> + Sync + Send + Serialize + MongoObject + Clone,
{
    options: Options,
    collection: Collection<T>,
    database: Database,
}
```

#### Fields

| Field | Type | Description |
|-------|------|-------------|
| `options` | `Options` | The options used to configure the MongoDB connection |
| `collection` | `Collection<T>` | The MongoDB collection for type T |
| `database` | `Database` | The MongoDB database instance |

#### Methods

| Method | Signature | Description |
|--------|-----------|-------------|
| `parse_id` | `fn parse_id(id: &str) -> String` | Static method that parses and validates a MongoDB ID, returning it as a String. Panics if the ID is invalid. |
| `add_credentials_to_url` | `fn add_credentials_to_url(url: &str, user: &str, password: &str) -> String` | Static method that adds authentication credentials to a MongoDB connection URL |
| `connect` | `async fn connect(options: Options) -> SimpleMongo<T>` | Connects to MongoDB with the specified options and returns a new SimpleMongo instance. Verifies the connection by making a test query. |
| `list_all` | `async fn list_all(&self) -> Vec<T>` | Retrieves all documents in the collection as a vector |
| `find_one_by_id` | `async fn find_one_by_id(&self, id: &str) -> Option<T>` | Finds a document by ID. Returns `None` if not found. |
| `insert_one` | `async fn insert_one(&self, item: T) -> Option<T>` | Inserts a document into the collection. Returns the inserted document with MongoDB-generated fields, or `None` if the insertion failed. |
| `remove_one_by_id` | `async fn remove_one_by_id(&self, id: &str) -> Option<T>` | Removes a document by ID. Returns the removed document, or `None` if not found or if the operation failed. |
| `clear` | `async fn clear(&self) -> bool` | Removes all documents from the collection. Returns `true` if the collection is empty after the operation. |
| `update_one_by_id` | `async fn update_one_by_id(&self, id: &str, update: T) -> Option<T>` | Updates a document by ID with the provided data. Returns the updated document, or `None` if not found or if the operation failed. |

## MongoDB Connection Testing

The `connect` method tests the connection by executing a find operation on the collection. If the connection fails (e.g., due to invalid credentials), an error will be thrown during this test.

## ID Parsing

The `parse_id` method expects a valid MongoDB ObjectID string (24 hexadecimal characters). It will panic with an error message if the ID is invalid.

## Thread Safety

All methods on `SimpleMongo<T>` are thread-safe, as the type parameter `T` is constrained to implement `Send` and `Sync`. This means `SimpleMongo<T>` can be safely shared between threads.

## Example Usage

```rust
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
    // Create connection options with credentials
    let options = Options::new(
        "mongodb://127.0.0.1:27017".to_string(),
        "my_database".to_string(),
        "users".to_string(),
        Credentials::new("username", "password"),
    );
    
    // Connect to MongoDB
    let db = SimpleMongo::<User>::connect(options).await;
    
    // Insert a new user
    let user = User::new("John Doe");
    let inserted = db.insert_one(user).await;
    
    if let Some(inserted_user) = inserted {
        println!("Inserted user: {:?}", inserted_user);
        
        // Find the user by ID
        let id = inserted_user.id();
        let found = db.find_one_by_id(&id).await;
        println!("Found user: {:?}", found);
        
        // Update the user
        let updated = db.update_one_by_id(&id, User::new("Jane Doe")).await;
        println!("Updated user: {:?}", updated);
        
        // Delete the user
        let removed = db.remove_one_by_id(&id).await;
        println!("Removed user: {:?}", removed);
    }
    
    // List all users
    let users = db.list_all().await;
    println!("All users: {:?}", users);
    
    // Clear the collection
    let cleared = db.clear().await;
    println!("Collection cleared: {}", cleared);
}
```