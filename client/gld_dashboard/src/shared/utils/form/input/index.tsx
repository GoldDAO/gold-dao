export const onKeyDownPreventNoDigits = (
  e: React.KeyboardEvent<HTMLInputElement>
) => {
  if (e.key === "e" || e.key === "-" || e.key === "+") e.preventDefault();
};

export const onPastePreventNoDigits = (
  e: React.ClipboardEvent<HTMLInputElement>
) => {
  const clipboardData = e.clipboardData;
  const text = clipboardData.getData("text");
  const number = parseFloat(text);
  if (number < 0) e.preventDefault();
  if (text === "e" || text === "+" || text === "-") e.preventDefault();
};
