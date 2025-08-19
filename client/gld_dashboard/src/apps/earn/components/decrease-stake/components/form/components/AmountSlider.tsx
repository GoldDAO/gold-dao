import { useState, ReactNode, useEffect } from "react";
import Slider from "@mui/material/Slider";
import Icon from "@shared/ui/icons";

const ThumbComponent = ({
  children,
  ...props
}: {
  children: ReactNode;
  ownerState?: unknown;
}) => {
  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  const { ownerState, ...other } = props;
  return (
    <div {...other}>
      {children}
      <Icon.Play height={16} width={16} className="-rotate-90" />
    </div>
  );
};

const AmountSlider = ({
  value,
  min = 0,
  max = 100,
  handleOnChange,
}: {
  value: number;
  min?: number;
  max?: number;
  handleOnChange: (value: number) => void;
}) => {
  const [v, setValue] = useState<number>(value || 0);

  useEffect(() => {
    setValue(value || 0);
  }, [value]);

  const handleChange = (_: Event, newValue: number) => {
    setValue(newValue);
    handleOnChange(newValue);
  };

  return (
    <div className="w-full">
      <Slider
        slots={{ thumb: ThumbComponent }}
        value={v}
        min={min}
        max={max}
        onChange={handleChange}
        shiftStep={1}
        sx={{
          color: "var(--color-copper)",
          position: "relative",
          transition: "all 0.3s ease-in-out",
          "& .MuiSlider-thumb": {
            color: "var(--color-copper)",
            position: "absolute",
            bottom: "-10px",
            marginLeft: "-7px",
            transition: "all 0.3s ease-in-out",
          },
          "& .MuiSlider-track": {
            height: 16,
            borderRadius: "4px",
            backgroundColor: "transparent",
            transition: "width 0.3s ease-in-out",
            backgroundImage:
              "linear-gradient(to right, var(--color-gold), var(--color-copper))",
          },
          "& .MuiSlider-rail": {
            color: "var(--color-surface-primary)",
            opacity: 1,
            height: 8,
            borderRadius: "4px",
          },
        }}
      />
    </div>
  );
};

export default AmountSlider;
