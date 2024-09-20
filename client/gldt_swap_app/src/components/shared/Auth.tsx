import { useWallet } from "@amerej/artemis-react";
import { Dialog, LoaderSpin } from "@components/ui";

const Auth = () => {
  const {
    status,
    handleSelectWallet,
    walletStatus,
    handleCloseWalletList,
    walletList,
  } = useWallet();

  return (
    <>
      <Dialog
        show={status == walletStatus.OpenWalletList}
        handleClose={handleCloseWalletList}
      >
        <div className="pt-6 pb-12 px-12">
          <div className="mb-8 text-center text-lg font-semibold">
            Connect Wallet
          </div>
          <div>
            {walletList.map(
              ({ id, icon, name }, i: number) =>
                !["stoic", "metamask"].includes(id) && (
                  <div
                    onClick={() => handleSelectWallet(id)}
                    key={i}
                    className="mb-3 cursor-pointer border-border border rounded-full"
                  >
                    <div className="flex items-center">
                      <div className="w-[48px] h-[48px] flex items-center bg-surface-2/40 dark:bg-surface-2 rounded-full p-2">
                        <img src={icon} alt="" className="rounded-full" />
                      </div>
                      <div className="ml-8">{name}</div>
                    </div>
                  </div>
                )
            )}
          </div>
        </div>
      </Dialog>
      <Dialog
        show={status == walletStatus.Connecting}
        handleClose={handleCloseWalletList}
      >
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

export default Auth;
