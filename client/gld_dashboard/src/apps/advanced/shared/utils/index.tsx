export type TabID = "gldt" | "explorer";

export interface Tab {
  id: TabID;
  label: string;
}

export const TabList: Tab[] = [
  {
    id: "gldt",
    label: "GLDT",
  },
  {
    id: "explorer",
    label: "Explorer",
  },
];

export const TabWhitelist: TabID[] = ["gldt", "explorer"];
