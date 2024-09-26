const Backdrop = ({
  handleOnClick = () => null,
  isClickable = false,
}: {
  handleOnClick?: () => null | void;
  isClickable?: boolean;
}) => {
  return (
    <div
      onClick={handleOnClick}
      className={`absolute h-full w-full z-50 ${
        isClickable ? "cursor-pointer" : ""
      }`}
    />
  );
};

export default Backdrop;
