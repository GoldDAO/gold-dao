import { Fragment } from "react";
import { useNavigate } from "react-router-dom";
import { Transition, TransitionChild, Dialog } from "@headlessui/react";
import { UserIcon } from "@heroicons/react/20/solid";
import { useWallet } from "@amerej/artemis-react";

import { Button, Tile, Tooltip, Skeleton } from "@components/ui";
import CopyToClipboard from "@components/shared/button/CopyToClipboard";
// import AuthButton from "@components/auth/Auth";

import { useUserBalanceOGY } from "@hooks/ogy_ledger";
import { useBalanceOGYUSD } from "@hooks/ogy_api";
import { useUserBalanceGLDT } from "@hooks/gldt_ledger";
import LogoutButton from "@components/shared/button/LogoutButton";

const AccountOverview = ({
  show,
  handleClose,
}: {
  show: boolean;
  handleClose: () => void;
}) => {
  const navigate = useNavigate();
  const { principalId } = useWallet();

  const { data: balanceOGY } = useUserBalanceOGY();
  const { data: balanceOGYUSD } = useBalanceOGYUSD({
    balance: balanceOGY?.number,
  });
  const { data: balanceGLDT, isSuccess: isSuccessBalanceGLDT } =
    useUserBalanceGLDT();

  const handleClickAccount = () => {
    navigate("swap/account");
    handleClose();
  };

  return (
    <Transition show={show} as={Fragment}>
      <div className="fixed z-50 inset-0 overflow-hidden">
        <Dialog as={Fragment} static open={show} onClose={handleClose}>
          <div className="absolute z-50 inset-0 overflow-hidden">
            <TransitionChild
              as={Fragment}
              enter="ease-in-out duration-500"
              enterFrom="opacity-0"
              enterTo="opacity-100"
              leave="ease-in-out duration-500"
              leaveFrom="opacity-100"
              leaveTo="opacity-0"
            >
              <div
                className="fixed inset-0 bg-black bg-opacity-50 transition-opacity"
                aria-hidden="true"
                onClick={() => handleClose()}
              />
            </TransitionChild>
            <div className="fixed inset-y-0 right-0 max-w-full flex">
              <TransitionChild
                as={Fragment}
                enter="transform transition ease-in-out duration-500 sm:duration-700"
                enterFrom="translate-x-full"
                enterTo="translate-x-0"
                leave="transform transition ease-in-out duration-500 sm:duration-700"
                leaveFrom="translate-x-0"
                leaveTo="translate-x-full"
              >
                <div className="bg-background px-4 sm:px-8 py-5 sm:max-w-[480px] max-w-80">
                  <div className="flex justify-end">
                    <LogoutButton/>
                    {/* auth button */}
                    {/* <AuthButton /> */}
                  </div>
                  <div className="mt-12 flex items-center bg-surface rounded-full py-1 px-1">
                    <div className="flex items-center w-full">
                      <Tile className="rounded-full h-8 w-8 bg-surface-2">
                        <UserIcon className="p-1 text-white" />
                      </Tile>

                      <div className="flex items-center truncate pr-4">
                        <div className="flex ml-4 items-center truncate text-sm">
                          <div className="mr-2 shrink-0">Principal ID: </div>
                          {principalId ? (
                            <>
                              <div
                                className="truncate"
                                data-tooltip-id="tooltip_principal_id"
                                data-tooltip-content={principalId}
                              >
                                {principalId}
                              </div>
                              <Tooltip id="tooltip_principal_id" />
                              <CopyToClipboard value={principalId as string} />
                            </>
                          ) : (
                            <Skeleton className="w-64" />
                          )}
                        </div>
                      </div>
                    </div>
                  </div>

                  <div className="bg-surface text-center mt-16 border border-border rounded-xl">
                    <div className="border-b border-border py-4 bg-surface rounded-t-xl">
                      Wallet Balance
                    </div>
                    <div className="grid grid-cols-1 gap-4 pb-8 px-4">
                      <div className="py-8">
                        <>
                          <div className="flex justify-center ml-2">
                            {isSuccessBalanceGLDT && balanceGLDT ? (
                              <div className="flex items-center justify-center sm:justify-start gap-3">
                                <img
                                  className="flex-none h-8"
                                  src={`/gldt_logo.svg`}
                                />
                                <div className="font-semibold text-2xl">
                                  {balanceGLDT.string}
                                </div>
                                <div className="font-semibold text-2xl">
                                  GLDT
                                </div>
                              </div>
                            ) : (
                              <Skeleton className="w-32" />
                            )}
                          </div>
                          <div className="flex justify-center text-sm">
                            <div className="text-content/60">todo $</div>
                          </div>
                        </>

                        <div className="border-t border-border my-8"></div>

                        <>
                          <div className="flex justify-center ml-2">
                            {balanceOGY !== null ? (
                              <div className="flex items-center justify-center sm:justify-start gap-3">
                                <img
                                  className="flex-none h-8"
                                  src={`/ogy_logo.svg`}
                                />
                                <div className="font-semibold text-2xl">
                                  {balanceOGY?.string}
                                </div>
                                <div className="font-semibold text-2xl">
                                  OGY
                                </div>
                              </div>
                            ) : (
                              <Skeleton className="w-32" />
                            )}
                          </div>
                          <div className="flex justify-center text-sm">
                            {balanceOGYUSD !== null ? (
                              <div className="text-content/60">
                                {balanceOGYUSD} $
                              </div>
                            ) : (
                              <Skeleton className="w-24" />
                            )}
                          </div>
                        </>
                      </div>
                      <Button className="px-8" onClick={handleClickAccount}>
                        My account
                      </Button>
                    </div>
                  </div>
                </div>
              </TransitionChild>
            </div>
          </div>
        </Dialog>
      </div>
    </Transition>
  );
};

export default AccountOverview;
