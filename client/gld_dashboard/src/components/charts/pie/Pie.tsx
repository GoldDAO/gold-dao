/* eslint-disable @typescript-eslint/ban-ts-comment */
// @ts-nocheck
import { useMemo } from "react";
import {
  PieChart as PieChartRecharts,
  Cell,
  Label,
  Text,
  Pie as PieRecharts,
  ResponsiveContainer,
} from "recharts";
import tc from "tinycolor2";
import { colors as themeColors } from "@theme/preset";
import { usePieChart } from "./context";

export interface PieChartData {
  name: string;
  value?: number;
  valueToString?: string;
  color?: string;
}

interface PieChartProps {
  data: PieChartData[];
  colors: string[];
}

const PieChart = ({ data, colors }: PieChartProps) => {
  const { activeIndex, setActiveIndex } = usePieChart();
  // const [activeIndex, setActiveIndex] = useState<number | null>(null);
  const sumData = useMemo(
    () => data.reduce((acc, cur) => acc + cur.value, 0),
    [data]
  );

  const handleOnMouseOverCell = (index: number) => setActiveIndex(index);
  const handleOnMouseLeaveCell = () => setActiveIndex(null);

  return (
    <ResponsiveContainer>
      <PieChartRecharts>
        <PieRecharts
          dataKey="value"
          data={data}
          innerRadius={80}
          outerRadius={140}
        >
          {data.map((_, index) => (
            <Cell
              key={`cell-${index}`}
              fill={
                activeIndex !== null && activeIndex !== index
                  ? tc(colors[activeIndex % colors.length]).setAlpha(0.5)
                  : colors[index % colors.length]
              }
              stroke={themeColors.surface.DEFAULT}
              strokeWidth={4}
              onMouseOver={() => handleOnMouseOverCell(index)}
              onMouseLeave={handleOnMouseLeaveCell}
            />
          ))}
          {activeIndex !== null && (
            <Label
              content={({ viewBox }) => {
                const commonPositioningProps = {
                  x: viewBox.cx,
                  y: viewBox.cy,
                  textAnchor: "middle",
                  verticalAnchor: "middle",
                };
                return (
                  <>
                    <Text
                      dy={-16}
                      fontSize={24}
                      fontWeight="bold"
                      fill={themeColors.content}
                      {...commonPositioningProps}
                    >
                      {((data[activeIndex].value / sumData) * 100).toFixed(1) +
                        " %"}
                    </Text>

                    <Text
                      dy={24}
                      fill={themeColors.content}
                      fontSize={12}
                      width={100}
                      {...commonPositioningProps}
                    >
                      {data[activeIndex].name}
                    </Text>
                  </>
                );
              }}
            />
          )}
        </PieRecharts>
      </PieChartRecharts>
    </ResponsiveContainer>
  );
};

PieChart.defaultProps = {
  data: [
    {
      name: "OGY not in the hand of the Foundation",
      value: 6957526202.66,
    },
    {
      name: "OGY locked in the hand of the Foundation",
      value: 6744999999.98,
    },
  ],
  colors: ["#645eff", "#333089"],
};

export default PieChart;
