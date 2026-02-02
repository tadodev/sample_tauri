import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { invoke }   from "@tauri-apps/api/core";
import type { Section, Force, StressResult, StressParams } from "@/types";

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

/// Mutation: recalculate stress with user-defined params.
/// On success, update the query cache with the new results.
export function useCalculateStress() {
    const queryClient = useQueryClient();

    return useMutation({
        mutationFn: (params: StressParams) => {
            console.log('Invoking calculate_stress with params:', params);
            return invoke<StressResult[]>("calculate_stress", {
                params: {
                    load_factors: params.load_factors,
                    level_range: params.level_range,
                }
            });
        },
        onSuccess: (data) => {
            console.log('Calculation succeeded, updating cache with', data.length, 'results');

            // Find actual level range in returned data
            const levels = data.map(r => r.level);
            const actualMin = levels.length > 0 ? Math.min(...levels) : 0;
            const actualMax = levels.length > 0 ? Math.max(...levels) : 0;

            queryClient.setQueryData(["stress_results"], data);
            queryClient.setQueryData(["calculated_level_range"], { min: actualMin, max: actualMax });
        },
        onError: (error) => {
            console.error('Calculation failed:', error);
        },
    });
}