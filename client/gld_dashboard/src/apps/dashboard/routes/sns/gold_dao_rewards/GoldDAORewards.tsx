import { useTranslation } from "react-i18next";

const GoldDAORewards = ({ className }: { className?: string }) => {
  const { t } = useTranslation("sns/sns_rewards");

  return (
    <div className={`${className}`}>
      <div className="grid grid-cols-1 xl:grid-cols-2 gap-16">
        <div className="">
          <div>{t("description")}</div>
          <div className="mt-6">
            <button>Learn More</button>
          </div>
        </div>
        <div className="">Slider</div>
      </div>
    </div>
  );
};

export default GoldDAORewards;
