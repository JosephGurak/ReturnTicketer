// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

#[derive(Debug)]
struct Ticket {
    ticketId: String,
    deviceId: String,
    location: String,
    issue: String,
}

impl Ticket {
    fn new(ticketId: String, deviceId: String, location: String, issue: String) -> Self {
        Self {
            ticketId,
            deviceId,
            location,
            issue,
        }
    }

    

    /*
        functions to add new ticket to vector, vector of tickets displayed in a column .
        vector can have entries removed ,
        print button to print all the tickets in vector as a docx
    
     */

}



#[tauri::command]
fn make_ticket(ticketId: &str, deviceId: &str, location: &str, issue: &str) -> String {
    //println!("TicketId: {}, DeviceID: {}, Location: {}, Issue: {}", ticketId,deviceId,location,issue);
    format!("TicketId: {}, DeviceID: {}, Location: {}, Issue: {}", ticketId,deviceId,location,issue)
    
}



#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![make_ticket])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
