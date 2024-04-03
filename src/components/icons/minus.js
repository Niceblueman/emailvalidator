import * as React from "react";

function Minus(props) {
  return (
    <svg
      width={props.width||24}
      height={props.width||24}
      fill="none"
      {...props}
    >
      <path
        d="M12 22c5.523 0 10-4.477 10-10S17.523 2 12 2 2 6.477 2 12s4.477 10 10 10zM8 12h8"
        stroke="#201F1F"
        strokeWidth={2}
        strokeLinecap="round"
        strokeLinejoin="round"
      />
    </svg>
  );
}

export default Minus;