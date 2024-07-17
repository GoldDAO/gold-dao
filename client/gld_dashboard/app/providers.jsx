'use client';

import { NextUIProvider } from '@nextui-org/react';

export default function Providers({ children }) {
  return (
    <NextUIProvider locale={'es-ES'}>
      {children}
    </NextUIProvider>
  );
}

// here