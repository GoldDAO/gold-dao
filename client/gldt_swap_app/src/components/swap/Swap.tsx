import { Mode, useSwap } from "@context/index";

import { NftProvider } from "@context/index";

import ForwardSwap from "@components/swap/forward/";
import ReverseSwap from "@components/swap/reverse";

const Swap = () => {
  const { state: swapState } = useSwap();

  return (
    <>
      {swapState.mode === Mode.FORWARD && (
        <NftProvider>
          <ForwardSwap />
        </NftProvider>
      )}
      {swapState.mode === Mode.REVERSE && (
        <NftProvider>
          <ReverseSwap />
        </NftProvider>
      )}
    </>
  );
};

export default Swap;
