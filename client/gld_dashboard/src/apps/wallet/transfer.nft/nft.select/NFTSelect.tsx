import { useAtom } from "jotai";
import clsx from "clsx";
import { PlusIcon, MinusIcon } from "@heroicons/react/20/solid";
import { CollectionNameNFT } from "@services/gld_nft/utils/interfaces";
import { SelectNFTStateReducerAtom } from "@atoms/NFTState";

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
  collection,
  className,
}: {
  collection: CollectionNameNFT;
  className?: string;
}) => {
  const [selectNFTState, dispatchSelectNFTState] = useAtom(
    SelectNFTStateReducerAtom
  );

  return (
    <div className={className}>
      <div className={clsx("grid grid-cols-3 gap-4 p-4")}>
        <div className="flex items-center justify-start gap-2">
          <img
            className="flex-none h-10"
            src={`/gold-bars/${selectNFTState[collection].name}.svg`}
          />
          <div className="flex-grow font-semibold">
            {selectNFTState[collection].label}
          </div>
        </div>
        <div className="flex justify-between items-center border border-border bg-surface px-1 rounded-lg">
          <BtnSelect
            handleOnClick={() =>
              dispatchSelectNFTState({
                type: "SET_REMOVE_NFT",
                value: collection,
              })
            }
            action="-"
            disabled={!selectNFTState[collection].nfts_selected.length}
          />
          <div className="w-6 flex justify-center">
            <div>{selectNFTState[collection].total_count_selected}</div>
          </div>
          <BtnSelect
            handleOnClick={() =>
              dispatchSelectNFTState({ type: "SET_ADD_NFT", value: collection })
            }
            action="+"
            disabled={!selectNFTState[collection].nfts.length}
          />
        </div>
        <div className="flex justify-center items-center text-content/80 font-light">
          {selectNFTState[collection].is_initialized ? (
            <div>
              {selectNFTState[collection].total_count_selected} /{" "}
              {selectNFTState[collection].total_count}
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
