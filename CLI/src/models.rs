pub enum Status {
    Open,
    InProgress,
    Resolved,
    Closed
}

pub struct Epic {


}

impl Epic {

}

pub struct Story {
    pub name: String,
    pub description: String,
    pub status: Status
}

impl Story {
    pub fn new(name: String, description: String) -> Self {
        Storey {
            name, 
            description,
            status: Status::Open
        }
    }
}

pub struct DBState {

}