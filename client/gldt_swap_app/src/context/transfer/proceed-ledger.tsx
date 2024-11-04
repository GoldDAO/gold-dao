import {
  createContext,
  useContext,
  ReactNode,
  useState,
  useMemo,
  useEffect,
} from "react";
import { useQueryClient } from "@tanstack/react-query";
import { useForm } from "react-hook-form";

import { OGY_TX_FEE, GLDT_TX_FEE } from "@constants";
import { divideBy1e8, roundAndFormatLocale } from "@utils/numbers";
import { useLedgerTransfer, useLedgerUserBalance } from "@hooks/ledger/index";

export interface TransferProceedLedgerState {
  ledger: string;
  to: string;
  fee: number;
  balanceAfterTransfer: {
    string: string;
    number: number;
  } | null;
  amount: {
    string: string;
    bigint: bigint;
  } | null;
}

const initialState: TransferProceedLedgerState = {
  ledger: "GLDT",
  to: "",
  amount: null,
  balanceAfterTransfer: null,
  fee: 0,
};

const TransferProceedLedgerContext = createContext<ReturnType<
  typeof useTransferProceedLedgerProviderValue
> | null>(null);

// eslint-disable-next-line react-refresh/only-export-components
export const useTransferProceedLedger = () => {
  const context = useContext(TransferProceedLedgerContext);
  if (!context) {
    throw new Error(
      "useTransferProceedLedger must be used within a TransferProceedLedgerProvider"
    );
  }
  return context;
};

const useTransferProceedLedgerProviderValue = ({
  ledger,
}: {
  ledger: string;
}) => {
  const queryClient = useQueryClient();
  const [state, setState] = useState<TransferProceedLedgerState>(initialState);
  const [show, setShow] = useState(false);
  const handleShowDialogConfirm = () => setShow(true);
  const handleCloseDialogConfirm = () => setShow(false);
  const mutation = useLedgerTransfer({ ledger });

  const balance = useLedgerUserBalance({ ledger });

  const form = useForm({
    mode: "onChange",
    shouldUnregister: true,
    shouldFocusError: false,
  });

  const handleSubmitForm = (data: { amount: string; to: string }) => {
    const _amount = Math.floor(Number(data.amount) * 1e8);
    const balanceAfterTransfer =
      balance.isSuccess && balance.data
        ? divideBy1e8(BigInt(balance.data.e8s) - BigInt(_amount))
        : 0;

    setState((prevState) => ({
      ...prevState,
      amount: {
        string: roundAndFormatLocale({ number: divideBy1e8(_amount) }),
        bigint: BigInt(_amount),
      },
      to: data.to,
      balanceAfterTransfer: {
        number: balanceAfterTransfer,
        string: roundAndFormatLocale({ number: balanceAfterTransfer }),
      },
    }));
  };

  const handleTransfer = () => {
    if (state.amount) {
      mutation.mutate(
        {
          amount: state.amount.bigint,
          to: state.to,
        },
        {
          onSuccess: () => {
            queryClient.invalidateQueries({
              queryKey: [`USER_FETCH_BALANCE_${ledger}`],
            });
          },
        }
      );
    }
  };

  const handleReset = (): void => {
    setState(initialState);
    form.reset();
    mutation.reset();
  };

  const getFeeByLedger = (ledger: string) => {
    switch (ledger) {
      case "GLDT":
        return GLDT_TX_FEE;
      default:
        return OGY_TX_FEE;
    }
  };

  useEffect(() => {
    setState((prevState) => ({
      ...prevState,
      ledger,
      fee: getFeeByLedger(ledger),
    }));
  }, [ledger]);

  const value = useMemo(
    () => ({
      state,
      form,
      handleTransfer,
      handleReset,
      show,
      handleShowDialogConfirm,
      handleCloseDialogConfirm,
      mutation,
      handleSubmitForm,
      balance,
    }),
    // eslint-disable-next-line react-hooks/exhaustive-deps
    [state, form.formState, balance.data]
  );
  return value;
};

export const TransferProceedLedgerProvider = ({
  children,
  ledger,
}: {
  children: ReactNode;
  ledger: string;
}) => {
  const contextValue = useTransferProceedLedgerProviderValue({ ledger });

  return (
    <TransferProceedLedgerContext.Provider value={contextValue}>
      {children}
    </TransferProceedLedgerContext.Provider>
  );
};
