import React, { useRef, useState } from 'react';
import { Canvas, useFrame } from '@react-three/fiber';
import { OrbitControls, Html } from '@react-three/drei';

function FaultBeacon({ position, severity, nodeName, residual }) {
  const meshRef = useRef();
  const [, setHover] = useState(false);

  useFrame((state, delta) => {
    if (meshRef.current) {
      meshRef.current.rotation.y += delta * 2;
      const scale = 1 + Math.sin(state.clock.elapsedTime * 6) * 0.15;
      meshRef.current.scale.set(scale, scale, scale);
    }
  });

  const color = severity === 'CRITICAL' ? '#ff1744' : severity === 'WARNING' ? '#ff9100' : '#00e676';

  return (
    <group position={position}>
      <mesh ref={meshRef} onPointerOver={() => setHover(true)} onPointerOut={() => setHover(false)}>
        <sphereGeometry args={[0.35, 32, 32]} />
        <meshStandardMaterial color={color} emissive={color} emissiveIntensity={0.8} wireframe />
      </mesh>

      <Html distanceFactor={12}>
        <div style={{
          background: 'rgba(10, 15, 25, 0.88)',
          border: `1px solid ${color}`,
          color: '#ffffff',
          padding: '8px 12px',
          borderRadius: '6px',
          fontFamily: 'monospace',
          fontSize: '12px',
          whiteSpace: 'nowrap',
          boxShadow: `0 0 12px ${color}66`
        }}>
          <div style={{ fontWeight: 'bold', color }}>[{severity}] {nodeName}</div>
          <div>Residual Shift: δ = {residual.toFixed(2)}</div>
          <div>XYZ: ({position.join(', ')})</div>
        </div>
      </Html>
    </group>
  );
}

export default function PlantCadViewer({ activeFaults = [] }) {
  return (
    <div style={{ width: '100%', height: '600px', background: '#090d16', borderRadius: '8px', position: 'relative' }}>
      <Canvas camera={{ position: [15, 12, 20], fov: 45 }}>
        <ambientLight intensity={0.4} />
        <directionalLight position={[10, 20, 15]} intensity={1.0} />
        <gridHelper args={[40, 40, '#1e293b', '#0f172a']} />

        <mesh position={[0, 0.5, 0]}>
          <boxGeometry args={[8, 1, 4]} />
          <meshStandardMaterial color="#334155" />
        </mesh>
        
        <mesh position={[-2, 2, 0]} rotation={[0, 0, Math.PI / 2]}>
          <cylinderGeometry args={[1.2, 1.2, 3, 32]} />
          <meshStandardMaterial color="#475569" />
        </mesh>

        {activeFaults.map((fault) => (
          <FaultBeacon
            key={fault.event_id}
            position={fault.spatial_telemetry.coordinates_xyz}
            severity={fault.diagnosis.severity}
            nodeName={fault.diagnosis.root_cause_node}
            residual={fault.diagnosis.structural_residual_shift}
          />
        ))}

        <OrbitControls makeDefault />
      </Canvas>
    </div>
  );
}
