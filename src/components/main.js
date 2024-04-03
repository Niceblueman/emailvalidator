/* eslint-disable no-useless-escape */
/* eslint-disable eqeqeq */
/* eslint-disable no-unused-expressions */
/* eslint-disable no-restricted-globals */
/* eslint-disable no-unused-vars */
import { useTranslation } from "react-i18next";
import styled from "styled-components";
import ReactJson from 'react-json-view'
import React, { useState } from "react";
import { toast } from 'react-toastify';
import { Client, uuid } from "../api/client";
import { Dna } from "react-loader-spinner";
import { Status } from "./status";
const Row = styled.div`
    display:flex;
    flex-direction:row;
    justify-content:space-between;
    align-items:center;
    gap: 10px;
    margin: 10px 0px;
    width: 100%;
`;

const Main = () => {
    const { t } = useTranslation();
    const [results, setResults] = useState({
        "email": "email@example.com",
        "email_syntax_valid": "Boolean",
        "mx_records": [
          {
            "host": "mx.example.com.",
            "preference": "Number"
          }
        ],
        "restricted": "Boolean",
        "smtp_can_catch_all": "Boolean",
        "smtp_deliverable": "Boolean",
        "smtp_disabled": "Boolean",
        "smtp_has_full_inbox": "Boolean",
        "smtp_live": "Boolean",
        "status": "Safe|Invalid|Unknown",
        "temporary_email": "Boolean"
      })
    const [email, setEmail] = useState("")
    const [isloading, setIsloading] = useState(false)
    const checkEmail = () => {
        let re = new RegExp(/^(([^<>()\[\]\\.,;:\s@"]+(\.[^<>()\[\]\\.,;:\s@"]+)*)|(".+"))@((\[[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}])|(([a-zA-Z\-0-9]+\.)+[a-zA-Z]{2,}))$/)
        if (!email || !re.test(email)) return toast.error(t("enter_email_to_check"), {
            position: "bottom-right",
            autoClose: 1000,
            hideProgressBar: false,
            closeOnClick: true,
            pauseOnHover: true,
            draggable: true,
            progress: undefined,
        });
        setIsloading(true)
        let data = JSON.stringify({
            "from_email": "no-reply@accounts.google.com",
            "to_email": email
        });
        Client.post("/demoCheck", data, {
            params: {
                uuid: uuid(),
            },
            timeout: 2000
        }).then((res) => {
            setIsloading(false)
            if (res.status) {
                setResults(res?.data?.data)
                toast.success(<p>{t("notice")}<br></br>{"🪺"+t("still")+":"}<strong>{50 - Number(res?.data?.free_request||0)}</strong></p>, {
                    position: "bottom-right",
                    autoClose: 5000,
                    style:{
                        fontSize:13
                    },
                    hideProgressBar: true,
                    closeOnClick: true,
                    pauseOnHover: true,
                    draggable: true,
                    progress: undefined,
                })
            } else {
                toast.error(t("exeeeded"), {
                    position: "bottom-right",
                    autoClose: 1000,
                    hideProgressBar: false,
                    closeOnClick: true,
                    pauseOnHover: true,
                    draggable: true,
                    progress: undefined,
                })
            }
        }).catch((res) => {
            setIsloading(false)
            toast.error(t("server_under_const"), {
                position: "bottom-right",
                autoClose: 1000,
                hideProgressBar: false,
                closeOnClick: true,
                pauseOnHover: true,
                draggable: true,
                progress: undefined,
            })
        })
    }
    return (
        <Wrapper>
            <BackgroundLeft src="./Background/left-top.svg" />
            <BackgroundRight src="./Background/right-top.svg" />
            <Container data-aos="fade-up"
                data-aos-anchor-placement="top-bottom">
                <Title>
                    {t('main-title')}
                </Title>
                <SubTitle>
                    {t('main-subtitle')}
                </SubTitle>
                <InputLine>
                    <Input type='email' onChange={(ev) => {
                        ev.preventDefault()
                        setEmail(ev?.target.value)
                    }} placeholder={`${t('enter-email')}`} value={email} />
                    <CheckBtn type="button" 
                        onKeyDown={(ev)=>{
                            if(ev.keyCode == 13){
                                checkEmail
                            }
                        }}
                        onClick={checkEmail}>{isloading ? <Dna
                        visible={true}
                        height="70"
                        width="70"
                        ariaLabel="dna-loading"
                        wrapperStyle={{}}
                        wrapperClass="dna-wrapper"
                    /> : t('check')}</CheckBtn>
                </InputLine>
                <Result>
                    <ResultContainer>
                        <ResultTitle>{t('result')}</ResultTitle>
                        <LeftSideContent>
                            <LeftSideData>
                                <Row>
                                    <LeftSideRow>E-mail</LeftSideRow>
                                    <LeftSideRow style={{ cursor:"pointer" }} className="btn" data-clipboard-text={results?.email} >{results?.email ? results?.email :<Status width={24} status={""} />}</LeftSideRow>
                                </Row>
                                <Row>
                                    <LeftSideRow>Status</LeftSideRow>
                                    <LeftSideRow><Status width={24} status={results?.status} /></LeftSideRow>
                                </Row>
                                <Row>
                                    <LeftSideRow>Valid Format</LeftSideRow>
                                    <LeftSideRow><Status width={24} status={results && 'email_syntax_valid' in results && results.email_syntax_valid != "Boolean" ? results.email_syntax_valid ? "Safe" : "Invalid" : ""} /></LeftSideRow>
                                </Row>
                                <Row>
                                    <LeftSideRow>SMTP</LeftSideRow>
                                    <LeftSideRow><Status width={24} status={results && 'smtp_live' in results && results.smtp_live != "Boolean" ?  results.smtp_live ?  "Safe" : "Invalid" : ""} /></LeftSideRow>
                                </Row>
                                <Row>
                                    <LeftSideRow>Restricted</LeftSideRow>
                                    <LeftSideRow><Status width={24} status={results &&'restricted' in results && results.restricted != "Boolean" ?  !results.restricted ?  "Safe" : "Risky" : ""} /></LeftSideRow>
                                </Row>
                                <Row>
                                    <LeftSideRow>Disposable</LeftSideRow>
                                    <LeftSideRow><Status width={24} status={results && 'temporary_email' in results && results.temporary_email != "Boolean" ? !results.temporary_email ?  "Safe" : "Invalid" : ""} /></LeftSideRow>
                                </Row>
                            </LeftSideData>
                        </LeftSideContent>
                    </ResultContainer>
                    <Separator />
                    <ResultContainer>
                        <ResultTitleJson>JSON</ResultTitleJson>
                        <ReactJson style={{
                            position: "absolute",
                            width: "100%",
                            top: 0,
                            left: 4,
                            padding: "2em 24px 0 24px",
                            overflow: "scroll",
                            maxHeight: "297px",
                        }} src={results} iconStyle="circle" displayDataTypes={false} displayObjectSize={false} />
                    </ResultContainer>
                </Result>
            </Container>
        </Wrapper>
    )
}

export default Main;

const Wrapper = styled.div`
    min-height: 100vh;
    width: 100%;
    background-color: var(--BackgroundColor);
    margin: 0px auto;
    box-shadow: inset 4px 4px 20px 0 rgba(0, 0, 0, 0.15), inset -4px -4px 20px 0 white;
    /* position: absolute; */
`

const Container = styled.div`
    width: 100%;
    max-width: 1000px;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: flex-start;
    margin: 100px auto 0px auto;

    @media screen and (max-width: 767px) {
        margin: 60px auto 0px auto;
        width: calc(100% - 32px);
    }
`

const BackgroundLeft = styled.img`
    position: absolute;
    top: 0;
    left: 0;
    z-index: 0;
`

const BackgroundRight = styled.img`
    position: absolute;
    top: 90px;
    right: 0;
    z-index: 0;

    @media screen and (max-width: 767px) {
        display: none;
    }
`

const Title = styled.div`
    position: relative;
    font-family: 'GothamProBold';
    font-size: 36px;
    line-height: 50px;
    text-align: center;
    margin: 100px auto 15px auto;
    color: var(--TextColor);
    max-width: 850px;

    @media screen and (max-width: 767px) {
        margin: 40px auto 15px auto;    
        font-size: 30px;
        line-height: 35px;
    }
`

const SubTitle = styled.div`
    max-width: 450px;
    font-family: 'GothamPro';
    font-size: 18px;
    line-height: 24px;
    text-align: center;
    color: var(--TextColor);
    margin: 15px auto;
    z-index: 10;

    @media screen and (max-width: 767px) {
        font-size: 14px;
        line-height: 20px;
    }
`

const InputLine = styled.div`
    margin: 10px auto 35px auto;
    height: 65px;
    width: fit-content;
    display: flex;
    flex-direction: row;
    align-items: center;

    @media screen and (max-width: 767px){
        flex-direction: column;
        height: fit-content;
        width: 100%;
    }
`

const Input = styled.input`
    width: 335px;
    height: 65px;
    padding-left: 30px;
    background: white;
    border: unset;
    z-index: 10;
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

    @media screen and (max-width: 767px) {
        width: calc(100% - 32px);
    }
`

const CheckBtn = styled.button`
    width: 155px;
    height: 65px;
    font-family: 'GothamProBold';
    font-weight: 100;
    font-size: 14px;
    line-height: 14px;
    background-color: #FF6C6C;
    margin: 0px 15px;
    border-radius: 5px;
    color: white;
    z-index: 10;
    cursor: pointer;

    @media screen and (max-width: 767px){
        margin: 20px 0px;
    }
`

const Result = styled.div`
    width: 85%;
    height: fit-content; 
    padding: 30px;
    margin: 35px auto;
    max-width: 820px;
    box-shadow: inset 4px 4px 20px 0 rgba(0, 0, 0, 0.15), inset -4px -4px 20px 0 white;
    border-radius: 5px;
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: space-between;

    @media screen and (max-width: 767px) {
        flex-direction: column;
        width: calc(100%);
        padding: 30px 0px;  
    }
`

const ResultContainer = styled.div`
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: flex-start;
    height: 320px;
    width: 45%;
    position: relative;
    height: fit-content;
    min-height: 329px;
    overflow: hidden;
    @media screen and (max-width: 767px) {
        width: 100%;
        height: fit-content;
    }
`

const Separator = styled.div`
    height: 320px;
    width: 10px;
    border-radius: 5px;
    box-shadow: -2px -2px 12px #FFFFFF, 2px 2px 12px rgba(0, 0, 0, 0.2);

    @media screen and (max-width: 767px) {
        /* transform: rotate(90deg); */
        height: 10px;
        width: calc(100% - 32px);
        margin: 20px 0px;
        /* display: none; */
    }
`

const ResultTitle = styled.div`
    padding: 4px 11px;
    display: flex;
    flex-direction: row;
    justify-content: center;
    align-items: center;
    border: 1px solid var(--Blue);
    font-family: 'GothamPro';
    font-weight: 400;
    font-size: 16px;
    line-height: 15px;
    color: var(--TextColor);
    margin: 0px;
    border-radius: 5px;
    position: absolute;
    top: 0;
    left: 10px;
`
const ResultTitleJson = styled.div`
    padding: 4px 11px;
    display: flex;
    flex-direction: row;
    justify-content: center;
    align-items: center;
    border: 1px solid var(--Blue);
    font-family: 'GothamPro';
    font-weight: 400;
    font-size: 16px;
    line-height: 15px;
    color: var(--TextColor);
    margin: 0px;
    border-radius: 5px;
    position: absolute;
    top: 0;
    right: 10px;
`

const LeftSideContent = styled.div`
    width: 100%;
    display: flex;
    flex-direction: row;
    align-items: flex-start;
    justify-content: flex-start;
    margin-top: 2em;
    @media screen and (max-width: 767px) {
        align-items: center;
        justify-content: center;
    }
`

const LeftSideData = styled.div`
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    justify-content: flex-start; 
    padding: 2em 26px 0px 26px;
    overflow: scroll;
    width: 100%;
`

const LeftSideRow = styled.div`
    font-family: 'GothamPro';
    font-size: 16px;
    line-height: 24px;
    margin: 0;
    display:flex;
    justify-content:center;
    align-items:center;
`


