import React, { Suspense, useEffect, useState } from 'react';
import { Canvas } from '@react-three/fiber';
import Model from './Model';
import { EffectComposer, Noise } from '@react-three/postprocessing';
import {
    AccumulativeShadows,
    CameraShake,
    Environment,
    OrbitControls,
    OrthographicCamera,
    RandomizedLight,
    Stats,
} from '@react-three/drei';

const Scene = () => {
    const [mousePosition, setMousePosition] = useState({ x: 0, y: 0 });

    useEffect(() => {
        const updateMousePosition = (e) => {
            const x = (e.clientX / window.innerWidth) * 2 - 1;
            const y = -((e.clientY / window.innerHeight) * 2 - 1);
            setMousePosition({ x, y });
        };
        window.addEventListener('mousemove', updateMousePosition);

        return () => {
            window.removeEventListener('mousemove', updateMousePosition);
        };
    }, []);

    return (
        <>
            <Canvas
                style={{
                    opacity: 0.9,
                    position: 'absolute',
                    zIndex: -1,
                }}
            >
                <Suspense fallback={null}>
                    <Model mouse={mousePosition} />
                </Suspense>
                <pointLight intensity={10} />
                <Environment preset="city" />
                <AccumulativeShadows
                    temporal
                    frames={100}
                    color="white"
                    colorBlend={2}
                    toneMapped={true}
                    alphaTest={0.75}
                    opacity={2}
                    scale={12}
                >
                    <RandomizedLight
                        intensity={Math.PI}
                        amount={8}
                        radius={4}
                        ambient={0.5}
                        position={[5, 5, -10]}
                        bias={0.001}
                    />
                </AccumulativeShadows>
                <CameraShake
                    maxYaw={0.03}
                    maxPitch={0.03}
                    maxRoll={0}
                    yawFrequency={0.5}
                    pitchFrequency={0.5}
                />
                <EffectComposer>
                    <Noise opacity={0.1} />
                </EffectComposer>
            </Canvas>
        </>
    );
};

export default Scene;
