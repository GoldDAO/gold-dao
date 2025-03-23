// import { InformationCircleIcon } from "@heroicons/react/20/solid";
import { Button } from "@components/index";
import { useTranslation } from "react-i18next";

// import { LoaderSpin, Logo } from "@components/index";

const GoldDAORewards = ({ className }: { className?: string }) => {
  const { t } = useTranslation("sns/sns_rewards");

  return (
    <div className={`${className}`}>
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-16">
        <div className="">
          <div>{t("description")}</div>
          <div className="mt-6">
            <Button>Learn More</Button>
          </div>
        </div>
        <div className="">Slider</div>
      </div>
    </div>
  );
};

export default GoldDAORewards;
