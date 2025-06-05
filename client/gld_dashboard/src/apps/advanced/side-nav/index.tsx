import clsx from "clsx";
import { ChevronRightIcon } from "@heroicons/react/24/solid";
import { useAuth } from "@auth/index";
import Button from "@components/buttons/Button";

const AdvancedLeftPanel = () => {
  const { isConnected, connect } = useAuth();
  return (
    <div className="flex flex-col items-center text-center xl:text-left xl:items-start xl:flex-grow px-4 xl:px-8">
      <div className="text-5xl xl:text-6xl flex flex-col justify-center items-center xl:items-start font-semibold mt-4">
        <div className="font-semibold text-primary/90">Advanced</div>
        <div className="font-light">features</div>
      </div>
      <div className="text-content/60 my-3">
        Dive into advanced features of the Gold DAO such as minting and burning
        of GLDT, and more to come.
      </div>
      <div className="mt-8 w-full">
        <div className="flex justify-between items-center py-3 px-4 border border-border rounded-xl bg-surface-secondary hover:bg-primary/20 transition-colors cursor-pointer">
          <div>GLDT</div>
          <ChevronRightIcon className={clsx("w-5 h-5")} />
        </div>
      </div>
      {!isConnected && (
        <Button
          className="mt-auto w-full px-4 py-3 bg-secondary xl:text-lg text-white rounded-md"
          onClick={connect}
        >
          Connect Wallet
        </Button>
      )}
    </div>
  );
};

export default AdvancedLeftPanel;
