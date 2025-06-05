import { millify } from "@shared/utils/numbers";

const CustomTooltip = ({
  active,
  payload,
  label,
}: {
  active: boolean;
  payload: Array<{ value: number }>;
  label: string;
}) => {
  if (active && payload?.length) {
    return (
      <div className="bg-surface-secondary rounded-xl p-3 shadow">
        <span>{label}</span>
        <br />
        {payload.map((e, index) => (
          <div key={index}>
            <small key={index} className="text-content/60">
              {e.value >= 1000 ? millify(e.value, 9) : e.value}
            </small>
            <br />
          </div>
        ))}
      </div>
    );
  }
  return null;
};

export default CustomTooltip;
