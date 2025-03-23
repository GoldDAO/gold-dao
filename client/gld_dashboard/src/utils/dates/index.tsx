import { DateTime, DurationObjectUnits, Duration } from "luxon";

export const getCurrentTimestampSeconds = () =>
  Math.floor(DateTime.local().toSeconds());

export const getCurrentDateInSeconds = () =>
  Math.floor(DateTime.local().startOf("hour").toSeconds()).toString();

export const getCurrentDateInNanoseconds = () =>
  DateTime.now().toMillis() * 1000000;

export const nowMinusOneDayToSeconds = () =>
  Math.floor(
    DateTime.local()
      .minus({ days: 1 })
      // .plus({ days: DateTime.local().weekday - 1 })
      .toSeconds()
  ).toString();

export const nowMinusOneWeekToSeconds = () =>
  Math.floor(
    DateTime.local()
      .minus({ weeks: 1 })
      // .plus({ days: DateTime.local().weekday - 1 })
      .toSeconds()
  ).toString();

export const nowMinusOneMonthToSeconds = () =>
  Math.floor(
    DateTime.local()
      .minus({ months: 1 })
      // .plus({ days: DateTime.local().weekday - 1 })
      .toSeconds()
  ).toString();

export const nowMinusOneYearToSeconds = () =>
  Math.floor(
    DateTime.local()
      .minus({ years: 1 })
      // .plus({ days: DateTime.local().weekday - 1 })
      .toSeconds()
  ).toString();

export const getDateUTC = (
  date: string | number,
  options?: {
    fromSeconds?: boolean;
    fromMillis?: boolean;
    fromISO?: boolean;
    fromNanos?: boolean;
  }
): string => {
  let dateTime = DateTime.fromMillis(Number(date), { zone: "utc" });
  if (options?.fromSeconds)
    dateTime = DateTime.fromSeconds(Number(date), { zone: "utc" });
  else if (options?.fromNanos)
    dateTime = DateTime.fromMillis(Number(date) / 10e5, { zone: "utc" });
  else if (options?.fromISO)
    dateTime = DateTime.fromISO(date.toString(), { zone: "utc" });
  const result = dateTime.toFormat("yyyy-LL-dd, hh:mm:ss a z");
  return result;
};

export const formatTimestampToYearsDifference = (timestamp: number) => {
  const difference = DateTime.fromSeconds(timestamp)
    .diff(DateTime.now(), ["years"])
    .toObject();
  const years = Math.round(difference.years ?? 0);
  return `${years} year${years > 1 ? "s" : ""}`;
};

export const formatTimestampToYears = (timestamp: number) => {
  const duration = Duration.fromObject({ seconds: timestamp }).shiftTo(
    "years",
    "days"
  );
  const years = Math.floor(duration.years);
  const days = Math.round(duration.days);
  return `${years} year${years > 1 ? "s" : ""}${
    days ? ` and ${days} day${days > 1 ? "s" : ""}` : ""
  }`;
};

export const calculateTimeDifferenceInSeconds = (timestamp: number) => {
  const currentTimestamp = Math.floor(DateTime.local().toSeconds());
  const start = timestamp >= currentTimestamp ? timestamp : currentTimestamp;
  const end = timestamp < currentTimestamp ? timestamp : currentTimestamp;

  const diffInSeconds = DateTime.fromSeconds(start)
    .diff(DateTime.fromSeconds(end))
    .as("seconds");

  return diffInSeconds;
};

export const getRoundedYears = (timestamp: number) => {
  const duration = Duration.fromObject({ seconds: timestamp }).shiftTo("years");
  return Math.round(duration.years);
};

export const getRoundedMonths = (timestamp: number) => {
  const duration = Duration.fromObject({ seconds: timestamp }).shiftTo(
    "months"
  );
  return Math.floor(duration.months);
};

export const getRoundedWeeks = (timestamp: number) => {
  const duration = Duration.fromObject({ seconds: timestamp }).shiftTo("weeks");
  return Math.floor(duration.weeks);
};

export const getRoundedDays = (timestamp: number) => {
  const duration = Duration.fromObject({ seconds: timestamp }).shiftTo(
    "days",
    "hours"
  );
  return Math.floor(duration.days);
};

export const getRoundedHours = (timestamp: number) => {
  const duration = Duration.fromObject({ seconds: timestamp }).shiftTo("hours");
  return Math.floor(duration.hours);
};

export const getRoundedMinutes = (timestamp: number) => {
  const duration = Duration.fromObject({ seconds: timestamp }).shiftTo(
    "minutes"
  );
  return Math.floor(duration.minutes);
};
export const formatRoundedTimeUnits = (
  timestamp: number,
  options?: {
    startFromUnit?: keyof DurationObjectUnits;
    includeMinutesWithHours?: boolean;
  }
) => {
  const startFromUnit = options?.startFromUnit || "years";
  const includeMinutesWithHours = options?.includeMinutesWithHours ?? false;

  const units: {
    unit: keyof DurationObjectUnits;
    label: string;
    getter: (timestamp: number) => number;
  }[] = [
    { unit: "years", label: "year", getter: getRoundedYears },
    { unit: "months", label: "month", getter: getRoundedMonths },
    { unit: "weeks", label: "week", getter: getRoundedWeeks },
    { unit: "days", label: "day", getter: getRoundedDays },
    { unit: "hours", label: "hour", getter: getRoundedHours },
    { unit: "minutes", label: "minute", getter: getRoundedMinutes },
  ];

  const startIndex = units.findIndex(({ unit }) => unit === startFromUnit);

  for (let i = startIndex; i < units.length; i++) {
    const { unit, label, getter } = units[i];
    const value = getter(timestamp);

    if (value > 0) {
      if (unit === "hours" && includeMinutesWithHours) {
        const minutes = getRoundedMinutes(timestamp % 3600);
        return `${value} ${label}${value > 1 ? "s" : ""}${
          minutes > 0 ? ` and ${minutes} minute${minutes > 1 ? "s" : ""}` : ""
        }`;
      }
      return `${value} ${label}${value > 1 ? "s" : ""}`;
    }
  }

  return "Just now";
};

export const timestampToRelativeCalendar = (timestamp: number) => {
  const dateTime = DateTime.fromSeconds(timestamp);
  return dateTime.toRelativeCalendar();
};
