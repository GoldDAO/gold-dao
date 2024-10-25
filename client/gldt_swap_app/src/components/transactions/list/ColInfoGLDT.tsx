import { ReactNode } from "react";

import { LogoGLDT } from "@components/shared/logos";

const ColInfoGLDT = ({
  children,
  className,
}: {
  children: ReactNode;
  className?: string;
}) => {
  return (
    <div className={className}>
      <div className="flex items-center gap-2">
        <LogoGLDT className="flex-none w-4 h-4" />
        <div className="font-semibold">{children}</div>
        <div className="text-content/60 text-sm">GLDT</div>
      </div>
    </div>
  );
};

export default ColInfoGLDT;
