const SpanBold = ({ children }: { children: React.ReactNode }) => (
  <span className="font-bold">{children}</span>
);

const Transparency = ({
  className,
  ...restProps
}: React.HTMLAttributes<HTMLElement>) => {
  return (
    <section className={className} {...restProps}>
      <div className="container mx-auto px-4">
        <div className="flex flex-col gap-4 md:gap-8 justify-center">
          <div className="text-center text-4xl md:text-6xl md:text-left">
            <h2 className="mb-2">Real Gold. Real Swiss security</h2>
            <h2 className="text-gold font-semibold">Total transparency</h2>
          </div>

          <div className="flex flex-col md:flex-row md:items-end gap-8 md:gap-32 mt-8 md:mt-16">
            <div className="mx-auto md:mx-0 max-w-full">
              <img src="/metalor.png" alt="Gold bar with parles" />
            </div>

            <div className="my-auto">
              <ul className="flex flex-col text-content/80 justify-start gap-8 text-left text-lg">
                <li className="flex items-start gap-4">
                  <div className="flex-shrink-0 flex items-start h-6 md:h-8 pt-1">
                    <img
                      src="/check.svg"
                      alt="check icon"
                      className="mx-auto w-6 h-6 align-center"
                    />
                  </div>
                  <div>
                    Sourced from <SpanBold>Metalor</SpanBold>, one of the
                    world’s leading refineries
                  </div>
                </li>
                <li className="flex items-start gap-4">
                  <div className="flex-shrink-0 flex items-start h-6 md:h-8 pt-1">
                    <img
                      src="/check.svg"
                      alt="check icon"
                      className="mx-auto w-6 h-6 align-center"
                    />
                  </div>
                  <div>
                    <SpanBold>Fully insured and audited</SpanBold> storage in
                    Swiss vaults
                  </div>
                </li>
                <li className="flex items-start gap-4">
                  <div className="flex-shrink-0 flex items-start h-6 md:h-8 pt-1">
                    <img
                      src="/check.svg"
                      alt="check icon"
                      className="mx-auto w-6 h-6 align-center"
                    />
                  </div>
                  <div>
                    <SpanBold>Proof of backing</SpanBold> via GLD NFT
                    (one-to-one link with the gold bar)
                  </div>
                </li>
                <li className="flex items-start gap-4">
                  <div className="flex-shrink-0 flex items-start h-6 md:h-8 pt-1">
                    <img
                      src="/check.svg"
                      alt="check icon"
                      className="mx-auto w-6 h-6 align-center"
                    />
                  </div>
                  <div>
                    <SpanBold>Audit Reports</SpanBold> available to all users
                  </div>
                </li>
              </ul>
            </div>
          </div>
        </div>
      </div>
    </section>
  );
};

export default Transparency;
