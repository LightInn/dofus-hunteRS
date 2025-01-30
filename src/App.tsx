// frontend/src/App.tsx
import {
    BrowserRouter as Router,
    Route,
    Link, Routes
} from "react-router-dom";
import Home from "./pages/home.tsx";
import SettingsPanel from "./components/SettingsPanel.tsx";


function App() {

    return (

        <Router>

            <div>
                <nav>
                    <ul>
                        <li>
                            <Link to="/">Home</Link>
                        </li>
                        <li>
                            <Link to="/settings">Settings</Link>
                        </li>

                    </ul>
                </nav>
            </div>


            <Routes>
                <Route path="/" element={<Home/>}/>
                <Route path="/settings" element={<SettingsPanel/>}/>
            </Routes>
        </Router>


    )
}

export default App