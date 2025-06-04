import { ExternalLink } from "@components/index";

const BottomNav = () => {
  return (
    <div className="flex flex-col xl:flex-row xl:justify-between items-center justify-center">
      <div></div>
      <div className="flex flex-col xl:flex-row items-center gap-2 xl:gap-6 text-content/60">
        <ExternalLink href="https://docs.gold-dao.org/">
          More about Gold DAO
        </ExternalLink>
        <div>Terms of use</div>
        <div>Version 1.0.0</div>
      </div>
    </div>
  );
};

export default BottomNav;
