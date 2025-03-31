import { useTranslation } from "react-i18next";
import { InformationCircleIcon } from "@heroicons/react/20/solid";

import { useAuth } from "@auth/index";

import { LoaderSpin, Logo } from "@components/index";

import {
  SNS_NEURONS_ICP_CANISTER_ID,
  SNS_NEURONS_OGY_CANISTER_ID,
} from "@constants";

import useFetchSNSNeuronsICP from "@services/sns_neurons/hooks/useFetchSNSNeuronsICP";
import useFetchSNSNeuronsOGY from "@services/sns_neurons/hooks/useFetchSNSNeuronsOGY";
import useFetchSNSNeuronsWTN from "@services/sns_neurons/hooks/useFetchSNSNeuronsWTN";

const GoldDAOOwnedNeuronsOverview = ({ className }: { className?: string }) => {
  const { t } = useTranslation("dashboard/gold_dao_owned_neurons_overview");

  const { unauthenticatedAgent } = useAuth();

  const neuronsICP = useFetchSNSNeuronsICP(
    SNS_NEURONS_ICP_CANISTER_ID,
    unauthenticatedAgent,
    { enabled: !!unauthenticatedAgent }
  );
  const neuronsOGY = useFetchSNSNeuronsOGY(
    SNS_NEURONS_OGY_CANISTER_ID,
    unauthenticatedAgent,
    { enabled: !!unauthenticatedAgent }
  );
  const neuronsWTN = useFetchSNSNeuronsWTN(
    SNS_NEURONS_OGY_CANISTER_ID,
    unauthenticatedAgent,
    { enabled: !!unauthenticatedAgent }
  );

  return (
    <div className={className}>
      <div className="border border-border rounded-xl bg-surface-primary">
        <div className="p-6">
          <div className="text-center lg:text-left mb-6">
            <h6 className="text-lg font-semibold mb-4 lg:mb-6">{t("title")}</h6>
            <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
              <div className="border border-border rounded-xl bg-surface-primary p-6">
                <div className="flex items-center gap-3 mb-4 justify-center xl:justify-start">
                  <div className="font-light text-content/60">
                    {t("icp_neurons.title")}
                  </div>
                  <InformationCircleIcon
                    className="size-5 text-accent/60"
                    data-tooltip-id="tooltip"
                    data-tooltip-content={t("icp_neurons.tooltip")}
                  />
                </div>

                {neuronsICP.isSuccess && (
                  <div className="flex items-center justify-center xl:justify-start gap-4">
                    <div className="font-semibold text-4xl">
                      {neuronsICP.data.totalStakedAmount.string}
                    </div>
                    <Logo className="flex-none h-8" name="icp" />
                  </div>
                )}
                {(neuronsICP.isLoading || neuronsICP.isError) && (
                  <div className="flex justify-center">
                    <LoaderSpin />
                  </div>
                )}
              </div>

              <div className="border border-border rounded-xl bg-surface-primary p-6">
                <div className="flex items-center gap-3 mb-4 justify-center xl:justify-start">
                  <div className="font-light text-content/60">
                    {t("ogy_neurons.title")}
                  </div>
                  <InformationCircleIcon
                    className="size-5 text-accent/60"
                    data-tooltip-id="tooltip"
                    data-tooltip-content={t("ogy_neurons.tooltip")}
                  />
                </div>

                {neuronsOGY.isSuccess && (
                  <div className="flex items-center justify-center xl:justify-start gap-4">
                    <div className="font-semibold text-4xl">
                      {neuronsOGY.data.totalStakedAmount.string}
                    </div>
                    <Logo className="flex-none h-8" name="ogy" />
                  </div>
                )}
                {(neuronsOGY.isLoading || neuronsOGY.isError) && (
                  <div className="flex justify-center">
                    <LoaderSpin />
                  </div>
                )}
              </div>

              <div className="border border-border rounded-xl bg-surface-primary p-6">
                <div className="flex items-center gap-3 mb-4 justify-center xl:justify-start">
                  <div className="font-light text-content/60">
                    {t("wtn_neurons.title")}
                  </div>
                  <InformationCircleIcon
                    className="size-5 text-accent/60"
                    data-tooltip-id="tooltip"
                    data-tooltip-content={t("wtn_neurons.tooltip")}
                  />
                </div>

                {neuronsWTN.isSuccess && (
                  <div className="flex items-center justify-center xl:justify-start gap-4">
                    <div className="font-semibold text-4xl">
                      {neuronsWTN.data.totalStakedAmount.string}
                    </div>
                    <Logo className="flex-none h-8" name="waterneuron" />
                  </div>
                )}
                {(neuronsWTN.isLoading || neuronsWTN.isError) && (
                  <div className="flex justify-center">
                    <LoaderSpin />
                  </div>
                )}
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default GoldDAOOwnedNeuronsOverview;
