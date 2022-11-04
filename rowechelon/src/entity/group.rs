
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


///
///The following is the group which contains
///associated rooms within this group.
///
///Group is reflective of both spaces and logical channel groups
///that the user wants.
///
struct Group {
    spaces_id: Option<String>,
    name: String,
    group_type: GroupType,
    rooms: Vec<Room>
}
