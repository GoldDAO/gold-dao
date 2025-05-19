import { ReactNode } from "react";

const Card = ({ children }: { children: ReactNode }) => {
  return (
    <div className="p-4 border border-border rounded-lg bg-surface-secondary">
      {children}
    </div>
  );
};

export default Card;
