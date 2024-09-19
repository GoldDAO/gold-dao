import { useWallet } from "@amerej/artemis-react";

import { Button } from "@components/ui";

const ConnectWallet = () => {
  const { handleOpenWalletList } = useWallet();
  return (
    <Button className="rounded-xl w-full py-3" onClick={handleOpenWalletList}>
      Connect a Wallet
    </Button>
  );
};

export default ConnectWallet;
