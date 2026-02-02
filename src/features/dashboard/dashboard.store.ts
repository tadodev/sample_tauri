import { create } from "zustand";
import { DEFAULT_PARAMS, type StressParams } from "@/types";

type DashboardState = {
    selectedPier: string;
    setSelectedPier: (pier: string) => void;

    // Calculation parameters
    params: StressParams;
    setLoadFactor: (combo: "gravity" | "wind" | "seismic", value: number) => void;
    setLevelRange: (range: [number, number]) => void;
    resetParams: () => void;
};

const useDashboardStore = create<DashboardState>((set) => ({
    selectedPier: "P1",
    setSelectedPier: (pier: string) => set({ selectedPier: pier }),

    params: DEFAULT_PARAMS,

    setLoadFactor: (combo, value) =>
        set((state) => ({
            params: {
                ...state.params,
                loadFactors: { ...state.params.load_factors, [combo]: value },
            },
        })),

    setLevelRange: (range) =>
        set((state) => ({
            params: { ...state.params, levelRange: range },
        })),

    resetParams: () => set({ params: DEFAULT_PARAMS }),
}));

export default useDashboardStore;