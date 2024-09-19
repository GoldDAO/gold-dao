import { ArrowDownIcon } from "@heroicons/react/20/solid";

const ArrowDown = () => {
  return (
    <div className="flex justify-center py-4">
      <button className="bg-content text-background rounded-full p-2 cursor-default">
        <ArrowDownIcon height={32} width={32} />
      </button>
    </div>
  );
};

export default ArrowDown;
