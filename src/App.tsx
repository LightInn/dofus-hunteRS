// frontend/src/App.tsx
import {
    BrowserRouter as Router,
    Route,
    Routes
} from "react-router-dom";
import Home from "./pages/home.tsx";
import SettingsPage from "./pages/SettingsPage.tsx";
import RegionSelector from "./components/RegionSelector.tsx";


function App() {

    return (
        <Router>
            <Routes>
                <Route path="/" element={<Home/>}/>
                <Route path="/settings" element={<SettingsPage/>}/>
                <Route path="/region/:region" element={<RegionSelector/>}/>
            </Routes>
        </Router>
    )
}

export default App