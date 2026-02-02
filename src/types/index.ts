// src/types/index.ts

// ── Generated Types (Single Source of Truth) ─────────────────────
export type { Section } from './bindings/Section';
export type { Force } from './bindings/Force';
export type { StressResult } from './bindings/StressResult';
export type { StressParams } from './bindings/StressParams';
export type { LoadFactors } from './bindings/LoadFactors';

// ── Frontend-Only Types ──────────────────────────────────────────
export type Combo = "Gravity" | "Wind" | "Seismic";

export const COMBOS: Combo[] = ["Gravity", "Wind", "Seismic"];
export const PIERS = ["P1", "P2", "P3", "P4", "P5"];

// ── Constants ────────────────────────────────────────────────────
import type { StressParams } from './bindings/StressParams';

export const DEFAULT_PARAMS: StressParams = {
  loadFactors: { gravity: 1.0, wind: 1.0, seismic: 1.0 },
  levelRange: [1, 1000000],
};