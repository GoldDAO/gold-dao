import { useTranslation } from "react-i18next";
import { Icon } from "@components/icons/Icon";

const BurnOverview = ({ className }: { className?: string }) => {
  const { t } = useTranslation("dashboard/buy_back_and_burn_overview");

  return (
    <div className={className}>
      <div className="border border-border rounded-xl bg-surface-primary p-6 h-full shadow-[10px_10px_60px_-20px_rgba(252,148,88,100)]">
        <div className="mb-6">
          <div className="flex items-start gap-3">
            <Icon name="burn" />
            <div>
              <div>{t("burn.gldgov_burnt")}</div>
              <div>(TODO)</div>
            </div>
          </div>
        </div>

        <div className=""></div>
      </div>
    </div>
  );
};

export default BurnOverview;
