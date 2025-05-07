import { useState, useCallback } from "react";
import { toast } from "react-hot-toast";
import { CopyToClipboard as ReactCopyToClipboard } from "react-copy-to-clipboard";
import { Copy } from "iconsax-react";

const CopyToClipboard = ({ value = "" }: { value?: string }) => {
  const [valueCopied] = useState(value);
  const [, setCopied] = useState(false);

  const onCopy = useCallback(() => {
    setCopied(true);
    toast.success("That's copied!");
  }, []);

  const handleClick = (e: React.MouseEvent<HTMLElement>) => {
    e.preventDefault();
    e.stopPropagation();
  };

  return (
    <ReactCopyToClipboard onCopy={onCopy} text={valueCopied}>
      <button onClick={(e) => handleClick(e)} className="cursor-pointer">
        <Copy size={16} className="text-content/60" />
      </button>
    </ReactCopyToClipboard>
  );
};

export default CopyToClipboard;
