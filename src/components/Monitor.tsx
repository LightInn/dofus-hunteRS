import useBotState from "../store/BotState.tsx";


function Monitor() {

    const state = useBotState((state) => state);


    if (!state) {
        return <div>Chargement en cours...</div>;
    }

    console.log("État actuel de l'application :", state);
    return (
        <div>
            <h1>État de l'application</h1>
            <div>
                <h2>Statut du bot : {state.running ? "En cours" : "Arrêté"}</h2>
                <h3>Données du bot :</h3>
                <p>Étape actuelle : {state.botData.steps.current}</p>
                <p>Étape totale : {state.botData.steps.total}</p>
                <p>Indice actuel : {state.botData.currentHint}</p>
                <p>Direction de la flèche : {state.botData.currentArrow}</p>
                <h3>Coordonnées :</h3>
                <p>
                    Départ : ({state.botData.coords.start.x}, {state.botData.coords.start.y})
                </p>
                <p>
                    Actuel : ({state.botData.coords.current.x}, {state.botData.coords.current.y})
                </p>
                <p>
                    Cible : ({state.botData.coords.target.x}, {state.botData.coords.target.y})
                </p>
            </div>
        </div>
    );
};

export default Monitor;