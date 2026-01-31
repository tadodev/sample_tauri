import { useMemo }       from "react";
import ReactECharts      from "echarts-for-react";
import useDashboardStore   from "@/features/dashboard/dashboard.store";
import { useStressResults } from "@/features/dashboard/dashboard.query";
import { COMBOS } from "@/types";
import type { StressResult } from "@/types";

// Colour palette per combo — distinct and readable
const COMBO_COLORS: Record<string, string> = {
    Gravity: "#f59e0b",   // amber
    Wind:    "#3b82f6",   // blue
    Seismic: "#ef4444",   // red
};

function buildChartOption(data: StressResult[], combo: string) {
    // data is already filtered to one pier + one combo, sorted by level
    const sorted = [...data].sort((a, b) => a.level - b.level);

    return {
        animationDuration: 0,   // disable per-update animation — 3 charts x 100 points on main thread = hang
        tooltip: { trigger: "axis" as const },
        grid:    { top: 28, right: 16, bottom: 32, left: 52 },
        xAxis: {
            type: "value" as const,
            name: "Stress (kPa)",
            nameLocation: "end" as const,
            nameTextStyle: { fontSize: 11 },
            axisLabel: { fontSize: 10 },
        },
        yAxis: {
            type: "value" as const,
            name: "Level",
            nameLocation: "end" as const,
            nameTextStyle: { fontSize: 11 },
            min: 1,
            max: 100,
            interval: 10,
            axisLabel: { fontSize: 10 },
        },
        series: [
            {
                name:  combo,
                type:  "line" as const,
                data:  sorted.map((r) => [r.stress, r.level]),
                color: COMBO_COLORS[combo],
                lineStyle: { width: 2 },
                showSymbol: false,
                smooth: 0.3,
            },
        ],
    };
}

export default function ChartPanel() {
    const selectedPier = useDashboardStore((s) => s.selectedPier);
    const { data: allStress, isLoading } = useStressResults();

    // Memoize: filter 1500 rows → 300 (one pier), then slice into 3 combos
    // Only recomputes when selectedPier or the source data changes
    const chartOptions = useMemo(() => {
        if (!allStress) return null;
        const pierData = allStress.filter((r) => r.pier === selectedPier);
        return COMBOS.map((combo) => ({
            combo,
            option: buildChartOption(
                pierData.filter((r) => r.combo === combo),
                combo
            ),
        }));
    }, [allStress, selectedPier]);

    if (isLoading || !chartOptions) {
        return (
            <div className="grid grid-cols-3 gap-4">
                {COMBOS.map((c) => (
                    <div key={c} className="rounded-lg border bg-card p-4 flex items-center justify-center text-muted-foreground text-sm" style={{ height: 260 }}>
                        Loading {c}…
                    </div>
                ))}
            </div>
        );
    }

    return (
        <div className="grid grid-cols-3 gap-4">
            {chartOptions.map(({ combo, option }) => (
                <div key={combo} className="rounded-lg border bg-card p-3">
                    <div className="text-xs font-semibold mb-1 flex items-center gap-2">
            <span
                className="inline-block w-3 h-3 rounded-sm"
                style={{ backgroundColor: COMBO_COLORS[combo] }}
            />
                        {combo} — {selectedPier}
                    </div>
                    <ReactECharts
                        option={option}
                        style={{ height: 230 }}
                    />
                </div>
            ))}
        </div>
    );
}