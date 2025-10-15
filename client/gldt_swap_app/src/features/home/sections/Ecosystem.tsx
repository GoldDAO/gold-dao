import GldnftCard from "@assets/cards/gldnft.jpg";
import UsdgCard from "@assets/cards/usdg.jpg";
import GoldaoCard from "@assets/cards/goldao.jpg";

const Card = ({
  badge,
  title,
  subtitle,
  img,
  icon,
}: {
  badge: string;
  title: string;
  subtitle: string;
  icon?: React.ReactNode;
  img: string;
}) => (
  <div className="bg-surface rounded-xl shadow-md h-full flex flex-col">
    <div className="border-x border-t border-border p-8 rounded-xl">
      <div className="flex justify-between items-start">
        <div>
          <div className="inline-flex rounded-full bg-gold text-black text-sm px-2">
            {badge}
          </div>
          <h4 className="text-3xl font-semibold mt-2 mb-2">{title}</h4>
          <p className="text-sm flex-grow text-content/80">{subtitle}</p>
        </div>
        <div>{icon}</div>
      </div>
    </div>

    <div>
      <img
        src={img}
        alt={title}
        className="max-w-[400px] rounded-b-xl h-auto"
      />
    </div>
  </div>
);

const Ecosystem = ({
  className,
  ...restProps
}: { className?: string } & React.HTMLAttributes<HTMLElement>) => {
  const Cards = [
    {
      badge: "Governance",
      title: "GOLDAO",
      subtitle: "Shape decisions through governance",
      icon: (
        <img src="/gold_dao.svg" alt="Governance icon" className="w-8 h-8" />
      ),
      img: GoldaoCard,
    },
    {
      badge: "NFT",
      title: "GLD NFTs",
      subtitle: "Trackable Gold ownership",
      icon: (
        <img src="/gold_nugget.svg" alt="NFT gold icon" className="w-8 h-8" />
      ),
      img: GldnftCard,
    },
    {
      badge: "Stablecoin",
      title: "USDG",
      subtitle: "A Gold-backed stablecoin",
      icon: <div className="px-4 rounded-full bg-surface-3">Coming soon</div>,
      img: UsdgCard,
    },
  ];

  return (
    <section className={className} {...restProps}>
      <div className="container mx-auto px-4">
        <div className="grid grid-cols-1 md:grid-cols-2 md:gap-16">
          <div>
            <div className="text-center text-4xl md:text-6xl md:text-left">
              <h2 className="mb-2">Inside the GLDT</h2>
              <h2 className="text-gold font-semibold">ecosystem</h2>
            </div>
            <h3 className="text-2xl font-light text-center md:text-left mt-4">
              One ecosystem. Endless utility.
            </h3>
          </div>

          <div className="flex flex-row items-center gap-8 overflow-x-auto w-full py-4 scrollbar-hide">
            {Cards.map((card) => (
              <div className="mt-8 min-w-[400px]" key={card.title}>
                <Card {...card} />
              </div>
            ))}
          </div>
        </div>
      </div>
    </section>
  );
};

export default Ecosystem;
