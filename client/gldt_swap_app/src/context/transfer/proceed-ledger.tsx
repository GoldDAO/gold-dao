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
import { divideBy1e8, numberToE8s } from "@utils/numbers";
import { useLedgerTransfer, useLedgerUserBalance } from "@hooks/ledger/index";

export interface TransferProceedLedgerState {
  ledger: string;
  to: string;
  amount: string;
  balance: number | null;
  balanceE8s: number | null;
  balanceAfterTransfer: number | null;
  fee: number;
}

const initialState: TransferProceedLedgerState = {
  ledger: "GLDT",
  to: "",
  amount: "",
  balance: null,
  balanceE8s: null,
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
    const _amount = Number(data.amount) * 100000000 - getFeeByLedger(ledger);
    setState((prevState) => ({
      ...prevState,
      amount: divideBy1e8(_amount).toString(),
      to: data.to,
      balanceAfterTransfer:
        state.balanceE8s !== null
          ? divideBy1e8(
              BigInt(state.balanceE8s) -
                BigInt(_amount) -
                BigInt(getFeeByLedger(ledger))
            )
          : 0,
    }));
  };

  const handleTransfer = () => {
    mutation.mutate(
      {
        amount: numberToE8s(state.amount),
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

  useEffect(() => {
    if (balance.isSuccess && balance.data) {
      setState((prevState) => ({
        ...prevState,
        balance: balance.data.number,
        balanceE8s: balance.data.e8s,
      }));
    }
  }, [balance.isSuccess, balance.data]);

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
