import { useWallet } from "@amerej/artemis-react";

import { ReverseSwapProceedProvider } from "@context/index";

import ConnectWallet from "@components/shared/button/ConnectWallet";
import ArrowDown from "@components/shared/tile/ArrowDown";
import FromCard from "@components/swap/card/From";
import Backdrop from "@components/shared/Backdrop";

import ReverseSwapFrom from "./from";
import ReverseSwapTo from "./to";
import ReverseSwapProceed from "./proceed";

const Reverse = () => {
  const { isConnected } = useWallet();

  return (
    <>
      <div className="relative">
        {!isConnected && <Backdrop />}
        <FromCard>
          <ReverseSwapFrom />
        </FromCard>
        <ArrowDown />
        <ReverseSwapTo />
      </div>
      <div className="mt-6">
        {!isConnected && <ConnectWallet />}
        {isConnected && (
          <ReverseSwapProceedProvider>
            <ReverseSwapProceed />
          </ReverseSwapProceedProvider>
        )}
      </div>
    </>
  );
};

export default Reverse;
