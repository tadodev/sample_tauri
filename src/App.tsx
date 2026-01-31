import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import useUIStore from "@/features/greet/greet.store";
import { useGreet } from "@/features/greet/greet.query";
import {AppProviders} from "@/providers.tsx";
function AppContent() {
    const name = useUIStore((s) => s.name);
    const setName = useUIStore((s) => s.setName);

    const greet = useGreet();

    return (
        <div className="min-h-screen flex items-center justify-center bg-muted p-6">
            <Card className="w-full max-w-md">
                <CardHeader>
                    <CardTitle>Tauri + React Query + Zustand</CardTitle>
                </CardHeader>

                <CardContent className="space-y-4">
                    <Input
                        placeholder="Enter your name"
                        value={name}
                        onChange={(e: { target: { value: string; }; }) => setName(e.target.value)}
                    />

                    <Button
                        className="w-full"
                        onClick={() => greet.mutate(name)}
                        disabled={greet.isPending || !name}
                    >
                        {greet.isPending ? "Greeting..." : "Greet"}
                    </Button>

                    {greet.isError && (
                        <p className="text-sm text-destructive">
                            Something went wrong
                        </p>
                    )}

                    {greet.data && (
                        <p className="text-sm font-medium text-green-600">
                            {greet.data}
                        </p>
                    )}
                </CardContent>
            </Card>
        </div>
    );
}

export default function App() {
    return (
        <AppProviders>
            <AppContent />
        </AppProviders>
    );
}
