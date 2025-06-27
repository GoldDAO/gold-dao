import { useCallback } from "react";
import { toast } from "react-hot-toast";

interface UseCopyToClipboardReturn {
  copyToClipboard: (text: string) => void;
}

export const useCopyToClipboard = (): UseCopyToClipboardReturn => {
  const copyToClipboard = useCallback(async (text: string) => {
    try {
      await navigator.clipboard.writeText(text);
      toast.success("That's copied!");
    } catch (error) {
      console.error("Failed to copy to clipboard:", error);
      toast.error("Failed to copy to clipboard");
    }
  }, []);

  return {
    copyToClipboard,
  };
};
