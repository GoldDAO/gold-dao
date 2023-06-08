import React, { useEffect } from 'react';
import { createClient } from "@connect2ic/core";
import { Connect2ICProvider } from "@connect2ic/react";
import { defaultProviders } from "@connect2ic/core/providers";
import "@connect2ic/core/style.css";

const C2ICProvider = ({ children }) => {
    const client = createClient({
        canisters: {},
        providers: defaultProviders,
        globalProviderConfig: {
            dev: true,
        },
    });

    useEffect(() => {
        console.log('client', client)
    }, [client])

    return (
        <Connect2ICProvider client={client} >
            {children}
        </Connect2ICProvider >
    );
};

export default C2ICProvider;