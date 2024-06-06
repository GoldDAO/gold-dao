/* eslint-disable no-nested-ternary */
/* eslint-disable consistent-return */
import { Bounce, toast } from 'react-toastify';
import { useEffect, useRef, useState } from 'react';

import AutosizeInput from 'react-input-autosize';
import Image from 'next/image';
import QRCode from 'qrcode.react';
import QrScanner from 'qr-scanner';
import { CopyButton } from '../../../utils/svgs';
import { copyContent } from '../../../utils/functions';
import useBalances from '../../../hooks/useBalances';
import useSession from '../../../hooks/useSession';
import useTransfer from '../../../hooks/useTransfer';

export default function ModalTransfer({
  title, amount, setGold, setIcp, setAmount,
}) {
  const [copyState, setCopyState] = useState(false);
  const [inputValue, setInputValue] = useState('');
  const [isReceive, setIsReceive] = useState(false);
  const [toPrincipal, setToPrincipal] = useState('');
  const { principal } = useSession();
  const videoRef = useRef(null);
  const [loadingQrScan, setLoadingQrScan] = useState(false);
  const [scanning, setScanning] = useState(false);
  const { getBalance } = useBalances();

  useEffect(() => {
    let stream = null;

    if (scanning) {
      const startCamera = async () => {
        try {
          stream = await navigator.mediaDevices.getUserMedia({
            video: { facingMode: 'environment' }, // Usa la cámara trasera
          });
          videoRef.current.srcObject = stream;
          const qrScanner = new QrScanner(videoRef.current, (result) => {
            setToPrincipal(result);
            setLoadingQrScan(true);
            setScanning(false);

            setTimeout(() => {
              setLoadingQrScan(false);
            }, 2000);
          });
          qrScanner.start();
          return () => {
            qrScanner.destroy();
            stream.getTracks().forEach((track) => track.stop());
          };
        } catch (error) {
          console.error('Error al acceder a la cámara:', error);
        }
      };
      startCamera();
    } else if (videoRef.current && videoRef.current.srcObject) {
      const tracks = videoRef.current.srcObject.getTracks();
      tracks.forEach((track) => track.stop());
    }
  }, [scanning]);

  const handleScanButtonClick = () => {
    setScanning(true);
  };

  const { icrc1Transfer, loading } = useTransfer({
    selectedToken: title === 'GLDGov' ? 'ledger' : 'icp',
    amount: inputValue,
    to: toPrincipal,
  });

  const handleTransfer = async () => {
    await icrc1Transfer();
    const newAmount = await getBalance(title === 'GLDGov' ? 'ledger' : 'icp');
    if (title === 'GLDGov') setGold({ loading: false, amount: newAmount });
    else setIcp({ loading: false, amount: newAmount });
    setInputValue('');
    setToPrincipal('');
    setAmount(newAmount / 1e8 / 1e8);
  };
  const disable = amount * 1e8 < inputValue + (title === 'GLDGov' ? 100000 : 10000)
    || amount === 0
    || inputValue === 0
    || !inputValue
    || !toPrincipal
    || inputValue < 0.00000001
    || loading;

  const handleMaxButtonClick = () => {
    setInputValue(amount === 0 ? amount : (amount * 1e8 - (title === 'GLDGov' ? 100000 : 10000) / 1e8).toFixed(8));
  };

  const handleToggle = () => {
    setIsReceive(!isReceive);
  };

  // const handleReloadClick = async () => {
  //   setReloadPrincipal(true);

  //   const newAmount = await getBalance(title === 'GLDGov' ? 'ledger' : 'icp');
  //   if (title === 'GLDGov') setGold({ loading: false, amount: newAmount });
  //   else setIcp({ loading: false, amount: newAmount });
  //   setAmount(newAmount / 1e8 / 1e8);

  //   // setParsedAmount(Number(await getBalance(title === "GLDGov" ? "ledger" : "icp")) / 10e7);
  //   setReloadPrincipal(false);
  // };

  useEffect(() => {
    if (copyState) {
      toast.success('Copied', {
        position: 'top-right',
        autoClose: 2000,
        hideProgressBar: false,
        closeOnClick: true,
        pauseOnHover: true,
        draggable: true,
        progress: undefined,
        theme: 'light',
        transition: Bounce,
      });
      setCopyState(false);
    }
  }, [copyState]);

  return (
    <>
      {loadingQrScan === true ? (
        <div className="h-[500px] flex justify-center items-center">
          <h1>loading...</h1>
        </div>
      ) : (
        <>
          <div
            className={`mt-6 width-[100%] flex justify-between items-center ${scanning && 'hidden'} text-xs`}
          >
            <label className="switch">
              <input type="checkbox" onClick={handleToggle}></input>
              <div className="slider">
                <span>Send</span>
                <span>Receive</span>
              </div>
            </label>
            <div>
              <button className="px-4 py-2 sm:px-10 mt-5 rounded-3xl text-black border-[black] border-[2px] font-bold">
                {(amount * 10e7)?.toString()?.slice(0, 7)} {title}
              </button>
              <div className="w-full flex justify-center text-xs mt-1">Total balance</div>
            </div>
          </div>

          {isReceive ? (
            <div className="mt-6 width-[90%] flex flex-col justify-center items-center">
              <div style={{ position: 'relative' }}>
                <QRCode
                  value={principal}
                  size={200}
                  bgColor="#FFFFFF"
                  fgColor="#000000"
                  level="Q"
                  includeMargin={false}
                  renderAs="svg"
                />
                <div
                  style={{
                    position: 'absolute',
                    top: '50%',
                    left: '50%',
                    transform: 'translate(-50%, -50%)',
                    zIndex: 1,
                  }}
                >
                  {/* <Image src={"/svg/dfinity-logo.svg"} alt="Logo" height={50} width={50} /> */}
                </div>
              </div>
              <div className="w-full mt-5 text-xs">
                <h1 className="font-bold text-xl">Principal</h1>
                <div className="flex justify-between items-center gap-2">
                  <p className="truncate">{principal}</p>
                  <div className="flex justify-center">
                    {/* <div
                      className={`bg-white rounded-full h-10 w-10 flex justify-center items-center
                      cursor-pointer ${reloadPrincipal ? 'animate-spin' : ''}`}
                      onClick={handleReloadClick}
                    >
                      {ReloadButton}
                    </div> */}
                    <div
                      onClick={() => copyContent(principal, setCopyState)}
                      className="bg-white rounded-full  h-10 w-10  flex justify-center items-center cursor-pointer"
                    >
                      {CopyButton}
                    </div>
                  </div>
                </div>
              </div>
            </div>
          ) : (
            <div
              className={`mt-6 w-full flex justify-between flex-col items-center ${scanning && 'hidden'}`}
            >
              <div className="flex max-w-[600px] justify-between items-center w-[350px] sm:w-[540px]">
                <div className="flex items-center w-full">
                  <AutosizeInput
                    type="number"
                    placeholderIsMinWidth
                    name="form-field-name"
                    inputClassName="focus:outline-none max-w-[200px] sm:max-w-[240px] text-3xl box-content sm:text-6xl font-bold focus:outline-none bg-CardBackground"
                    placeholder="0.00"
                    aria-label="Amount"
                    value={inputValue}
                    onChange={(e) => setInputValue(e.target.value)}
                  />
                  <h3 className="text-[#C6C6C6] text-3xl sm:text-5xl ml-1">{title}</h3>
                </div>
                <button
                  className="py-4 px-7 rounded-[100px] bg-[white] w-[100px] text-black border text-[18px] font-bold hidden sm:flex"
                  onClick={handleMaxButtonClick}
                >
                  Max
                </button>
              </div>
              {(amount * 10e7).toFixed(8) === inputValue && (
                <span className="text-start w-full text-sm text-[#494947]">
                  * Remember that fee is 0.0001
                </span>
              )}
              <label className="input input-bordered flex items-center gap-2 w-full rounded-md bg-white mt-10">
                <input
                  type="text"
                  value={toPrincipal}
                  onChange={(e) => setToPrincipal(e.target.value)}
                  className="grow"
                  placeholder="Principal"
                />{' '}
                <Image
                  src="svg/qr.svg"
                  alt="qr button"
                  height={20}
                  width={20}
                  className="cursor-pointer"
                  onClick={handleScanButtonClick}
                />
                {/* <ScannerComponent scanning={scanning}
                setScanning={setScanning} videoRef={videoRef} /> */}
              </label>
              <div className="mt-6 width-[90%] flex justify-center items-center">
                <button
                  onClick={handleTransfer}
                  className={
                    'py-4 px-7 rounded-full bg-[#D3B871] text-white text-xs font-bold disabled:opacity-50 hidden sm:flex'
                  }
                  disabled={disable}
                >
                  {loading === true ? 'loading...' : 'Confirm'}
                </button>
              </div>
            </div>
          )}
          <div className={`${scanning === true ? '' : 'hidden'}`}>
            <video
              ref={videoRef}
              className="w-full h-80"
              style={{
                transform: 'scaleX(-1)', // Voltea horizontalmente para usar la cámara trasera
                objectFit: 'cover', // Ajusta el video para cubrir todo el contenedor
              }}
            />
          </div>
          <div className={`mt-20 ${scanning === false ? 'hidden' : ''}`}>
            <button
              onClick={() => setScanning(false)}
              className={
                'px-10 py-4 rounded-3xl bg-[#D3B871] text-white text-md font-bold disabled:opacity-50 sm:hidden flex w-full justify-center'
              }
            >
              Cancel
            </button>
          </div>
          <div className="w-[100%] pt-5 ">
            {!inputValue && !isReceive ? (
              <div className={`${scanning === true ? 'hidden' : ''}`}>
                <button
                  className="py-2 px-4 rounded-3xl justify-center w-full bg-[white] text-black border text-sm font-bold flex sm:hidden"
                  onClick={handleMaxButtonClick}
                >
                  Max
                </button>
              </div>
            ) : !isReceive ? (
              <button
                onClick={handleTransfer}
                className="py-2 px-7 rounded-full bg-[#D3B871] text-white text-xs font-bold sm:hidden flex w-full justify-center disabled:opacity-35"
                disabled={disable}
              >
                {loading === true ? 'Loading...' : 'Check Order'}
              </button>
            ) : null}
          </div>
          {/* <div className={`${scanning && "hidden"}`}>
            {!isReceive && <Keyboard inputValue={inputValue} onKeyPress={handleKeyPress} />}
          </div> */}
        </>
      )}
    </>
  );
}
