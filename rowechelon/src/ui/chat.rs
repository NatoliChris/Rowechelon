
use crate::entity::room::Room;

///
/// Each window will have at least 1 tab,
///
///
struct ChatTab {
    title: String,
    room: Room,
}

impl ChatTab {
    pub fn new(title: String, room: Room) -> Self {
        ChatTab {
            title, room
        }
    }
}


///
/// The title of the chat window will hold the current
/// chat tab. 
/// Each room tab will hold its own room entity information
///
struct ChatWindow {
    title: String,
    roomtabs: Vec<ChatTab>,
    events: 
}
