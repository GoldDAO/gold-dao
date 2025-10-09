import clsx from "clsx";
import { ReactNode } from "react";
import NumberToLocaleString from "@shared/components/numbers/NumberToLocaleString";

const styles = {
  container: clsx("border border-success bg-surface-primary rounded-xl"),
  title: clsx("text-success text-center xl:text-left"),
  totalAmount: clsx("font-semibold text-xl"),
  description: clsx("text-sm text-content/60"),
};

const RewardsAvailable = ({
  amount,
  claimRewardsButton,
}: {
  amount: number;
  claimRewardsButton: ReactNode;
}) => {
  // const [, dispatchClaimReward] = useAtom(ClaimRewardStateReducerAtom);

  return (
    <div className={styles.container}>
      <div className="rounded-[inherit] p-4 bg-success/10">
        <div className={styles.title}>Unclaimed rewards available</div>
        <div className="flex flex-col xl:flex-row justify-between items-center mt-2 gap-4">
          <div className="flex flex-col items-center xl:items-start shrink-0">
            <div className={styles.totalAmount}>
              Total of:{" "}
              <span>
                <NumberToLocaleString value={amount} decimals={5} />$
              </span>
            </div>
            <div className={styles.description}>
              dispatched in GOLDAO, ICP and OGY
            </div>
          </div>
          {claimRewardsButton}
        </div>
      </div>
    </div>
  );
};

export default RewardsAvailable;
