pub enum MessageType {
    Info,
    Success,
    Error,
    Process,
}
pub enum MessageEvent {
    Add,
    Delete,
    Read,
    Save,
    Scan,
    Move,
    Exist,
    NonExist,
}

pub struct Message {
    pub msg_type: MessageType,
    pub msg_event: MessageEvent,
}

impl Message {
    pub fn new(msg_type: MessageType, msg_event: MessageEvent) -> Self {
        Self {
            msg_type,
            msg_event,
        }
    }

    // fn generate_msg(&self) -> String {
    //     match self.msg_type {
    //         MessageType::Info => format!("Info: {} \r", self.get_message()),
    //         MessageType::Success => format!("Success: {} \r", self.get_message()),
    //         MessageType::Process => format!("Processed: {} \r", self.get_message()),
    //         MessageType::Error => format!("Error: {} \r", self.get_message()),
    //     }
    // }

    // fn get_message(&self) -> String {
    //     println!("\n");
    //     match self.msg_event {
    //         MessageEvent::Add => "Item added successfully".to_string(),
    //         MessageEvent::Delete => "Item deleted successfully".to_string(),
    //         MessageEvent::Read => "Item read successfully".to_string(),
    //         MessageEvent::Save => "Data saved successfully".to_string(),
    //         MessageEvent::Scan => "Scanning completed successfully".to_string(),
    //         MessageEvent::Move => "Item moved successfully".to_string(),
    //         MessageEvent::Exist => "Item exists".to_string(),
    //         MessageEvent::NonExist => "Item does not exist".to_string(),
    //     }
    // }
}
