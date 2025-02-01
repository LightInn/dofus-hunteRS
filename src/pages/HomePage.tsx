// frontend/src/App.tsx
import {invoke} from "@tauri-apps/api/core";
import Monitor from "../components/Monitor.tsx";
import Navbar from "../components/Navbar.tsx";

function HomePage() {

    function handleCurrentCoord() {
        invoke('call_current_coord').then((response) => {
            console.log(response)
        })
    }

    function handleCaptureAnalyse() {
        invoke('call_capture_analyse').then((response) => {
            console.log(response)
        })
    }


    function handleArrowDirection() {
        invoke('call_arrow_direction').then((response) => {
            console.log(response)
        })
    }

    function handleSendApiRequest() {
        invoke('call_send_api_request').then((response) => {
            console.log(response)
        })
    }


    function handlePython() {
        invoke('call_python').then((response) => {
            console.log(response)
        })
    }


    return (
        <div>
            <Navbar/>

            <h1>Dofus Bot</h1>
            <Monitor/>
            <div style={{display: 'flex', flexDirection: "column", gap: '1rem'}}>
                <button onClick={handleCurrentCoord}>Current Coord</button>
                <button onClick={handleCaptureAnalyse}>Hunt Panel</button>
                <button onClick={handleArrowDirection}>Arrow Direction</button>
                <button onClick={handleSendApiRequest}>send API</button>
                <button onClick={handlePython}>ALL AUTO</button>
            </div>
        </div>
    )
}

export default HomePage