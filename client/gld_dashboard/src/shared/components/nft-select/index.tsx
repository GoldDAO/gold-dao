import { useAtom } from "jotai";
import clsx from "clsx";
import { PlusIcon, MinusIcon } from "@heroicons/react/20/solid";
import { CollectionNameNFT } from "@services/gld_nft/utils/interfaces";
import { SelectNFTStateReducerAtom } from "@shared/atoms/NFTStateAtom";

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

const NFTSelectTitle = ({ name, label }: { name: string; label: string }) => (
  <div className="flex items-center justify-start gap-2">
    <img className="flex-none h-10" src={`/gold-bars/${name}.svg`} />
    <div className="flex-grow font-semibold">{label}</div>
  </div>
);

const NFTSelectCounter = ({
  onRemove,
  onAdd,
  canRemove,
  canAdd,
  count,
}: {
  onRemove: () => void;
  onAdd: () => void;
  canRemove: boolean;
  canAdd: boolean;
  count: number;
}) => (
  <div className="flex justify-between items-center border border-border bg-surface-primary p-1 rounded-xl">
    <BtnSelect handleOnClick={onRemove} action="-" disabled={!canRemove} />
    <div className="w-6 flex justify-center">
      <div>{count}</div>
    </div>
    <BtnSelect handleOnClick={onAdd} action="+" disabled={!canAdd} />
  </div>
);

const NFTSelectStatus = ({
  isInitialized,
  countSelected,
  totalCount,
  className,
}: {
  isInitialized: boolean;
  countSelected: number;
  totalCount: number;
  className?: string;
}) => (
  <div className={className}>
    <div className="flex justify-center items-center text-content/60 font-light">
      {isInitialized ? (
        <div>
          {countSelected} / {totalCount}
        </div>
      ) : (
        <div className="flex justify-center items-center">Loading...</div>
      )}
    </div>
  </div>
);

const NFTSelect = ({ collection }: { collection: CollectionNameNFT }) => {
  const [selectNFTState, dispatchSelectNFTState] = useAtom(
    SelectNFTStateReducerAtom
  );
  const state = selectNFTState[collection];

  return (
    <div className="@container">
      <div
        className={clsx(
          "bg-surface-secondary grid grid-cols-1 @sm:grid-cols-3 items-center gap-2 @sm:gap-4 py-3 px-4 rounded-xl border border-border"
        )}
      >
        <div className="flex items-center justify-between @sm:justify-start">
          <NFTSelectTitle name={state.name} label={state.label} />
          <NFTSelectStatus
            className="block @sm:hidden"
            isInitialized={state.is_initialized}
            countSelected={state.total_count_selected}
            totalCount={state.total_count}
          />
        </div>

        <NFTSelectCounter
          onRemove={() =>
            dispatchSelectNFTState({
              type: "SET_REMOVE_NFT",
              value: collection,
            })
          }
          onAdd={() =>
            dispatchSelectNFTState({ type: "SET_ADD_NFT", value: collection })
          }
          canRemove={!!state.nfts_selected.length}
          canAdd={!!state.nfts.length}
          count={state.total_count_selected}
        />

        <NFTSelectStatus
          className="hidden @sm:block"
          isInitialized={state.is_initialized}
          countSelected={state.total_count_selected}
          totalCount={state.total_count}
        />
      </div>
    </div>
  );
};

export default NFTSelect;
