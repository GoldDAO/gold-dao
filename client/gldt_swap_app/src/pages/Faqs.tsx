import NavbarHome from "@components/shared/navbars/Home";
import { FrequentlyAskedQuestions } from "@components/landing-page";

export const Faqs = () => {
  return (
    <>
      <div className="bg-surface-2">
        <NavbarHome />
        <section className="container mx-auto px-4 py-8 xl:py-16">
          <div className="text-4xl font-bold mb-1 xl:mb-2 text-center">
            Frequently Asked Questions
          </div>
          <div className="mt-16 px-8">
            <FrequentlyAskedQuestions className="col-span-2" />
          </div>
        </section>
      </div>
    </>
  );
};
