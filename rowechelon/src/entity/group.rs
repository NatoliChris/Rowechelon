
use crate::entity::room::Room;

///
///First three are default types of grouping
///enabled within the matrix network
///Spaces are custom groups designed in the matrix network
///Logical groups are for the client only grouping
///
enum GroupType {
    Favourite,
    LowPriority,
    Direct,
    Space,
    Logical
}


struct Group {
    spaces_id: Option<String>,
    name: String,
    group_type: GroupType,
    rooms: Vec<Room>
    
}
