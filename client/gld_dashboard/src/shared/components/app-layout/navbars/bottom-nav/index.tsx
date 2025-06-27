import { VERSION_DAPP } from "@constants";
import Icon from "@shared/ui/icons";
import { ThemeToggle } from "@shared/components/theme-toggle";

const BottomNav = () => {
  return (
    <div className="flex flex-col xl:flex-row xl:justify-between items-center justify-center gap-4 xl:gap-0">
      <div>
        <ThemeToggle />
      </div>
      <div className="flex flex-col xl:flex-row items-center gap-2 xl:gap-6 text-content/60 text-xs">
        <a
          href="https://docs.gold-dao.org"
          target="_blank"
          rel="noopener noreferrer"
          className="inline-flex items-center hover:underline"
        >
          Docs
          <Icon.ExternalLink width={16} height={16} className="ml-2" />
        </a>
        <div>Version {VERSION_DAPP ?? "undefined"}</div>
      </div>
    </div>
  );
};

export default BottomNav;
