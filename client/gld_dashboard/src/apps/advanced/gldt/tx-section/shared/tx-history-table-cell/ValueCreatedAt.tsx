const RenderValueCreatedAt = ({ value }: { value: string }) => {
  const [datePart, timePart] = value.split(",");
  return (
    <>
      <div>{datePart}</div>
      <div className="text-content/60 text-xs">{timePart}</div>
    </>
  );
};

export default RenderValueCreatedAt;
