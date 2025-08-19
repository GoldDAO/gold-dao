import Icon from "@shared/ui/icons";
import VerticalButton from "@shared/ui/button/VerticalButton";

const DecreaseStakeButton = ({
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
        icon={<Icon.MinusCircle width={24} aria-label="Decrease Stake" />}
        onClick={handleOnClick}
        disabled={disabled}
      >
        Decrease Stake
      </VerticalButton>
    </div>
  );
};

export default DecreaseStakeButton;
