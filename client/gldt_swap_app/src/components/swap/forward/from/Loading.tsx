import { LoaderSpin } from "@components/ui";

const Loading = () => {
  return (
    <div className="border border-border py-8 px-4 flex flex-col justify-center items-center rounded-xl">
      <div className="mb-6 font-semibold">Fetching your NFT's...</div>
      <div>
        <LoaderSpin />
      </div>
    </div>
  );
};

export default Loading;
