import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

const App = () => {
  const [ticket, setTicket] = useState("");
  const [ticketId, setticketId] = useState("");
  const [deviceId, setdeviceId] = useState("");
  const [location, setlocation] = useState("");
  const [issue, setissue] = useState("");
  
  async function makeTicket() {
    setTicket(await invoke("make_ticket", {ticketId,deviceId,location,issue}));
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
          onChange={(e) => setticketId(e.currentTarget.value)}
          placeholder="Enter Ticket Number"
        />
        <input
          id="greet-input"
          onChange={(e) => setdeviceId(e.currentTarget.value)}
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
        <button type="submit">Print</button>
      </form>
      <p>{ticket}</p>
      
    </main>
  ]
  return (
    <div>
      {body}
    </div>
  );
}

export default App;
