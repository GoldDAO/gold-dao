import { CollectionName } from "@context/index";

export const Title = ({
  collectionName: name,
}: {
  collectionName: CollectionName;
}) => {
  return (
    <div className="flex items-center justify-start gap-4">
      <img className="flex-none h-8" src={`gold-bars/${name}.svg`} />
      <div className="flex-grow font-semibold">{name}</div>
    </div>
  );
};
