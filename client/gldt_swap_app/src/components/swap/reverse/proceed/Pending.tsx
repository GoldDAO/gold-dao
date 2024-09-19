import { LoaderSpin } from "@components/ui";

const Pending = () => {
  return (
    <div className="flex flex-col items-center">
      <div className="w-full border border-orange-400 bg-orange-400/5 p-6 rounded-xl text-center text-orange-400">
        <div>The swapping process can take a little time.</div>
        <div className="text-lg mt-4 font-semibold text-orange-400">
          Do not refresh the page !
        </div>
      </div>
      <div className="my-8 font-semibold">Swapping tokens...</div>
      <LoaderSpin />
    </div>
  );
};

export default Pending;
