import {Flag, X, ArrowRight} from "lucide-react";
import {HistoryPoint} from "../models/models.tsx";


interface HistoryListProps {
    points: HistoryPoint[];
    onSelect: (point: HistoryPoint) => void;
    onDelete: (id: string) => void;
}

export function HistoryList({points, onSelect, onDelete}: HistoryListProps) {


    if (points.length === 0) {
        return (
            <div
                style={{
                    padding: "1rem",
                    display: "flex",
                    justifyContent: "center",
                    alignItems: "center",
                    gap: "1rem",
                    borderRadius: "0.5rem",
                    border: "1px solid #ccc",
                }}
            >
                <span>No history yet</span>
            </div>
        )
    }


    return (
        <div
            style={{
                borderRadius: "0.5rem",
                border: "1px solid #ccc",
                overflow: "hidden",
                overflowY: "auto",
                maxHeight: "10rem",
            }}
        >
            <div style={{padding: "1rem", display: "flex", flexDirection: "column", gap: "0.5rem"}}>
                {points.map((point) => (
                    <div
                        // concatentate x + y to create a unique key
                        key={`${point.coord.x}${point.coord.y}`}
                        style={{
                            display: "flex",
                            alignItems: "center",
                            justifyContent: "space-between",
                            gap: "1rem",
                            borderRadius: "0.5rem",
                            border: "1px solid #ccc",
                            padding: "0.5rem",
                        }}
                    >
                        <div style={{display: "flex", alignItems: "center", gap: "0.5rem"}}>
                            {point.historyType == "start" ? (
                                <Flag style={{height: "1rem", width: "1rem", color: "green"}}/>
                            ) : (
                                <ArrowRight style={{height: "1rem", width: "1rem"}}/>
                            )}
                            <button
                                onClick={() => onSelect(point)}
                                style={{
                                    background: "none",
                                    border: "none",
                                    cursor: "pointer",
                                    fontSize: "1rem",
                                }}
                            >
                                ({point.coord.x}, {point.coord.y})
                            </button>
                        </div>
                        <button
                            onClick={() => onDelete(`${point.coord.x}${point.coord.y}`)}
                            style={{
                                height: "2rem",
                                width: "2rem",
                                display: "flex",
                                alignItems: "center",
                                justifyContent: "center",
                                border: "none",
                                background: "transparent",
                                cursor: "pointer",
                            }}
                        >
                            <X style={{height: "1rem", width: "1rem"}}/>
                        </button>
                    </div>
                ))}
            </div>
        </div>
    );
}
