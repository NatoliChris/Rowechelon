
///
///User that is represented in rooms
/// :userid is the identifier and the domain
/// :nickname is the display name that the user picks
/// displaypic is the image they have for their user
pub struct User {
    pub userid: String,
    pub nickname: Option<String>,
    pub displaypic: Option<String>
}
