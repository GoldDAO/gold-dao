import React, { Suspense } from 'react';
import { Canvas } from '@react-three/fiber';
import Model from './Model';
import { Box } from '@chakra-ui/react';
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

const Scene = () => {
    return (
        <Canvas
            style={{
                opacity: 0.9,
            }}
        >
            <Suspense fallback={null}>
                <Model />
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
                    {/* <SSAO
                        blendFunction={BlendFunction.MULTIPLY} // blend mode
                        samples={30} // amount of samples per pixel (shouldn't be a multiple of the ring count)
                        rings={4} // amount of rings in the occlusion sampling pattern
                        distanceThreshold={1.0} // global distance threshold at which the occlusion effect starts to fade out. min: 0, max: 1
                        distanceFalloff={0.0} // distance falloff. min: 0, max: 1
                        rangeThreshold={0.5} // local occlusion range threshold at which the occlusion starts to fade out. min: 0, max: 1
                        rangeFalloff={0.1} // occlusion range falloff. min: 0, max: 1
                        luminanceInfluence={0.9} // how much the luminance of the scene influences the ambient occlusion
                        radius={20} // occlusion sampling radius
                        scale={0.5} // scale of the ambient occlusion
                        bias={0.5} // occlusion bias
                    /> */}
                </EffectComposer>
            </Suspense>
        </Canvas>
    );
};

export default Scene;
