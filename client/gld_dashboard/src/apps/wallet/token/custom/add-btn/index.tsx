import clsx from "clsx";
import { useSetAtom } from "jotai";
import Icon from "@shared/ui/icons";
import { AddCustomTokenDialogAtom } from "@wallet/shared/atoms/CustomTokensAtom";

// Last row of the wallet list: opens the "Add token" dialog.
const AddCustomTokenBtn = ({
  handleOnClick,
}: {
  // Runs before the dialog opens, e.g. to close the list it was opened from.
  handleOnClick?: () => void;
}) => {
  const setIsOpen = useSetAtom(AddCustomTokenDialogAtom);

  return (
    <button
      type="button"
      className={clsx(
        "shrink-0 flex justify-center items-center gap-2",
        "rounded-xl border border-dashed border-border p-4 cursor-pointer",
        "text-sm font-semibold text-content/60",
        "hover:border-gold hover:text-gold"
      )}
      onClick={() => {
        handleOnClick?.();
        setIsOpen(true);
      }}
    >
      <Icon.Plus width={16} />
      Add token
    </button>
  );
};

export default AddCustomTokenBtn;
