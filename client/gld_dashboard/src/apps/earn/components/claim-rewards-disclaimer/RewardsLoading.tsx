import { LoaderSpin } from "@components/loaders";
import clsx from "clsx";

const styles = {
  container: clsx(
    "flex items-center justify-center gap-4",
    "border border-border bg-surface-primary",
    "rounded-xl p-4 py-10"
  ),
};

const RewardsLoading = () => {
  return (
    <div className={styles.container}>
      <LoaderSpin size="sm" />
      <div>Fetching your rewards...</div>
    </div>
  );
};

export default RewardsLoading;
