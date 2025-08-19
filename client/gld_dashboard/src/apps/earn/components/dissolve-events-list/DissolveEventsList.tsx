import { HTMLAttributes, PropsWithChildren } from "react";
import { DissolveEvent } from "@earn/interfaces";
import { formatRoundedTimeUnits } from "@shared/utils/dates";
// import BtnPrimary from "@shared/ui/button/HorizontalButton";
import NumberToLocaleString from "@shared/components/numbers/NumberToLocaleString";

interface DissolveEventsListProps
  extends PropsWithChildren<HTMLAttributes<HTMLDivElement>> {
  events: DissolveEvent[];
}

const DissolveEventsList = ({ events, ...props }: DissolveEventsListProps) => {
  return (
    <div {...props}>
      {events.map((event, index) => (
        <div
          key={index}
          className="flex justify-between items-center p-4 border border-border rounded-xl"
        >
          <div>
            <div className="text-lg font-semibold">
              <NumberToLocaleString value={event.amount} /> GLDT
            </div>
            {event.is_withdrawable && (
              <div className="text-sm text-content/60">Unlocked</div>
            )}
            {!event.is_withdrawable && (
              <div className="text-sm text-content/60">
                Unlocking in{" "}
                {formatRoundedTimeUnits(Number(event.remaining_time) / 1000)}
              </div>
            )}
          </div>
        </div>
      ))}
    </div>
  );
};

export default DissolveEventsList;
