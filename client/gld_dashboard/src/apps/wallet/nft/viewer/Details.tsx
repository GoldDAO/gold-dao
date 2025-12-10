import { useState } from "react";
import { NFT } from "@services/nft/utils/interfaces";
import Dialog from "@shared/ui/dialog/DialogV2";
import Icon from "@shared/ui/icons";

const ImageViewer = ({
  src,
  alt,
  open,
  onClose,
}: {
  src: string;
  alt: string;
  open: boolean;
  onClose: () => void;
}) => {
  if (!open) return null;

  const handleBackdropClick = (e: React.MouseEvent) => {
    if (e.target === e.currentTarget) {
      onClose();
    }
  };

  const handleButtonClick = (e: React.MouseEvent<HTMLButtonElement>) => {
    e.preventDefault();
    e.stopPropagation();
    onClose();
  };

  return (
    <div
      className="fixed inset-0 z-[60] bg-black/90 backdrop-blur-sm"
      onClick={handleBackdropClick}
    >
      <button
        onClick={handleButtonClick}
        className="fixed top-8 right-8 p-2 rounded-full bg-black/50 hover:bg-black/70 text-white z-[70] pointer-events-auto cursor-pointer"
        type="button"
      >
        <Icon.Close width={24} height={24} />
      </button>
      <div className="relative w-full h-full flex items-center justify-center pointer-events-none">
        <div
          className="relative max-w-[95vw] max-h-[95vh] p-4 pointer-events-auto"
          onClick={(e) => e.stopPropagation()}
        >
          <img
            src={src}
            alt={alt}
            className="max-w-full max-h-[95vh] object-contain rounded-lg"
            onClick={(e) => e.stopPropagation()}
          />
        </div>
      </div>
    </div>
  );
};

const Details = ({
  nft,
  open,
  onClose,
}: {
  nft: NFT;
  open: boolean;
  onClose: () => void;
}) => {
  const [selectedImage, setSelectedImage] = useState<string | null>(null);

  const handleImageClick = (src: string) => {
    setSelectedImage(src);
  };

  const handleCloseImageViewer = () => {
    setSelectedImage(null);
  };

  const handleDialogClose = () => {
    if (selectedImage) {
      setSelectedImage(null);
      return;
    }
    onClose();
  };

  const handleCloseButtonClick = () => {
    if (selectedImage) {
      setSelectedImage(null);
    } else {
      onClose();
    }
  };

  return (
    <>
      <Dialog open={open} onClose={handleDialogClose} size="xl">
        <div className="flex items-center justify-end mb-4 shrink-0">
          <Dialog.CloseBtn onClick={handleCloseButtonClick} />
        </div>
        <div className="flex-1 space-y-6">
          <h2 className="text-xl font-semibold text-content mb-4">
            {nft.name}
          </h2>
          <div className="flex flex-col md:flex-row gap-6">
            {nft.img_preview && (
              <div className="flex-shrink-0 md:w-1/3">
                <img
                  src={nft.img_preview}
                  alt={nft.name}
                  className="w-full h-auto object-cover cursor-pointer hover:opacity-80 transition-opacity"
                  onClick={() => handleImageClick(nft.img_preview!)}
                />
              </div>
            )}
            <div className="flex-1">
              <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                {nft.name && (
                  <div>
                    <div className="text-sm font-semibold text-content/60 mb-1">
                      Name
                    </div>
                    <div className="text-content">{nft.name}</div>
                  </div>
                )}
                <div>
                  <div className="text-sm font-semibold text-content/60 mb-1">
                    ID
                  </div>
                  <div className="text-content">{nft.id.toString()}</div>
                </div>
                {nft.serial_number && (
                  <div>
                    <div className="text-sm font-semibold text-content/60 mb-1">
                      Serial Number
                    </div>
                    <div className="text-content">{nft.serial_number}</div>
                  </div>
                )}
                {nft.weight && (
                  <div>
                    <div className="text-sm font-semibold text-content/60 mb-1">
                      Weight
                    </div>
                    <div className="text-content">{nft.weight}</div>
                  </div>
                )}
              </div>
              {nft.description && (
                <div className="mt-4">
                  <div className="text-sm font-semibold text-content/60 mb-1">
                    Description
                  </div>
                  <p className="text-content/80">{nft.description}</p>
                </div>
              )}
            </div>
          </div>

          {(nft.img_front || nft.img_back) && (
            <div className="border-t border-border pt-4">
              <h2 className="text-xl font-semibold text-content mb-4">
                Images
              </h2>
              <div className="grid grid-cols-1 sm:grid-cols-2 gap-4">
                {nft.img_front && (
                  <div>
                    <h3 className="text-sm font-semibold text-content/60 mb-2">
                      Front
                    </h3>
                    <img
                      src={nft.img_front}
                      alt={`${nft.name} Front`}
                      className="w-full h-48 object-cover rounded-lg border border-border cursor-pointer hover:opacity-80 transition-opacity"
                      onClick={() => handleImageClick(nft.img_front!)}
                    />
                  </div>
                )}
                {nft.img_back && (
                  <div>
                    <h3 className="text-sm font-semibold text-content/60 mb-2">
                      Back
                    </h3>
                    <img
                      src={nft.img_back}
                      alt={`${nft.name} Back`}
                      className="w-full h-48 object-cover rounded-lg border border-border cursor-pointer hover:opacity-80 transition-opacity"
                      onClick={() => handleImageClick(nft.img_back!)}
                    />
                  </div>
                )}
              </div>
            </div>
          )}

          {(nft.weight ||
            nft.fineness ||
            nft.dimensions ||
            nft.hardness ||
            nft.manufacturer) && (
            <div className="border-t border-border pt-4">
              <h2 className="text-xl font-semibold text-content mb-4">
                Gold specifications
              </h2>
              <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                {nft.weight && (
                  <div>
                    <div className="text-sm font-semibold text-content/60 mb-1">
                      Weight
                    </div>
                    <div className="text-content">{nft.weight}</div>
                  </div>
                )}
                {nft.fineness && (
                  <div>
                    <div className="text-sm font-semibold text-content/60 mb-1">
                      Fineness
                    </div>
                    <div className="text-content">{nft.fineness}</div>
                  </div>
                )}
                {nft.dimensions && (
                  <div>
                    <div className="text-sm font-semibold text-content/60 mb-1">
                      Dimensions
                    </div>
                    <div className="text-content">{nft.dimensions}</div>
                  </div>
                )}
                {nft.hardness && (
                  <div>
                    <div className="text-sm font-semibold text-content/60 mb-1">
                      Hardness
                    </div>
                    <div className="text-content">{nft.hardness}</div>
                  </div>
                )}
                {nft.manufacturer && (
                  <div>
                    <div className="text-sm font-semibold text-content/60 mb-1">
                      Manufacturer
                    </div>
                    <div className="text-content">{nft.manufacturer}</div>
                  </div>
                )}
              </div>
            </div>
          )}
        </div>
      </Dialog>

      {selectedImage && (
        <ImageViewer
          src={selectedImage}
          alt={nft.name ?? "NFT Image"}
          open={!!selectedImage}
          onClose={handleCloseImageViewer}
        />
      )}
    </>
  );
};

export default Details;
