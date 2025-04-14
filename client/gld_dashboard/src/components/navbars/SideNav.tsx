import { Link, useLocation } from "react-router-dom";
import clsx from "clsx";

import navItems from "./navItems.utils";

const SideNav = ({ className }: { className?: string }) => {
  const location = useLocation();
  const active = location.pathname;

  return (
    <nav className={className}>
      <div className="h-full flex flex-col items-center justify-center gap-4 text-content/60">
        {navItems.map(({ title, url, icon }, i) => (
          <Link
            type="button"
            to={url}
            className={clsx(
              "flex flex-col justify-center items-center rounded-xl shrink-0 w-full p-2",
              "text-sm hover:text-white dark:hover:text-black",
              "hover:bg-linear-to-tr hover:from-[#A0812E] hover:from-30% hover:via-primary hover:via-60% hover:to-[#FFF0CA] hover:to-100%",
              "hover:dark:from-[#A0812E] hover:dark:via-[#A0812E]/60 hover:dark:to-[#A0812E]/30 border hover:border-surface-primary",
              `${
                active === url
                  ? "bg-linear-to-tr from-[#A0812E] from-30% via-primary via-70% to-[#FFF0CA] to-100% dark:from-[#A0812E] dark:via-[#A0812E]/80 dark:to-[#A0812E]/40 text-white dark:text-black border-surface-primary"
                  : "border-border"
              }`
            )}
            key={i}
          >
            <div className="h-5">{icon}</div>
            <div className="mt-1 w-full text-center">{title}</div>
          </Link>
        ))}
      </div>
    </nav>
  );
};

export default SideNav;

// bg-radial-[at_25%_25%] from-white to-zinc-900 to-75%
