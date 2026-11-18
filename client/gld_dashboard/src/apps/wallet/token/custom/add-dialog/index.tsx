import { ReactNode } from "react";
import clsx from "clsx";
import { useAtom, useAtomValue, useSetAtom } from "jotai";
import { useForm } from "react-hook-form";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import { useSearchParams } from "react-router-dom";
import { useAuth } from "@auth/index";
import Dialog from "@shared/ui/dialog/Dialog";
import Icon from "@shared/ui/icons";
import BtnPrimary from "@shared/ui/button/HorizontalButton";
import TokenLogo, { ledgerLogoQueryKey } from "@shared/components/token-logo";
import { TOKENS } from "@shared/utils/tokens";
import { toCustomToken } from "@shared/utils/tokens/custom";
import { isValidPrincipal } from "@shared/utils/validators/isValidPrincipal";
import {
  AddCustomTokenAtom,
  AddCustomTokenDialogAtom,
  CustomTokensAtom,
} from "@wallet/shared/atoms/CustomTokensAtom";
import lookupCustomToken, {
  CustomTokenLookup,
} from "@wallet/token/custom/lookupCustomToken";

interface FormValues {
  ledger_id: string;
  index_id: string;
}

const Field = ({
  label,
  error,
  hint,
  children,
}: {
  label: string;
  error?: string;
  hint?: string;
  children: ReactNode;
}) => (
  <div>
    <div className="text-copper text-sm font-semibold mb-2">{label}</div>
    <div className="p-4 border border-border rounded-lg bg-surface-secondary">
      {children}
    </div>
    {error ? (
      <div className="text-danger text-sm mt-1">{error}</div>
    ) : (
      hint && <div className="text-content/60 text-sm mt-1">{hint}</div>
    )}
  </div>
);

const inputClassName = clsx(
  "w-full outline-none focus:outline-none focus:ring-0 bg-surface-secondary",
  "placeholder:text-content/40"
);

const Row = ({ label, children }: { label: string; children: ReactNode }) => (
  <div className="flex flex-col gap-1 text-sm">
    <div className="text-content/60">{label}</div>
    <div className="break-all">{children}</div>
  </div>
);

const Review = ({
  lookup,
  handleAdd,
}: {
  lookup: CustomTokenLookup;
  handleAdd: () => void;
}) => {
  const { token, decimals } = lookup;

  return (
    <>
      <div className="flex items-center gap-3 mb-6">
        <TokenLogo token={toCustomToken(token)} className="h-12 w-12" />
        <div className="min-w-0">
          <div className="text-xl font-semibold truncate">{token.symbol}</div>
          <div className="text-content/60 truncate">{token.name}</div>
        </div>
      </div>

      <div className="flex flex-col gap-4 border border-border rounded-lg p-4">
        <Row label="Ledger canister ID">{token.ledger_id}</Row>
        <Row label="Index canister ID">
          {token.index_id ??
            "None. The wallet will not show a transaction history for this token."}
        </Row>
        <Row label="Decimals">{decimals}</Row>
      </div>

      <div className="flex gap-3 border border-warning bg-warning/5 p-4 rounded-xl mt-4 text-sm">
        <Icon.Warning
          width={20}
          height={20}
          className="text-warning shrink-0"
        />
        <div>
          Anyone can create a token and give it any name, symbol and logo.
          Compare the ledger canister ID with a source you trust before you add
          it.
        </div>
      </div>

      <BtnPrimary onClick={handleAdd} className="w-full mt-8">
        Add token
      </BtnPrimary>
    </>
  );
};

// Adds a token to the wallet by the canister ID of its ICRC-1 ledger: a form
// for the IDs, then a review of what the ledger reports.
const AddCustomTokenDialog = () => {
  const { unauthenticatedAgent } = useAuth();
  const queryClient = useQueryClient();
  const [, setSearchParams] = useSearchParams();
  const [isOpen, setIsOpen] = useAtom(AddCustomTokenDialogAtom);
  const customTokens = useAtomValue(CustomTokensAtom);
  const addCustomToken = useSetAtom(AddCustomTokenAtom);

  const {
    register,
    handleSubmit,
    reset,
    formState: { errors },
  } = useForm<FormValues>({
    defaultValues: { ledger_id: "", index_id: "" },
  });

  const lookup = useMutation({
    mutationFn: lookupCustomToken,
    // The logo came with the rest; keep it so it is not fetched again.
    onSuccess: ({ token, logo }) => {
      queryClient.setQueryData(ledgerLogoQueryKey(token.ledger_id), logo);
    },
  });

  const handleClose = () => {
    setIsOpen(false);
    reset();
    lookup.reset();
  };

  const handleLookup = (values: FormValues) => {
    if (!unauthenticatedAgent) return;
    lookup.mutate({
      agent: unauthenticatedAgent,
      ledgerId: values.ledger_id.trim(),
      indexId: values.index_id.trim(),
      walletTokens: [...TOKENS, ...customTokens],
    });
  };

  const handleAdd = () => {
    if (!lookup.data) return;
    const { token } = lookup.data;
    addCustomToken(token);
    setSearchParams({ token: token.symbol });
    handleClose();
  };

  const isCanisterId = (value: string) =>
    isValidPrincipal(value.trim()) || "This is not a valid canister ID";

  return (
    <Dialog
      open={isOpen}
      handleOnClose={handleClose}
      size="md"
      handlePreviousStep={lookup.isSuccess ? () => lookup.reset() : undefined}
    >
      <div className="text-4xl text-center mb-8">
        Add <span className="font-semibold text-gold">token</span>
      </div>

      {lookup.isSuccess ? (
        <Review lookup={lookup.data} handleAdd={handleAdd} />
      ) : (
        <form onSubmit={handleSubmit(handleLookup)}>
          <div className="flex flex-col gap-4">
            <Field
              label="Ledger canister ID"
              error={errors.ledger_id?.message}
              hint="The canister ID of the token's ICRC-1 ledger."
            >
              <input
                id="ledger_id"
                type="text"
                autoComplete="off"
                spellCheck={false}
                placeholder="mxzaz-hqaaa-aaaar-qaada-cai"
                className={inputClassName}
                {...register("ledger_id", {
                  onChange: () => lookup.reset(),
                  validate: (value) =>
                    value.trim() === ""
                      ? "The ledger canister ID is required"
                      : isCanisterId(value),
                })}
              />
            </Field>
            <Field
              label="Index canister ID (optional)"
              error={errors.index_id?.message}
              hint="Used for the transaction history. Leave it empty to take the index canister the ledger names."
            >
              <input
                id="index_id"
                type="text"
                autoComplete="off"
                spellCheck={false}
                className={inputClassName}
                {...register("index_id", {
                  onChange: () => lookup.reset(),
                  validate: (value) =>
                    value.trim() === "" || isCanisterId(value),
                })}
              />
            </Field>
          </div>

          <div className="text-content/60 text-sm mt-4">
            The token is saved in this browser. It will not show up on your
            other devices.
          </div>

          {lookup.isError && (
            <div className="text-danger text-sm mt-4">
              {lookup.error.message}
            </div>
          )}

          <BtnPrimary
            type="submit"
            disabled={lookup.isPending || !unauthenticatedAgent}
            className="w-full mt-8"
          >
            {lookup.isPending ? "Reading the ledger..." : "Continue"}
          </BtnPrimary>
        </form>
      )}
    </Dialog>
  );
};

export default AddCustomTokenDialog;
