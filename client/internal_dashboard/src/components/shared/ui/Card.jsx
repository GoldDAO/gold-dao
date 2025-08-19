import { ArrowUpRightIcon } from "@heroicons/react/20/solid";

const Card = ({ children, className }) => {
  return (
    <div className={className}>
      <div className="bg-white/70 dark:bg-neutral-700/40 rounded-4xl p-6">
        {children}
      </div>
    </div>
  );
};

Card.Header = ({ children, className }) => {
  return (
    <div className={className}>
      <div className="h-10 flex justify-between items-center md:mb-1">
        {children}
      </div>
    </div>
  );
};

Card.Title = ({ children }) => {
  return (
    <h3 className="text-lg md:text-xl xl:text-2xl font-light">{children}</h3>
  );
};

Card.Link = ({ href }) => {
  return (
    <a href={href} target="_blank" rel="noopener noreferrer">
      <button className="w-10 h-10 rounded-full bg-white dark:bg-neutral-900 hover:bg-white/80 dark:hover:bg-neutral-900/80 cursor-pointer">
        <div className="flex justify-center items-center w-full h-full">
          <ArrowUpRightIcon className="w-5 h-5" />
        </div>
      </button>
    </a>
  );
};

export default Card;
