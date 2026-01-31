import { PropsWithChildren } from "react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";

/* =========================
   React Query Client
   ========================= */
const queryClient = new QueryClient({
    defaultOptions: {
        queries: {
            refetchOnWindowFocus: false, // good default for desktop apps
            retry: 1,
        },
    },
});

/* =========================
   App Providers
   ========================= */
export function AppProviders({ children }: PropsWithChildren) {
    return (
        <QueryClientProvider client={queryClient}>
            {children}
            </QueryClientProvider>
    );
}
