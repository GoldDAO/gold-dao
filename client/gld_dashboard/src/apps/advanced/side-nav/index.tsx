import clsx from "clsx";
import { ChevronRightIcon } from "@heroicons/react/24/solid";
import { useAuth } from "@auth/index";
import BtnConnectWallet from "@shared/components/connect-wallet-btn";

const AdvancedLeftPanel = () => {
  const { isConnected } = useAuth();
  return (
    <>
      <div className="text-4xl xl:text-6xl flex flex-col justify-center items-center xl:items-start">
        <div className="font-semibold text-gold">Advanced</div>
        <div className="font-light">features</div>
      </div>
      <div className="text-content/60 text-center xl:text-left my-3">
        Dive into advanced features of the Gold DAO such as minting and burning
        of GLDT, and more to come.
      </div>
      <div className="mt-8 w-full">
        <div className="flex justify-between items-center py-3 px-4 border border-border rounded-xl bg-surface-secondary hover:bg-gold/20 cursor-pointer">
          <div>GLDT</div>
          <ChevronRightIcon className={clsx("w-5 h-5")} />
        </div>
      </div>
      {!isConnected && <BtnConnectWallet className="mt-auto w-full" />}
    </>
  );
};

export default AdvancedLeftPanel;
