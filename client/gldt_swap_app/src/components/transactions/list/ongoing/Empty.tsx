const Empty = ({ className }: { className?: string }) => {
  return (
    <div className={`${className}`}>
      <div className="border border-border rounded-xl bg-surface p-8">
        <div className="font-semibold text-content/60 text-lg text-center">
          No ongoing transactions.
        </div>
      </div>
    </div>
  );
};

export default Empty;
