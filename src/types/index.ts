export type Section = {
    level: number;
    pier:  string;
    w:     number;
    d:     number;
};

export type Force = {
    level: number;
    pier:  string;
    combo: string;
    force: number;
};

export type StressResult = {
    level:  number;
    pier:   string;
    combo:  string;
    area:   number;
    force:  number;
    stress: number;
    id:     string;   // "P1_42"
};

export type Combo = "Gravity" | "Wind" | "Seismic";

export const COMBOS: Combo[] = ["Gravity", "Wind", "Seismic"];
export const PIERS        = ["P1", "P2", "P3", "P4", "P5"];