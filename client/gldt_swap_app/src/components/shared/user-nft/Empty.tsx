import { Link } from "react-router-dom";
import { Button } from "@components/ui";

const Empty = () => {
  return (
    <div className="border border-orange-500 bg-orange-500/5 py-8 px-4 flex flex-col justify-center items-center rounded-xl">
      <div className="mb-6 font-semibold text-orange-500 text-center">
        You currently don't own any GLD NFTs!
      </div>
      <div>
        <Link
          to="https://gold.bity.com"
          target="_blank"
          rel="noopener noreferrer"
        >
          <Button>Buy GLD NFTs</Button>
        </Link>
      </div>
    </div>
  );
};

export default Empty;
