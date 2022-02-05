
use crate::entity::user::User;

pub struct Room {
    internal_room_id: String,
    name: Option<String>,
    topic: Option<String>,
    displaypic: Option<String>,
    room_version: u32,
    room_addresses: Vec<String>,
    members: Vec<User>
    /*
     * A lot more fields are required to be added
     * just haven't investigated what is required
     * There is also a permission model required
     */
}

impl Room {
    fn debug() -> Room {
        unimplemented!()
    }

    fn new() -> Room {
        unimplemented!()
    }
}
