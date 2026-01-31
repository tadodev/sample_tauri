import { useMutation } from "@tanstack/react-query";
import {invoke} from "@tauri-apps/api/core";

/* =========================
   React Query: IPC
   ========================= */
export function useGreet() {
    return useMutation({
        mutationFn: (name: string) =>
            invoke<string>("greet", { name }),
    });
}