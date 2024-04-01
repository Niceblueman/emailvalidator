import * as React from "react";

function Settings(props) {
  return (
    <svg
      width={props.width || 18}
      height={props.width*16/18 || 16}
      viewBox="0 0 18 16"
      fill="none"
      {...props}
    >
      <path
        d="M3.25 14.75V9.5m0-3V1.25m6 13.5V8m0-3V1.25m6 13.5V11m0-3V1.25M1 9.5h4.5M7 5h4.5m1.5 6h4.5"
        stroke="#fff"
        strokeLinecap="round"
        strokeLinejoin="round"
      />
    </svg>
  );
}

export default Settings;
