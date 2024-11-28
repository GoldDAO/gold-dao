import NavbarHome from "@components/shared/navbars/Home";
import { FrequentlyAskedQuestions } from "@components/landing-page";

export const Faqs = () => {
  return (
    <>
      <div className="bg-surface-2">
        <NavbarHome />
        <section className="container mx-auto max-w-6xl px-4 py-8 xl:py-16">
          <div className="mb-8">
            <div className="text-4xl font-semibold text-gold">GLDT</div>
            <div className="text-4xl">Frequently Asked Questions</div>
          </div>

          <div className="mt-16">
            <FrequentlyAskedQuestions />
          </div>
        </section>
      </div>
    </>
  );
};
