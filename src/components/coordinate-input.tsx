import {RefreshCw} from "lucide-react";
import {useEffect, useState} from "react";
import {invoke} from "@tauri-apps/api/core";
import useBotState from "../store/BotState.tsx";

export function CoordinateInput() {

    const stateCoordinates = useBotState((state) => state.botData.coords.current);
    const [coordinates, setCoordinates] = useState({x: 0, y: 0});


    useEffect(() => {
        setCoordinates(stateCoordinates);
    }, [stateCoordinates]);


    const handleChange = (axis: "x" | "y", value: string) => {
        const newCoordinates = {
            ...coordinates,
            [axis]: Number.parseInt(value) || 0,
        };
        setCoordinates(newCoordinates);

        invoke('call_set_coord', {x: newCoordinates.x, y: newCoordinates.y}
        ).then((response) => {
            console.log(response)
        })
    };


    function handleReload() {
        invoke('call_current_coord').then((response) => {
            console.log(response)
        })
    }


    return (
        <div style={{display: "flex", alignItems: "center", gap: "1rem"}}>
            <div style={{display: "flex", gap: "0.5rem"}}>
                <div style={{width: "4em", display: "flex", alignItems: "center", justifyContent: "space-between"}}>

                    <input
                        type="number"
                        value={coordinates.x}
                        onChange={(e) => handleChange("x", e.target.value)}
                        placeholder="X"
                        style={{
                            width: "80%",
                            padding: "0.5rem",
                            border: "1px solid #ccc",
                            borderRadius: "4px",
                        }}
                    />
                </div>
                <div style={{width: "4em", display: "flex", alignItems: "center", justifyContent: "space-between"}}>

                    <input
                        type="number"
                        value={coordinates.y}
                        onChange={(e) => handleChange("y", e.target.value)}
                        placeholder="Y"
                        style={{
                            width: "80%",
                            padding: "0.5rem",
                            border: "1px solid #ccc",
                            borderRadius: "4px",
                        }}
                    />
                </div>
            </div>
            <button
                onClick={handleReload}
                style={{
                    height: "2.5rem",
                    width: "2.5rem",
                    display: "flex",
                    alignItems: "center",
                    justifyContent: "center",
                    border: "none",
                    background: "transparent",
                    cursor: "pointer",
                }}
            >
                <RefreshCw style={{height: "1rem", width: "1rem"}}/>
            </button>
        </div>
    );
}
