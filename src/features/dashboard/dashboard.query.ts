import { useQuery } from "@tanstack/react-query";
import { invoke }   from "@tauri-apps/api/core";
import type { Section, Force, StressResult } from "@/types";

// staleTime: Infinity — data is static (generated once in Rust managed state),
// so tell React Query to never consider it stale and never auto-refetch.
// Each invoke() fires exactly once on first mount, then serves from cache.
const STATIC_QUERY = {
    staleTime: Infinity,
    gcTime:    Infinity,
} as const;

export function useSections() {
    return useQuery({
        queryKey: ["sections"],
        queryFn:  () => invoke<Section[]>("get_sections"),
        ...STATIC_QUERY,
    });
}

export function useForces() {
    return useQuery({
        queryKey: ["forces"],
        queryFn:  () => invoke<Force[]>("get_forces"),
        ...STATIC_QUERY,
    });
}

export function useStressResults() {
    return useQuery({
        queryKey: ["stress_results"],
        queryFn:  () => invoke<StressResult[]>("get_stress_results"),
        ...STATIC_QUERY,
    });
}