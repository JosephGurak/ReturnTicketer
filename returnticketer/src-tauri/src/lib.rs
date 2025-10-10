// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use std::sync::{Mutex, OnceLock};


static GLOBAL_TICKET: OnceLock<Mutex<Vec<Ticket>>> = OnceLock::new();

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
    fn add_ticket(self) {
        let add = GLOBAL_TICKET.get().unwrap();
        let mut add_lock = add.lock().unwrap();
        add_lock.push(self);
        //println!("{:?}", add_lock);
        
    }  

   

}


// now look into crates for displaying with docx and also removing unwanted entries before printing
#[tauri::command]
fn make_ticket(ticketId: &str, deviceId: &str, location: &str, issue: &str){
    GLOBAL_TICKET.get_or_init(|| Mutex::new(Vec::new()));

    let ticket = Ticket::new(ticketId.to_string(), deviceId.to_string(), location.to_string(), issue.to_string());
    ticket.add_ticket();

    let ticket_list = GLOBAL_TICKET.get().unwrap().lock().unwrap();
    println!("{:#?}", *ticket_list); 
   
}



#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![make_ticket])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
