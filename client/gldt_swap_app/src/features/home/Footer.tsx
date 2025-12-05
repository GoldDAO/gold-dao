// import { useForm } from "react-hook-form";

const socialLinks = [
  { name: "X (TWITTER)", href: "https://x.com/gldtorg" },
  { name: "TELEGRAM", href: "https://t.me/gldrwa" },
  { name: "LINKEDIN", href: "https://www.linkedin.com/showcase/gold-dao" },
  { name: "MEDIUM", href: "https://medium.com/@GoldDAO" },
];

const Footer = () => {
  // const { register, handleSubmit, reset } = useForm<{ email: string }>();

  // const onSubmit = (data: { email: string }) => {
  //   // Traitement du formulaire ici (ex: appel API)
  //   console.log("Submitted email:", data.email);
  //   reset();
  // };

  return (
    <footer className="bg-surface-invert text-content-invert">
      <div className="container mx-auto px-4 py-16 md:py-24">
        <div className="flex flex-col md:flex-row justify-center">
          {/* <div className="max-w-[496px]">
            <h2 className="text-3xl md:text-4xl text-center md:text-left font-normal leading-tight mb-2">
              Sign up for the Where
              <br />
              Money Moves{" "}
              <span className="font-semibold text-gold">Newsletter</span>
            </h2>
            <form className="mt-8 mb-4" onSubmit={handleSubmit(onSubmit)}>
              <div className="flex flex-col gap-2">
                <label
                  className="text-sm font-semibold text-primary mb-1 text-center md:text-left"
                  htmlFor="footer-email"
                >
                  Email
                </label>
                <div className="flex justify-between bg-surface dark:bg-[#faf9f8] border border-[#cecece] rounded-lg items-center pl-4 pr-2 py-2">
                  <input
                    id="footer-email"
                    type="email"
                    placeholder="email@email.com"
                    className="bg-transparent text-content dark:text-content-invert overflow-hidden outline-none text-base"
                    {...register("email", {
                      required: "Email is required",
                      pattern: {
                        value: /^[^\s@]+@[^\s@]+\.[^\s@]+$/,
                        message: "Invalid email address",
                      },
                    })}
                  />
                  <button
                    type="submit"
                    className="bg-burgundy hover:bg-burgundy/90 text-content-invert rounded-lg px-6 py-3 flex items-center transition"
                  >
                    <span className="text-content-invert dark:text-content mx-auto text-lg">
                      →
                    </span>
                  </button>
                </div>
              </div>
            </form>
            <p className="text-sm mt-6 text-center md:text-left text-content-invert/80 max-w-lg">
              Get regular updates and in-depth research on all things
              stablecoins. Never spammed, never shared—just the info you need to
              stay ahead
            </p>
          </div> */}

          <div className="mt-12 md:mt-0 text-center md:text-left flex flex-col md:flex-row gap-8 md:gap-16 items-center md:items-start justify-center md:justify-end">
            <div>
              <h4 className="font-semibold mb-4">MORE</h4>
              <ul className="space-y-3 text-sm">
                <li>
                  <a
                    href="https://docs.gold-dao.org/legal/terms-of-service"
                    target="_blank"
                    rel="noopener noreferrer"
                    className="hover:underline"
                  >
                    Terms
                  </a>
                </li>
                <li>
                  <a
                    href="https://docs.gold-dao.org/legal/privacy-policy"
                    className="hover:underline"
                  >
                    Privacy
                  </a>
                </li>
                <li>
                  <a
                    href="mailto:info@gold-dao.org"
                    className="hover:underline"
                  >
                    Contact
                  </a>
                </li>
              </ul>
            </div>
            <div>
              <h4 className="font-semibold mb-4">THE GOLD DAO</h4>
              <ul className="space-y-3 text-sm">
                <li className="flex items-center gap-2">
                  <span>&copy; 2025 GOLD DAO</span>
                </li>
              </ul>
            </div>
          </div>
        </div>
      </div>
      {/* Bottom bar */}
      <div className="bg-surface-invert border-t border-[#444]">
        <div className="container mx-auto px-4 py-6">
          <div className="flex flex-col md:flex-row justify-between items-center w-full gap-4">
            <div className="font-bold text-base mb-2 md:mb-0">FOLLOW US</div>
            {socialLinks.map((link) => (
              <a
                key={link.name}
                href={link.href}
                className="text-content-invert font-medium hover:underline underline-offset-2 text-center hover:text-primary transition"
                target="_blank"
                rel="noopener noreferrer"
              >
                {link.name}
              </a>
            ))}
          </div>
        </div>
      </div>
    </footer>
  );
};

export default Footer;
