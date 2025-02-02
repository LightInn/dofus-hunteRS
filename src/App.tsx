// frontend/src/App.tsx
import {
    BrowserRouter as Router,
    Route,
    Routes
} from "react-router-dom";
import HomePage from "./pages/HomePage.tsx";
import SettingsPage from "./pages/SettingsPage.tsx";
import RegionSelector from "./components/RegionSelector.tsx";
import DebugPage from "./pages/DebugPage.tsx";
import {useEffect} from "react";
import {InnerAppState} from "./models/models.tsx";
import {listen} from "@tauri-apps/api/event";
import useBotState from "./store/BotState.tsx";


function App() {


    const updateBotState = useBotState((state) => state.updateBotState);

    // Écouter les événements `state_changed`
    useEffect(() => {
        const unlisten = listen<InnerAppState>("state_changed", (event) => {
            console.log("Nouvel état reçu :", event.payload);
            updateBotState(event.payload);
        });


        // Nettoyer l'écouteur d'événements lors du démontage du composant
        return () => {
            unlisten.then((fn) => fn());
        };
    }, []);

    return (
        <Router>
            <Routes>
                <Route path="/" element={<HomePage/>}/>
                <Route path="/settings" element={<SettingsPage/>}/>
                <Route path="/debug" element={<DebugPage/>}/>
                <Route path="/region/:region" element={<RegionSelector/>}/>
            </Routes>
        </Router>
    )
}

export default App