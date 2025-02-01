import { RefreshCw } from "lucide-react";
import { useState } from "react";

interface CoordinateInputProps {
    onReload?: () => void;
    onChange?: (x: number, y: number) => void;
}

export function CoordinateInput({ onReload, onChange }: CoordinateInputProps) {
    const [coordinates, setCoordinates] = useState({ x: 0, y: 0 });

    const handleChange = (axis: "x" | "y", value: string) => {
        const newCoordinates = {
            ...coordinates,
            [axis]: Number.parseInt(value) || 0,
        };
        setCoordinates(newCoordinates);
        onChange?.(newCoordinates.x, newCoordinates.y);
    };

    return (
        <div style={{ display: "flex", alignItems: "center", gap: "1rem" }}>
            <div style={{ display: "flex", gap: "0.5rem" }}>
                <div style={{ width: "6rem" }}>
                    <input
                        type="number"
                        value={coordinates.x}
                        onChange={(e) => handleChange("x", e.target.value)}
                        placeholder="X"
                        style={{
                            width: "100%",
                            padding: "0.5rem",
                            border: "1px solid #ccc",
                            borderRadius: "4px",
                        }}
                    />
                </div>
                <div style={{ width: "6rem" }}>
                    <input
                        type="number"
                        value={coordinates.y}
                        onChange={(e) => handleChange("y", e.target.value)}
                        placeholder="Y"
                        style={{
                            width: "100%",
                            padding: "0.5rem",
                            border: "1px solid #ccc",
                            borderRadius: "4px",
                        }}
                    />
                </div>
            </div>
            <button
                onClick={onReload}
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
                <RefreshCw style={{ height: "1rem", width: "1rem" }} />
            </button>
        </div>
    );
}
