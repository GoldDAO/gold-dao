import { ReactNode } from "react";
import { Link, useLocation } from "react-router-dom";
import clsx from "clsx";
import navItems from "@shared/components/app-layout/navbars/shared/utils";

const NavLink = ({
  title,
  subtitle,
  url,
  icon,
  isActive,
  isDisabled = false,
}: {
  title: string;
  subtitle?: string;
  url: string;
  icon: ReactNode;
  isActive: boolean;
  isDisabled?: boolean;
}) => {
  return (
    <Link
      type="button"
      to={isDisabled ? "#" : url}
      className={clsx(
        "flex flex-col justify-center items-center rounded-xl shrink-0 w-full py-2 min-w-24",
        "text-sm hover:text-white dark:hover:text-black",
        "hover:bg-linear-to-tr hover:from-[#A0812E] hover:from-30% hover:via-primary hover:via-60% hover:to-[#FFF0CA] hover:to-100%",
        "hover:dark:from-[#A0812E] hover:dark:via-[#A0812E]/60 hover:dark:to-[#A0812E]/30 border hover:border-surface-primary",
        {
          "pointer-events-none opacity-50 cursor-not-allowed": isDisabled,
          "bg-linear-to-tr from-[#A0812E] from-30% via-primary via-70% to-[#FFF0CA] to-100% dark:from-[#A0812E] dark:via-[#A0812E]/80 dark:to-[#A0812E]/40 text-white dark:text-black border-surface-primary":
            isActive && !isDisabled,
          "border-border": !isActive || isDisabled,
        }
      )}
      aria-disabled={isDisabled}
    >
      <div className="h-5">{icon}</div>
      <div className="mt-1 w-full text-center">
        {title}
        {subtitle && <div className="text-xs">{subtitle}</div>}
      </div>
    </Link>
  );
};

const SideNav = ({ className }: { className?: string }) => {
  const location = useLocation();
  const active = location.pathname;

  return (
    <nav className={className}>
      <div className="h-full flex flex-col items-center justify-center gap-4 text-content/60">
        {navItems.map(({ title, subtitle, url, icon }, i) => (
          <NavLink
            key={i}
            title={title}
            subtitle={subtitle}
            url={url}
            icon={icon}
            isActive={active.startsWith(url)}
            isDisabled={"/earn".startsWith(url)} // You can set this based on your logic
          />
        ))}
      </div>
    </nav>
  );
};

export default SideNav;
