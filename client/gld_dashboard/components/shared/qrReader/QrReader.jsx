import React, { useRef, useEffect, useState } from "react";
import QrScanner from "qr-scanner";

const ScannerComponent = () => {
  const videoRef = useRef(null);
  const [scanning, setScanning] = useState(false);

  return (
    <div className="flex flex-col items-center justify-center h-screen">
      {!scanning && (
        <div
          className="w-32 h-32 border border-white"
          style={{
            position: "absolute",
            top: "50%",
            left: "50%",
            transform: "translate(-50%, -50%)",
          }}
        ></div>
      )}
      {!scanning && (
        <button
          onClick={handleScanButtonClick}
          className="mt-4 px-4 py-2 bg-blue-500 text-white rounded-lg shadow-md hover:bg-blue-600 focus:outline-none focus:ring-2 focus:ring-blue-400 focus:ring-opacity-75"
        >
          Escanear QR
        </button>
      )}
    </div>
  );
};

export default ScannerComponent;
