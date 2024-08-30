import { Button } from "@components/ui";

const Empty = () => {
  return (
    <div className="border border-orange-500 bg-orange-500/5 py-8 px-4 flex flex-col justify-center items-center rounded-xl">
      <div className="mb-6 font-semibold text-orange-500">
        You currently don't own any GLD NFT!
      </div>
      <div>
        <Button>Buy GLD NFT</Button>
      </div>
    </div>
  );
};

export default Empty;
