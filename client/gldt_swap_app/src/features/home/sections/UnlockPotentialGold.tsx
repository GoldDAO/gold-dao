import { useRef, useEffect, useState } from "react";

// const SpanBold = ({ children }: { children: React.ReactNode }) => (
//   <span className="font-bold">{children}</span>
// );

const UnlockPotentialGold = ({
  className,
  ...restProps
}: { className?: string } & React.HTMLAttributes<HTMLElement>) => {
  const imgRef = useRef<HTMLImageElement>(null);
  const [inView, setInView] = useState(false);

  useEffect(() => {
    const observer = new window.IntersectionObserver(
      ([entry]) => setInView(entry.isIntersecting),
      { threshold: 0.5 }
    );
    if (imgRef.current) observer.observe(imgRef.current);
    return () => observer.disconnect();
  }, []);

  return (
    <section className={className} {...restProps}>
      <div className="max-w-[800px] mx-auto px-4">
        <div className="flex flex-col gap-4 md:gap-8 items-center justify-center">
          <div className="text-center text-4xl md:text-6xl">
            <h2 className="mb-2">Unlock the full</h2>
            <h2 className="text-gold font-semibold">potential of Gold</h2>
          </div>

          <div className="flex flex-col gap-4 mt-8">
            <div className="flex flex-col md:flex-row justify-center items-center md:items-end gap-4 rounded-xl bg-surface">
              <div className="flex flex-col text-center order-1 md:order-0 md:text-left gap-2 p-4 md:p-8">
                <div className="text-2xl md:text-4xl">
                  <h3>Backed by real,</h3>
                  <h3 className="text-gold font-semibold">audited Gold bars</h3>
                </div>
                <div className="max-w-96 text-content/80">
                  Each token is backed by a physical gold bar, produced and
                  securely stored in Switzerland. Our vault is audited every 4
                  months with full transparency. Zero storage fees for holders.
                </div>
              </div>
              <div className="mx-auto order-0 md:order-1 max-w-48 md:max-w-full overflow-hidden">
                <img
                  ref={imgRef}
                  src="/gold-bar-parles.svg"
                  alt="Gold bar with parles"
                  className={`transition-transform duration-700 ease-in-out ${
                    inView ? "scale-110" : "scale-100"
                  }`}
                />
              </div>
            </div>

            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div className="rounded-xl bg-burgundy text-white p-4 md:p-8">
                <div>
                  <div className="flex justify-end">
                    <img src="/flower.svg" alt="Flower" className="w-32" />
                  </div>
                  <div className="text-xl md:text-3xl mt-16 mb-4">
                    <h3>Leverage your Gold</h3>
                    <h3 className="text-gold font-semibold">in DeFi</h3>
                  </div>
                  <div className="">
                    Enter the world of DeFi and user the token in independent
                    borrow/lending protocols and more.
                  </div>
                </div>
              </div>
              <div className="rounded-xl bg-surface-invert text-content-invert p-4 md:p-8">
                <div>
                  <div className="flex justify-end">
                    <img
                      src={
                        window.matchMedia &&
                        window.matchMedia("(prefers-color-scheme: dark)")
                          .matches
                          ? "/globe_dark.svg"
                          : "/globe_light.svg"
                      }
                      alt="Globe"
                      className="w-32"
                    />
                  </div>
                  <div className="text-xl md:text-3xl mt-16 mb-4">
                    <h3>Use</h3>
                    <h3 className="text-gold font-semibold">anywhere</h3>
                  </div>
                  <div className="">
                    GLDT is blockchain agnostic and borderless. Use it on ICP,
                    Base, Ethereum, Arbitrum and many more.
                  </div>
                </div>
              </div>
            </div>
            {/* <div className="flex justify-center mt-8">
              <Link
                to="https://app.gldt.org/earn"
                target="_blank"
                rel="noopener noreferrer"
              >
                <Button className="rounded-full flex items-center justify-center gap-2 px-4 md:px-6 py-3 md:py-4 md:text-lg">
                  Stake your Gold
                </Button>
              </Link>
            </div> */}
          </div>
        </div>
      </div>
    </section>
  );
};

export default UnlockPotentialGold;
