import React from "react";

function Avatar(props) {
    return (
        <svg 
        width={props.width ? props.width : 48}  height={props.width ? props.width : 48} 
        viewBox="0 0 48 48">
            <circle cx='24' cy='24' r='20' fill='#5294E2'></circle>
            <path
                fill='#fff'
                d='M24 4A20 20 0 004 24a20 20 0 00.021.582A20 20 0 0124 5a20 20 0 0119.979 19.418 20 20 0 00.02-.418A20 20 0 0024 4z'
                opacity='0.2'
            ></path>
            <path
                d='M43.979 24.418A20 20 0 0124 44 20 20 0 014.022 24.582 20 20 0 004 25a20 20 0 0020 20 20 20 0 0020-20 20 20 0 00-.021-.582z'
                opacity='0.2'
            ></path>
            <path
                d='M24 12a5 5 0 100 10 5 5 0 000-10zm0 13c-9.999.006-10 6.4-10 6.4V34s1.846 4 10 4 10-4 10-4v-2.6s0-6.404-9.998-6.4z'
                opacity='0.2'
            ></path>
            <path
                fill='#fff'
                d='M24 11a5 5 0 100 10 5 5 0 000-10zm0 13c-9.999.006-10 6.4-10 6.4V33s1.846 4 10 4 10-4 10-4v-2.6s0-6.404-9.998-6.4z'
            ></path>
        </svg>
    );
}

export default React.memo(Avatar);

