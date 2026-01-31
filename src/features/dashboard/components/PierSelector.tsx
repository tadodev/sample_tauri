import useDashboardStore from "@/features/dashboard/dashboard.store";
import { PIERS } from "@/types";

export default function PierSelector() {
    const selectedPier    = useDashboardStore((s) => s.selectedPier);
    const setSelectedPier = useDashboardStore((s) => s.setSelectedPier);

    return (
        <div className="flex gap-2">
            {PIERS.map((pier) => (
                <button
                    key={pier}
                    onClick={() => setSelectedPier(pier)}
                    className={[
                        "px-4 py-1.5 rounded-md text-sm font-semibold transition-colors duration-150",
                        selectedPier === pier
                            ? "bg-primary text-primary-foreground"
                            : "bg-secondary text-secondary-foreground hover:bg-accent",
                    ].join(" ")}
                >
                    {pier}
                </button>
            ))}
        </div>
    );
}