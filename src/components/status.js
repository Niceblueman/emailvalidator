import React from "react"
export const Status = (props) => {
    switch (props?.status) {
        case "Risky":
            return (
                <svg
                    width={props.width || 24}
                    height={props.width || 24}
                    fill="none"
                    viewBox="0 0 24 24"
                    {...props}
                >
                    <path
                        d="M12 22c5.523 0 10-4.477 10-10S17.523 2 12 2 2 6.477 2 12s4.477 10 10 10zM12 8v4M12 16h.01"
                        stroke="#EDB934"
                        strokeWidth={2}
                        strokeLinecap="round"
                        strokeLinejoin="round"
                    />
                </svg>
            );
        case "Invalid":
            return (
                <svg
                    width={props.width || 24}
                    height={props.width || 24}
                    fill="none"
                    viewBox="0 0 24 24"
                    {...props}
                >
                    <path
                        d="M12 22c5.523 0 10-4.477 10-10S17.523 2 12 2 2 6.477 2 12s4.477 10 10 10zM12 8v4M12 16h.01"
                        stroke="#ED343F"
                        strokeWidth={2}
                        strokeLinecap="round"
                        strokeLinejoin="round"
                    />
                </svg>
            );
        case "Unknown":
            return (
                <svg
                    width={props.width || 24}
                    height={props.width || 24}
                    fill="none"
                    viewBox="0 0 24 24"
                    {...props}
                >
                    <path
                        d="M12 22c5.523 0 10-4.477 10-10S17.523 2 12 2 2 6.477 2 12s4.477 10 10 10zM12 8v4M12 16h.01"
                        stroke="#ED343F"
                        strokeWidth={2}
                        strokeLinecap="round"
                        strokeLinejoin="round"
                    />
                </svg>
            );
        case "Safe":
            return (
                <svg
                    width={props.width || 24}
                    height={props.width || 24}
                    fill="none"
                    viewBox="0 0 24 24"
                    {...props}
                >
                    <path
                        d="M10.243 16.314L6 12.07l1.414-1.414 2.829 2.828 5.656-5.657 1.415 1.415-7.071 7.07z"
                        fill="#3FC500"
                    />
                    <path
                        fillRule="evenodd"
                        clipRule="evenodd"
                        d="M1 12C1 5.925 5.925 1 12 1s11 4.925 11 11-4.925 11-11 11S1 18.075 1 12zm11 9a9 9 0 110-18 9 9 0 010 18z"
                        fill="#3FC500"
                    />
                </svg>
            );
        default:
            return (
                <svg
                    width={props.width || 24}
                    height={props.width || 24}
                    fill="none"
                    viewBox="0 0 24 24"
                    {...props}
                >
                    <path
                        d="M12 2v4M12 18v4M4.93 4.93l2.83 2.83M16.24 16.24l2.83 2.83M2 12h4M18 12h4M4.93 19.07l2.83-2.83M16.24 7.76l2.83-2.83"
                        stroke="#4E4E4E"
                        strokeWidth={2}
                        strokeLinecap="round"
                        strokeLinejoin="round"
                    />
                </svg>
            );
    }
}

