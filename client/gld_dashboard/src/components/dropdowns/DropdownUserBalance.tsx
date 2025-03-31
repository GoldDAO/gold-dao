import { Menu, MenuButton, MenuItem, MenuItems } from "@headlessui/react";
import { ChevronDownIcon } from "@heroicons/react/16/solid";

import { useAuth } from "@auth/index";
import {
  ICP_LEDGER_CANISTER_ID,
  GOLDAO_LEDGER_CANISTER_ID,
  GLDT_LEDGER_CANISTER_ID,
  OGY_LEDGER_CANISTER_ID,
} from "@constants";
import { Ledger } from "@services/ledger/utils/interfaces";

import useFetchUserBalance from "@services/ledger/hooks/useFetchUserBalance";

import { Skeleton, Logo } from "@components/index";

import E8sToLocaleString from "@components/numbers/E8sToLocaleString";

const UserBalanceMenuItem = ({
  token,
}: {
  token: { name: Ledger; label: string; canisterId: string };
}) => {
  const { unauthenticatedAgent, principalId, isConnected } = useAuth();
  const { canisterId, name, label } = token;
  const { data, isSuccess } = useFetchUserBalance(
    canisterId,
    unauthenticatedAgent,
    {
      ledger: name,
      owner: principalId,
      enabled: !!unauthenticatedAgent && !!isConnected,
    }
  );
  return (
    isSuccess && (
      <MenuItem>
        <div className="flex items-center gap-2 pl-2 py-2 font-semibold text-sm">
          <Logo name={name} className="flex-none w-5 h-5" />
          <E8sToLocaleString value={data} />
          <div>{label}</div>
        </div>
      </MenuItem>
    )
  );
};

export const DropdownUserBalance = () => {
  const { unauthenticatedAgent, principalId, isConnected } = useAuth();

  const tokensList: { name: Ledger; label: string; canisterId: string }[] = [
    { name: "gldt", label: "GLDT", canisterId: GLDT_LEDGER_CANISTER_ID },
    { name: "icp", label: "ICP", canisterId: ICP_LEDGER_CANISTER_ID },
    { name: "ogy", label: "OGY", canisterId: OGY_LEDGER_CANISTER_ID },
  ];

  const { data, isSuccess } = useFetchUserBalance(
    GOLDAO_LEDGER_CANISTER_ID,
    unauthenticatedAgent,
    {
      ledger: "gldgov",
      owner: principalId,
      enabled: !!unauthenticatedAgent && !!isConnected,
    }
  );

  return (
    <div className="text-center">
      {isSuccess ? (
        <Menu>
          {({ open }) => (
            <>
              <div
                className={`fixed inset-0 bg-black transition-opacity ${
                  open ? "opacity-20" : "opacity-0 pointer-events-none"
                }`}
              />

              <>
                <MenuButton className="inline-flex items-center gap-2 rounded-lg bg-surface-secondary py-1.5 px-3 text-sm/6 font-semibold">
                  <div className="max-w-48 flex items-center gap-2">
                    <Logo name="gldgov" className="flex-none w-6 h-6" />
                    <div className="truncate hidden sm:block">
                      <div className="truncate font-semibold text-sm">
                        <E8sToLocaleString value={data} /> GOLDAO
                      </div>
                    </div>
                  </div>
                  <ChevronDownIcon className="size-4 fill-content/60" />
                </MenuButton>

                <MenuItems
                  transition
                  anchor="bottom end"
                  className="max-w-fit min-w-32 z-50 border border-border origin-top-right rounded-xl bg-surface-primary py-1 pl-1 pr-3 mt-1 text-sm/6 transition duration-100 ease-out [--anchor-gap:var(--spacing-1)] focus:outline-none data-[closed]:scale-95 data-[closed]:opacity-0"
                >
                  {tokensList.map((token) => (
                    <div key={token.name}>
                      <UserBalanceMenuItem token={token} />
                    </div>
                  ))}
                </MenuItems>
              </>
            </>
          )}
        </Menu>
      ) : (
        <Skeleton className="w-32" />
      )}
    </div>
  );
};
