import { create } from "zustand";
import type { WaterStats, WaterIntake } from "../../schemas/water";

interface WaterState {
  stats: WaterStats | null;
  history: WaterIntake[];
  isLoading: boolean;

  // Actions
  setStats: (stats: WaterStats) => void;
  setHistory: (history: WaterIntake[]) => void;
  setIsLoading: (isLoading: boolean) => void;
  addIntake: (intake: WaterIntake) => void;
}

export const useWaterStore = create<WaterState>((set) => ({
  stats: null,
  history: [],
  isLoading: false,

  setStats: (stats) => set({ stats }),

  setHistory: (history) => set({ history }),

  setIsLoading: (isLoading) => set({ isLoading }),

  addIntake: (intake) =>
    set((state) => ({
      history: [intake, ...state.history],
      stats: state.stats
        ? {
            ...state.stats,
            today_ml: state.stats.today_ml + intake.amount_ml,
            percentage: ((state.stats.today_ml + intake.amount_ml) / state.stats.goal_ml) * 100,
            entries_today: state.stats.entries_today + 1,
            last_log: intake.timestamp,
          }
        : null,
    })),
}));
