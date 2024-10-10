const Error = ({ error }: { error: string }) => {
  return (
    <div className="border border-dark-orange bg-dark-orange/5 py-8 px-4 flex flex-col justify-center items-center rounded-xl">
      <div className="font-semibold text-dark-orange">{error}</div>
    </div>
  );
};

export default Error;
