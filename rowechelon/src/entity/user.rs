
use std::collections::HashMap;

use crate::entity::device_session::DeviceSession;

///
///User that is represented in rooms
/// :userid is the identifier and the domain
/// :nickname is the display name that the user picks
/// displaypic is the image they have for their user
pub struct User {
    pub userid: String,
    pub nickname: Option<String>,
    pub displaypic: Option<String>,
    pub sessions: HashMap<String, DeviceSession>
}

impl User {
    
    ///
    /// Creates a new user object
    /// Requires userid, nickname, displaypic.
    /// Sessions hashmap is constructed to store individual sessions
    ///
    pub fn new(userid: String, nickname: Option<String>, 
           displaypic: Option<String>) -> Self {
        
        User {
            userid,
            nickname,
            displaypic,
            sessions: HashMap::new()
        }
    }

    ///
    ///Sets a new nickname for the user, these will be triggered
    ///on a response on a nickname event
    pub fn set_nickname(&mut self, nickname: Option<String>) {
        self.nickname = nickname;
    }

    pub fn set_displaypic(&mut self, displaypic: Option<String>) {
        self.displaypic = displaypic;
    }
    
    ///
    /// Adds a new session that the user may have
    /// These sessions are devices that the user has going on.
    /// ie: Session on their current OS/Device and the identifier
    pub fn add_session(&mut self, session: DeviceSession) {
        self.sessions.insert(session.get_device_id().to_string(), session);
    } 

}
