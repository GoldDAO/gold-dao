import { useAuth } from "@auth/index";
import BtnPrimary from "@shared/components/ui/button/BtnPrimary";

const ConnectWalletBtn = ({
  className,
  size = "md",
}: {
  className?: string;
  size?: "sm" | "md" | "lg";
}) => {
  const { connect } = useAuth();
  return (
    <BtnPrimary className={className} size={size} onClick={connect}>
      Connect Wallet
    </BtnPrimary>
  );
};

export default ConnectWalletBtn;
