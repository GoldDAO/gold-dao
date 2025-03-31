import { SetStateAction, useAtom, WritableAtom } from "jotai";
import clsx from "clsx";
import { PlusIcon, MinusIcon } from "@heroicons/react/20/solid";

import { GLDT_VALUE_1G_NFT } from "@constants";
import { CollectionNFT } from "@atoms/NFTState";
import { IdNFT } from "@services/gld_nft/utils/interfaces";

const BtnSelect = ({
  handleOnClick,
  action,
  disabled = false,
}: {
  handleOnClick: () => void;
  action: "+" | "-";
  disabled?: boolean;
}) => {
  return (
    <button
      onClick={handleOnClick}
      className="flex items-center bg-surface-secondary p-1 rounded-md cursor-pointer disabled:cursor-not-allowed"
      type="button"
      disabled={disabled}
    >
      {action === "+" ? (
        <PlusIcon className="h-6 w-6" />
      ) : (
        <MinusIcon className="h-6 w-6" />
      )}
    </button>
  );
};

const NFTSelect = ({
  collectionAtom,
  className,
}: {
  collectionAtom: WritableAtom<
    CollectionNFT,
    [SetStateAction<CollectionNFT>],
    void
  >;
  className?: string;
}) => {
  const [state, setState] = useAtom(collectionAtom);

  const addNFT = () => {
    if (state.nfts.length) {
      const nfts = state.nfts;
      const nft = nfts.shift() as IdNFT;
      const nftsSelected = [...state.nftsSelected, nft];
      const totalCountSelected = nftsSelected.length;
      const totalGramsSelected = totalCountSelected * state.value;
      const totalGLDTSelected = totalGramsSelected * GLDT_VALUE_1G_NFT;
      setState((state) => ({
        ...state,
        nfts,
        nftsSelected,
        totalCountSelected,
        totalGramsSelected,
        totalGLDTSelected,
      }));
    }
  };

  const removeNFT = () => {
    if (state.nftsSelected.length) {
      const nftsSelected = state.nftsSelected;
      const nft = nftsSelected.pop() as IdNFT;
      const nfts = [...state.nfts, nft];
      const totalCountSelected = nftsSelected.length;
      const totalGramsSelected = totalCountSelected * state.value;
      const totalGLDTSelected = totalGramsSelected * GLDT_VALUE_1G_NFT;
      setState((state) => ({
        ...state,
        nfts,
        nftsSelected,
        totalCountSelected,
        totalGramsSelected,
        totalGLDTSelected,
      }));
    }
  };

  return (
    <div className={className}>
      <div className={clsx("grid grid-cols-3 gap-4 p-4")}>
        <div className="flex items-center justify-start gap-2">
          <img
            className="flex-none h-10"
            src={`/gold-bars/${state.name}.svg`}
          />
          <div className="flex-grow font-semibold">{state.label}</div>
        </div>
        <div className="flex justify-between items-center border border-border bg-surface px-1 rounded-lg">
          <BtnSelect
            handleOnClick={removeNFT}
            action="-"
            disabled={!state.nftsSelected.length}
          />
          <div className="w-6 flex justify-center">
            <div>{state.totalCountSelected}</div>
          </div>
          <BtnSelect
            handleOnClick={addNFT}
            action="+"
            disabled={!state.nfts.length}
          />
        </div>
        <div className="flex justify-center items-center text-content/80 font-light">
          {state.isInititialized ? (
            <div>
              {state.totalCountSelected} / {state.totalCount}
            </div>
          ) : (
            <div className="flex justify-center items-center">Loading...</div>
          )}
        </div>
      </div>
    </div>
  );
};

export default NFTSelect;
