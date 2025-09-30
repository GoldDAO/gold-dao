import { ReactNode } from "react";

const SpanBold = ({ children }: { children: React.ReactNode }) => (
  <span className="font-bold">{children}</span>
);

const Badge = ({
  className,
  title,
  subtitle,
  icon,
}: {
  className?: string;
  title: ReactNode;
  subtitle: ReactNode;
  icon: ReactNode;
}) => {
  return (
    <div className={className}>
      <div className="flex items-center gap-2 bg-burgundy text-white rounded-full px-4 py-2">
        <div className="bg-white h-10 w-10 rounded-full flex items-center justify-center">
          {icon}
        </div>
        <div className="text-left">
          <div className="">{title}</div>
          <div className="text-sm">{subtitle}</div>
        </div>
      </div>
    </div>
  );
};

const WhatIsGLDT = ({
  className,
  ...restProps
}: { className?: string } & React.HTMLAttributes<HTMLElement>) => {
  const BADGES: {
    title: ReactNode;
    subtitle: ReactNode;
    icon: ReactNode;
  }[] = [
    {
      icon: (
        <div>
          <img
            src="/Gold.svg"
            alt="Gold icon"
            className="mx-auto w-6 h-6 align-center"
          />
        </div>
      ),
      title: (
        <>
          Backed by NFT <span className="text-gold font-semibold">per bar</span>
        </>
      ),
      subtitle: <>Counter to PAXG or Tether Gold</>,
    },
    {
      icon: (
        <div>
          <img
            src="/AiOutlineFall.svg"
            alt="Low transaction fee icon"
            className="mx-auto w-6 h-6 align-center"
          />
        </div>
      ),
      title: (
        <>
          <span className="text-gold font-semibold">Low</span> transaction fees
        </>
      ),
      subtitle: <>Counter to PAXG or Tether Gold</>,
    },
    {
      icon: (
        <div>
          <img
            src="/BsSafe.svg"
            alt="Storage transparency icon"
            className="mx-auto w-6 h-6 align-center"
          />
        </div>
      ),
      title: (
        <>
          Storage <span className="text-gold font-semibold">transparency</span>
        </>
      ),
      subtitle: <>Counter to PAXG or Tether Gold</>,
    },
    {
      icon: (
        <div>
          <img
            src="/MdOutlineBadge.svg"
            alt="No KYC icon"
            className="mx-auto w-6 h-6 align-center"
          />
        </div>
      ),
      title: (
        <>
          <span className="text-gold font-semibold">No KYC</span> required
        </>
      ),
      subtitle: <>Counter to PAXG or Tether Gold</>,
    },
  ];

  return (
    <section className={className} {...restProps}>
      <div className="flex items-center justify-center px-4 py-16 md:py-24">
        <div className="flex flex-col gap-4 md:gap-8 justify-center text-content">
          <div className="text-4xl md:text-6xl text-center">
            <h2 className="mb-2">
              What is <span className="text-gold font-semibold">GLDT?</span>
            </h2>
          </div>
          <h3 className="max-w-[800px] mx-auto text-center">
            The Gold token (GLDT) is a digital token backed 100% by real gold.
            Each token <SpanBold>equals 0.01 grams of physical gold</SpanBold>,
            safely stored in vaults in Switzerland. It lets you own gold without
            needing to store or handle it yourself—plus, you can{" "}
            <SpanBold>send it instantly and trade it</SpanBold> easily, with{" "}
            <SpanBold>no storage fees</SpanBold>.
          </h3>

          <div className="grid grid-cols-1 md:grid-cols-4 gap-4 md:gap-8 my-8">
            {BADGES.map((badge, index) => (
              <Badge
                key={index}
                title={badge.title}
                subtitle={badge.subtitle}
                icon={badge.icon}
              />
            ))}
          </div>

          <div>
            <video
              src="/video_gldt.mp4"
              autoPlay
              loop
              muted
              playsInline
              controls
              className="w-full h-auto rounded-xl object-cover"
              aria-label="Présentation GLDT"
            />
          </div>
        </div>
      </div>
    </section>
  );
};

export default WhatIsGLDT;
