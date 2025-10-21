// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use docx_rs::*;
use std::sync::{Mutex, OnceLock};


static GLOBAL_TICKET: OnceLock<Mutex<Vec<Ticket>>> = OnceLock::new();

#[derive(Debug, Clone)]
struct Ticket {
    id: String,
    device: String,
    location: String,
    issue: String,
}

impl Ticket {
    fn new(id: String, device: String, location: String, issue: String) -> Self {
        Self {
            id,
            device,
            location,
            issue,
        }
    }
    
    fn add_ticket(&self) {
        let add = GLOBAL_TICKET.get().unwrap();
        let mut add_lock = add.lock().unwrap();
        add_lock.push(self.clone());
       // println!("Ticekts: {:?}", add_lock);
    }

   

}


fn remove_ticket_by_id(id: &str) {
    if let Some(tickets) = GLOBAL_TICKET.get() {
        let mut lock = tickets.lock().unwrap();
        if let Some(pos) = lock.iter().position(|ticket| ticket.id == id) {
            lock.remove(pos);
        }
    }
}

fn edit_issue(id: &str, new_issue: &str) {
    if let Some(tickets) = GLOBAL_TICKET.get() {
        let mut lock = tickets.lock().unwrap();
        if let Some(ticket) = lock.iter_mut().find(|t| t.id == id) {
            ticket.issue = new_issue.to_string();
        } else {
            eprintln!("Ticket id: {} not found", id);
        }
    } else {
        eprintln!("Could not access GLOBAL_TICKETS");
    }
}

fn edit_device(id: &str, new_device: &str) {
    if let Some(tickets) = GLOBAL_TICKET.get() {
        let mut lock = tickets.lock().unwrap();
        if let Some(ticket) = lock.iter_mut().find(|t| t.id == id) {
            ticket.device = new_device.to_string();
        } else {
            eprintln!("Ticket id: {} not found", id);
        }
    } else {
        eprintln!("Could not access GLOBAL_TICKETS");
    }
}

fn edit_location(id: &str, new_location: &str) {
    if let Some(tickets) = GLOBAL_TICKET.get() {
        let mut lock = tickets.lock().unwrap();
        if let Some(ticket) = lock.iter_mut().find(|t| t.id == id) {
            ticket.location = new_location.to_string();
        } else {
            eprintln!("Ticket id: {} not found", id);
        }
    } else {
        eprintln!("Could not access GLOBAL_TICKETS");
    }
}

fn edit_id(id: &str, new_id: &str) {
    if let Some(tickets) = GLOBAL_TICKET.get() {
        let mut lock = tickets.lock().unwrap();
        if let Some(ticket) = lock.iter_mut().find(|t| t.id == id) {
            ticket.id = new_id.to_string();
        } else {
            eprintln!("Ticket id: {} not found", id);
        }
    } else {
        eprintln!("Could not access GLOBAL_TICKETS");
    }
}


#[tauri::command]
fn make_ticket(id: &str, device: &str, location: &str, issue: &str){
    GLOBAL_TICKET.get_or_init(|| Mutex::new(Vec::new()));

    let ticket = Ticket::new(id.to_string(), device.to_string(), location.to_string(), issue.to_string());
    ticket.add_ticket();
    
    // this and create_word_docx cant run at same time, possible blocking
    // let ticket_list = GLOBAL_TICKET.get().unwrap().lock().unwrap();
    // println!("{:#?}", *ticket_list);

}

#[tauri::command]
async fn create_word_docx() {
    let ticket_list = GLOBAL_TICKET.get().unwrap().lock().unwrap();
    let mut doc = Docx::new();

    for ticket in ticket_list.iter() {
        let paragraph = Paragraph::new()
            .add_run(Run::new().add_text(format!("Ticket ID: {}", ticket.id)))
            .add_run(Run::new().add_break(BreakType::TextWrapping))
            .add_run(Run::new().add_text(format!("Device: {}", ticket.device)))
            .add_run(Run::new().add_break(BreakType::TextWrapping))
            .add_run(Run::new().add_text(format!("Location: {}", ticket.location)))
            .add_run(Run::new().add_break(BreakType::TextWrapping))
            .add_run(Run::new().add_text(format!("Issue: {}", ticket.issue)))
            .add_run(Run::new().add_break(BreakType::TextWrapping))
            .add_run(Run::new().add_break(BreakType::TextWrapping));

        doc = doc.add_paragraph(paragraph);
    }

    let path = std::path::Path::new("./tickets.docx");
    let file = std::fs::File::create(path).unwrap();
    doc.build().pack(file);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![make_ticket, create_word_docx])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
