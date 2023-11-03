import React, { Suspense } from 'react';
import { Canvas } from 'react-three-fiber';
import Model from './Model';

const Scene = () => {
    return (
        <Canvas>
            <Suspense>
                <Model />
            </Suspense>
        </Canvas>
    );
};

export default Scene;
