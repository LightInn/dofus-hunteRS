import {useState, useEffect} from 'react';
import {invoke} from "@tauri-apps/api/core";
import {WebviewWindow} from "@tauri-apps/api/webviewWindow";
import Navbar from "../components/Navbar.tsx";

interface BotConfig {
    api: {
        url: string;
        token: string;
    };
    window: {
        title: string;
        focus_chat_binding: string;
    };
    regions: {
        coordinates: [number, number, number, number];
        hunt_panel: [number, number, number, number];
        chat: [number, number, number, number];
    };
    // ... autres sections
}

export default function SettingsPage() {
    const [config, setConfig] = useState<BotConfig | null>(null);
    useEffect(() => {
        loadConfig();
    }, []);

    const loadConfig = async () => {
        const config = await invoke<BotConfig>('call_get_config');
        setConfig(config);
    };

    const updateConfig = async (section: keyof BotConfig, field: string, value: any) => {
        if (!config) return

        const updated = {...config, [section]: {...config[section], [field]: value}};
        await invoke('call_update_config', {newConfig: updated});
        setConfig(updated);
    };


    async function createOverlayWindow(name: string) {
        const overlay = new WebviewWindow('region_selector', {
            url: `region/${name}`, // Chargez une page dédiée pour la sélection de région
            width: 1920,
            height: 1080,
            hiddenTitle: true, // Masque le titre de la fenêtre
            transparent: true, // Rend la fenêtre transparente
            decorations: false, // Supprime les bordures et la barre de titre
            fullscreen: true, // Rend la fenêtre pleine écran
            alwaysOnTop: true, // Garde la fenêtre au premier plan
            skipTaskbar: true, // Masque la fenêtre de la barre des tâches
            closable: true,
        });

        // Gestion des événements
        overlay.once('tauri://created', () => {
            console.log('Overlay window created successfully');
        });

        overlay.once('tauri://error', (e) => {
            console.error('Failed to create overlay window:', e);
        });
    }

    if (!config) return <div>Loading...</div>;

    return (
        <div className="settings-container">


            <Navbar/>


            <div className="settings-section">
                <h3>Window Settings</h3>
                <label style={{display: 'block'}}>
                    Window Title:

                    <input
                        type="text"
                        value={config.window.title}
                        onChange={(e) => updateConfig('window', 'title', e.target.value)}
                    />
                </label>


                <label style={{display: 'block'}}>
                    Focus Chat Binding:

                    <input
                        type="text"
                        value={config.window.focus_chat_binding}
                        onChange={(e) => updateConfig('window', 'focus_chat_binding', e.target.value)}
                    />
                </label>
            </div>


            <div className="settings-section">
                <h3>API Settings</h3>
                <label style={{display: 'block'}}>
                    API url
                    <input
                        type="text"
                        value={config.api.url}
                        onChange={(e) => updateConfig('api', 'url', e.target.value)}
                    />
                </label>


                <label style={{display: 'block'}}>
                    API Token
                    <input
                        type="text"
                        value={config.api.token}
                        onChange={(e) => updateConfig('api', 'token', e.target.value)}
                    />
                </label>
            </div>

            <div className="settings-container">
                <div className="settings-section">
                    <h3>Screen Regions</h3>
                    {Object.entries(config?.regions || {}).map(([name]) => (
                        <div key={name} className="region-input">
                            <label>
                                {name.replace('_', ' ')}:
                                <button onClick={() => createOverlayWindow(name)}>
                                    Select Region
                                </button>
                            </label>
                        </div>
                    ))}
                </div>
            </div>

            {/* Ajouter d'autres sections de configuration ici */}
        </div>
    );
}