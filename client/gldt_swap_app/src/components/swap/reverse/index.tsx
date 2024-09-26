import { useAuth } from "@context/auth";
import { ReverseSwapProceedProvider } from "@context/index";

import { Button } from "@components/ui";

import ArrowDown from "@components/shared/tile/ArrowDown";
import FromCard from "@components/swap/card/From";
import Backdrop from "@components/shared/Backdrop";

import ReverseSwapFrom from "./from";
import ReverseSwapTo from "./to";
import ReverseSwapProceed from "./proceed";
import TransactionDetails from "./TransactionDetails";

import { useNft } from "@context/index";

const Reverse = () => {
  const { state: authState, connect } = useAuth();
  const { isConnected } = authState;
  const { getSelectedTotal } = useNft();
  const hasSelectedNfts = !!getSelectedTotal();

  return (
    <>
      <div className="relative">
        {!isConnected && (
          <Backdrop isClickable={true} handleOnClick={connect} />
        )}
        <FromCard>
          <ReverseSwapFrom />
        </FromCard>
        <ArrowDown />
        <ReverseSwapTo />
        {hasSelectedNfts && <TransactionDetails className="w-full  mt-8" />}
      </div>
      <div className="mt-6">
        {!isConnected && (
          <Button className="w-full rounded-lg py-3" onClick={connect}>
            Connect a wallet
          </Button>
        )}
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
