const Error = ({ error }: { error: Error | null }) => {
  return (
    <div className="border border-red-500 bg-red-500/5 py-8 px-4 flex flex-col justify-center items-center rounded-xl">
      <div className="font-semibold text-red-500">
        Error while fetching your NFT's!
      </div>
      {error?.message && (
        <div className="text-red-500 mt-6">{error?.message}</div>
      )}
    </div>
  );
};

export default Error;
