import {Settings, LucideBug} from "lucide-react";
import {StatusIndicator} from "../components/status-indicator";
import {CoordinateInput} from "../components/coordinate-input";
import {HistoryList} from "../components/history-list";
import {useEffect, useState} from "react";
import {Link} from "react-router-dom";
import ArrowSelector from "../components/arrow-selector.tsx";
import useBotState from "../store/BotState.tsx";
import {invoke} from "@tauri-apps/api/core";

export default function HomePage() {
    const state = useBotState((state) => state);

    const [isRunning, setIsRunning] = useState(false);
    const [apiStatus] = useState<"inactive" | "loading" | "active" | "error">("inactive");
    const [hint, setHint] = useState("");
    const [points, setPoints] = useState([
        {id: "1", isStart: true, x: 0, y: 0},
        {id: "2", isStart: false, x: 10, y: 20},
    ]);

    useEffect(() => {

        setIsRunning(state.running);
        setHint(state.botData.currentHint);

    }, [state]);

    function handleLaunch() {
        setIsRunning(!isRunning);
        invoke('call_python').then((response) => {
            console.log(response)
        })
    }

    function handleManual(){
            setIsRunning(!isRunning);
            invoke('call_manual').then((response) => {
                console.log(response)
            })
    }


    return (
        <div
            style={{minHeight: "100vh", backgroundColor: "var(--background)", overflow: "hidden", overflowY: "hidden"}}>
            <header
                style={{
                    display: "flex",
                    alignItems: "center",
                    justifyContent: "space-between",
                    borderBottom: "1px solid #ccc",
                    padding: "1rem",
                }}
            >
                <div style={{fontSize: "1.5rem", fontWeight: "bold"}}>Logo</div>

                <div style={{display: "flex", gap: "1rem"}}>
                    <Link to="/debug">
                        <LucideBug style={{height: "1.25rem", width: "1.25rem"}}/>
                    </Link>

                    <Link to="/settings">
                        <Settings style={{height: "1.25rem", width: "1.25rem"}}/>
                    </Link>
                </div>
            </header>

            <main
                style={{
                    maxWidth: "48rem",
                    margin: "0 auto",
                    padding: "1.5rem",
                    display: "flex",
                    flexDirection: "column",
                    gap: "1.5rem",
                }}
            >
                <div style={{display: "flex", gap: "1rem"}}>
                    <StatusIndicator label="Running" status={state.running ? 'active' : 'inactive'}/>
                    <StatusIndicator label="API" status={state.apiStatus}/>
                </div>

                <div style={{display: "flex", alignItems: "center", gap: "1rem"}}>
                    <CoordinateInput onReload={() => console.log("Reloading coordinates...")}/>
                    <ArrowSelector/>
                </div>

                <input
                    value={hint}
                    onChange={(e) => setHint(e.target.value)}
                    placeholder="Current hint..."
                    style={{
                        padding: "0.5rem",
                        border: "1px solid #ccc",
                        borderRadius: "4px",
                        width: "100%",
                    }}
                />

                <div style={{display: "flex", gap: "0.5rem"}}>
                    <button onClick={() => handleLaunch()}>
                        {isRunning ? "Stop" : "Start"}
                    </button>
                    <button>Setup</button>
                    <button>New</button>
                    <button onClick={() => handleManual()}>Manual</button>
                </div>

                <HistoryList
                    points={points}
                    onSelect={(point) => console.log("Selected point:", point)}
                    onDelete={(id) => setPoints(points.filter((p) => p.id !== id))}
                />
            </main>
        </div>
    );
}
