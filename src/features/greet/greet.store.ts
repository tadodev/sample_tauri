import {create} from "zustand";

type UIState = {
    name: string;
    setName: (name: string) => void;
};

const useUIStore = create<UIState>((set) => ({
    name: "",
    setName: (name) => set({ name }),
}));

export default useUIStore