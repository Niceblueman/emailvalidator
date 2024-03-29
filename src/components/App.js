import React from "react";
import styled from "styled-components";
import { Virtual, Pagination, Navigation } from 'swiper';
import { Swiper, SwiperSlide } from 'swiper/react';
import 'swiper/css';
import 'swiper/css/virtual';
import "swiper/css/pagination";
import "swiper/css/navigation";
import Mac from "./icons/Mac"
import Win from "./icons/Win"
import Linux from "./icons/Linux"
import { useTranslation } from "react-i18next";




let IMAGES = [
    "https://res.cloudinary.com/dupagadir/image/upload/v1665660525/Screen_Shot_2022-10-13_at_2.18.09_PM_rfh9mv.png",
    "https://res.cloudinary.com/dupagadir/image/upload/v1665660525/Screen_Shot_2022-10-13_at_2.18.22_PM_giqdad.png",
    "https://res.cloudinary.com/dupagadir/image/upload/v1665660525/Screen_Shot_2022-10-13_at_2.18.42_PM_wffibs.png"
]
export default function AppSection({ ...props }) {
    const { t } = useTranslation()
    return (
        <Wrapper>
            <BackgroundLeft src="./Background/left-top.svg" />
            <BackgroundRight src="./Background/right-top.svg" />
            <Btns>
                <p>
                    {t("app_download_title")}
                </p>
                <ul>
                    <li>
                        <Mac width={220} onClick={()=>{
                            window?.open("https://github.com/KM8Oz/BULKUS-RELEASE/raw/main/BULKUS_0.1.4_x64.dmg")
                        }} />
                    </li>
                    <li>
                        <Win width={220} onClick={()=>{
                            window?.open("https://github.com/KM8Oz/BULKUS-RELEASE/raw/main/BULKUS_0.1.4_x64_en-US.msi")
                        }}/>
                    </li>
                    <li>
                        <Linux width={220} onClick={()=>{
                            window?.open("https://github.com/KM8Oz/BULKUS-RELEASE/raw/main/bulkus_0.1.4_amd64.deb")
                        }}/>
                    </li>
                </ul>
            </Btns>
            <Styledswiper  
              className="appSwiper"
              grabCursor={true}
              pagination={true}
              navigation={true}
              modules={[Pagination, Navigation, Virtual]}
            slidesPerView={1} virtual>
                {
                    IMAGES.map((src, index) => (
                        <SwiperSlide key={src} virtualIndex={index}>
                            <Screen src={src} alt={"bulkus_app_screen_" + index} />
                        </SwiperSlide>
                    ))
                }
            </Styledswiper>
        </Wrapper>
    )
}
const Wrapper = styled.div`
    width: auto;
    height:auto;
    display:flex;
    overflow: hidden;
    background-color: #f2f2f2;
    flex-direction:row;
    flex-wrap:wrap-reverse;
    padding:1em;
    `;
const Btns = styled.div`
    flex: 1;
    display: flex;
    flex-direction: column;
    justify-content: space-around;
    align-items: center;
    position:relative;
    width: auto;
    min-height: 262px;
    p {
        font-family: 'GothamPro';
        font-size: 16px;
    }
    ul {
        min-height: 200px;
        width: 100%;
        height: 39%;
        padding:unset;
        margin:unset;
        list-style:none;
        display:flex;
        flex-direction:column;
        align-items:center;
        justify-content:space-around;
        li {
            display:flex;
            flex-direction:column;
            align-items:center;
            justify-content:center;
            cursor: pointer;
            transform:scale(1);
            transition: transfrom ease-in-out 100ms;
            :active{
                transform:scale(.98);
            }
        }
    }
`;
const BackgroundLeft = styled.img`
    position: absolute;
    top: 145vh;
    left: 0;
    z-index: 0;

    @media screen and (max-width: 767px) {
        top: 250vh;
        transform: rotate(45deg);
    }
`

const BackgroundRight = styled.img`
    position: absolute;
    top: 145vh;
    right: 0;
    z-index: 0;

    @media screen and (max-width: 767px) {
        display: none;
    }
`
const Styledswiper = styled(Swiper)`
    flex:1;
    width: auto;
    height: 500px;
    position: relative;
    overflow:hidden;
    min-width:360px;
    .swiper-slide {
        display:flex;
            flex-direction:column;
            align-items:center;
            justify-content:center
    }
`;
const Screen = styled.img`
    max-width: 100%;
    width: auto;
    max-height: 500px;
    height: auto;
    object-fit: contain;
    border-radius: 5px;
`;

