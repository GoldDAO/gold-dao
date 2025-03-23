import { useTranslation } from "react-i18next";
import { Icon } from "@components/icons/Icon";

const BuyBackOverview = ({ className }: { className?: string }) => {
  const { t } = useTranslation("dashboard/buy_back_and_burn_overview");

  return (
    <div className={className}>
      <div className="border border-border rounded-xl bg-surface-primary p-6 h-full shadow-[-10px_-10px_60px_-20px_rgba(201,212,174,100)]">
        <div className="mb-6">
          <div className="flex items-start gap-3">
            <Icon name="buy_back" />
            <div>
              <div>{t("buy_back.canister_icp_balance")}</div>
              <div>(TODO)</div>
            </div>
          </div>
        </div>
        <div className="">
          <div className="flex items-start gap-3">
            <Icon name="buy_back" />
            <div>
              <div>{t("buy_back.total_icp_used")}</div>
              <div>(TODO)</div>
            </div>
          </div>
        </div>
        <div className=""></div>
      </div>
    </div>
  );
};

export default BuyBackOverview;
