import { useWallet } from "@amerej/artemis-react";

import { Button } from "@components/ui";

const LogoutButton = () => {
  const { handleDisconnectWallet } = useWallet();
  return (
    <Button className="rounded-xl w-full py-3" onClick={handleDisconnectWallet}>
      Logout
    </Button>
  );
};

export default LogoutButton;
