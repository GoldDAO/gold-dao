import { ReactNode } from "react";

const ChartError = ({
  //   errorMessage = "Unexpected error",
  children,
}: {
  errorMessage?: string;
  children: ReactNode;
}) => {
  // TODO: Add dialog with error message and send report
  return (
    <div className="flex items-center justify-center h-72">
      <div className="flex flex-col items-center">
        <h3 className="mb-6 text-lg font-semibold">{children}</h3>
        <button className="border border-red-400 text-red-400 rounded-full px-6 py-2 font-semibold">
          Show Error
        </button>
      </div>
    </div>
  );
};

export default ChartError;
