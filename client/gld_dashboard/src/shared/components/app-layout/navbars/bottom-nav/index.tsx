import { ExternalLink } from "@components/index";
import { VERSION_DAPP } from "@constants";
import { ThemeToggle } from "@shared/components/theme-toggle";

const BottomNav = () => {
  return (
    <div className="flex flex-col xl:flex-row xl:justify-between items-center justify-center gap-4 xl:gap-0">
      <div>
        <ThemeToggle />
      </div>
      <div className="flex flex-col xl:flex-row items-center gap-2 xl:gap-6 text-content/60 text-xs">
        <ExternalLink href="https://docs.gold-dao.org/">Docs</ExternalLink>
        <div>Version {VERSION_DAPP ?? "undefined"}</div>
      </div>
    </div>
  );
};

export default BottomNav;
