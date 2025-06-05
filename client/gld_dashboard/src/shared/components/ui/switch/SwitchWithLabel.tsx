import { Button } from "@components/index";

const SwitchWithLabel = ({
  value,
  labelLeft,
  labelRight,
  handleClickLeft,
  handleClickRight,
}: {
  value: string;
  labelLeft: string;
  labelRight: string;
  handleClickLeft: () => void;
  handleClickRight: () => void;
}) => {
  return (
    <div className="inline-flex justify-center items-center bg-surface-secondary rounded-full">
      <Button
        onClick={handleClickLeft}
        className={`rounded-full px-6 py-3 font-medium ${
          value.toLocaleLowerCase() !== labelLeft.toLocaleLowerCase()
            ? `bg-surface-secondary text-content/60`
            : `bg-secondary text-white`
        }`}
      >
        {labelLeft}
      </Button>
      <Button
        onClick={handleClickRight}
        className={`rounded-full px-6 py-3 font-medium ${
          value.toLocaleLowerCase() !== labelRight.toLocaleLowerCase()
            ? `bg-surface-secondary text-content/60`
            : `bg-secondary text-white`
        }`}
      >
        {labelRight}
      </Button>
    </div>
  );
};

export default SwitchWithLabel;
