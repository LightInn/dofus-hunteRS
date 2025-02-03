import {ArrowDown, ArrowLeft, ArrowRight, ArrowUp, CircleIcon} from "lucide-react";
import {getCurrentWebviewWindow} from "@tauri-apps/api/webviewWindow";
import {PhysicalSize, PhysicalPosition} from "@tauri-apps/api/dpi";
import {useState, CSSProperties, useEffect} from "react";
import useBotState from "../store/BotState.tsx";


// Fonction d'interpolation pour une animation fluide
function lerp(start: number, end: number, t: number): number {
    return start * (1 - t) + end * t;
}

// Fonction pour animer la fenêtre
async function animateWindow(
    window: any,
    startSize: { width: number; height: number },
    endSize: { width: number; height: number },
    startPos: { x: number; y: number },
    endPos: { x: number; y: number },
    duration: number = 1
) {
    const steps = 4; // Nombre d'étapes pour l'animation
    const stepDuration = duration / steps;

    for (let i = 0; i <= steps; i++) {
        const t = i / steps; // Progression de 0 à 1

        // Utilisation d'une fonction d'accélération (easing)
        const easeT = 1 - Math.pow(1 - t, 3); // Cubic ease-out

        // Interpolation de la taille et de la position
        const currentWidth = Math.round(lerp(startSize.width, endSize.width, easeT));
        const currentHeight = Math.round(lerp(startSize.height, endSize.height, easeT));
        const currentX = Math.round(lerp(startPos.x, endPos.x, easeT));
        const currentY = Math.round(lerp(startPos.y, endPos.y, easeT));

        await window.setSize(new PhysicalSize(currentWidth, currentHeight));
        await window.setPosition(new PhysicalPosition(currentX, currentY));

        // Attendre un court instant entre chaque étape
        await new Promise(resolve => setTimeout(resolve, stepDuration));
    }
}


export default function ArrowSelector()
 {
    const arrowState = useBotState((state) => state.botData.currentArrow);

    const [isMenuOpen, setIsMenuOpen] = useState(false);
    const [isOverlayOpen, setIsOverlayOpen] = useState(false);
    const [originalSize, setOriginalSize] = useState<PhysicalSize | null>(null);
    const [originalPosition, setOriginalPosition] = useState<PhysicalPosition | null>(null);
    const [selectedDirection, setSelectedDirection] = useState(arrowState);


    const SMALL_SIZE = 300;


    useEffect(() => {
        setSelectedDirection(arrowState);
    }, [arrowState]);


    async function handleMouseDown(e: React.MouseEvent) {
        e.preventDefault();

        const window = getCurrentWebviewWindow();
        const currentPosition = await window.outerPosition();
        setOriginalPosition(currentPosition);
        const currentSize = await window.size();
        setOriginalSize(currentSize);

        // Obtenir les coordonnées globales de la souris
        // e.clientX/Y sont relatifs à la fenêtre, on ajoute donc la position actuelle de la fenêtre
        const globalMouseX = currentPosition.x + e.clientX;
        const globalMouseY = currentPosition.y + e.clientY;

        // Calculer la nouvelle position pour que la souris soit au centre
        // On soustrait la moitié de la nouvelle taille pour centrer
        const newX = globalMouseX - (SMALL_SIZE / 2);
        const newY = globalMouseY - (SMALL_SIZE / 2);

        setIsOverlayOpen(true);

        // Animer la transition au lieu de la faire instantanément
        await animateWindow(
            window,
            {width: currentSize.width, height: currentSize.height},
            {width: SMALL_SIZE, height: SMALL_SIZE},
            {x: currentPosition.x, y: currentPosition.y},
            {x: newX, y: newY}
        );

        setIsMenuOpen(true);
    }

    async function handleDirectionSelect(newDirection: "up" | "down" | "left" | "right") {
        setSelectedDirection(newDirection);

        // todo : envoyer la nouvelle direction au bot
        // onDirectionChange?.(newDirection);
        //
        setIsMenuOpen(false);


        const window = getCurrentWebviewWindow();
        if (originalSize && originalPosition) {
            // Animer le retour à la taille/position d'origine
            await animateWindow(
                window,
                {width: SMALL_SIZE, height: SMALL_SIZE},
                {width: originalSize.width, height: originalSize.height},
                await window.outerPosition(),
                originalPosition
            );
        }

        setIsOverlayOpen(false);


    }

    const getDirectionIcon = (dir: typeof arrowState) => {
        const iconStyle: CSSProperties = {
            height: '1.5rem',
            width: '1.5rem',
        };
        const icons = {
            up: <ArrowUp style={iconStyle}/>,
            down: <ArrowDown style={iconStyle}/>,
            left: <ArrowLeft style={iconStyle}/>,
            right: <ArrowRight style={iconStyle}/>,
            unknown: <CircleIcon style={iconStyle}/>
        };
        return icons[dir];
    };

    const containerStyle: CSSProperties = {
        position: 'relative',
    };

    const mainButtonStyle: CSSProperties = {
        width: '64px',
        height: '64px',
        borderRadius: '50%',
        backgroundColor: '#f3f4f6',
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'center',
        position: 'relative',
        zIndex: 10,
        border: 'none',
        cursor: 'pointer',
        transition: 'all 0.2s ease',
        ...(isMenuOpen && {
            boxShadow: '0 0 0 2px #3b82f6'
        })
    };

    const overlayStyle: CSSProperties = {
        position: 'fixed',
        width: `100vw`,
        height: `100vh`,
        backgroundColor: 'rgb(255,255,255)',
        top: 0,
        left: 0,
        zIndex: 20,
    };

    const radialMenuStyle: CSSProperties = {
        position: 'fixed',
        width: `${SMALL_SIZE}px`,
        height: `${SMALL_SIZE}px`,
        top: 0,
        left: 0,
        zIndex: 30,
    };

    const getRadialButtonStyle = (dir: typeof arrowState): CSSProperties => {
        const baseStyle: CSSProperties = {
            position: 'absolute',
            width: '56px',
            height: '56px',
            borderRadius: '50%',
            backgroundColor: 'white',
            boxShadow: '0 4px 6px -1px rgba(0, 0, 0, 0.1)',
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'center',
            border: 'none',
            cursor: 'pointer',
            transition: 'all 0.2s ease',
            ...(dir === selectedDirection && {
                boxShadow: '0 0 0 2px #3b82f6'
            })
        };

        // Position les boutons par rapport au centre de la fenêtre redimensionnée
        const positions = {
            "up": {
                top: `${SMALL_SIZE / 2 - 92}px`,
                left: `${SMALL_SIZE / 2 - 28}px`
            },
            "down": {
                top: `${SMALL_SIZE / 2 + 36}px`,
                left: `${SMALL_SIZE / 2 - 28}px`
            },
            "left": {
                top: `${SMALL_SIZE / 2 - 28}px`,
                left: `${SMALL_SIZE / 2 - 92}px`
            },
            "right": {
                top: `${SMALL_SIZE / 2 - 28}px`,
                left: `${SMALL_SIZE / 2 + 36}px`
            },
            "unknown": {
                top: `${SMALL_SIZE / 2 - 28}px`,
                left: `${SMALL_SIZE / 2 - 28}px`
            }
        };

        return {...baseStyle, ...positions[dir]};
    };


    async function handleOverlay() {

        console.log('overlay');

        setIsMenuOpen(false);
        const window = getCurrentWebviewWindow();
        if (originalSize && originalPosition) {
            await animateWindow(
                window,
                {width: SMALL_SIZE, height: SMALL_SIZE},
                {width: originalSize.width, height: originalSize.height},
                await window.outerPosition(),
                originalPosition
            );
        }

        setIsOverlayOpen(false);


    }

    return (
        <div style={containerStyle}>
            <button
                style={mainButtonStyle}
                onMouseDown={handleMouseDown}
                onMouseEnter={(e) => {
                    (e.target as HTMLButtonElement).style.backgroundColor = '#f3f4f6';
                }}
                onMouseLeave={(e) => {
                    (e.target as HTMLButtonElement).style.backgroundColor = '#f3f4f6';
                }}
            >
                {getDirectionIcon(selectedDirection)}
            </button>

            {isOverlayOpen && (

                <div
                    style={overlayStyle}
                />)}

            {isMenuOpen && (
                <>
                    <div style={radialMenuStyle}
                         onMouseLeave={handleOverlay}
                         onMouseUp={handleOverlay}
                         onClick={handleOverlay}>
                        {(['up', 'down', 'left', 'right'] as const).map((dir) => (
                            <button
                                key={dir}
                                style={getRadialButtonStyle(dir)}
                                onClick={() => handleDirectionSelect(dir)}
                                onMouseEnter={(e) => {
                                    const btn = e.target as HTMLButtonElement;
                                    btn.style.backgroundColor = '#f3f4f6';
                                    btn.style.transform = btn.style.transform + ' scale(1.1)';
                                }}
                                onMouseLeave={(e) => {
                                    const btn = e.target as HTMLButtonElement;
                                    btn.style.backgroundColor = 'white';
                                    btn.style.transform = btn.style.transform.replace(' scale(1.1)', '');
                                }}
                                onMouseUp={() => {
                                    handleDirectionSelect(dir)
                                }}
                            >
                                {getDirectionIcon(dir)}
                            </button>
                        ))
                        }
                    </div>
                </>
            )}
        </div>
    );
}