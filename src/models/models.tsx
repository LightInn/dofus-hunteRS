// Définir les types pour l'état
interface Coord {
    x: number;
    y: number;
}

interface Coords {
    start: Coord;
    current: Coord;
    target: Coord;
}

interface Steps {
    current: number;
    total: number;
}

interface HistoryPoint {
    coord: Coord;
    historyType: "start" | "normal";
}

interface BotData {
    steps: Steps;
    coords: Coords;
    currentHint: string;
    currentArrow: "up" | "down" | "left" | "right" | "unknown";
    history: HistoryPoint[];
}


interface InnerAppState {
    running: boolean;
    apiStatus: "inactive" | "active" | "loading" | "error";
    botData: BotData;
}


export type {InnerAppState, Coord, Coords, Steps, BotData, HistoryPoint};