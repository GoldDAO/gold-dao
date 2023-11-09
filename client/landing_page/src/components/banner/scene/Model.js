import React, { useLayoutEffect, useRef, useMemo, useEffect, useState } from 'react';
import * as THREE from 'three';
import { useTexture, useGLTF, SpotLight } from '@react-three/drei';
import { useFrame, useThree } from '@react-three/fiber';
import { FlakesTexture } from 'three-stdlib';

const Model = ({ mouse }) => {
    const { scene, nodes, materials } = useGLTF('/token_plus.glb');
    const ref = useRef();
    const [rEuler, rQuaternion] = useMemo(() => [new THREE.Euler(), new THREE.Quaternion()], []);
    const { pointer, clock } = useThree();

    useFrame(() => {
        if (ref.current && mouse) {
            rEuler.set((-mouse.y * Math.PI) / 2, (mouse.x * Math.PI) / 3, 0);
            ref.current.quaternion.slerp(rQuaternion.setFromEuler(rEuler), 0.1);
        }
    });

    useLayoutEffect(() => {
        scene.traverse((obj) => obj.isMesh && (obj.receiveShadow = obj.castShadow = true));
        materials['Material.001'].color.set('orange');
        materials['Material.001'].roughness = 0;
        materials['Material.001'].normalMap = new THREE.CanvasTexture(
            new FlakesTexture(),
            THREE.UVMapping,
            THREE.RepeatWrapping,
            THREE.RepeatWrapping,
        );
        materials['Material.001'].normalMap.repeat.set(40, 40);
        materials['Material.001'].normalScale.set(0.01, 0.01);
    });
    return <primitive object={scene} rotation={[-0.63, 0, 0]} ref={ref} />;
};

export default Model;
