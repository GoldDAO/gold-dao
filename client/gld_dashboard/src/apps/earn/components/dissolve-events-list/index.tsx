import { HTMLAttributes, PropsWithChildren } from "react";
import { UseQueryResult } from "@tanstack/react-query";
import clsx from "clsx";
import { LoaderSpin } from "@components/loaders";
import { Position } from "@earn/interfaces";
import List from "./DissolveEventsList";

interface DissolveEventsListProps
  extends PropsWithChildren<HTMLAttributes<HTMLDivElement>> {
  position: UseQueryResult<Position, Error>;
}

const styles = {
  container: clsx(
    "flex items-center justify-center gap-4",
    "border border-border bg-surface-primary",
    "rounded-xl p-4"
  ),
};

const DissolveEventsList = ({
  position,
  ...props
}: DissolveEventsListProps) => {
  return (
    <div {...props}>
      {(position.isLoading || position.isError) && (
        <div className={styles.container}>
          <LoaderSpin size="sm" />
          <div>Fetching dissolve events...</div>
        </div>
      )}
      {position.isSuccess && position.data.dissolve_events.length >= 1 && (
        <List
          className="flex flex-col gap-2"
          events={position.data.dissolve_events}
        />
      )}
      {position.isSuccess && position.data.dissolve_events.length === 0 && (
        <div className={styles.container}>
          <div className="text-sm font-semibold">No dissolve events found.</div>
        </div>
      )}
    </div>
  );
};

export default DissolveEventsList;
