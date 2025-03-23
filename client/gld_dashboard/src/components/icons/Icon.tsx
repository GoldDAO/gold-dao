export const Icon = ({
  name,
  className,
}: {
  name: string | "buy_back" | "burn";
  className?: string;
}) => {
  const pathIcons = "/icons";
  const icons = {
    buy_back: { alt: "Buy back icon", src: "/buy_back.svg" },
    burn: { alt: "Burn icon", src: "/burn.svg" },
  };
  return (
    <img
      src={pathIcons + icons[name].src}
      alt={icons[name].alt}
      className={className ? className : "w-8 h-8"}
    />
  );
};
