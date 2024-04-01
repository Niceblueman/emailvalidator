/* eslint-disable no-unused-vars */
import './App.css';
import DerrivedFooter from './components/Footers/derrivedFooter';
import Footer from './components/Footers/footer';
import Header from './components/header';
import Main from './components/main';
import SwiperPage from './components/SwiperPages/SwiperPage';
import RegisterLogin from './components/RegisterLogin';
import Profile from './components/Profile';
import { useState, useReducer } from 'react';
import { useEffect } from 'react';
import { getCookie } from './api/client';
import { ToastContainer } from 'react-toastify';
import 'react-toastify/dist/ReactToastify.css';
import { CookieNotice } from 'react-cookienotice'
import 'react-cookienotice/dist/style.css'
import { SidePane } from "react-side-pane";
import styled from "styled-components"
import "aos/dist/aos.css";
import AOS from "aos";
import ClipboardJS from "clipboard"
import { toast } from 'react-toastify';
import AppSection from "./components/App.js"
let ntn = new ClipboardJS('.btn');

ntn.on("success", () => {
    toast.success("Copied", {
        position: "bottom-right",
        autoClose: 1000,
        hideProgressBar: false,
        closeOnClick: true,
        pauseOnHover: true,
        draggable: true,
        progress: undefined,
    });
})



AOS.init();

function App() {
    const [profile, setProfile] = useState({})
    const [open, dispatchOpen] = useReducer((prev) => !prev, false);
    const [size, setSize] = useState(40);
    const [Login, setLogin] = useState({
        login: false,
        register: false,
        recover: false
    })
    const [myheader, setHeader] = useState(false)
    useEffect(() => {
        let ads = getCookie("___ads")
        if (ads && ads.length > 20) {
            setHeader(true)
        }
        setSize(window.innerWidth <= 900 ? 90 : 40)
        window.addEventListener("resize", (win, ev) => {
            setSize(window.innerWidth <= 1200 ? 90 : 40)
        })
        return () => {
            window.removeEventListener("resize", (a, v) => null)
        }
    }, [])
    return (
        <>
            <RegisterLogin login={Login.login} recover={Login.recover} setHeader={setHeader} register={Login.register} setLogin={setLogin} />
            <Header _setProfile={setProfile} setOpenPanel={dispatchOpen} setLogin={setLogin} header={myheader} />
            <Main />
            {/* <DerrivedFooter /> */}
            <SwiperPage setLogin={setLogin} id="swiper" data-aos="fade-up"
                data-aos-anchor-placement="top-bottom" />
            {/* <AppSection /> */}
            <Footer data-aos="fade-up"
                data-aos-anchor-placement="top-bottom" />
            <CookieNotice />
            <SidePane
                open={open}
                onClose={dispatchOpen}
                width={size}
            >
                <>
                    <Close onClick={dispatchOpen}>
                        x
                    </Close>
                    <Profile profile={profile} />
                </>
            </SidePane>
            <ToastContainer
                position="bottom-right"
                autoClose={5000}
                hideProgressBar={false}
                newestOnTop={false}
                closeOnClick
                rtl={false}
                pauseOnFocusLoss
                draggable
                pauseOnHover
            />
            {/* <Widget src="https://discord.com/widget?id=1029676804530786305&theme=dark" width="350" height="500" allowtransparency="true" frameborder="0" sandbox="allow-popups allow-popups-to-escape-sandbox allow-same-origin allow-scripts"></Widget> */}
        </>
    );
}

export default App;
const Widget = styled.iframe`
    position:fixed;
    bottom:10px;
    right:20px;
    max-width:250px;
    max-height:400px;
    overflow:hidden;
    outline:unset !important;
    background:unset;
    border:unset;
    z-index: 99;
`;
const Close = styled.div`
    width: 30px;
    height: 30px;
    z-index:99 ;
    border-radius: 50%;
    color: #fff;
    background-color: red;
    position:absolute;
    top:10px;
    left:10px;
    font-family: 'GothamPro';
    display: flex;
    justify-content: center;
    align-items: center;
    font-weight: bold;
    transform: scale(1);
    user-select: none;
    cursor: pointer;
    transition: transform  ease-in-out 100ms ;
    :active{
        transform: scale(.98)
    }
`;
