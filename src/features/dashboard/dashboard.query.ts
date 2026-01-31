import { useQuery } from "@tanstack/react-query";
import { invoke }   from "@tauri-apps/api/core";
import type { Section, Force, StressResult } from "@/types";

export function useSections() {
    return useQuery({
        queryKey: ["sections"],
        queryFn:  () => invoke<Section[]>("get_sections"),
    });
}

export function useForces() {
    return useQuery({
        queryKey: ["forces"],
        queryFn:  () => invoke<Force[]>("get_forces"),
    });
}

export function useStressResults() {
    return useQuery({
        queryKey: ["stress_results"],
        queryFn:  () => invoke<StressResult[]>("get_stress_results"),
    });
}