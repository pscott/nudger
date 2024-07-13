import { create } from "zustand";

type DemoStore = {
  selectedDemo: string;
  setSelectedDemo: (demo: string) => void;
};

export const useDemoStore = create<DemoStore>((set) => ({
  selectedDemo: "navbar", // default value
  setSelectedDemo: (demo) => set({ selectedDemo: demo }),
}));
