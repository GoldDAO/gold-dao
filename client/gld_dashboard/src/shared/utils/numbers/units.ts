// Conversion between a decimal amount as text ("1.5") and the smallest unit of
// a token, done on the digits. Going through a floating point number loses
// precision once an amount has more than 15 significant digits, which a token
// with 18 decimals reaches below 0.01.

// Returns null when the text is not a plain, non-negative decimal number.
// Digits beyond the token's decimals are dropped.
export const parseUnits = (value: string, decimals: number): bigint | null => {
  const match = /^(\d*)(?:\.(\d*))?$/.exec(value.trim());
  if (!match) return null;
  const whole = match[1];
  const fraction = match[2] ?? "";
  if (whole === "" && fraction === "") return null;
  return BigInt(
    (whole || "0") + fraction.slice(0, decimals).padEnd(decimals, "0")
  );
};

// The exact decimal text of an amount given in the smallest unit.
export const formatUnits = (value: bigint, decimals: number): string => {
  const digits = value.toString().padStart(decimals + 1, "0");
  const whole = digits.slice(0, digits.length - decimals);
  const fraction = digits.slice(digits.length - decimals).replace(/0+$/, "");
  return fraction ? `${whole}.${fraction}` : whole;
};
