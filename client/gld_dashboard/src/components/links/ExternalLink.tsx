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
      className={`inline-flex items-center ${className} hover:underline`}
    >
      <div>{children}</div>
      <ArrowTopRightOnSquareIcon className="ml-2 h-4 w-4" />
    </a>
  );
};

export default ExternalLink;
