import Layout from '@/components/UI/layout/Layout';
import { createClient } from '@connect2ic/core';
import { Connect2ICProvider } from '@connect2ic/react';
import { Provider as JotaiProvider, createStore } from 'jotai';
import { RouterProvider, createBrowserRouter } from 'react-router-dom';
import { defaultProviders } from '@connect2ic/core/providers';
import { SafeHydrate } from '@/utils/SafeHydrate';
import dynamic from 'next/dynamic';
import '@connect2ic/core/style.css';

import { gldNftCanisters } from '@/services/agents/';

const myStore = createStore();

const Providers = ({ children }) => {

  const whitelist = Object.values(gldNftCanisters).map((canister) => canister.canisterId);

  let client = createClient({
    canisters: gldNftCanisters,
    providers: defaultProviders,
    globalProviderConfig: {
      host: 'https://icp0.io',
      dev: false,
      whitelist,
    },
  });

  if (!client) {
    return <></>;
  }

  return (
    <Connect2ICProvider client={client}>
      <SafeHydrate>
        <JotaiProvider store={myStore}>{children}</JotaiProvider>
      </SafeHydrate>
    </Connect2ICProvider>
  );
};

export default Providers;
