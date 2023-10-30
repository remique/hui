use serde::{Deserialize, Serialize};

/// A `Request` structure to make requests with.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Request {
    url: String,
    method: String,
    name: String,
}

/// A `Collection` can contain either a `Request` or another `Collction`.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Collection {
    name: String,
    fields: Vec<CollectionOrRequest>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum CollectionOrRequest {
    Request(Request),
    Collection(Collection),
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Collections {
    pub collections: Vec<CollectionOrRequest>,
}

impl Collections {
    pub fn from_str(s: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let cols: Collections = ron::from_str(s)?;

        Ok(cols)
    }

    pub fn find_collection_mut(&mut self, path: &str) -> Option<&mut Vec<CollectionOrRequest>> {
        let mut current_collections = &mut self.collections;

        for name in path.split("/") {
            if name.is_empty() {
                continue;
            }

            // We can't use `find` as this would only get &CollectionOrRequest, we also
            // can't use iter_mut() as we cannot borrow self as mutable more than once
            let index = current_collections.iter().position(|c| match c {
                CollectionOrRequest::Collection(cc) => cc.name == name,
                _ => false,
            });

            if let Some(i) = index {
                match &mut current_collections[i] {
                    CollectionOrRequest::Collection(col) => {
                        current_collections = &mut col.fields;
                    }
                    _ => return None,
                }
            }
        }

        Some(current_collections)
    }

    pub fn find_collection(&self, path: &str) -> Option<&Vec<CollectionOrRequest>> {
        let mut current_collections = &self.collections;

        for name in path.split("/") {
            if name.is_empty() {
                continue;
            }

            let index = current_collections.iter().position(|c| match c {
                CollectionOrRequest::Collection(cc) => cc.name == name,
                _ => false,
            });

            if let Some(i) = index {
                match &current_collections[i] {
                    CollectionOrRequest::Collection(col) => {
                        current_collections = &col.fields;
                    }
                    _ => return None,
                }
            }
        }

        Some(current_collections)
    }

    // We only need to obtain the reference to Request and then use it somewhere else in `App`
    pub fn get_request(&self, collection_path: &str, request_name: &str) -> Option<&Request> {
        let col = self.find_collection(collection_path);

        if let Some(c) = col {
            for item in c {
                match item {
                    CollectionOrRequest::Request(r) => {
                        if r.name == request_name {
                            return Some(r);
                        }
                    }
                    _ => {
                        continue;
                    }
                }
            }
        }

        None
    }
}

#[test]
fn test_collection_structure() {
    let raw_input = r#"
    Collections( 
        collections: [
            Request(
                Request (
                    url: "localhost:3000", 
                    method: "GET", 
                    name: "getFirst" 
                ),
            ),
            Collection(
                Collection (
                    name: "firstCollection",
                    fields: [
                        Collection (
                            Collection (
                                name: "secondCollection",
                                fields: []
                            )
                        )
                    ]
                )
            )
        ] 
    )
    "#;

    let obj = Collections {
        collections: vec![
            CollectionOrRequest::Request(Request {
                url: "localhost:3000".to_string(),
                method: "GET".to_string(),
                name: "getFirst".to_string(),
            }),
            CollectionOrRequest::Collection(Collection {
                name: "firstCollection".to_string(),
                fields: vec![CollectionOrRequest::Collection(Collection {
                    name: "secondCollection".to_string(),
                    fields: vec![],
                })],
            }),
        ],
    };

    assert_eq!(Collections::from_str(raw_input).unwrap(), obj);
}

#[test]
fn test_correct_get_by_path() {
    let raw_input_before = r#"
    Collections( 
        collections: [
            Request(
                Request (
                    url: "localhost:3000", 
                    method: "GET", 
                    name: "getFirst" 
                ),
            ),
            Collection(
                Collection (
                    name: "firstCollection",
                    fields: [
                        Collection (
                            Collection (
                                name: "secondCollection",
                                fields: [
                                    Request (
                                        Request (
                                            url: "localhost:1234", 
                                            method: "POST", 
                                            name: "getThird" 
                                        )
                                    )
                                ]
                            )
                        )
                    ]
                )
            )
        ] 
    )
    "#;

    let raw_input_after = r#"
    Collections( 
        collections: [
            Request(
                Request (
                    url: "localhost:3000", 
                    method: "GET", 
                    name: "getFirst" 
                ),
            ),
            Collection(
                Collection (
                    name: "firstCollection",
                    fields: [
                        Collection (
                            Collection (
                                name: "secondCollection",
                                fields: []
                            )
                        )
                    ]
                )
            )
        ] 
    )
    "#;

    // Mutate the object
    let mut modify = Collections::from_str(raw_input_before).unwrap();
    let get_second_collection = modify
        .find_collection_mut("firstCollection/secondCollection")
        .unwrap();
    get_second_collection.clear();

    let after = Collections::from_str(raw_input_after).unwrap();
    assert_eq!(modify, after);
}

#[test]
fn test_get_request() {
    let raw_input = r#"
    Collections( 
        collections: [
            Request(
                Request (
                    url: "localhost:3000", 
                    method: "GET", 
                    name: "getFirst" 
                ),
            ),
            Collection(
                Collection (
                    name: "firstCollection",
                    fields: [
                        Collection (
                            Collection (
                                name: "secondCollection",
                                fields: [
                                    Request (
                                        Request (
                                            url: "localhost:1234", 
                                            method: "POST", 
                                            name: "getThird" 
                                        )
                                    )
                                ]
                            )
                        )
                    ]
                )
            )
        ] 
    )
    "#;

    let c = Collections::from_str(raw_input).unwrap();
    let req = c.get_request("firstCollection/secondCollection", "getThird");

    let expected = Request {
        url: "localhost:1234".to_string(),
        method: "POST".to_string(),
        name: "getThird".to_string(),
    };

    assert_eq!(req, Some(&expected));
}
