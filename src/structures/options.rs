use crate::structures::credentials::Credentials;

#[derive(Clone)]
pub struct Options {
    pub uri: String,
    pub database_name: String,
    pub collection_path: String,
    pub credentials: Option<Credentials>,
}

impl Options {
    pub fn new(
        uri: String,
        database_name: String,
        collection_path: String,
        credentials: Option<Credentials>,
    ) -> Options {
        Options {
            uri,
            database_name,
            collection_path,
            credentials: credentials
        }
    }

    // pub fn set_credentials(&mut self,creds:Credentials){
    //     self.credentials = Some(creds);
    // }
}