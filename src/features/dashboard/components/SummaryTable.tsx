import {
    createColumnHelper,
    flexRender,
    getCoreRowModel,
    useReactTable,
} from "@tanstack/react-table";
import { useMemo }          from "react";
import useDashboardStore   from "@/features/dashboard/dashboard.store";
import { useStressResults } from "@/features/dashboard/dashboard.query";
import { PIERS }            from "@/types";
import { cn }               from "@/lib/utils";

// ─── Derived row type ────────────────────────────────────────────────────────
type SummaryRow = {
    pier:          string;
    area:          number;   // area at level 1 (base)
    maxGravity:    number;
    maxWind:       number;
    maxSeismic:    number;
    maxStress:     number;   // overall max across all combos
};

// ─── Column definitions ──────────────────────────────────────────────────────
const colHelper = createColumnHelper<SummaryRow>();

const fmt = (n: number) => n.toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 2 });

const columns = [
    colHelper.accessor("pier", {
        header: "Pier",
        cell:   (info) => <span className="font-semibold">{info.getValue()}</span>,
    }),
    colHelper.accessor("area", {
        header: "Base Area (m²)",
        cell:   (info) => fmt(info.getValue()),
    }),
    colHelper.accessor("maxGravity", {
        header: "Max Gravity (kPa)",
        cell:   (info) => <span className="text-amber-600 font-medium">{fmt(info.getValue())}</span>,
    }),
    colHelper.accessor("maxWind", {
        header: "Max Wind (kPa)",
        cell:   (info) => <span className="text-blue-600 font-medium">{fmt(info.getValue())}</span>,
    }),
    colHelper.accessor("maxSeismic", {
        header: "Max Seismic (kPa)",
        cell:   (info) => <span className="text-red-600 font-medium">{fmt(info.getValue())}</span>,
    }),
    colHelper.accessor("maxStress", {
        header: "Overall Max (kPa)",
        cell:   (info) => <span className="font-bold">{fmt(info.getValue())}</span>,
    }),
];

// ─── Derive summary rows from stress results ────────────────────────────────
function buildSummaryRows(stress: import("@/types").StressResult[]): SummaryRow[] {
    return PIERS.map((pier) => {
        const pierRows = stress.filter((r) => r.pier === pier);
        const baseArea = pierRows.length > 0
            ? (pierRows.find((r) => r.level === 1)?.area ?? 0)
            : 0;

        const maxByCombo = (combo: string) =>
            Math.max(0, ...pierRows.filter((r) => r.combo === combo).map((r) => r.stress));

        const grav    = maxByCombo("Gravity");
        const wind    = maxByCombo("Wind");
        const seismic = maxByCombo("Seismic");

        return {
            pier,
            area:       baseArea,
            maxGravity: grav,
            maxWind:    wind,
            maxSeismic: seismic,
            maxStress:  Math.max(grav, wind, seismic),
        };
    });
}

// ─── Component ───────────────────────────────────────────────────────────────
export default function SummaryTable() {
    const selectedPier    = useDashboardStore((s) => s.selectedPier);
    const setSelectedPier = useDashboardStore((s) => s.setSelectedPier);
    const { data: stress, isLoading } = useStressResults();

    const rows = useMemo(
        () => (stress ? buildSummaryRows(stress) : []),
        [stress]
    );

    const table = useReactTable({
        data:      rows,
        columns,
        getCoreRowModel: getCoreRowModel(),
    });

    if (isLoading) {
        return <div className="text-sm text-muted-foreground p-4">Loading summary…</div>;
    }

    return (
        <div className="rounded-lg border bg-card overflow-hidden">
            <table className="w-full text-sm">
                <thead>
                <tr className="border-b bg-muted/40">
                    {table.getHeaderGroups().map((hg) =>
                        hg.headers.map((header) => (
                            <th
                                key={header.id}
                                className="px-4 py-2.5 text-left font-semibold text-muted-foreground text-xs uppercase tracking-wide"
                            >
                                {flexRender(header.column.columnDef.header, header.getContext())}
                            </th>
                        ))
                    )}
                </tr>
                </thead>
                <tbody>
                {table.getRowModel().rows.map((row) => {
                    const isSelected = row.original.pier === selectedPier;
                    return (
                        <tr
                            key={row.id}
                            onClick={() => setSelectedPier(row.original.pier)}
                            className={cn(
                                "border-b last:border-b-0 cursor-pointer transition-colors duration-100",
                                isSelected
                                    ? "bg-primary/10 border-primary/20"
                                    : "hover:bg-muted/50"
                            )}
                        >
                            {row.getVisibleCells().map((cell) => (
                                <td key={cell.id} className="px-4 py-2.5">
                                    {flexRender(cell.column.columnDef.cell, cell.getContext())}
                                </td>
                            ))}
                        </tr>
                    );
                })}
                </tbody>
            </table>
        </div>
    );
}