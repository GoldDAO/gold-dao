import { Link } from "react-router-dom";
import { useWallet } from "@amerej/artemis-react";

import { Button } from "@components/ui";
import DropdownBalance from "./dropdown-balance/DropdownBalance";
import DropdownUser from "./dropdown-user/DropdownUser";

const Swap = () => {
  const { isConnected, handleOpenWalletList } = useWallet();

  return (
    <>
      <nav className="sticky top-0 px-6 py-5 z-40 bg-background">
        <div className="grid grid-cols-2 items-center h-10">
          <div className="flex-shrink-0">
            <Link to="/swap" className="flex items-center space-x-2">
              <img src="/vite.svg" alt="GLDT Logo" />
              <span className="self-center text-xl font-semibold whitespace-nowrap hidden sm:block">
                GLDT Swap App
              </span>
            </Link>
          </div>
          <div className="flex justify-self-end items-center">
            {!isConnected && (
              <Button onClick={handleOpenWalletList}>Connect</Button>
            )}

            {isConnected && (
              <div className="flex items-center gap-2">
                <DropdownBalance />
                <DropdownUser />
              </div>
            )}
          </div>
        </div>
      </nav>
    </>
  );
};

export default Swap;
