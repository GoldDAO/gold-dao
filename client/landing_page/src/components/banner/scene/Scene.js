import React, { Suspense, useEffect, useState } from 'react';
import { Canvas } from '@react-three/fiber';
import Model from './Model';
import { Box, Text } from '@chakra-ui/react';
import {
    EffectComposer,
    DepthOfField,
    Noise,
    SSAO,
    LensFlare,
    BrightnessContrast,
} from '@react-three/postprocessing';
import { BlendFunction } from 'postprocessing';
import {
    AccumulativeShadows,
    CameraShake,
    Environment,
    OrbitControls,
    OrthographicCamera,
    RandomizedLight,
    Stats,
} from '@react-three/drei';
import * as THREE from 'three';
import { useFrame, useThree } from '@react-three/fiber';

const Scene = () => {
    const [mouse, setMouse] = useState();
    const GetPos = ({ setMouse }) => {
        const { pointer, clock } = useThree();
        useEffect(() => {
            setMouse(pointer);
        }, [pointer]);
    };
    return (
        <>
            <Canvas
                style={{
                    opacity: 0.9,
                    // width: '100vw',
                    // height: '100vh',
                    // top: 0,
                    position: 'absolute',
                    // left: 0,
                    zIndex: -1,
                }}
            >
                <Suspense fallback={null}>
                    <Model mouse={mouse} />
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
            <Canvas
                style={{
                    width: '100vw',
                    height: '100vh',
                    top: 0,
                    position: 'fixed',
                    left: 0,
                    zIndex: 100,
                }}
            >
                <GetPos setMouse={setMouse} />
            </Canvas>
        </>
    );
};

export default Scene;
