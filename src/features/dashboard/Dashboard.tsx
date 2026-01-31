import PierSelector  from "./components/PierSelector";
import ChartPanel    from "./components/ChartPanel";
import SummaryTable  from "./components/SummaryTable";

export default function Dashboard() {
    return (
        <div className="min-h-screen bg-muted p-6 space-y-6">
            {/* ── Header ───────────────────────────────────────────────── */}
            <div className="flex items-center justify-between">
                <div>
                    <h1 className="text-xl font-bold tracking-tight">Pier Stress Dashboard</h1>
                    <p className="text-xs text-muted-foreground mt-0.5">
                        100-story · 5 piers · 1 500 load cases · stress = force / area
                    </p>
                </div>
                <PierSelector />
            </div>

            {/* ── Charts row ───────────────────────────────────────────── */}
            <ChartPanel />

            {/* ── Summary table ────────────────────────────────────────── */}
            <div>
                <h2 className="text-sm font-semibold mb-2 text-muted-foreground uppercase tracking-wide">
                    Summary — click a row to select pier
                </h2>
                <SummaryTable />
            </div>
        </div>
    );
}