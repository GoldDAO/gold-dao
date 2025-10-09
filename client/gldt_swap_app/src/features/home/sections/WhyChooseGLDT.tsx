const SpanBold = ({ children }: { children: React.ReactNode }) => (
  <span className="font-bold text-white">{children}</span>
);

const WhyChooseGLDT = ({
  className,
  ...restProps
}: { className?: string } & React.HTMLAttributes<HTMLElement>) => {
  return (
    <section className={className} {...restProps}>
      <div
        className="bg-burgundy grid grid-cols-1 md:grid-cols-2 md:rounded-[inherit] md:rounded-b-none gap-0 md:gap-32"
        style={{
          background: "linear-gradient(90deg, #3a0d1a 0%, #5a1830 100%)",
        }}
      >
        <div className="md:rounded-[inherit] order-0 md:order-1 relative w-full h-full flex items-stretch">
          <svg
            viewBox="0 0 800 600"
            className="md:rounded-[inherit] absolute inset-0 w-full h-full z-0"
            style={{ display: "block" }}
            aria-hidden="true"
          >
            <defs>
              <clipPath id="goldMask" clipPathUnits="objectBoundingBox">
                <path d="M0,0 H0.45 Q1,0 1,0.7 V1 H0 Z" />
              </clipPath>
            </defs>
            <rect
              width="800"
              height="600"
              fill="#D4AF37"
              clipPath="url(#goldMask)"
            />
          </svg>
          <video
            src="https://sos-ch-gva-2.exo.io/daolink-gold-dao-website-medias/landing_page/texture_fluid_gold.mp4"
            autoPlay
            loop
            muted
            playsInline
            aria-label="Gold texture moving"
            className="md:rounded-[inherit] w-full h-full object-cover z-10"
            style={{
              WebkitClipPath: "url(#goldMask)",
              clipPath: "url(#goldMask)",
              position: "absolute",
              top: 0,
              left: 0,
              width: "100%",
              height: "100%",
            }}
          />
        </div>
        <div className="flex items-center p-8 order-1 md:order-0 py-12 md:py-24">
          <div className="flex flex-col gap-4 md:gap-8 justify-center text-white">
            <div className="text-4xl md:text-6xl text-center md:text-left">
              <h2 className="mb-2">
                Why choose{" "}
                <span className="text-gold font-semibold">GLDT?</span>
              </h2>
            </div>
            <h3 className="text-2xl font-light text-white text-center md:text-left">
              The modern way to invest in Gold.
            </h3>
            <div>
              <ul className="flex flex-col gap-4 text-lg text-left text-white/60">
                <li>
                  → Earn yield on gold with <SpanBold>&gt;3% APY</SpanBold>
                </li>
                <li>
                  → <SpanBold>Redeem</SpanBold> your GLDT for real, physical
                  gold anytime
                </li>
                <li>
                  → Use in <SpanBold>DeFi platforms</SpanBold> and crypto tools
                </li>
                <li>
                  → <SpanBold>Backed 1 to 1</SpanBold> by high-quality physical
                  gold
                </li>
                <li>
                  → Ultra-low transaction <SpanBold>fees from $0.10</SpanBold>
                </li>
              </ul>
            </div>
          </div>
        </div>
      </div>
    </section>
  );
};

export default WhyChooseGLDT;
