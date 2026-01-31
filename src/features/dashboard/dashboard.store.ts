import { create } from "zustand";

type DashboardState = {
    selectedPier: string;
    setSelectedPier: (pier: string) => void;
};

const useDashboardStore = create<DashboardState>((set) => ({
    selectedPier: "P1",
    setSelectedPier: (pier: string) => set({ selectedPier: pier }),
}));

export default useDashboardStore;