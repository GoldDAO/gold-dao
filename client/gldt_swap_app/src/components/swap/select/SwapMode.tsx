import { Mode, useSwap } from "@context/index";
import { Select } from "@components/ui";

const SWAP_MODE_OPTIONS = [
  { value: "GLD NFT", icon: "vite.svg", label: "NFT" },
  { value: "GLDT", icon: "vite.svg", label: "ICRC-1" },
];

const SelectSwapMode = () => {
  const { setMode, state: swapState } = useSwap();

  const handleOnChangeMode = (mode: Mode): void => {
    setMode(mode);
  };

  return (
    <Select
      options={SWAP_MODE_OPTIONS}
      value={swapState.mode === Mode.FORWARD ? "GLD NFT" : "GLDT"}
      handleOnChange={(value) =>
        handleOnChangeMode(value === "GLD NFT" ? Mode.FORWARD : Mode.REVERSE)
      }
    />
  );
};

export default SelectSwapMode;
