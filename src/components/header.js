/* eslint-disable no-unused-expressions */
/* eslint-disable no-restricted-globals */
/* eslint-disable no-unused-vars */
/* eslint-disable eqeqeq */
/* eslint-disable no-useless-escape */
/* eslint-disable react-hooks/exhaustive-deps */
import { useEffect, useState } from "react";
import { useTranslation } from "react-i18next"
import styled from "styled-components"
import { Client, getCookie, uuid } from "../api/client";
// import useLogin from "../hooks/login";
import { toast } from 'react-toastify';
import Avatar from "./icons/avatar";
import { animated } from "@react-spring/web"
import { useToast } from "../hooks/login";
const AvatarWrapper = styled(animated.div)`
    width: fit-content;
    height: fit-content;
    display: flex;
    justify-content: center;
    align-items: center;
    background-color: #fff;
    padding: 0px;
    margin: 0px 12px;
    /* border: 2px #b1ddfe solid; */
    border-radius: 50%;
    cursor:pointer;
    transform: scale(1);
    :active {
        transition: transform ease-in-out 100ms;
        transform: scale(0.98);
    }
`;

const Header = ({ header, setOpenPanel, _setProfile, setLogin }) => {
    const { t } = useTranslation();
    const [isChecked, setIsChecked] = useState(false);
    const [ToastId, setToastId] = useState(0);
    const [profile, setProfile] = useState({
        username: "",
        email: "",
        customer_id: ""
    });
    useEffect(() => {
        var ___ads = getCookie('____ads')
        if (___ads) {
            Client.get("/profile", {
                params: {
                    "ads": ___ads
                }
            }).then((res) => {
                if (res.data.success) {
                    setProfile(res.data.Object)
                } else {
                    if (!useToast.is_open) {
                        useToast.is_open = toast.error(t(res.data.reason), {
                            position: "bottom-right",
                            autoClose: 1000,
                            hideProgressBar: false,
                            closeOnClick: true,
                            pauseOnHover: true,
                            draggable: true,
                            progress: undefined,
                        })
                    }
                }
            }).catch((err) => null)
        }
    }, [header])
    return (
        <Wrapper>
            <Container>
                <Logo  src="/icons/logo-blue.svg" />
                <NavLine>
                    <NavItem onClick={() => {
                        var el = document.querySelector(".swiper div");
                        var _swiper = document.querySelector(".swiper");
                        el?.scrollIntoView({ behavior: "smooth", block: "end", inline: "nearest" });
                        _swiper?.swiper?.slideTo(3)
                    }}>{t('cost')}</NavItem>
                    {/* <NavItem onClick={() => {
                        var el = document.querySelector("#partners");
                        el?.scrollIntoView({ behavior: "smooth", block: "end", inline: "nearest" });
                    }}>{t('partners')}</NavItem> */}
                    <NavItem onClick={() => {
                        window.open('https://mailvalidator.dup.company/rapidoc/')
                    }}>{t('documentation')}</NavItem>
                    <NavItem onClick={() => {
                        window.open('https://mailvalidator.dup.company/swagger-ui/')
                    }}>{"Swagger"}</NavItem>
                    {(profile.username && profile.email) ?
                        <AvatarWrapper onClick={() => {
                            _setProfile(profile)
                            setOpenPanel(s => !s)
                        }}>
                            <Avatar width={50} />
                        </AvatarWrapper> :
                        <>
                            <LoginBtn onClick={() => setLogin(s => ({ register: false, login: true, recover: false }))} >{t('enter')}</LoginBtn>
                            <RegistrationBtn onClick={() => setLogin(s => ({ login: false, register: true, recover: false }))}>{t('registration')}</RegistrationBtn>
                        </>}
                </NavLine>
            </Container>
            <WrapperMobile>
                <Logo src="/icons/apple-touch-icon.png" />
                <CheckBox type='checkbox' name="" id="menu-toggle" checked={isChecked} onChange={() => setIsChecked(!isChecked)} />
                <HamburgerLine>
                    <Line className="line1"></Line>
                    <Line className="line2"></Line>
                    <Line className="line3"></Line>
                </HamburgerLine>
                <MenuItems htmlFor="menu-toggle">
                    <MenuItemsLi onClick={() => {
                        var _swiper = document.querySelector("#payment_seg");
                        _swiper?.scrollIntoView({ behavior: "smooth", block: "start", inline: "nearest" });
                        setIsChecked(!isChecked)
                    }}>
                        {t('cost')}
                    </MenuItemsLi>
                    <MenuItemsLi onClick={() => {
                        var el = document.querySelector("#partners");
                        el?.scrollIntoView({ behavior: "smooth", block: "end", inline: "nearest" });
                        setIsChecked(!isChecked)
                    }}>
                        {t('partners')}
                    </MenuItemsLi>
                    <MenuItemsLi onClick={() => {
                        window.open('https://mailvalidator.dup.company/rapidoc/')
                    }}>
                        {t('documentation')}
                    </MenuItemsLi>
                    <MenuItemsLi onClick={() => {
                        window.open('https://mailvalidator.dup.company/swagger-ui/')
                    }}>
                        {"Swagger"}
                    </MenuItemsLi>
                    <MenuItemsLi onClick={() => {
                        setIsChecked(!isChecked)
                        setLogin(s => ({ register: false, login: true, recover: false }))
                    }}>
                        {t('enter')}
                    </MenuItemsLi>
                    <MenuItemsLi onClick={() => {
                        setIsChecked(!isChecked)
                        setLogin(s => ({ login: false, register: true, recover: false }))
                    }}>
                        {t('registration')}
                    </MenuItemsLi>
                </MenuItems>
            </WrapperMobile>
        </Wrapper>
    )
}

export default Header

const Wrapper = styled.header`
    width: 100%;
    height: 100px;
    display: flex;
    background-color: var(--BackgroundColor);
    box-shadow: 4px 4px 20px 0 rgba(0, 0, 0, 0.15), -4px -4px 20px 0 white;
    z-index: 100;
    position: absolute;
    top: 0;

    @media screen and (max-width: 767px) {
       height: 60px; 
       max-width: 100%;
    }
`

const Container = styled.div`
    width: 100%;
    height: 100%;
    align-self: center;
    margin: 0px auto;
    max-width: 1200px;
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: space-between;
    z-index: 100;
    overflow: hidden;
    @media screen and (max-width: 767px) {
        display: none;
    }

`

const Logo = styled.img`
    height: 58px;
    flex: 2;
    object-fit: contain;
    transition: transform;
    transform: rotate(0deg);
    &:hover {
        transform: rotate(360deg);
    }
    @media screen and (max-width: 767px) {
        width: 130px;
        height: 54px;
        object-fit: contain;
    }
`

const NavLine = styled.div`
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: space-between;
    flex: 10;
    padding: 0px 18px;
`

const NavItem = styled.div`
    font-family: 'GothamPro';
    font-size: 14px;
    font-weight: 400;
    line-height: 13.5px;
    color: var(--TextColor);
    cursor: pointer;
    &:hover {
        color:  rgba(77, 77, 77, 0.6)
    }
`

const LoginBtn = styled.button`
    width: 95px;
    height: 45px;
    border: 1px solid var(--Blue);
    border-radius: 5px;
    padding: 15px 25px 15px 25px;
    font-family: 'GothamPro';
    font-size: 14px;
    font-weight: 400;
    background-color: unset;
    color: var(--TextColor);
    cursor: pointer;
`

const RegistrationBtn = styled.button`
    width: 145px;
    height: 45px;
    background-color: var(--Blue);
    border-radius: 5px;
    color: white;
    font-family: 'GothamPro';
    font-size: 14px;
    font-weight: 400;
    cursor: pointer;
`

const WrapperMobile = styled.div`
    display: none;
    @media screen and (max-width: 767px){
        display: block;
        width: calc(100% - 32px);
        /* height: 80px; */
        display: flex;
        flex-direction: row;
        align-items: center;
        justify-content: space-between;
        margin: 0px auto;
    }
 `

const MenuItems = styled.div`
    margin-top: 59px;
    padding-top: 20px;
    height: 100vh;
    width: 100%;
    transform: translate(150%);
    display: flex;
    flex-direction: column;
    margin-left: -40px;
    padding-left: 25px;
    transition: transform 0.25s ease-in-out;
    text-align: center;
    position: absolute;
    top: 0;
    background-color: rgba(255, 255, 255, 0.9);
    display: none;
`

const HamburgerLine = styled.div`
    display: block;
    height: 26px;
    width: 32px;
    position: absolute;
    top: 16px;
    right: 16px;
    z-index: 2;
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    & .line1 {
        transform-origin: 0% 0%;
        transition: transform 0.3s ease-in-out;
    }
    & .line2 {
        transition: transform 0.2s ease-in-out;
    }
    & .line3 {
        transform-origin: 0% 100%;
        transition: transform 0.3s ease-in-out;
    }
`
const CheckBox = styled.input`
    position: absolute;
    display: block;
    height: 26px;
    width: 32px;
    top: 16px;
    right: 16px;
    z-index: 5;
    opacity: 0;
    cursor: pointer;
    &:checked ~ ${MenuItems} {
        transform: translateX(0);
        display: block;
    }
    &:checked ~ ${HamburgerLine} .line1 {
        transform: rotate(45deg);
    }
    &:checked ~ ${HamburgerLine} .line2 {
        transform: scaleY(0)
    }
    &:checked ~ ${HamburgerLine} .line3 {
        transform: rotate(-45deg);
    }
`

const Line = styled.span`
    display: block;
    height: 4px;
    width: 100%;
    border-radius: 10px;
    background: black;
`

const MenuItemsLi = styled.div`
    margin-bottom: 1.2rem;
    font-size: 1.5rem;
    font-weight: 500;
    font-family: 'GothamPro';
    cursor: pointer;
    @media screen  and (max-width: 767px){
        margin-bottom: 40px;
    }
    `