import { decodeIcrcAccount } from "@dfinity/ledger-icrc";
import { useTransferProceedLedger } from "@context/transfer/proceed-ledger";

import Input from "@components/ui/form/Input";

const To = ({ className }: { className?: string }) => {
  const { form } = useTransferProceedLedger();
  const {
    register,
    formState: { errors, dirtyFields },
  } = form;

  const isValidRecipientAddress = (value: string) => {
    try {
      decodeIcrcAccount(value);
      return true;
      // eslint-disable-next-line @typescript-eslint/no-unused-vars
    } catch (err) {
      return false;
    }
  };
  return (
    <div className={className}>
      <label htmlFor="to" className="text-gold text-sm font-semibold mb-2">
        Principal ID
      </label>
      <Input
        className="px-4 py-3 mt-2 mb-1 bg-surface-2 border border-border rounded-lg w-full text-center"
        id="to"
        placeholder="0x-000-000-000"
        type="text"
        {...register("to", {
          required: "Principal ID is required.",
          validate: {
            isValidRecipientAddress: (v) =>
              isValidRecipientAddress(v) || "Invalid Principal ID.",
          },
        })}
        // ? fix form set errors when amount is setted via max button
        errors={Object.keys(dirtyFields).length !== 0 && errors?.to}
      />
    </div>
  );
};

export default To;
