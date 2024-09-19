import { useState } from "react";
import { FieldValues, useWatch } from "react-hook-form";
import { divideBy1e8 } from "@utils/numbers";
import { Button, LoaderSpin } from "@components/ui";
import { useTransferProceedLedger } from "@context/transfer/proceed-ledger";

import FieldPrincipalId from "../form/field/ledger/To";
import FieldAmount from "../form/field/ledger/Amount";
import Balance from "../balance/Balance";

import ConfirmDialog from "./confirm-dialog";

const TransferLedger = () => {
  const { form, handleShowDialogConfirm, handleSubmitForm, balance, state } =
    useTransferProceedLedger();
  const { fee, ledger } = state;

  const {
    handleSubmit,
    control,
    formState: { errors, isValid },
  } = form;
  const [txFee] = useState(divideBy1e8(fee));

  const onSubmit = (data: FieldValues) => {
    handleSubmitForm(data as { amount: string; to: string });
    handleShowDialogConfirm();
  };

  const Amount = () => {
    const watchedAmount = useWatch({
      name: "amount",
      control,
      defaultValue: 0,
    });

    const nAmount = Number(watchedAmount);
    if (isNaN(nAmount) || nAmount === 0 || errors?.amount) {
      return <div>0 {ledger}</div>;
    } else {
      const result = divideBy1e8(nAmount * 100000000 - fee);
      return (
        <div>
          {result} {ledger}
        </div>
      );
    }
  };

  return (
    <>
      {balance.isLoading && (
        <div className="flex justify-center py-16">
          <LoaderSpin />
        </div>
      )}
      {balance.isSuccess && (
        <>
          <form onSubmit={handleSubmit(onSubmit)}>
            <div className="mt-4 p-4 border border-border rounded-xl">
              <FieldPrincipalId className="" />
            </div>
            <div className="mt-4 p-4 border border-border rounded-xl">
              <FieldAmount balance={balance.data} transactionFee={fee} />
            </div>

            <div className="flex justify-between items-center mt-8 mx-2">
              <div className="inline-flex justify-start items-center text-content/60 text-sm rounded-lg">
                <div>Fee: </div>
                <div className="flex items-center">
                  <img
                    className="mx-2 h-4 w-4"
                    src={`/${ledger.toLocaleLowerCase()}_logo.svg`}
                    alt={`${ledger} Logo`}
                  />
                  <span>
                    {txFee} {ledger}
                  </span>
                </div>
              </div>
              <div className="inline-flex justify-start items-center text-content/60 text-sm rounded-lg">
                <div>Amount received: </div>
                <div className="flex items-center">
                  <img
                    className="mx-2 h-4 w-4"
                    src={`/${ledger.toLocaleLowerCase()}_logo.svg`}
                    alt={`${ledger} Logo`}
                  />
                  <Amount />
                </div>
              </div>
            </div>

            <Button
              type="submit"
              disabled={!isValid}
              className="mt-8 w-full py-3 rounded-lg"
            >
              Transfer
            </Button>

            <div className="flex justify-center mt-6">
              <Balance ledger={ledger} balance={balance.data.number} />
            </div>
          </form>
          <ConfirmDialog />
        </>
      )}
    </>
  );
};

export default TransferLedger;
