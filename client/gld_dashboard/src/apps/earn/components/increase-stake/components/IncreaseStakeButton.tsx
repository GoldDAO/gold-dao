import Icon from "@shared/ui/icons";
import VerticalButton from "@shared/ui/button/VerticalButton";

const IncreaseStakeButton = ({
  handleOnClick,
  disabled,
  className,
}: {
  handleOnClick?: () => void;
  disabled?: boolean;
  className?: string;
}) => {
  return (
    <div className={className}>
      <VerticalButton
        icon={<Icon.PlusCircle width={24} aria-label="Increase Stake" />}
        onClick={handleOnClick}
        disabled={disabled}
      >
        Increase Stake
      </VerticalButton>
    </div>
  );
};

export default IncreaseStakeButton;
