import Dialog from "@shared/ui/dialog/DialogV2";

const DetailsDialog = ({
  isOpen,
  onClose,
}: {
  isOpen: boolean;
  onClose: () => void;
}) => {
  return (
    <Dialog open={isOpen} onClose={onClose}>
      <div className="flex items-center justify-end mb-4">
        <Dialog.CloseBtn onClick={onClose} />
      </div>
      <div>Swap token details</div>
    </Dialog>
  );
};

export default DetailsDialog;
