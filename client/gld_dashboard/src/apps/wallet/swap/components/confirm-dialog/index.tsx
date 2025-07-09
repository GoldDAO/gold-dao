import Dialog from "@shared/ui/dialog/DialogV2";

const ConfirmDialog = ({
  open,
  onClose,
}: {
  open: boolean;
  onClose: () => void;
}) => {
  return (
    <Dialog open={open} onClose={onClose}>
      <div className="flex items-center justify-end mb-4">
        <Dialog.CloseBtn onClick={onClose} />
      </div>
      <div>Swap confirm</div>
    </Dialog>
  );
};

export default ConfirmDialog;
