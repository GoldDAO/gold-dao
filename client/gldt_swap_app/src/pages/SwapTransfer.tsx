import { Card } from "@components/ui";
import { useAuth } from "@context/auth";
import { useSwapApp, View } from "@context/index";

import { SwapProvider } from "@context/swap/swap";
import { TransferProvider } from "@context/transfer/index";

import ToggleBtnSwapTransfer from "@components/shared/ToggleBtnSwapTransfer";
import Backdrop from "@components/shared/Backdrop";

import Swap from "@components/swap/Swap";
import Transfer from "@components/transfer/Transfer";

const SwapTransfer = () => {
  const { state: authState, connect } = useAuth();
  const { isConnected } = authState;
  const { state: swapAppState } = useSwapApp();
  const { view } = swapAppState;

  return (
    <div className="flex justify-center items-center mt-4 sm:mt-8">
      <Card className="w-full sm:w-[600px] p-2 md:p-6 border border-border">
        <div className="relative flex justify-center mb-6">
          {!isConnected && (
            <Backdrop isClickable={true} handleOnClick={connect} />
          )}
          <ToggleBtnSwapTransfer />
        </div>
        <div>
          {view === View.SWAP ? (
            <SwapProvider>
              <Swap />
            </SwapProvider>
          ) : (
            <TransferProvider>
              <Transfer />
            </TransferProvider>
          )}
        </div>
      </Card>
    </div>
  );
};

export default SwapTransfer;
