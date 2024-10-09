import { LoaderSpin } from "@components/ui";

const ConfirmPending = () => {
  return (
    <div className="px-4 pb-6 flex flex-col justify-center items-center">
      <LoaderSpin />
      <div className="font-semibold text-xl mt-8">
        Transfer is being processed!
      </div>
      <div className="text-content/60">This can take a few seconds</div>
    </div>
  );
};

export default ConfirmPending;
