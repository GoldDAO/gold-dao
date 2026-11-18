import { useSetAtom } from "jotai";
import Dialog from "@shared/ui/dialog/Dialog";
import BtnPrimary from "@shared/ui/button/HorizontalButton";
import { Token } from "@shared/utils/tokens";
import { RemoveCustomTokenAtom } from "@wallet/shared/atoms/CustomTokensAtom";

// Asks before a custom token is taken out of the wallet list. Only the entry
// saved in the browser goes; nothing happens on the ledger.
const RemoveCustomTokenDialog = ({
  token,
  open,
  handleClose,
}: {
  token: Token;
  open: boolean;
  handleClose: () => void;
}) => {
  const removeCustomToken = useSetAtom(RemoveCustomTokenAtom);

  const handleRemove = () => {
    // The wallet page selects GOLDAO once the token is gone.
    removeCustomToken(token.canister_id);
    handleClose();
  };

  return (
    <Dialog open={open} handleOnClose={handleClose} size="md">
      <div className="text-4xl text-center mb-8 break-words">
        Remove <span className="font-semibold text-gold">{token.name}</span>
      </div>

      <div className="text-content/60">
        {token.name} will no longer show in your wallet. Your balance stays on
        the ledger, and you can add the token again with its ledger canister
        ID:
      </div>
      <div className="mt-4 p-4 border border-border rounded-lg text-sm break-all">
        {token.canister_id}
      </div>

      <div className="flex gap-4 mt-8">
        <BtnPrimary
          onClick={handleClose}
          variant="outlined"
          className="w-full"
        >
          Cancel
        </BtnPrimary>
        <BtnPrimary onClick={handleRemove} className="w-full">
          Remove
        </BtnPrimary>
      </div>
    </Dialog>
  );
};

export default RemoveCustomTokenDialog;
