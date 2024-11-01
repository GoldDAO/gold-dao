import { Mode, useSwap } from "@context/index";
import { Select } from "@components/ui";
import { LogoGLDT } from "@components/shared/logos";

const SWAP_MODE_OPTIONS = [
  {
    value: "GLD NFT",
    icon: <img src="/nft_logo.svg" className="w-4 h-4" alt="Gold bar logo" />,
    label: "NFT",
  },
  { value: "GLDT", icon: <LogoGLDT className="w-4 h-4" />, label: "ICRC-2" },
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
