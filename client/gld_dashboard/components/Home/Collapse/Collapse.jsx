/* eslint-disable no-unused-expressions */
import { useEffect, useState } from 'react';

import Link from 'next/link';

const Collapse = ({
  title, children, setInfoModal, item, name, redirect,
}) => {
  const [isOpen, setIsOpen] = useState(false);
  const [isMobile, setIsMobile] = useState(false);

  useEffect(() => {
    const mediaQuery = window.matchMedia('(max-width: 768px)'); // Change the value of 768px according to what you consider as mobile

    const handleResize = (e) => {
      setIsMobile(e.matches);
      setIsOpen(false);
    };

    handleResize(mediaQuery);
    mediaQuery.addListener(handleResize);
    return () => {
      mediaQuery.removeListener(handleResize);
    };
  }, []);

  const toggleCollapse = () => {
    if (!isMobile) {
      setIsOpen(!isOpen);
    }
  };

  return (
    <div
      className="shadow-[0_0_12px_0_#00000026] w-full border-[0.1px] border-[#C6C6C6] card bg-[#F3F3F3] sm:rounded-[36px] rounded-[36px]"
      onClick={() => {
        isMobile ? document.getElementById(name).showModal() : '';
        isMobile ? setInfoModal(item) : '';
      }}
      key={name}
    >
      <div
        className="flex items-center justify-between px-7 py-5 cursor-pointer"
        onClick={toggleCollapse}
      >
        <h2 className="text-lg font-bold">{title}</h2>
        <div className="flex justify-end items-center gap-5">
          {isOpen && (
            <Link
              onClick={(e) => {
                e.stopPropagation();
              }}
              className="bg-[#D3B871] rounded-[60px] font-bold text-white text-xs py-3 px-6 border-none z-10 hover:bg-[#D3B871] "
              href={`/${redirect}`}
              rel="noopener noreferrer"
            >
              View All
            </Link>
          )}
          {!isMobile && (
            <svg
              className={`w-6 h-6 ${isOpen ? 'transform rotate-180' : ''}`}
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth="2"
                d="M19 9l-7 7-7-7"
              />
            </svg>
          )}
        </div>{' '}
      </div>
      {isOpen && (
        <div className="relative overflow-x-auto w-[100%] mx-0">
          <table className="w-[100%]">{children}</table>
        </div>
      )}
    </div>
  );
};

export default Collapse;
