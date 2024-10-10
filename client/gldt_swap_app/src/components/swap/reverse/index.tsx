import { useAuth } from "@auth/index";
import { useReverseSwapProceed } from "@context/index";

import { Button } from "@components/ui";

import ArrowDown from "@components/shared/tile/ArrowDown";
import FromCard from "@components/swap/card/From";
import Backdrop from "@components/shared/Backdrop";

import ReverseSwapFrom from "./from";
import ReverseSwapTo from "./to";
import ReverseSwapProceed from "./proceed";
import TransactionDetails from "./TransactionDetails";
import InsufficientFundsGLDT from "./insufficient-gldt-disclaimer";

const Reverse = () => {
  const { isConnected, connect } = useAuth();
  const { state: reverseSwapProceedState } = useReverseSwapProceed();
  const { canReverseSwap, isInsufficientGLDTFunds } = reverseSwapProceedState;

  return (
    <>
      <div className="relative">
        {!isConnected && (
          <Backdrop isClickable={true} handleOnClick={connect} />
        )}
        <FromCard>
          {isInsufficientGLDTFunds ? (
            <InsufficientFundsGLDT className="mt-6" />
          ) : (
            <ReverseSwapFrom />
          )}
        </FromCard>
        <ArrowDown />
        <ReverseSwapTo />
        {canReverseSwap && <TransactionDetails className="w-full  mt-8" />}
      </div>
      <div className="mt-6">
        {!isConnected && (
          <Button className="w-full rounded-lg py-3" onClick={connect}>
            Connect a wallet
          </Button>
        )}
        {isConnected && <ReverseSwapProceed />}
      </div>
    </>
  );
};

export default Reverse;
