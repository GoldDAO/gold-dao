import React, { useLayoutEffect, useRef, useMemo } from 'react';
import * as THREE from 'three';
import { useTexture, useGLTF, SpotLight } from '@react-three/drei';
import { useFrame, useThree } from '@react-three/fiber';
import { FlakesTexture } from 'three-stdlib';

const Model = () => {
    const { scene, nodes, materials } = useGLTF('/cleantoken.glb');
    const ref = useRef();
    const [rEuler, rQuaternion] = useMemo(() => [new THREE.Euler(), new THREE.Quaternion()], []);
    const { mouse, clock } = useThree();

    useFrame(() => {
        if (ref.current) {
            rEuler.set((-mouse.y * Math.PI) / 2, (mouse.x * Math.PI) / 3, 0);
            ref.current.quaternion.slerp(rQuaternion.setFromEuler(rEuler), 0.1);
            // ref.current.material.time = clock.getElapsedTime() * 6;
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
        materials['Material.001'].normalScale.set(0.1, 0.1);
    });
    return <primitive object={scene} rotation={[-0.63, 0, 0]} ref={ref} />;
};

export default Model;

{
    /* <mesh
                position={[0, -0, -0]}
                geometry={nodes.token.geometry}
                ref={ref}
                castShadow
                color="orange"
            >
                <meshStandardMaterial roughness={0.1} metalness={0.6} color={'orange'} />
            </mesh>
            <ambientLight intensity={10} color={0x404040} />
            <pointLight
                distance={5}
                intensity={10}
                angle={1}
                attenuation={5}
                anglePower={5} // Diffuse-cone anglePower (default: 5)
            /> */
}
