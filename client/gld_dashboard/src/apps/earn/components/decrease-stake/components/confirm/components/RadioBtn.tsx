import Radio from "@mui/material/Radio";

const RadioIcon = () => (
  <div className="w-4 h-4 rounded-full border border-border bg-white hover:bg-copper" />
);

const RadioCheckedIcon = () => (
  <div className="w-4 h-4 rounded-full bg-copper flex items-center justify-center transition-colors">
    <div className="w-1.5 h-1.5 rounded-full bg-white" />
  </div>
);

const RadioBtn = ({
  checked = false,
  handleOnChange,
}: {
  checked?: boolean;
  handleOnChange: () => void;
}) => {
  const handleChange = () => {
    handleOnChange();
  };

  return (
    <Radio
      onChange={handleChange}
      checked={checked}
      icon={<RadioIcon />}
      checkedIcon={<RadioCheckedIcon />}
      sx={{
        "&:hover": {
          backgroundColor: "rgba(from var(--color-copper) r g b / 0.1)",
        },
        "& .MuiTouchRipple-root": {
          color: "rgba(from var(--color-copper) r g b / 1)",
        },
      }}
    />
  );
};

export default RadioBtn;
