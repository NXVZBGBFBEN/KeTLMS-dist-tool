import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";
import { open } from "@tauri-apps/api/shell";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke("greet", { name }));
  }

  async function authorize() {
    const unlisten = await listen<string>("oauth2-authorization", event => {
      open(event.payload);
      unlisten();
    });
    invoke("authorize").then(() => console.log("ok"));
  }

  return (
    <div className="">
      <div className="navbar bg-base-100">
        <h1 className="text-2xl">KeTLMS-dist-tool</h1>
      </div>

      <div className="grid grid-cols-3">
        <div className="col-span-1">
          <ul className="steps steps-vertical">
            <li className="step">Login</li>
          </ul>
        </div>

        <div className="col-span-2">
          <h1>Welcome to Tauri!</h1>
          <h1 className="text-3xl font-bold underline">Welcome to Tauri!</h1>

          <p>Click on the Tauri, Vite, and React logos to learn more.</p>

          <form
            className="row"
            onSubmit={(e) => {
              e.preventDefault();
              greet();
            }}
          >
            <input
              id="greet-input"
              onChange={(e) => setName(e.currentTarget.value)}
              placeholder="Enter a name..."
            />
            <button type="submit">Greet</button>
          </form>

          <p>{greetMsg}</p>

          <button onClick={authorize}>Authorize</button>
        </div>
      </div>
    </div>
  );
}

export default App;
