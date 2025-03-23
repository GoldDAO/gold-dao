import { ReactNode } from "react";
import { ArrowTopRightOnSquareIcon } from "@heroicons/react/24/outline";

const ExternalLink = ({
  children,
  className,
  href,
}: {
  children: ReactNode;
  className?: string;
  href: string;
}) => {
  return (
    <a
      href={href}
      target="_blank"
      rel="noopener noreferrer"
      className={`flex items-center hover:font-semibold text-accent ${className}`}
    >
      <div>{children}</div>
      <ArrowTopRightOnSquareIcon className="ml-2 h-5 w-5" />
    </a>
  );
};

export default ExternalLink;
