import { useAuth } from "@auth/index";
import { Dialog, LoaderSpin } from "@components/index";

const ConnectingDialog = () => {
  const { isInitializing } = useAuth();

  return (
    <>
      <Dialog open={isInitializing} disableClose={true}>
        <div className="pt-6 pb-12 px-4 text-center">
          <div className="mb-8 font-semibold text-lg">Connecting...</div>
          <div className="flex items-center justify-center">
            <LoaderSpin />
          </div>
        </div>
      </Dialog>
    </>
  );
};

export default ConnectingDialog;
