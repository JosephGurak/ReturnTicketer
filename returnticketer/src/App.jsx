import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

const App = () => {
  //const [ticket, setTicket] = useState("");
  const [id, setid] = useState("");
  const [device, setdevice] = useState("");
  const [location, setlocation] = useState("");
  const [issue, setissue] = useState("");
  
  async function makeTicket() {
    await invoke("make_ticket", {id,device,location,issue});
  }

  async function create_word_docx() {
    await invoke("create_word_docx");
  }

  const body = [
    <main className="container">
      <h1>Return ticketer</h1>
      
      <p>for generating return tickets to place on devices</p>
      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          makeTicket();
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setid(e.currentTarget.value)}
          placeholder="Enter Ticket Number"
        />
        <input
          id="greet-input"
          onChange={(e) => setdevice(e.currentTarget.value)}
          placeholder="Enter Device Id"
        />
        <input
          id="greet-input"
          onChange={(e) => setlocation(e.currentTarget.value)}
          placeholder="Enter Location"
        />
        <input
          id="greet-input"
          onChange={(e) => setissue(e.currentTarget.value)}
          placeholder="Enter Issue"
        />
        <button type="submit">Add Ticket</button>
      </form>
      <form className="row"
            onSubmit={(e) => {
              e.preventDefault();
              create_word_docx();
            }}
      >
        <button type="submit">docx</button>
      </form>
      
    </main>
  ]
  return (
    <div>
      {body}
    </div>
  );
}

export default App;
