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


function App() {

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