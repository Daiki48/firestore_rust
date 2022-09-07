use firestore_db_and_auth::{
    documents,               // get data
    documents::List,         // get data
    Credentials,             // auth
    ServiceSession           // auth
};

use serde::{
    Serialize,
    Deserialize
};

#[derive(Serialize, Deserialize, Debug)]
struct MyDATA {
    name: String,
    age: u32,
}

fn main() {
    println!("Read data from the FireStore.");

    // firestore-rust-firebase-adminsdk-cjbwm-73e4ba12f1.json is firestore configuration information.

    let cred = Credentials::from_file("./key/firestore-rust-firebase-adminsdk-cjbwm-73e4ba12f1.json").unwrap();
    
    let auth = ServiceSession::new(cred).unwrap();

    let documents: List<MyDATA, ServiceSession> = documents::list(&auth, "sample1");

    for doc in documents {
        let (data, _document) = doc.unwrap();
        println!("{:?}", data);
    }
}
