import {
  AreaChart,
  ResponsiveContainer,
  Area as AreaRechart,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
} from "recharts";
import styled from "styled-components";
import { colors } from "@theme/preset";
import CustomTooltip from "./utils/CustomTooltip";
import { millify } from "@utils/numbers";

type Data = {
  name: string;
  value: number;
};

type AreaChart = {
  data: Data[];
  fill?: string;
};

const StyledAreaChart = styled(AreaChart)`
  .recharts-cartesian-grid-vertical line {
    stroke: ${colors.surface[3]} !important;
  }
  .recharts-cartesian-grid-horizontal line:first-child,
  .recharts-cartesian-grid-horizontal line:last-child,
  .recharts-cartesian-grid-vertical line:first-child,
  .recharts-cartesian-grid-vertical line:last-child {
    stroke-opacity: 0 !important;
  }
`;

const Area = ({ data, fill }: AreaChart) => {
  const minValue = Math.min(...data.map((d) => d.value));
  const maxValue = Math.max(...data.map((d) => d.value));
  return (
    <ResponsiveContainer>
      <StyledAreaChart
        width={500}
        height={400}
        data={data}
        margin={{
          top: 10,
          right: 30,
          left: 25,
          bottom: 0,
        }}
      >
        <defs>
          <linearGradient id={`fill${fill}`} x1="0" y1="0" x2="0" y2="1">
            <stop offset="5%" stopColor={fill} stopOpacity={0.8} />
            <stop offset="100%" stopColor={fill} stopOpacity={0} />
          </linearGradient>
        </defs>
        <CartesianGrid
          vertical={true}
          strokeDasharray="5 5"
          horizontal={false}
        />
        <XAxis
          dataKey="name"
          tickLine={false}
          tick={{ fill: colors.surface[1] }}
          axisLine={false}
        />
        <YAxis
          tickFormatter={(value) => (value >= 1000 ? millify(value) : value)}
          tickLine={false}
          axisLine={false}
          domain={[
            minValue - (maxValue - minValue) * 0.1,
            maxValue + (maxValue - minValue) * 0.1,
          ]}
        />
        <Tooltip
          content={<CustomTooltip active={false} payload={[]} label={""} />}
        />
        <AreaRechart
          type="monotone"
          dataKey="value"
          stroke={fill}
          strokeWidth={2}
          fillOpacity={1}
          fill={`url(#fill${fill})`}
        />
      </StyledAreaChart>
    </ResponsiveContainer>
  );
};

Area.defaultProps = {
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
  fill: "#00A2F7",
};

export default Area;
