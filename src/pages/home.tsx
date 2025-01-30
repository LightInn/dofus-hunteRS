// frontend/src/App.tsx
import {invoke} from "@tauri-apps/api/core";
import {useState} from 'react'

function Home() {
    const [status, setStatus] = useState('stopped')

    const handleStart = async () => {
        await invoke('start_bot')
        setStatus('running')
    }

    const handleStop = async () => {
        await invoke('stop_bot')
        setStatus('stopped')
    }

    function handleScreenshot() {
        invoke('take_screenshot').then((response) => {
            console.log(response)
        })
    }

    function handleCapture() {
        invoke('capture_game_region').then((response) => {
            console.log(response)
        })
    }

    function handleCaptureAnalyse() {
        invoke('capture_analyse').then((response) => {
            console.log(response)
        })
    }

    return (
        <div>
            <h1>Dofus Bot</h1>
            <p>Status: {status}</p>
            <div style={{display: 'flex', flexDirection: "column", gap: '1rem'}}>
                <button onClick={handleStart}>Start</button>
                <button onClick={handleStop}>Stop</button>
                <button onClick={handleScreenshot}>Screenshot</button>
                <button onClick={handleCapture}>Capture</button>
                <button onClick={handleCaptureAnalyse}>Capture + Analyse</button>
            </div>
        </div>
    )
}

export default Home