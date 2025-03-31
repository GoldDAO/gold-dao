import { Link, useLocation } from "react-router-dom";

import navItems from "./navItems.utils";

const SideNav = ({ className }: { className?: string }) => {
  const location = useLocation();
  const active = location.pathname;

  return (
    <nav className={className}>
      <div className="h-full flex flex-col items-center justify-center gap-8">
        {navItems.map(({ title, url }, i) => (
          <Link
            to={url}
            className={`text-content/60 text-sm hover:font-semibold ${
              active === url ? "font-semibold" : ""
            }`}
            key={i}
          >
            {title}
          </Link>
        ))}
      </div>
    </nav>
  );
};

export default SideNav;
