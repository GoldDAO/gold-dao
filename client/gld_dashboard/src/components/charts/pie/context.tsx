import {
  createContext,
  useContext,
  ReactNode,
  useState,
  Dispatch,
  SetStateAction,
} from "react";

interface IPieChartContext {
  activeIndex: number | null;
  setActiveIndex: Dispatch<SetStateAction<number | null>>;
}

const PieChartContext = createContext<IPieChartContext | undefined>(undefined);

// eslint-disable-next-line react-refresh/only-export-components
export const usePieChart = () => {
  const context = useContext(PieChartContext);
  if (!context)
    throw new Error("usePieChart must be used within a PieChartProvider");
  return context;
};

export const PieChartProvider = ({ children }: { children: ReactNode }) => {
  const [activeIndex, setActiveIndex] = useState<number | null>(null);

  return (
    <PieChartContext.Provider
      value={{
        activeIndex,
        setActiveIndex,
      }}
    >
      {children}
    </PieChartContext.Provider>
  );
};
