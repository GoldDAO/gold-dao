import React, { useRef } from 'react';
import { useLoader } from '@react-three/fiber';
import { GLTFLoader } from 'three/examples/jsm/loaders/GLTFLoader';
import { useGLTF } from '@react-three/drei';

const Model = () => {
    const gltf = useLoader(GLTFLoader, '/token.glb');
    const { nodes, materials } = useGLTF('/token.glb');

    return (
        <group>
            <mesh castShadow receiveShadow geometry={nodes.Ellipse_2} material={materials} />
            <primitive object={gltf.scene} />
        </group>
    );
};

export default Model;
