/* eslint-disable no-nested-ternary */

'use client';

import { Bounce, toast } from 'react-toastify';
import { useEffect, useState } from 'react';

import { AuthClient } from '@dfinity/auth-client';
import { DelegationIdentity } from '@dfinity/identity';
import Image from 'next/image';
import {
  CloseButton, CopyButton, Dot, LogoutButton,
} from '../../../utils/svgs';
import { copyContent, truncatePrincipal } from '../../../utils/functions';
import { shortPrincipal } from '../../../utils/parsers';
import useSession from '../../../hooks/useSession';

const LoginButton = () => {
  const {
    setIdentity, isConnecting, isConnected, principal, setConnecting, logout,
  } = useSession();
  const [authClient, setAuthClient] = useState(null);
  const [isOpen, setIsOpen] = useState(false);
  const [copyState, setCopyState] = useState(false);

  const toggleDropdown = () => {
    setIsOpen(!isOpen);
  };

  const onConnect = async () => {
    const identity = authClient.getIdentity();
    if (identity instanceof DelegationIdentity) {
      const p = identity.getPrincipal().toString();
      setIdentity(identity, p);
      // to track when a user logs
      const timestamp = new Date().getTime();
      const utcTimestamp = Math.floor(timestamp / 1000);
      window.plausible?.('Login', { props: { login_time: utcTimestamp, user: p } });
    } else setConnecting(false);
  };
  const handleLogin = async () => {
    if (!authClient) return;

    try {
      // start the login process and wait for it to finish
      await authClient.login({
        identityProvider: 'https://identity.ic0.app',
        derivationOrigin: process.env.ENV === 'prod' ? 'https://rbsh4-yyaaa-aaaal-qdigq-cai.icp0.io' : null,
        onSuccess: onConnect,
        onError: () => console.log('onError'),
      });

      // At this point we're authenticated, and we can get the identity from the auth client:
    } catch (err) {
      console.log('handleLogin error:', err);
    }
  };

  const handleLogout = async () => {
    if (!authClient) return;
    await authClient.logout();
    logout();
  };

  // set authClient
  useEffect(() => {
    (async () => setAuthClient(await AuthClient.create()))();
  }, []);

  // auto login
  useEffect(() => {
    if (!authClient) return;
    onConnect();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [authClient]);

  useEffect(() => {
    if (copyState) {
      toast.success('Copied', {
        position: 'top-right',
        autoClose: 2000,
        hideProgressBar: false,
        closeOnClick: true,
        pauseOnHover: true,
        draggable: true,
        progress: undefined,
        theme: 'light',
        transition: Bounce,
      });
      setCopyState(false);
    }
  }, [copyState]);

  return (
    <div
      className={'w-full sm:w-fit dropdown dropdown-end rounded-full bg-black text-white font-bold text-2xs sm:text-xs flex gap-1 sm:gap-2 items-center px-2 sm:px-4 py-2 cursor-pointer'}
      role="button"
      tabIndex={0}
      onClick={isConnecting ? null : isConnected ? toggleDropdown : handleLogin}
    >
      {isConnecting ? (
        'Connecting...'
      ) : isConnected ? (
        <>
          {shortPrincipal(principal)}
          <Dot />
          <Image width={20} height={20} src="svg/walletIcon.svg" alt="wallet icon" />
          {isOpen && (
            <>
              <div
                className="fixed inset-0 bg-black opacity-50 z-10"
                onClick={toggleDropdown}
              ></div>
              <div
                tabIndex={0}
                className="absolute dropdown-content mt-72 z-20 menu p-8 shadow rounded-4xl w-80 sm:w-120 bg-SoftGrey text-black border-[0.5px] border-DarkGrey h-36 sm:h-44"
              >
                <div className="flex flex-col">
                  <div className="flex justify-between items-center -mt-1">
                    <h3 className="font-bold text-3xl sm:text-4xl">Wallet</h3>
                    <button
                      className="btn btn-circle hover:bg-transparent hover:border-none hover:shadow-none bg-transparent border-none shadow-none -mt-4 -mr-2"
                      onClick={toggleDropdown}
                    >
                      {CloseButton}
                    </button>
                  </div>
                  <div className="divider my-1"></div>
                  <div className="flex justify-between items-center">
                    <p className="text-3xl sm:text-4xl">{truncatePrincipal(principal)}</p>
                    <div className="flex gap-x-2 sm:gap-x-4">
                      <button
                        className="flex justify-center items-center size-7 sm:size-9"
                        onClick={() => copyContent(principal, setCopyState)}
                      >
                        {CopyButton}
                      </button>
                      <button
                        className="flex justify-center items-center size-7 sm:size-9"
                        onClick={handleLogout}
                      >
                        {LogoutButton}
                      </button>
                    </div>
                  </div>
                </div>
              </div>
            </>
          )}
        </>
      ) : (
        <>
          <p>Connect Wallet</p>
          <Image width={16} height={16} src="svg/walletIcon.svg" alt="wallet icon" />
        </>
      )}
    </div>
  );
};

export default LoginButton;
