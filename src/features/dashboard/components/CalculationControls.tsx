import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import useDashboardStore from "@/features/dashboard/dashboard.store";
import { useCalculateStress } from "@/features/dashboard/dashboard.query";
import { useQueryClient } from "@tanstack/react-query";

export default function CalculationControls() {
    const params          = useDashboardStore((s) => s.params);
    const setLoadFactor   = useDashboardStore((s) => s.setLoadFactor);
    const setLevelRange   = useDashboardStore((s) => s.setLevelRange);
    const resetParams     = useDashboardStore((s) => s.resetParams);

    const calculate = useCalculateStress();
    const queryClient = useQueryClient();

    // Get the actual calculated level range after mutation succeeds
    const calculatedRange = queryClient.getQueryData<{ min: number; max: number }>(["calculated_level_range"]);

    const handleRecalculate = () => {
        calculate.mutate(params);
    };

    return (
        <div className="rounded-lg border bg-card p-4 space-y-4">
            <div className="flex items-center justify-between">
                <h3 className="text-sm font-semibold">Calculation Parameters</h3>
                <Button
                    size="sm"
                    variant="outline"
                    onClick={resetParams}
                    disabled={calculate.isPending}
                >
                    Reset
                </Button>
            </div>

            {/* Load factors */}
            <div className="grid grid-cols-3 gap-4">
                {(["gravity", "wind", "seismic"] as const).map((combo) => (
                    <div key={combo} className="space-y-2">
                        <label className="text-xs font-medium capitalize flex items-center justify-between">
                            <span>{combo}</span>
                            <span className="text-muted-foreground">{params.load_factors[combo].toFixed(2)}x</span>
                        </label>
                        <input
                            type="range"
                            min="0.5"
                            max="2.0"
                            step="0.1"
                            value={params.load_factors[combo]}
                            onChange={(e) => setLoadFactor(combo, parseFloat(e.target.value))}
                            className="w-full h-2 bg-muted rounded-lg appearance-none cursor-pointer accent-primary"
                            disabled={calculate.isPending}
                        />
                    </div>
                ))}
            </div>

            {/* Level range */}
            <div className="space-y-2">
                <label className="text-xs font-medium">Level Range</label>
                <div className="flex gap-2 items-center">
                    <Input
                        type="number"
                        min={1}
                        max={params.level_range[1]}
                        value={params.level_range[0]}
                        onChange={(e) => setLevelRange([parseInt(e.target.value) || 1, params.level_range[1]])}
                        className="w-20 h-8 text-sm"
                        disabled={calculate.isPending}
                    />
                    <span className="text-xs text-muted-foreground">to</span>
                    <Input
                        type="number"
                        min={params.level_range[0]}
                        value={params.level_range[1]}
                        onChange={(e) => setLevelRange([params.level_range[0], parseInt(e.target.value) || 100])}
                        className="w-20 h-8 text-sm"
                        disabled={calculate.isPending}
                    />
                </div>
            </div>

            {/* Recalculate button */}
            <Button
                className="w-full"
                onClick={handleRecalculate}
                disabled={calculate.isPending}
            >
                {calculate.isPending ? "Calculating..." : "Recalculate"}
            </Button>

            {calculate.isError && (
                <p className="text-xs text-destructive">
                    Calculation failed. Please try again.
                </p>
            )}

            {calculate.isSuccess && calculatedRange && (
                <div className="text-xs space-y-1">
                    <p className="text-green-600 font-medium">
                        ✓ Calculation complete
                    </p>
                    {calculatedRange.max < params.level_range[1] && (
                        <p className="text-amber-600">
                            Note: Data available only up to level {calculatedRange.max} (requested {params.level_range[1]})
                        </p>
                    )}
                    <p className="text-muted-foreground">
                        Showing levels {calculatedRange.min}–{calculatedRange.max}
                    </p>
                </div>
            )}
        </div>
    );
}