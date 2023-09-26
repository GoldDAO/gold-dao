import dynamic from 'next/dynamic';
import React from 'react';

const Header = () => {
    const C2icButton = dynamic(() => import('../c2ic/C2icButton'), {
        ssr: false,
    });
    return (
        <>
            <C2icButton />
        </>
    );
};

export default Header;
