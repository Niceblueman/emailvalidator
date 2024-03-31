/* eslint-disable no-unused-expressions */
/* eslint-disable no-restricted-globals */
/* eslint-disable no-unused-vars */
/* eslint-disable eqeqeq */
/* eslint-disable jsx-a11y/anchor-is-valid */
/* eslint-disable react/jsx-no-comment-textnodes */
/* eslint-disable no-useless-escape */
/* eslint-disable react-hooks/exhaustive-deps */
import React, { useState } from "react"
// import { animated, useSpring } from '@react-spring/web';
// import useLogin from "../hooks/login";
import styled from "styled-components";
import { useTranslation } from "react-i18next"
import { Client, setCookie } from "../api/client";
import { toast } from 'react-toastify';

export default ({ login, register, recover, setLogin, setHeader }) => {
    let re = new RegExp(/^(([^<>()\[\]\\.,;:\s@"]+(\.[^<>()\[\]\\.,;:\s@"]+)*)|(".+"))@((\[[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}])|(([a-zA-Z\-0-9]+\.)+[a-zA-Z]{2,}))$/)
    const [error, setError] = useState("")
    const { t } = useTranslation();
    const [logindata, setLogindata] = useState({
        username: "",
        password: ""
    })
    const [recoverdata, setRecoverdata] = useState({
        email: "",
    })
    const [policy, setPolicy] = useState(false);
    const [registerdata, setRegisterdata] = useState({
        username: "",
        password: "",
        passwordAgain: "",
        email: ""
    })
    const sendrecovery = () => {
        if (!re.test(recoverdata.email)) return setError(t("please_enter") + ": " + t("email"))
        Client.post('/recover', JSON.stringify(recoverdata), {
            timeout: 3500,
        }).then(async (res) => {
            if (res.data.success) {
                toast.success(t(res.data.msg), {
                    position: "bottom-right",
                    autoClose: 5000,
                    hideProgressBar: false,
                    closeOnClick: true,
                    pauseOnHover: true,
                    draggable: true,
                    progress: undefined,
                });
                setLogin({ login: false, register: false, recover: false })
                // setTimeout(() => {
                //     setHeader(true)
                // }, 1000);
            } else {
                setError(res.data.msg)
            }
        })
        .catch((err)=>{
            console.log(err);
        })
    }
    const sendlogin = () => {
        if (!logindata.password) return setError(t("please_enter") + ": " + t("password"))
        if (!logindata.username) return setError(t("please_enter") + ": " + t("username"))
        if (logindata.username.search("@") != -1 && !re.test(logindata.username)) return setError(t("please_enter") + ": " + t("username"))
        Client.post('/login', JSON.stringify(logindata), {
            timeout: 1500,
        }).then(async (res) => {
            console.log(res.data);
            if (res.data.success) {
                setCookie("____ads", res.data.id, 'session')
                toast.success(res.data.reason, {
                    position: "bottom-right",
                    autoClose: 5000,
                    hideProgressBar: false,
                    closeOnClick: true,
                    pauseOnHover: true,
                    draggable: true,
                    progress: undefined,
                });
                setLogin({ login: false, register: false, recover: false })
                document.location.reload()
                // setTimeout(() => {
                //     setHeader(true)
                // }, 1000);
            } else {
                setError(res.data.msg)
            }
        })
    }
    const sendRegister = () => {
        if (!registerdata.password) return setError(t("please_enter") + ": " + t("password"))
        if (!registerdata.username) return setError(t("please_enter") + ": " + t("username0"))
        if (!policy) return setError(t("please_enter") + ": " + t("checkbox"))
        if (!re.test(registerdata.email)) return setError(t("please_enter") + ": " + t("email"))
        if (registerdata.passwordAgain != registerdata.password) return setError(t("error_pass_diff"))
        var reg = registerdata;
        delete reg.passwordAgain
        // setRegisterdata(s => ({ ...s, passwordAgain: "" }))
        Client.post('/register', JSON.stringify(reg), {
            timeout: 1500,
        }).then(async (res) => {
            if (res.data?.success) {
                toast.success(res.data?.msg, {
                    position: "bottom-right",
                    autoClose: 5000,
                    hideProgressBar: false,
                    closeOnClick: true,
                    pauseOnHover: true,
                    draggable: true,
                    progress: undefined,
                });
                setCookie("____ads", res.data.id, 30)
                setLogin({ login: false, register: false, recover: false })
                // setTimeout(() => {
                //     setHeader(true)
                // }, 1000);
            } else {
                setError(t(res.data.msg))
            }
        })
    }
    // const animating = useSpring({
    //     transform: `scale: ${login ? 1 : 0.89}`,
    //     config: {
    //         mass: 5,
    //         friction: 120,
    //         tension: 120
    //     }
    // })
    const switch_to = function () {
        setLogin(s => ({ register: !s.register, login: !s.login, recover: false }))
    }
    return (
        (login || register || recover) &&
        <Wrapper >
            {login && <LoginModel data-aos="fade-up"
                data-aos-anchor-placement="top-bottom">
                <Close onClick={() => setLogin(s => ({ register: false, login: false, recover: false }))}>
                    x
                </Close>
                <LoginContent>
                    <Title>{t("enter")}:</Title>
                    <Welcome>
                        {t("welcome")}
                    </Welcome>
                    <LoginUsername onChange={(ev) => {
                        setError("")
                        ev.preventDefault()
                        setLogindata((s) => ({
                            ...s,
                            username: ev.target.value
                        }))
                    }} label={"username"} type="text" placeholder={t("username")} value={logindata.username} />
                    <LoginPassword onChange={(ev) => {
                        setError("")
                        ev.preventDefault()
                        setLogindata((s) => ({
                            ...s,
                            password: ev.target.value
                        }))
                    }} type="password" placeholder={t("password")} value={logindata.password} />
                    {
                        error &&
                        <LoginError>{t(error)} <a onClick={() => setError("")}>x</a></LoginError>
                    }
                    {/* <Terms>
                        <input type={"checkbox"} checked={policy} onChange={(ev) => {
                            ev.preventDefault()
                            setPolicy(ev.target.checked)
                        }} />
                        <p>{t("terms")}<kbd className="tooltp">{t("policy")}</kbd></p>
                    </Terms> */}
                    <LoginBtn onClick={sendlogin} > {t("enter")} </LoginBtn>
                </LoginContent>
                <LoginFooter>
                    <NoAccountYet onClick={switch_to}>{t("still_not_registred")}</NoAccountYet>
                    <NoAccountYet onClick={()=>setLogin((s)=>({login:false, register:false, recover:true}))}>{t("lost_password")}</NoAccountYet>
                </LoginFooter>
            </LoginModel>}
            {recover && <LoginModel data-aos="fade-up"
                data-aos-anchor-placement="top-bottom">
                <Close onClick={() => setLogin(s => ({ register: false, login: false, recover: false }))}>
                    x
                </Close>
                <LoginContent>
                    <Title>{t("enter")}:</Title>
                    <Welcome>
                        {t("recovery")}
                    </Welcome>
                    <LoginUsername onChange={(ev) => {
                        setError("")
                        ev.preventDefault()
                        setRecoverdata((s) => ({
                            email: ev.target.value
                        }))
                    }} label={"Email"} type="text" placeholder={t("email")} value={recoverdata.email} />
                    {
                        error &&
                        <LoginError>{t(error)} <a onClick={() => setError("")}>x</a></LoginError>
                    }
                    {/* <Terms>
                        <input type={"checkbox"} checked={policy} onChange={(ev) => {
                            ev.preventDefault()
                            setPolicy(ev.target.checked)
                        }} />
                        <p>{t("terms")}<kbd className="tooltp">{t("policy")}</kbd></p>
                    </Terms> */}
                    <LoginBtn onClick={sendrecovery} > {t("Send")} </LoginBtn>
                </LoginContent>
                <LoginFooter>
                    <NoAccountYet onClick={switch_to}>{t("still_not_registred")}</NoAccountYet>
                </LoginFooter>
            </LoginModel>}
            {register && <LoginModel data-aos="fade-up"
                data-aos-anchor-placement="top-bottom">
                <Close onClick={() => setLogin({ register: false, login: false, recover: false })}>
                    x
                </Close>
                <LoginContent>
                    <Title>{t("register")}:</Title>
                    <Welcome>
                        {t("register_notice")}
                    </Welcome>
                    <LoginUsername onChange={(ev) => {
                        ev.preventDefault()
                        setError("")
                        setRegisterdata((s) => ({
                            ...s,
                            username: ev.target.value
                        }))
                    }} label={t("username")} type="text" placeholder={t("username")} value={registerdata.username} />
                    <LoginUsername onChange={(ev) => {
                        ev.preventDefault()
                        setError("")
                        setRegisterdata((s) => ({
                            ...s,
                            email: ev.target.value
                        }))
                    }} label={t("email")} type="email" placeholder={t("email")} value={registerdata.email} />
                    <LoginPassword onChange={(ev) => {
                        ev.preventDefault()
                        setRegisterdata((s) => ({
                            ...s,
                            password: ev.target.value
                        }))
                        setError("")
                    }} type="password" placeholder={t("password")} value={registerdata.password} />
                    <LoginPassword onChange={(ev) => {
                        ev.preventDefault()
                        setRegisterdata((s) => ({
                            ...s,
                            passwordAgain: ev.target.value
                        }))
                        setError("")
                    }} type="password" placeholder={t("password")} value={registerdata.passwordAgain} />
                    {
                        error &&
                        <LoginError>{t(error)} <a onClick={() => setError("")}>x</a></LoginError>
                    }
                    <Terms>
                        <input type={"checkbox"} checked={policy} onChange={(ev) => {
                            // ev.preventDefault()
                            setPolicy(ev.target.checked)
                        }} />
                        <p>{t("terms")}<kbd className="tooltp">{t("policy")}</kbd></p>
                    </Terms>
                    <LoginBtn onClick={sendRegister} > {t("continue")} </LoginBtn>
                </LoginContent>
                <LoginFooter>
                    <NoAccountYet onClick={switch_to}>{t("enter")}</NoAccountYet>
                </LoginFooter>
            </LoginModel>}
        </Wrapper>
    )
}
const Terms = styled.div`
    max-width: 346px;
    display: flex;
    flex-direction: row;
    font-size: 12px;
    justify-content: space-around;
    gap: 15px;
    margin: 10px 0px;
    p {
        text-decoration: underline;
        cursor: pointer;
        .tooltp {
        visibility: hidden;
        position: absolute;
        z-index: 1000;
        height: 409px;
        width: 500px;
        overflow: scroll;
        top: 0;
        left: 100px;
        padding: 10px;
        background-color: #12121287;
        color: #fff;
        border-radius: 5px;
        }
        &:hover{
            .tooltp {
            visibility: visible;
            }
        }
    }
    
`;


const Title = styled.h3`
    width: 100%;
    text-align: start;
    max-width: 360px;
    font-weight: bold;
    font-size: xx-large;
`;


const Welcome = styled.p`
    width: 100%;
    text-align: start;
    font-weight: bold;
    font-size: 15px;
    max-width: 360px;
    text-align-last: left;
    text-align: justify;
    text-justify: distribute;
    word-break: break-all;
    word-spacing: 2px;
    word-wrap: break-word;
    hyphens: auto;
`;

const LoginError = styled.span`
        background-color: #fee8e8;
        border-left: 2px solid #b00;
        padding: 0.5em 4em;
        color: #000;
        font-size: 12px;
        font-family: 'GothamPro';
        border-radius: 3px;
        margin: 1em 0em;
        position: relative;
        user-select: none;
        display: flex;
        align-items: center;
        max-width: 240px;
        text-align: start;
        font-weight: bolder;
     a {
        font-size: 15px;
        font-family: 'GothamPro';
        position: absolute;
        right: 10px;
        cursor: pointer;
    }
`;


const Close = styled.div`
    width: 30px;
    height: 30px;
    border-radius: 50%;
    color: #fff;
    background-color: red;
    position:absolute;
    top:10px;
    right:10px;
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
const LoginBtn = styled.button`
    border: unset;
    padding: 1em 6em;
    border-radius: 6px;
    font-size: 15px;
    cursor: pointer;
`;


const NoAccountYet = styled.button`
    width: 200px;
    border-radius: 20px;
    background: linear-gradient(101.82deg, #439AFF 0%, #0068E1 100%);
    border:unset;
    outline:unset;
    padding:7px 10px;
    color: #fff;
    margin: 1em 0px;
    cursor: pointer;
    transform: scale(1);
    transition: transform  ease-in-out 100ms ;
    :active{
        transform: scale(.98)
    }
`;
const LoginPassword = styled.input`
    width: 335px;
    height: 65px;
    padding-left: 30px;
    background: white;
    border: unset;
    z-index: 10;
    margin: 6px 0px;
    border-radius: 5px;
    box-shadow: inset 1px 3px 6px 0 rgba(0,0,0,0.15),inset -4px -4px 1px 0px white;    font-family: 'GothamPro';
    font-size: 14px;
    line-height: 14px;
    font-size:  1em;
    &::placeholder {
        font-family: 'GothamPro';
        font-size: 14px;
        line-height: 14px;
    }

    &:focus {
        outline: unset;
    }

    /* @media screen and (max-width: 767px) {
        width: calc(100% - 12px);
    } */
`;
const LoginUsername = styled.input`
    width: 335px;
    height: 65px;
    padding-left: 30px;
    background: white;
    border: unset;
    z-index: 10;
    margin: 6px 0px;
    border-radius: 5px;
    box-shadow: inset 1px 3px 6px 0 rgba(0,0,0,0.15),inset -4px -4px 1px 0px white;    font-family: 'GothamPro';
    font-size: 14px;
    line-height: 14px;
    font-size:  1em;
    &::placeholder {
        font-family: 'GothamPro';
        font-size: 14px;
        line-height: 14px;
    }

    &:focus {
        outline: unset;
    }
`;
const LoginFooter = styled.div`
    flex:.2;
    display:flex;
    flex-direction:column;
    justify-content:center;
    align-items:center;
`;
const LoginContent = styled.div`
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    flex:1;
    max-height: 100%;
    overflow-x: hidden;
    overflow-y: scroll;
`;
const Wrapper = styled.div`
  position: fixed;
  width: 100vw;
  height: 100vh;
  overflow: hidden;
  top: 0;
  background: #1e1e1e99;
  z-index: 9900;
  display: flex;
  justify-content: center;
  align-items: center;
  overflow-x: hidden;
  overflow-y: scroll;
`;


const LoginModel = styled.div`
        width: 99%;
        max-width: fit-content;
        max-height: fit-content;
        padding: 0.2em 3em;
        background: #fff;
        min-width: 248px;
        position:relative;
        border-radius: 1%;
        display: flex;
        flex-Direction: column;
        justify-content: space-around;
        align-items: center;
        @media screen  and (max-height: 567px){
        margin-top: 40%;
        }
        @media screen  and (max-height: 767px){
        margin-top: 25%;
        margin-bottom: 10%;
        };

