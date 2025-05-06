import { usePagination } from "@utils/table/useTable";
import { useAuth } from "@auth/index";
import { Button } from "@components/index";

import UserNeuronsTable from "./-components/user-neurons-table/UserNeuronsTable";
import BalanceOverview from "./-components/BalanceOverview";

const Account = () => {
  const { isConnected, connect } = useAuth();

  const [pagination, setPagination] = usePagination({
    pageIndex: 0,
    pageSize: 10,
  });
  return (
    <>
      <div className="mt-4 sm:mt-8 mb-8">
        <div className="text-4xl font-bold text-accent">Gold DAO</div>
        <div className="text-4xl">Account</div>
      </div>

      {isConnected && (
        <>
          <section className="mb-6">
            <BalanceOverview />
          </section>
          <section className="rounded-xl bg-surface-primary/40 mb-6">
            <div className="flex items-center justify-between px-6 pt-6 mb-4 xl:mb-6">
              <h6 className="text-lg font-semibold">My GLDGov Neurons</h6>
              {/* <div className="font-semibold text-accent cursor-pointer text-sm">
        Claim All
      </div> */}
            </div>
            <UserNeuronsTable
              pagination={pagination}
              setPagination={setPagination}
            />
          </section>
        </>
      )}
      {!isConnected && (
        <div className="flex items-center justify-center mt-48">
          <div className="flex flex-col items-center justify-center">
            <div className="mb-6 text-center">
              Connect to your wallet to view your account
            </div>
            <Button className="px-6" onClick={connect}>
              Connect Wallet
            </Button>
          </div>
        </div>
      )}
    </>
  );
};

export default Account;
