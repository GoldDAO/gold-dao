import { HTMLAttributes, PropsWithChildren } from "react";
import { Position } from "@earn/interfaces";
import List from "./DissolveEventsList";

interface DissolveEventsListProps
  extends PropsWithChildren<HTMLAttributes<HTMLDivElement>> {
  position: Position;
}

const DissolveEventsList = ({
  position,
  ...props
}: DissolveEventsListProps) => {
  return (
    <div {...props}>
      <List className="flex flex-col gap-2" events={position.dissolve_events} />
    </div>
  );
};

export default DissolveEventsList;
