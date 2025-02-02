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

interface BotData {
    steps: Steps;
    coords: Coords;
    currentHint: string;
    currentArrow: string; // Ou un enum si vous préférez
}

interface InnerAppState {
    running: boolean;
    botData: BotData;
}


export type { InnerAppState, Coord, Coords, Steps, BotData };