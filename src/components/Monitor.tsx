import React, { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";

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

const Monitor: React.FC = () => {
    // État local pour stocker les données reçues
    const [appState, setAppState] = useState<InnerAppState | null>(null);

    // Écouter les événements `state_changed`
    useEffect(() => {
        const unlisten = listen<InnerAppState>("state_changed", (event) => {
            console.log("Nouvel état reçu :", event.payload);
            setAppState(event.payload);
        });


        // Nettoyer l'écouteur d'événements lors du démontage du composant
        return () => {
            unlisten.then((fn) => fn());
        };
    }, []);

    if (!appState) {
        return <div>Chargement en cours...</div>;
    }

    console.log("État actuel de l'application :", appState);
    return (
        <div>
            <h1>État de l'application</h1>
            <div>
                <h2>Statut du bot : {appState.running ? "En cours" : "Arrêté"}</h2>
                <h3>Données du bot :</h3>
                <p>Étape actuelle : {appState.botData.steps.current}</p>
                <p>Étape totale : {appState.botData.steps.total}</p>
                <p>Indice actuel : {appState.botData.currentHint}</p>
                <p>Direction de la flèche : {appState.botData.currentArrow}</p>
                <h3>Coordonnées :</h3>
                <p>
                    Départ : ({appState.botData.coords.start.x}, {appState.botData.coords.start.y})
                </p>
                <p>
                    Actuel : ({appState.botData.coords.current.x}, {appState.botData.coords.current.y})
                </p>
                <p>
                    Cible : ({appState.botData.coords.target.x}, {appState.botData.coords.target.y})
                </p>
            </div>
        </div>
    );
};

export default Monitor;