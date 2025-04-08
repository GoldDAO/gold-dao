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
              "hover:bg-linear-to-bl hover:from-primary/60 hover:to-primary hover:dark:from-primary hover:dark:to-primary/50 border hover:border-surface-primary",
              `${
                active === url
                  ? "bg-linear-to-bl from-primary/40 to-primary dark:from-primary dark:to-primary/40 text-white dark:text-black border-surface-primary"
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
