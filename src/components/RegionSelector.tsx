import {useEffect, useRef, useState} from 'react';
import {invoke} from '@tauri-apps/api/core';
import {listen} from '@tauri-apps/api/event';
import {useParams} from "react-router";
import {getCurrentWindow} from "@tauri-apps/api/window";

export default function RegionSelector() {
    const {region} = useParams<{ region?: string }>();
    console.log(region);


    const canvasRef = useRef<HTMLCanvasElement>(null);
    const [isDrawing, setIsDrawing] = useState(false);
    const [startPos, setStartPos] = useState({x: 0, y: 0});
    const [currentPos, setCurrentPos] = useState({x: 0, y: 0});


    async function handleSaveRegion() {
        await invoke('save_region', {
            regionData: {
                region,
                coordinates: [startPos.x, startPos.y, currentPos.x, currentPos.y]
            }
        });
    }


    useEffect(() => {
        const setupCanvas = async () => {
            const canvas = canvasRef.current!;
            canvas.width = window.innerWidth;  // Utiliser la largeur de la fenêtre
            canvas.height = window.innerHeight; // Utiliser la hauteur de la fenêtre
        };

        setupCanvas();

        const unlisten = listen('keydown', (event) => {
            if (event.payload === 'Escape') {
                // Fermer la fenêtre ou annuler la sélection
            }
        });

        return () => {
            unlisten.then(f => f());
        };
    }, []);

    const draw = () => {
        const canvas = canvasRef.current!;
        const ctx = canvas.getContext('2d')!;
        ctx.clearRect(0, 0, canvas.width, canvas.height);

        if (isDrawing) {
            ctx.strokeStyle = '#ff0000';
            ctx.lineWidth = 2;
            ctx.strokeRect(
                startPos.x,
                startPos.y,
                currentPos.x - startPos.x,
                currentPos.y - startPos.y
            );
        }
    };

    useEffect(() => {
        draw();
    }, [currentPos, isDrawing]);

    const handleMouseDown = (e: React.MouseEvent) => {
        const canvas = canvasRef.current!;
        const rect = canvas.getBoundingClientRect(); // Obtenir la position réelle du canvas

        setIsDrawing(true);
        setStartPos({
            x: e.clientX - rect.left, // Ajuster les coordonnées X
            y: e.clientY - rect.top,  // Ajuster les coordonnées Y
        });
        setCurrentPos({
            x: e.clientX - rect.left,
            y: e.clientY - rect.top,
        });
    };

    const handleMouseMove = (e: React.MouseEvent) => {
        if (!isDrawing) return;

        const canvas = canvasRef.current!;
        const rect = canvas.getBoundingClientRect(); // Obtenir la position réelle du canvas

        setCurrentPos({
            x: e.clientX - rect.left, // Ajuster les coordonnées X
            y: e.clientY - rect.top,  // Ajuster les coordonnées Y
        });
    };

    const handleMouseUp = () => {
        if (!isDrawing) return;
        setIsDrawing(false);

        const x1 = Math.min(startPos.x, currentPos.x);
        const x2 = Math.max(startPos.x, currentPos.x);
        const y1 = Math.min(startPos.y, currentPos.y);
        const y2 = Math.max(startPos.y, currentPos.y);

        console.log([x1, y1, x2, y2]);
        handleSaveRegion().then(() => {
            // Fermer la fenêtre
            getCurrentWindow().destroy();
        });
    };

    return (
        <div style={{
            position: 'fixed',
            top: 0,
            left: 0,
            width: '100vw',
            height: '100vh',
            backgroundColor: 'rgba(0,0,0,0.3)',
            cursor: 'crosshair'
        }}>
            <canvas
                ref={canvasRef}
                onMouseDown={handleMouseDown}
                onMouseMove={handleMouseMove}
                onMouseUp={handleMouseUp}
                style={{width: '100%', height: '100%'}}
            />
        </div>
    );
}