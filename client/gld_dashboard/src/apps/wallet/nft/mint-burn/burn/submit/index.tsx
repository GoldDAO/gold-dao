import { useEffect, useState } from "react";
import { useAtom, useAtomValue } from "jotai";
import { GLDT_LEDGER_CANISTER_ID, REVERSE_GLDT_TX_FEE } from "@constants";
import { useAuth } from "@auth/index";
import { LoaderSpin } from "@components/index";
import { NFTCollections } from "@shared/utils/nfts";
import {
  TotalGLDTSelectedAtom,
  SelectNFTStateReducerAtom,
  TotalNFTSelectedAtom,
  TotalCollectionSelectedAtom,
} from "@shared/atoms/NFTStateAtom";
import useFetchLedgerBalance from "@shared/hooks/useFetchLedgerBalance";
import SwapNFTReducerAtom from "../../atoms/SwapNFTAtom";
import InsufficientGLDTDisclaimer from "./insufficient-gldt-disclaimer";
import BtnPrimary from "@shared/ui/button/HorizontalButton";
import { NFTCollectionSection } from "@wallet/nft/shared/NFTSelectionGrid";
import BurnInfos from "./burn-infos";

const Submit = () => {
  const { principalId, unauthenticatedAgent, isConnected } = useAuth();
  const [, dispatchSwapNFT] = useAtom(SwapNFTReducerAtom);
  const [selectNFTState] = useAtom(SelectNFTStateReducerAtom);
  const totalGLDTSelected = useAtomValue(TotalGLDTSelectedAtom);
  const totalNFTSelected = useAtomValue(TotalNFTSelectedAtom);
  const totalCollectionSelected = useAtomValue(TotalCollectionSelectedAtom);

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
      totalGLDTSelected * 10 ** balance.data.decimals +
        totalNFTSelected * REVERSE_GLDT_TX_FEE +
        totalCollectionSelected * 10000000 <=
        balance.data.balance_e8s
    ) {
      setCanBurnNFT(true);
    } else {
      setCanBurnNFT(false);
    }
  }, [
    balance.isSuccess,
    totalGLDTSelected,
    totalNFTSelected,
    totalCollectionSelected,
    balance.data,
  ]);

  const handleSubmit = () => {
    dispatchSwapNFT({ type: "SUBMIT", value: selectNFTState });
  };

  return balance.isSuccess ? (
    <>
      <div className="rounded-xl p-4 border border-border">
        <div className="text-copper text-sm font-semibold mb-2">From</div>
        {!canBurnNFT ? (
          <InsufficientGLDTDisclaimer
            totalGLDTSelected={totalGLDTSelected}
            totalNFTSelected={totalNFTSelected}
            totalCollectionSelected={totalCollectionSelected}
            balance={balance.data.balance}
          />
        ) : (
          <>
            <div className="flex justify-center items-center p-4 border border-border rounded-xl bg-surface-secondary">
              <div>{totalGLDTSelected} GLDT</div>
            </div>
            {totalNFTSelected > 0 && (
              <div className="mt-4">
                <BurnInfos
                  totalGLDTSelected={totalGLDTSelected}
                  totalNFTSelected={totalNFTSelected}
                  totalCollectionSelected={totalCollectionSelected}
                />
              </div>
            )}
          </>
        )}
      </div>

      <div className="mt-4 rounded-xl p-4 border border-border">
        <div className="text-copper text-sm font-semibold mb-2">To</div>
        <div>
          {NFTCollections.map((collection) => (
            <NFTCollectionSection
              key={collection.name}
              collectionName={collection.name}
              fetchType="available"
            />
          ))}
        </div>
      </div>

      <div className="mt-8">
        <BtnPrimary
          onClick={handleSubmit}
          disabled={!canBurnNFT || totalNFTSelected === 0}
          className="w-full"
        >
          Submit
        </BtnPrimary>
      </div>
    </>
  ) : (
    <div className="flex flex-col justify-center items-center gap-4 px-4 py-8">
      <LoaderSpin />
      <div>Fetching your balance and more...</div>
    </div>
  );
};

export default Submit;
