pub struct Request {
    url: String,
}

pub struct Collection {
    name: String,
    requests: Vec<Request>,
}
