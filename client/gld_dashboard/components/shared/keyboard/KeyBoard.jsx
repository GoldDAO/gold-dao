import { KeyboardArrow } from "../../../utils/svgs";
import React from "react";

const Keyboard = ({ onKeyPress, inputValue }) => {
  const handleKeyPress = (key) => {
    if (key === "DEL") {
      onKeyPress(inputValue.slice(0, -1));
    } else {
      onKeyPress(inputValue + key);
    }
  };
  const keyboardButtons = [
    ["1", "2", "3"],
    ["4", "5", "6"],
    ["7", "8", "9"],
    [",", "0", "DEL"],
  ];

  return (
    <div className="flex flex-col items-center w-[100%] sm:hidden ">
      {keyboardButtons.map((buttons, rowIndex) => (
        <div key={rowIndex} className="grid grid-cols-3 justify-center w-full gap-6">
          {buttons.map((button, colIndex) => (
            <button
              key={colIndex}
              className="my-1 p-2 rounded text-2xl flex justify-center items-center"
              onClick={() => handleKeyPress(button)}
            >
              {button === "DEL" ? KeyboardArrow : button}
            </button>
          ))}
        </div>
      ))}
    </div>
  );
};

export default Keyboard;
