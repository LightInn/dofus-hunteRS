interface StatusIndicatorProps {
    label: string;
    status: "inactive" | "active" | "warning" | "error" | "loading";
}

export function StatusIndicator({ label, status }: StatusIndicatorProps) {
    const getStatusColor = () => {
        switch (status) {
            case "active":
                return "blue";
            case "inactive":
                return "gray";
            case "warning":
                return "yellow";
            case "error":
                return "red";
            case "loading":
                return "gray";
            default:
                return "gray";
        }
    };

    return (
        <div style={{ display: "flex", alignItems: "center", gap: "0.5rem" }}>
            <div
                style={{
                    height: "0.75rem",
                    width: "0.75rem",
                    borderRadius: "50%",
                    backgroundColor: getStatusColor(),
                }}
            />
            <span style={{ fontSize: "0.875rem", fontWeight: "500" }}>{label}</span>
        </div>
    );
}
