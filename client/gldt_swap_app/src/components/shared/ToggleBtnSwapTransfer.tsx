import { Button } from "@components/ui";

import { useSwapApp, View } from "@context/index";

const ToggleBtnSwapTransfer = () => {
  const { state: swapAppState, setView } = useSwapApp();
  const { view } = swapAppState;

  return (
    <div className="flex justify-center items-center bg-surface-2 rounded-full">
      <Button
        onClick={() => setView(View.SWAP)}
        className={`px-6 ${
          view !== View.SWAP ? `bg-surface-2 text-content/60` : ``
        }`}
      >
        Swap
      </Button>
      <Button
        onClick={() => setView(View.TRANSFER)}
        className={`px-6 ${
          view !== View.TRANSFER ? `bg-surface-2 text-content/60` : ``
        }`}
      >
        Transfer
      </Button>
    </div>
  );
};

export default ToggleBtnSwapTransfer;
