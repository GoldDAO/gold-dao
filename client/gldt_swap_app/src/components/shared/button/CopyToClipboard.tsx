import { useState, useCallback } from "react";
import { toast } from "react-hot-toast";
import { CopyToClipboard as ReactCopyToClipboard } from "react-copy-to-clipboard";
import { ClipboardDocumentIcon } from "@heroicons/react/24/outline";

const CopyToClipboard = ({ value = "" }: { value: string | undefined }) => {
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
      <button onClick={(e) => handleClick(e)} className="p-1">
        <ClipboardDocumentIcon className="h-4 w-4" />
      </button>
    </ReactCopyToClipboard>
  );
};

export default CopyToClipboard;
