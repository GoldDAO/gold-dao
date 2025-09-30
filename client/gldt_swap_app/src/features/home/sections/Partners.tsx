import Slider from "react-slick";

const Partners = ({
  className,
  ...restProps
}: React.HTMLAttributes<HTMLElement>) => {
  const settings = {
    dots: false,
    infinite: true,
    slidesToShow: 5,
    autoplay: true,
    speed: 2000,
    autoplaySpeed: 0,
    cssEase: "linear",
    arrows: false,
    draggable: false,
    swipe: false,
    touchMove: false,
    responsive: [
      {
        breakpoint: 1024,
        settings: {
          slidesToShow: 2,
        },
      },
    ],
  };

  const PARTNERS_LOGOS = [
    { name: "METALOR", alt: "Metalor brand logo" },
    { name: "ORIGYN", alt: "Origyn brand logo" },
    { name: "KPMG", alt: "KPMG brand logo" },
    { name: "LOOMIS", alt: "Loomis brand logo" },
    { name: "ICP", alt: "ICP brand logo" },
    { name: "BITY", alt: "BITY brand logo" },
  ];
  return (
    <div className={className} {...restProps}>
      <div className="container mx-auto grid grid-cols-1 md:grid-cols-12">
        <div className="md:col-span-2 flex flex-col items-center md:items-start gap-4 text-content-invert">
          <div className="text-center md:text-left">
            <div className="">OUR TRUSTED PARTNERS</div>
            <div className="">Built with world-class partners</div>
          </div>
          <div className="">
            <button className="rounded-full bg-surface py-3 px-6 font-semibold text-content">
              <a href="mailto:info@gold-dao.org">Become a partner</a>
            </button>
          </div>
        </div>
        <div className="slider-container md:col-span-10 relative md:my-auto mt-16">
          <div className="absolute left-0 top-0 h-full w-16 pointer-events-none z-10 bg-gradient-to-r from-surface-invert to-transparent" />
          <div className="absolute right-0 top-0 h-full w-16 pointer-events-none z-10 bg-gradient-to-l from-surface-invert to-transparent" />
          <Slider {...settings}>
            {PARTNERS_LOGOS.map(({ name, alt }) => (
              <div
                className="flex justify-center items-center mx-8 md:mx-12"
                key={name}
              >
                <img
                  className=""
                  src={`/landing-page-assets/powered-by-logos/${name}.svg`}
                  alt={alt}
                />
              </div>
            ))}
          </Slider>
        </div>
      </div>
    </div>
  );
};

export default Partners;
