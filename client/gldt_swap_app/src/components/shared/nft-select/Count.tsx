import { PlusIcon, MinusIcon } from "@heroicons/react/20/solid";
import { CollectionIndex } from "@context/index";

export const Count = ({
  collectionIndex: i,
  count,
  handleOnChangeCount = () => {},
}: {
  collectionIndex: CollectionIndex;
  count: number;
  handleOnChangeCount: (
    collectionIndex: number,
    type: string
  ) => void | (() => void);
}) => {
  return (
    <div className="flex items-center border border-border bg-surface p-1 rounded-lg gap-4">
      <button
        onClick={() => handleOnChangeCount(i, "-")}
        className="flex items-center bg-surface-2 p-1 rounded-md"
        type="button"
      >
        <MinusIcon className="h-4 w-4" />
      </button>
      <div className="w-8 flex justify-center">
        <div>{count}</div>
      </div>
      <button
        onClick={() => handleOnChangeCount(i, "+")}
        className="flex items-center bg-surface-2 p-1 rounded-md"
        type="button"
      >
        <PlusIcon className="h-4 w-4" />
      </button>
    </div>
  );
};
