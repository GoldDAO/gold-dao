import { Logo } from "@components/logos";

const RenderValueGLDT = ({ value }: { value: number }) => {
  return (
    <div className="flex items-center gap-1">
      <Logo className="flex-none w-8 h-8" name="gldt" />
      <div>
        <div className="font-semibold">{value}</div>
        <div className="text-content/60 text-xs">GLDT</div>
      </div>
    </div>
  );
};

export default RenderValueGLDT;
