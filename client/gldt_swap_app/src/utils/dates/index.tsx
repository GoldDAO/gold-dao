import { DateTime, DurationObjectUnits } from "luxon";

export const getCurrentTimestamp = () =>
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
  options?: { fromSeconds?: boolean; fromMillis?: boolean; fromISO?: boolean }
): string => {
  let dateTime = DateTime.fromMillis(Number(date), { zone: "utc" });
  if (options?.fromSeconds)
    dateTime = DateTime.fromSeconds(Number(date), { zone: "utc" });
  // else if (options?.fromMillis)
  //   dateTime = DateTime.fromMillis(Number(date) / 1000000, { zone: "utc" });
  else if (options?.fromISO)
    dateTime = DateTime.fromISO(date.toString(), { zone: "utc" });
  const result = dateTime.toFormat("yyyy-LL-dd, hh:mm:ss a z");
  return result;
};

export const formatYearsDifference = (timestamp: number) => {
  const givenDate = DateTime.fromSeconds(timestamp);
  const now = DateTime.now();
  const difference = givenDate.diff(now, ["years"]);
  const { years } = difference.toObject();
  const roundedYears = Math.round(years ?? 0);
  return `${roundedYears} year${roundedYears > 1 ? "s" : ""}`;
};

export const formatTimestampDifference = (timestamp: number) => {
  const givenDate = DateTime.fromSeconds(timestamp);
  const now = DateTime.now();
  const difference = now.diff(givenDate, [
    "years",
    "months",
    "days",
    "hours",
    "minutes",
    "seconds",
  ]);

  const units: { unit: keyof DurationObjectUnits; label: string }[] = [
    { unit: "years", label: "year" },
    { unit: "months", label: "month" },
    { unit: "days", label: "day" },
    { unit: "hours", label: "hour" },
    { unit: "minutes", label: "minute" },
    { unit: "seconds", label: "second" },
  ];
  const formattedDifference = units.find(({ unit }) => difference[unit]);
  return formattedDifference
    ? `${difference[formattedDifference.unit]} ${formattedDifference.label}${
        difference[formattedDifference.unit] > 1 ? "s" : ""
      }`
    : "Just now";
};
