export type NeuronState = "not dissolving" | "dissolving" | "dissolved";

export const BadgeNeuronState = ({
  state,
  className,
}: {
  state: "not dissolving" | "dissolving" | "dissolved";
  className?: string;
}) => {
  const neuronState = {
    "not dissolving": {
      label: "Not Dissolving",
      bgCn: "bg-[#e0e8ff]",
      textCn: "text-sky",
    },
    dissolving: {
      label: "Dissolving",
      bgCn: "bg-[#d9f2e8]",
      textCn: "text-jade",
    },
    dissolved: {
      label: "Dissolved",
      bgCn: "bg-[#feefd8]",
      textCn: "text-jade",
    },
  };
  const ns = neuronState[state];

  return (
    <div className={className ?? ""}>
      <div
        className={`rounded-full px-4 py-2 flex items-center justify-center gap-2 ${ns.bgCn}`}
      >
        <div className={`text-xs font-semibold shrink-0 text-black`}>
          {ns.label}
        </div>
      </div>
    </div>
  );
};
