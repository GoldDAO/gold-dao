import { useAuth } from "@context/auth";
import { ForwardSwapProceedProvider } from "@context/index";

import { Button } from "@components/ui";

import ArrowDown from "@components/shared/tile/ArrowDown";
import FromCard from "@components/swap/card/From";
import Backdrop from "@components/shared/Backdrop";
import TransactionDetails from "./TransactionDetails";

import ForwardSwapFrom from "./from";
import ForwardSwapTo from "./to";
import ForwardSwapProceed from "./proceed";

import { useNft } from "@context/index";

const Forward = () => {
  const { state: authState, connect } = useAuth();
  const { isConnected } = authState;
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
        {!isConnected && (
          <Button className="w-full rounded-lg py-3" onClick={connect}>
            Connect a wallet
          </Button>
        )}
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
