import { Link } from "react-router-dom";
import { useTranslation, Trans } from "react-i18next";
import clsx from "clsx";

import BuyBackOverview from "./-components/BuyBackOverview";
import BurnOverview from "./-components/BurnOverview";

const BuyBackAndBurnOverview = ({ className }: { className?: string }) => {
  const { t } = useTranslation("dashboard/buy_back_and_burn_overview");
  const goldDAOKnowledgeHubURL = "https://docs.gold-dao.org/";

  return (
    <div className={className}>
      <div className="border border-border rounded-xl bg-surface-primary p-6">
        <div className="grid grid-cols-1 xl:grid-cols-3 gap-6">
          <BuyBackOverview />
          <div className="p-6 text-center">
            <div className="flex items-center">
              <div>
                <h3 className="text-3xl font-semibold mb-6">
                  <Trans
                    t={t}
                    i18nKey="title"
                    components={{
                      1: (
                        <span
                          className={clsx(
                            `bg-gradient-to-r from-orange-600 to-orange-200 inline-block text-transparent bg-clip-text`
                          )}
                        />
                      ),
                    }}
                  />
                </h3>
                <p className="mb-6">{t("description")}</p>
                <Link
                  className="text-content/60"
                  to={goldDAOKnowledgeHubURL}
                  target="_blank"
                  rel="noopener noreferrer"
                >
                  {t("learn_more_link")}
                </Link>
              </div>
            </div>
          </div>
          <BurnOverview />
        </div>
      </div>
    </div>
  );
};

export default BuyBackAndBurnOverview;
