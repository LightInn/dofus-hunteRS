import { Flag, X, ArrowRight } from "lucide-react";

interface HistoryPoint {
    id: string;
    isStart: boolean;
    x: number;
    y: number;
}

interface HistoryListProps {
    points: HistoryPoint[];
    onSelect: (point: HistoryPoint) => void;
    onDelete: (id: string) => void;
}

export function HistoryList({ points, onSelect, onDelete }: HistoryListProps) {
    return (
        <div
            style={{
                borderRadius: "0.5rem",
                border: "1px solid #ccc",
                overflow: "hidden",
            }}
        >
            <div style={{ padding: "1rem", display: "flex", flexDirection: "column", gap: "0.5rem" }}>
                {points.map((point) => (
                    <div
                        key={point.id}
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
                        <div style={{ display: "flex", alignItems: "center", gap: "0.5rem" }}>
                            {point.isStart ? (
                                <Flag style={{ height: "1rem", width: "1rem", color: "green" }} />
                            ) : (
                                <ArrowRight style={{ height: "1rem", width: "1rem" }} />
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
                                ({point.x}, {point.y})
                            </button>
                        </div>
                        <button
                            onClick={() => onDelete(point.id)}
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
                            <X style={{ height: "1rem", width: "1rem" }} />
                        </button>
                    </div>
                ))}
            </div>
        </div>
    );
}
