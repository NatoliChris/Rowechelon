
///
/// DeviceSession object that represents  
/// device_name - records the device name of the session
/// ip_addr - ip address that it was registered for
/// last_seen - When this device was last active
/// verified - If it has been verified by your account
///
pub struct DeviceSession {
    device_name: String,
    device_id: String,
    last_seen: u64,
    verified: bool
}

impl DeviceSession {

    ///
    /// Creates a new DeviceSession object for the frontend
    ///
    pub fn new(device_name: String,
               device_id: String,
                         last_seen: u64,
                         verified: bool) -> Self {
        DeviceSession {
            device_name,
            device_id,
            last_seen,
            verified
        }
    }

    ///
    ///Retrieves the device name that was registered
    ///
    pub fn get_device_name(&self) -> &str {
        self.device_name.as_str()
    }

    ///
    /// Retrieves the device id
    ///
    pub fn get_device_id(&self) -> &str {
        self.device_id.as_str()
    }

    ///
    /// Gets the last seen timestamp
    ///
    pub fn get_last_seen(&self) -> u64 {
        self.last_seen
    }
    
    ///
    /// Returns if the gets the verified component
    ///
    pub fn get_verified(&self) -> bool {
        self.verified
    }

    ///
    /// Sets verified state
    ///
    pub fn set_verified(&mut self) {
        self.verified = true;
    }
}
