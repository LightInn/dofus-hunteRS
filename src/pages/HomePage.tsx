import {Settings, ArrowUp, ArrowDown, ArrowLeft, ArrowRight, LucideBug} from "lucide-react";
import {StatusIndicator} from "../components/status-indicator";
import {CoordinateInput} from "../components/coordinate-input";
import {HistoryList} from "../components/history-list";
import {useState} from "react";
import {Link} from "react-router-dom";

export default function ControlPanel() {
    const [isRunning, setIsRunning] = useState(false);
    const [apiStatus, setApiStatus] = useState<"inactive" | "loading" | "active" | "error">("inactive");
    const [direction, setDirection] = useState<"up" | "down" | "left" | "right">("up");
    const [hint, setHint] = useState("");
    const [points, setPoints] = useState([
        {id: "1", isStart: true, x: 0, y: 0},
        {id: "2", isStart: false, x: 10, y: 20},
    ]);

    const getDirectionIcon = () => {
        const iconStyle = {height: "1rem", width: "1rem"};
        switch (direction) {
            case "up":
                return <ArrowUp style={iconStyle}/>;
            case "down":
                return <ArrowDown style={iconStyle}/>;
            case "left":
                return <ArrowLeft style={iconStyle}/>;
            case "right":
                return <ArrowRight style={iconStyle}/>;
        }
    };


    return (
        <div style={{minHeight: "100vh", backgroundColor: "var(--background)"}}>
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
                    <StatusIndicator label="Running" status={isRunning ? "active" : "inactive"}/>
                    <StatusIndicator label="API" status={apiStatus}/>
                </div>

                <div style={{display: "flex", alignItems: "center", gap: "1rem"}}>
                    <CoordinateInput onReload={() => console.log("Reloading coordinates...")}/>
                    <button style={{width: "5rem"}}>{getDirectionIcon()}</button>
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
                    <button onClick={() => setIsRunning(!isRunning)}>
                        {isRunning ? "Stop" : "Start"}
                    </button>
                    <button>Setup</button>
                    <button>New</button>
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
