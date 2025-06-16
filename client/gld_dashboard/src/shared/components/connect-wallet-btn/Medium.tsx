import { useAuth } from "@auth/index";
import PrimaryBtn from "@shared/components/ui/button/medium/Primary";

const ConnectWalletBtn = ({ className }: { className?: string }) => {
  const { connect } = useAuth();
  return (
    <PrimaryBtn className={className} onClick={connect}>
      Connect Wallet
    </PrimaryBtn>
  );
};

export default ConnectWalletBtn;
