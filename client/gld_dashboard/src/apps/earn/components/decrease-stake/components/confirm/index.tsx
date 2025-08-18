import { useAtom } from "jotai";
import Button from "@shared/ui/button/HorizontalButton";
import { DecreaseStakeStateReducerAtom } from "../../atoms";
import { DissolveMode } from "../../interfaces";
import OptionBox from "./components/OptionBox";
import RadioBtn from "./components/RadioBtn";
import DissolveInformations from "./components/DissolveInformations";
import DissolveInstantlyInformations from "./components/DissolveInstantlyInformations";

const Confirm = () => {
  const [state, dispatch] = useAtom(DecreaseStakeStateReducerAtom);

  const onSetDissolveMode = (mode: DissolveMode) => {
    dispatch({
      type: "SET_DISSOLVE_MODE",
      value: mode,
    });
  };

  const onConfirm = () => {
    if (state.dissolve_mode === "DISSOLVE") {
      dispatch({
        type: "SET_STEP",
        value: "details_dissolving",
      });
    } else if (state.dissolve_mode === "DISSOLVE_INSTANTLY") {
      dispatch({
        type: "SET_STEP",
        value: "details_dissolving_instantly",
      });
    }
  };

  return (
    <div>
      <OptionBox checked={state.dissolve_mode === "DISSOLVE"}>
        <div className="flex items-center gap-1">
          <RadioBtn
            checked={state.dissolve_mode === "DISSOLVE"}
            handleOnChange={() => onSetDissolveMode("DISSOLVE")}
          />
          <div className="text-lg">Unlock and wait one week</div>
        </div>
        <DissolveInformations
          checked={state.dissolve_mode === "DISSOLVE"}
          className="mt-2"
        />
      </OptionBox>

      <OptionBox
        checked={state.dissolve_mode === "DISSOLVE_INSTANTLY"}
        className="mt-4"
      >
        <div className="flex items-center gap-1">
          <RadioBtn
            checked={state.dissolve_mode === "DISSOLVE_INSTANTLY"}
            handleOnChange={() => onSetDissolveMode("DISSOLVE_INSTANTLY")}
          />
          <div className="text-lg">Unlock immediately</div>
        </div>
        <DissolveInstantlyInformations
          checked={state.dissolve_mode === "DISSOLVE_INSTANTLY"}
          className="mt-2"
        />
      </OptionBox>

      <Button className="mt-6 w-full" onClick={onConfirm}>
        Confirm
      </Button>
    </div>
  );
};

export default Confirm;
