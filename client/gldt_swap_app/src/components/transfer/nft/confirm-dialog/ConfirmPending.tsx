import { LoaderSpin } from "@components/ui";

const ConfirmPending = () => {
  return (
    <div className="flex flex-col justify-center items-center">
      <div className="w-full border border-orange-400 bg-orange-400/5 p-6 rounded-xl text-center text-orange-400">
        <div>The transferring process can take a little time.</div>
        <div className="text-lg mt-4 font-semibold text-orange-400">
          Do not refresh the page !
        </div>
      </div>
      <div className="font-semibold mt-8 mb-6">
        Transfer is being processed!
      </div>
      <LoaderSpin />
    </div>
  );
};

export default ConfirmPending;
