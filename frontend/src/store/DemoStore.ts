import { create } from "zustand";

type DemoStore = {
  selectedDemo: string;
  setSelectedDemo: (demo: string) => void;
};

export const useDemoStore = create<DemoStore>((set) => ({
  selectedDemo: "navbar",
  setSelectedDemo: (demo) => set({ selectedDemo: demo }),
}));
