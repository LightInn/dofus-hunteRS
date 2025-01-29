// frontend/src/App.tsx
import { invoke } from "@tauri-apps/api/core";
import { useState } from 'react'

function App() {
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

    return (
        <div>
            <h1>Dofus Bot</h1>
            <p>Status: {status}</p>
            <button onClick={handleStart}>Start</button>
            <button onClick={handleStop}>Stop</button>
            <button onClick={handleScreenshot}>Screenshot</button>
        </div>
    )
}

export default App