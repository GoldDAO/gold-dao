import {
  AreaChart,
  Area,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  ResponsiveContainer,
} from "recharts";
import useGetAPYHistory from "../../hooks/useGetAPYHistory";
import { getCanister } from "../../utils/getCanister";
import NumberToLocaleString from "../shared/NumberToLocaleString";
import Card from "../shared/ui/Card";

const ApyHistory = ({ env }) => {
  const apyHistory = useGetAPYHistory(getCanister(env).GLDT_STAKE_CANISTER_ID);

  const CustomTooltip = ({ active, payload, label }) => {
    if (active && payload && payload.length) {
      return (
        <div className="bg-white dark:bg-neutral-800 p-4 rounded-xl shadow-sm">
          <p className="text-neutral-900 dark:text-neutral-50 text-sm">{`Date: ${label}`}</p>
          <p className="text-neutral-900 dark:text-neutral-50 font-semibold">
            {`APY: ${payload[0].value}%`}
          </p>
        </div>
      );
    }
    return null;
  };

  // Formater les dates pour l'axe X
  const formatXAxisDate = (dateStr) => {
    const date = new Date(dateStr);
    return date.toLocaleDateString("en-US", {
      month: "short",
      day: "numeric",
    });
  };

  if (!apyHistory.isSuccess) {
    return (
      <div className="flex justify-center items-center p-8">
        <div className="">Fetching APY history...</div>
      </div>
    );
  }

  return (
    <Card className="w-full [--chart-primary:theme(colors.yellow.500)] [--chart-grid:theme(colors.neutral.200)] dark:[--chart-grid:theme(colors.neutral.800)] [--chart-axis:theme(colors.neutral.400)] dark:[--chart-axis:theme(colors.neutral.400)]">
      <ResponsiveContainer width="100%" height={400}>
        <AreaChart data={apyHistory.data} margin={{ left: -25 }}>
          <defs>
            <linearGradient id="colorAPY" x1="0" y1="0" x2="0" y2="1">
              <stop
                offset="5%"
                stopColor="var(--chart-primary)"
                stopOpacity={0.3}
              />
              <stop
                offset="95%"
                stopColor="var(--chart-primary)"
                stopOpacity={0}
              />
            </linearGradient>
          </defs>
          <CartesianGrid
            strokeDasharray="3 3"
            stroke="var(--chart-grid)"
            horizontal={false}
          />
          <XAxis
            dataKey="date"
            tickFormatter={formatXAxisDate}
            stroke="var(--chart-axis)"
            fontSize={12}
          />
          <YAxis
            stroke="var(--chart-axis)"
            fontSize={12}
            tickFormatter={(value) => `${value}%`}
          />
          <Tooltip content={<CustomTooltip />} />
          <Area
            type="monotone"
            dataKey="value"
            stroke="var(--chart-primary)"
            strokeWidth={2}
            fillOpacity={1}
            fill="url(#colorAPY)"
          />
        </AreaChart>
      </ResponsiveContainer>
    </Card>
  );
};

export default ApyHistory;
