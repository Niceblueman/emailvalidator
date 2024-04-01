import { useTranslation } from "react-i18next"
import styled from "styled-components"
import { Swiper, SwiperSlide } from "swiper/react"
import Benefits from "./Benefits"
import SwiperPaymentsPage from "./SwiperPaymentPage";
import SwiperPaymentPageMobile from "./SwiperPaymentPageMobile";
import 'swiper/css';
import 'swiper/css/navigation';
import 'swiper/css/pagination';
import 'swiper/css/scrollbar';
import { Navigation, Pagination, Scrollbar, A11y } from 'swiper';
import Info from "./Info";
// import VerifyPage from "./VerifyPage";
// import { useEffect } from "react";
// import { useState } from "react";


const SwiperPage = ({setLogin}) => {  
    const { t } = useTranslation()
    
    return (
        <Wrapper id="payment_seg">
            <BackgroundLeft src="./Background/left-top.svg" />
            <BackgroundRight src="./Background/right-top.svg" />
            <Container>
                <Disclaymer>{t('separate')}</Disclaymer>
                <Swiper
                    id="Swiper"
                    direction={"vertical"}
                    slidesPerView={1}
                    modules={[Navigation, Pagination, Scrollbar, A11y]}
                    spaceBetween={10}
                    simulateTouch
                    navigation={{
                        nextEl: '.swiper-button-next-2',
                        prevEl: '.swiper-button-prev-2',
                        enabled: true
                    }}
                    pagination={{
                        type: "fraction",
                        clickable: true,
                        el: "bollets"
                    }}
                    className="mySwiper"
                >
                    <SwiperSlide>
                        <Info data-aos="fade-up"
                            data-aos-anchor-placement="top-bottom" />
                    </SwiperSlide>
                    {/* <SwiperSlide>
                        <VerifyPage data-aos="fade-up"
                            data-aos-anchor-placement="top-bottom" />
                    </SwiperSlide> */}
                    <SwiperSlide>
                        <Benefits data-aos="fade-up"
                            data-aos-anchor-placement="top-bottom" />
                    </SwiperSlide>
                    <SwiperSlide>
                        <SwiperPaymentsPage setLogin={setLogin} data-aos="fade-up"
                            data-aos-anchor-placement="top-bottom" />
                    </SwiperSlide>
                </Swiper>
                <SwiperPaymentPageMobile   setLogin={setLogin} data-aos="fade-up"
                            data-aos-anchor-placement="top-bottom" />
                <div className="swiper-button-next-2"></div>
                <div className="swiper-button-prev-2"></div>
            </Container>
        </Wrapper>
    )
}

export default SwiperPage

const Wrapper = styled.div`
    min-height: 100vh;
    width: 100%;
    background-color: var(--BackgroundColor);
    margin: 0px auto;
    box-shadow: inset 4px 4px 20px 0 rgba(0, 0, 0, 0.15), inset -4px -4px 20px 0 white;

    @media screen  and (max-width: 767px){
        /* display: none; */
    }
`

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

const Container = styled.div`
    width: 100%;
    max-width: 1200px;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: flex-start;
    margin: 0px auto;
    /* background-color: var(--BackgroundColor); */
    z-index: 5;
    position: relative ;

`

const Disclaymer = styled.div`
    max-width: 850px;
    box-shadow:  4px 4px 20px 0 rgba(0, 0, 0, 0.15), -4px -4px 20px 0 white;
    background-color: var(--BackgroundColor);
    font-family: 'GothamPro';
    font-weight: bold;
    font-size: 24px;
    line-height: 32px;
    text-align: center;
    color: var(--TextColor);
    border-radius: 5px;
    padding: 30px 50px;
    z-index: 5;
    margin: 100px 0px 50px 0px;

    @media screen and (max-width: 767px) {
        width: calc(100% - 32px);
        padding: 30px 0px;
        font-size: 20px;
        line-height: 25px;
        margin: 50px 0px;
    }
`

