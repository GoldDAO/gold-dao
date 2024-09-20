import { useWallet } from "@amerej/artemis-react";

import { ForwardSwapProceedProvider } from "@context/index";

import ConnectWallet from "@components/shared/button/ConnectWallet";
import ArrowDown from "@components/shared/tile/ArrowDown";
import FromCard from "@components/swap/card/From";
import Backdrop from "@components/shared/Backdrop";
import TransactionDetails from "./TransactionDetails";

import ForwardSwapFrom from "./from";
import ForwardSwapTo from "./to";
import ForwardSwapProceed from "./proceed";

import { useNft } from "@context/index";

const Forward = () => {
  const { isConnected } = useWallet();
  const { getSelectedTotal } = useNft();
  const hasSelectedNfts = !!getSelectedTotal();

  return (
    <>
      <div className="relative">
        {!isConnected && <Backdrop />}
        <FromCard>
          <ForwardSwapFrom />
        </FromCard>
        <ArrowDown />
        <ForwardSwapTo />
        {hasSelectedNfts && <TransactionDetails className="w-full  mt-8" />}
      </div>
      <div className="mt-6">
        {!isConnected && <ConnectWallet />}
        {isConnected && (
          <ForwardSwapProceedProvider>
            <ForwardSwapProceed />
          </ForwardSwapProceedProvider>
        )}
      </div>
    </>
  );
};

export default Forward;
