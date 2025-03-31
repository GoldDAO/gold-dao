import {
  BarChart,
  Bar as BarRechart,
  XAxis,
  YAxis,
  CartesianGrid,
  Legend,
  ResponsiveContainer,
} from "recharts";
import styled from "styled-components";
import { millify } from "@utils/numbers";
import { colors } from "@theme/preset";

type BarColor = {
  fill?: string;
  bgFill?: string;
};

type Bar = {
  barColor: BarColor;
  radius: number;
};

type Data = {
  name: string;
  value: number;
};

type BarChart = {
  data: Data[];
  barFill?: string | undefined;
  barBgFill?: string;
  barBgRadius?: number;
  legendValue?: string | undefined;
};

const StyledLegendIndicator = styled.div`
  background-color: ${({ color }) => color};
`;

const renderLegend = (
  legendValue: string | undefined,
  color: string | undefined
) => {
  // const { payload } = props;

  return (
    <div className="flex items-center mt-4">
      <StyledLegendIndicator color={color} className="h-4 w-4 rounded-full" />
      <div className="ml-2">{legendValue}</div>
    </div>
  );
};

const Bar = ({
  data,
  barFill,
  barBgFill,
  barBgRadius,
  legendValue,
}: BarChart) => {
  return (
    <ResponsiveContainer>
      <BarChart
        data={data}
        margin={{
          top: 5,
          right: 5,
          left: 5,
          bottom: 5,
        }}
      >
        <CartesianGrid vertical={false} strokeDasharray="5 5" />
        <XAxis dataKey="name" />
        <YAxis tickFormatter={(value) => millify(value)} />
        <Legend
          margin={{ top: 8, left: 0, right: 0, bottom: 0 }}
          content={() => renderLegend(legendValue, barFill)}
        />
        <BarRechart
          dataKey="value"
          fill={barFill}
          background={{ fill: barBgFill, radius: barBgRadius }}
        />
      </BarChart>
    </ResponsiveContainer>
  );
};

Bar.defaultProps = {
  data: [
    {
      name: "27 feb",
      value: 2000,
    },
    {
      name: "27 mar",
      value: 1500,
    },
    {
      name: "27 apr",
      value: 1200,
    },
    {
      name: "27 may",
      value: 1000,
    },
    {
      name: "27 jun",
      value: 850,
    },
    {
      name: "27 jul",
      value: 500,
    },
    {
      name: "27 aug",
      value: 200,
    },
  ],
  barFill: "#38bdf8",
  barBgFill: colors.surface["2"],
  barBgRadius: [50, 50, 0, 0],
  legendValue: "Total OGY Supply",
};

export default Bar;
