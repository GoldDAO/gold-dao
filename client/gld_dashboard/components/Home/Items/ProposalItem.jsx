/* eslint-disable no-nested-ternary */
import Image from 'next/image';

const ProposalItem = ({
  id, title, topic, status, votes, url,
}) => {
  const votesYes = votes ? ((votes.yes * 100) / votes.total).toFixed(2) : 0;
  const votesNo = votes ? ((votes.no * 100) / votes.total).toFixed(2) : 0;

  return (
    <div>
      <div className=" text-center py-6  sm:w-[100%] w-full  grid grid-cols-5 px-0 sm:px-10 sm:gap-5 gap-2 items-center ">
        <p className=" basis-1/5 sm:min-w-40 min-w-fit  text-start pl-5 text-xs sm:text-[16px]  sm:pl-20">
          {id}
        </p>
        <p className="basis-1/5 sm:min-w-40 text-start text-xs sm:text-[16px] ">{title}</p>

        <p className="basis-1/5 sm:min-w-40 text-start text-xs sm:text-[16px] ">{topic || 'SNS'}</p>

        <div className="basis-1/5 sm:min-w-40 min-w-fit flex sm:justify-start justify-center ">
          <span
            className={`sm:py-2  sm:w-24 w-fit px-4 text-[8px] sm:text-[12px] font-bold text-white rounded h-5 sm:h-10 flex justify-center items-center sm:h-15 ${status === 'Open' ? 'bg-[#127E00]' : status === 'Failed' ? 'bg-red-700' : 'bg-[#C6C6C6]'} `}
          >
            {status}
          </span>
        </div>
        <div className="basis-1/5 flex sm:justify-start justify-center items-center sm:min-w-40 min-w-fit sm:h-10">
          {status === 'Open' ? (
            <a
              href={url}
              target="_blank"
              rel="noreferrer noopener"
              className="sm:py-3 py-1.5 sm:px-8 px-2 bg-black text-white rounded-full sm:text-[12px] text-[8px] font-bold flex gap-2 items-center"
            >
              Vote Now
              <Image
                width={10}
                height={10}
                src="/svg/open-window.svg"
                alt="open window"
                className="hidden sm:flex"
              />
            </a>
          ) : (
            <>
              <p className="text-[16px] hidden sm:flex">
                Yes: {votesYes}% | No: {votesNo}%
              </p>
              <p className="text-[10px] text-start flex sm:hidden flex-col">
                <span> Yes: {votesYes}%</span> <span>No: {votesNo}%</span>
              </p>
            </>
          )}
        </div>
      </div>
    </div>
  );
};
export default ProposalItem;
