import Image from "next/image";

const ProposalsHeader = () => {
  // Definir un array de objetos que contenga la información de cada columna
  const headerItems = [
    { label: "ID", iconSrc: "/svg/id.svg", padding: true },
    { label: "Title", iconSrc: "/svg/title.svg" },
    { label: "Topic", iconSrc: "/svg/topic.svg" },
    { label: "Status", iconSrc: "/svg/status.svg" },
    { label: "Votes", iconSrc: "/svg/votes.svg" },
  ];

  return (
    <thead className="border-t-[0.5px] border-y-[#C6C6C6] flex flex-row items-center justify-around py-[20px] w-[100%] px-10">
      <tr className="grid grid-cols-5  w-[100%] gap-5">
        {headerItems.map((item) => (
          <th
            key={item.label}
            className={`flex flex-row justify-start  items-center  w-[100%] min-w-40 gap-4 ${item.padding ? "pl-5 sm:pl-20" : ""} `}
          >
            <Image alt={`${item.label} icon`} height={30} width={30} src={item.iconSrc} />
            <p className="text-xs font-bold text-[#D3B871]">{item.label}</p>
          </th>
        ))}
      </tr>
    </thead>
  );
};

export default ProposalsHeader;
