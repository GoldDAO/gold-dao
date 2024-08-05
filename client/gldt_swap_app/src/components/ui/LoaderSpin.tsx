const LoaderSpin = ({
  size,
  className,
}: {
  size?: "xs" | "sm" | "md" | "lg" | "xl";
  className?: string;
}) => {
  const getSize = (size?: string) => {
    switch (size) {
      case "xs":
        return "border-2 h-4 w-4 xl:h-4 xl:w-4 xl:border-2";
      case "sm":
        return "border-4 h-8 w-8";
      case "md":
        return "border-4 h-12 w-12";
      case "lg":
        return "border-4 h-16 w-16 xl:h-24 xl:w-24 xl:border-6";
      case "xl":
        return "border-4 h-16 w-16 xl:h-24 xl:w-24 xl:border-6";
      default:
        return "border-4 h-12 w-12"; // sm
    }
  };
  return (
    <div className={`${className}`}>
      <div
        className={`border-accent/20 border-t-accent animate-spin rounded-full ${getSize(
          size
        )}`}
      />
    </div>
  );
};

export default LoaderSpin;
