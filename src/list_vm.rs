extern crate serde_derive;

#[derive(Serialize, Deserialize)]
struct ListingVM{
    name : String,
    id: String
}
