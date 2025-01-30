import React, {useState, useEffect} from 'react';
import {invoke} from "@tauri-apps/api/core";

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

export default function SettingsPanel() {
    const [config, setConfig] = useState<BotConfig | null>(null);

    useEffect(() => {
        loadConfig();
    }, []);

    const loadConfig = async () => {
        const config = await invoke<BotConfig>('get_config');
        setConfig(config);
    };

    const updateConfig = async (section: keyof BotConfig, field: string, value: any) => {
        const updated = {...config, [section]: {...config[section], [field]: value}};
        await invoke('update_config', {newConfig: updated});
        setConfig(updated);
    };

    if (!config) return <div>Loading...</div>;

    return (
        <div className="settings-container">
            <div className="settings-section">
                <h3>Window Settings</h3>
                <label>
                    Window Title:
                    <input
                        type="text"
                        value={config.window.title}
                        onChange={(e) => updateConfig('window', 'title', e.target.value)}
                    />
                </label>

                <label>
                    Focus Chat Binding:
                    <input
                        type="text"
                        value={config.window.focus_chat_binding}
                        onChange={(e) => updateConfig('window', 'focus_chat_binding', e.target.value)}
                    />
                </label>
            </div>

            <div className="settings-section">
                <h3>Screen Regions</h3>
                {Object.entries(config.regions).map(([name, values]) => (
                    <div key={name} className="region-input">
                        <label>
                            {name.replace('_', ' ')}:
                            <div className="coordinate-inputs">
                                {values.map((val, i) => (
                                    <input
                                        key={i}
                                        type="number"
                                        value={val}
                                        onChange={(e) => {
                                            const newValues = [...values];
                                            newValues[i] = parseInt(e.target.value);
                                            updateConfig('regions', name, newValues);
                                        }}
                                    />
                                ))}
                            </div>
                        </label>
                    </div>
                ))}
            </div>

            {/* Ajouter d'autres sections de configuration ici */}
        </div>
    );
}