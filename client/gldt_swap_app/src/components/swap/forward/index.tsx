import { useAuth } from "@auth/index";
import { ForwardSwapProceedProvider } from "@context/index";

import { Button } from "@components/ui";

import ArrowDown from "@components/shared/tile/ArrowDown";
import FromCard from "@components/swap/card/From";
import Backdrop from "@components/shared/Backdrop";
import TransactionDetails from "./TransactionDetails";

import ForwardSwapFrom from "./From";
import ForwardSwapTo from "./To";
import ForwardSwapProceed from "./proceed";

import { useNft } from "@context/index";

const Forward = () => {
  const { isConnected, connect } = useAuth();
  const { getSelectedTotal } = useNft();
  const hasSelectedNfts = !!getSelectedTotal();

  return (
    <>
      <div className="relative">
        {!isConnected && (
          <Backdrop isClickable={true} handleOnClick={connect} />
        )}
        <FromCard>
          <ForwardSwapFrom />
        </FromCard>
        <ArrowDown />
        <ForwardSwapTo />
        {hasSelectedNfts && <TransactionDetails className="w-full mt-8" />}
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
