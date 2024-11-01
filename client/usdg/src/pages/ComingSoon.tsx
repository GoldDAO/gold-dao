import { Link } from "react-router-dom";
import usdgLogo from "@assets/logo_usdg.svg";

export const ComingSoon = () => {
  return (
    <div className="px-8 flex text-center h-screen bg-cover-img bg-cover bg-fixed bg-background">
      <div className="m-auto">
        <div className="flex justify-center mb-10">
          <img
            src={usdgLogo}
            className="h-48 w-48 animate-pulse"
            alt="USDG logo"
          />
        </div>
        <h1 className="text-4xl xl:text-6xl font-semibold mb-3">Coming soon</h1>
        <p className="text-content/60 text-lg">
          In the meantime, go to{" "}
          <Link
            to="https://gldt.org"
            target="_blank"
            rel="noopener noreferrer"
            className="text-gold/80 font-semibold"
          >
            gldt.org
          </Link>{" "}
          and mint some GLDT!
        </p>
      </div>
    </div>
  );
};
