import { LoaderSpin } from "@components/index";

const ChartLoader = () => {
  return (
    <div className="flex items-center justify-center h-72">
      <LoaderSpin size="xl" />
    </div>
  );
};

export default ChartLoader;
