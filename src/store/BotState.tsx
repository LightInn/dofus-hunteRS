import {create} from 'zustand'
import {InnerAppState} from "../models/models.tsx";


type Action = {
    updateBotState: (state: InnerAppState) => void;
}
const useBotState = create<InnerAppState & Action>((set) => ({
    botData: {
        steps: {
            current: 0,
            total: 0
        },
        coords: {
            start: {
                x: 0,
                y: 0
            },
            current: {
                x: 0,
                y: 0
            },
            target: {
                x: 0,
                y: 0
            },
        },
        currentHint: "",
        currentArrow: "unknown"
    },
    running: false,


    updateBotState: (state: InnerAppState) => set(state)

}))

export default useBotState;