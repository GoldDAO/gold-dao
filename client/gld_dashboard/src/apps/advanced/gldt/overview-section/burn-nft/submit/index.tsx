import { useEffect, useState } from "react";
import { useAtom, useAtomValue } from "jotai";
import { GLDT_LEDGER_CANISTER_ID } from "@constants";
import { useAuth } from "@auth/index";
import { Button, LoaderSpin } from "@components/index";
import AvailableNFTSelect from "@shared/components/nft-select/AvailableNFTSelect";
import { NFTCollections } from "@shared/utils/nfts";
import {
  IsOneOrMoreSelectedNFTAtom,
  TotalGLDTSelectedAtom,
  SelectNFTStateReducerAtom,
  TotalNFTSelectedAtom,
} from "@shared/atoms/NFTStateAtom";
import useFetchLedgerBalance from "@shared/hooks/useFetchLedgerBalance";
import SwapNFTReducerAtom from "@advanced/gldt/overview-section/shared/atoms/SwapNFTAtom";
// import BurnInfos from "./burn-infos";
import InsufficientGLDTDisclaimer from "./insufficient-gldt-disclaimer";

const Submit = () => {
  const { principalId, unauthenticatedAgent, isConnected } = useAuth();
  const [, dispatchSwapNFT] = useAtom(SwapNFTReducerAtom);
  const [selectNFTState] = useAtom(SelectNFTStateReducerAtom);
  const IsOneOrMoreSelectedNFT = useAtomValue(IsOneOrMoreSelectedNFTAtom);
  const totalGLDTSelected = useAtomValue(TotalGLDTSelectedAtom);
  const totalNFTSelected = useAtomValue(TotalNFTSelectedAtom);
  const [canBurnNFT, setCanBurnNFT] = useState(false);

  const balance = useFetchLedgerBalance(
    GLDT_LEDGER_CANISTER_ID,
    unauthenticatedAgent,
    {
      ledger: "GLDT",
      owner: principalId,
      enabled: !!unauthenticatedAgent && isConnected,
    }
  );

  useEffect(() => {
    if (
      balance.isSuccess &&
      totalGLDTSelected + totalNFTSelected <= balance.data.balance
    ) {
      setCanBurnNFT(true);
    } else {
      setCanBurnNFT(false);
    }
  }, [
    balance.isSuccess,
    totalGLDTSelected,
    totalNFTSelected,
    balance.data?.balance,
  ]);

  const handleSubmit = () => {
    dispatchSwapNFT({ type: "SUBMIT", value: selectNFTState });
  };

  return balance.isSuccess ? (
    <div className="max-h-[75vh] overflow-y-auto p-4">
      <div className="rounded-xl p-4 border border-border">
        <div className="text-primary mb-4">From</div>
        {!canBurnNFT ? (
          <InsufficientGLDTDisclaimer
            totalGLDTSelected={totalGLDTSelected}
            totalNFTSelected={totalNFTSelected}
            balance={balance.data.balance}
          />
        ) : (
          <>
            <div className="flex justify-center items-center p-4 border border-border rounded-xl bg-surface-secondary">
              <div>{totalGLDTSelected} GLDT</div>
            </div>
            {/* {totalNFTSelected > 0 && (
              <div className="mt-4">
                <BurnInfos
                  totalGLDTSelected={totalGLDTSelected}
                  totalNFTSelected={totalNFTSelected}
                />
              </div>
            )} */}
          </>
        )}
      </div>

      <div className="mt-8 rounded-xl p-4 border border-border">
        <div className="text-primary mb-4">To</div>
        <div className="flex flex-col gap-2">
          {NFTCollections.map((collection) => (
            <AvailableNFTSelect
              key={collection.name}
              collection={collection.name}
            />
          ))}
        </div>
      </div>

      <div className="mt-8">
        <Button
          onClick={handleSubmit}
          disabled={!IsOneOrMoreSelectedNFT || !canBurnNFT}
          className="w-full px-6 py-3 bg-secondary text-white font-medium rounded-md"
        >
          Submit
        </Button>
      </div>
    </div>
  ) : (
    <div className="flex flex-col justify-center items-center gap-4 px-4 py-8">
      <LoaderSpin />
      <div>Fetching your balance and more...</div>
    </div>
  );
};

export default Submit;
