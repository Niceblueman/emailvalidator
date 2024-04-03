/* eslint-disable eqeqeq */
import React, { useEffect, useState } from "react"
import { useTranslation } from "react-i18next"
import styled from "styled-components"
import { Client, getCookie } from "../../api/client";
import { toast } from 'react-toastify';

const SwiperPaymentPageMobile = ({ setLogin }) => {
    const { t } = useTranslation();
    const [plans, setPlan] = useState();
    // AddEdit, Delete
    const add_to_basket = (id, plan) => {
        new Promise((resolve, reject) => {
            Client.post("/myplans", {
                name: plan?.name,
                action: "AddEdit",
                plan_count: 1
            }, {
                params: {
                    id
                }
            }).then((res)=>{
                if(res.status==200){
                    resolve(res.data)
                } else {
                    reject(res.data)
                }
            })
            .catch((err)=>{
                reject(err)
            })
        })
    }
    const checkLogin = () => {
        return new Promise((resolve, reject) => {
            let cookie = getCookie("____ads");
            if (!cookie) {
                setLogin({
                    login: true,
                    register: false, 
                    recover: false
                })
                resolve("ok")
            }
            if (!cookie.split(":")[1]) {
                setLogin({
                    login: true,
                    register: false, 
                    recover: false
                })
                resolve("ok")
            }
            resolve(cookie.split(":")[1])
        })
    }
    useEffect(() => {
        Client.post("/plans").then((res) => {
            if (res.status == 200) {
                setPlan(res.data)
            }
        })
    }, [])
    return (
        plans && <Container id="second_payment_seg">
            <PaymentItem>
                <PriceContainer>
                    <Title>
                        {t(plans?.demo?.name || 'Demo')}
                    </Title>
                    <PriceLine>
                        <Price>{t('currency')}{plans?.demo?.price}</Price>
                        <Period>/ {t('monthly')}</Period>
                    </PriceLine>
                    <UnderPrice>
                        {t('under-price-free-plan')}
                    </UnderPrice>
                    <Options>
                        <ApproveIcon src="./icons/Approve.svg" />
                        <OptionsTxt>{plans?.demo?.requests_per_day} {t('API-request')}</OptionsTxt>
                    </Options>
                    <Options>
                        <ApproveIcon src="./icons/Approve.svg" />
                        <OptionsTxt>{t('validations-tools')}</OptionsTxt>
                    </Options>
                </PriceContainer>
                <Btn onClick={() => {
                    checkLogin().then((id) => {
                        if(add_to_basket){
                            add_to_basket(id, plans?.demo)?.then((res)=>{
                                toast.success(t("added_plan"), {
                                    position: "bottom-right",
                                    autoClose: 5000,
                                    hideProgressBar: false,
                                    closeOnClick: true,
                                    pauseOnHover: true,
                                    draggable: true,
                                    progress: undefined,
                                });
                            })
                        }
                    })
                }} >{t('more')}</Btn>
            </PaymentItem>
            <PaymentItem>
                <PriceContainer>
                    <Title>
                        {t('basicPlan')}
                    </Title>

                    <PriceLine>
                        <Price>{t('currency')}{plans?.starter?.price}</Price>
                        <Period>/ {t('monthly')}</Period>
                    </PriceLine>
                    <UnderPrice>
                        {t('under-price-basic-plan')}
                    </UnderPrice>
                    <Options>
                        <ApproveIcon src="./icons/Approve.svg" />
                        <OptionsTxt>{plans?.starter?.requests_per_day} {t('API-request')}</OptionsTxt>
                    </Options>
                    <Options>
                        <ApproveIcon src="./icons/Approve.svg" />
                        <OptionsTxt>{t('validations-tools')}</OptionsTxt>
                    </Options>
                    <Options>
                        <ApproveIcon src="./icons/Approve.svg" />
                        <OptionsTxt>{t('secure')}</OptionsTxt>
                    </Options>
                </PriceContainer>
                <Btn onClick={() => {
                    checkLogin().then((id) => {
                        if(add_to_basket){
                            add_to_basket(id, plans?.starter)?.then((res)=>{
                                toast.success(t("added_plan"), {
                                    position: "bottom-right",
                                    autoClose: 5000,
                                    hideProgressBar: false,
                                    closeOnClick: true,
                                    pauseOnHover: true,
                                    draggable: true,
                                    progress: undefined,
                                });
                            })
                        }
                    })
                }}>{t('more')}</Btn>
            </PaymentItem>
            <PaymentItem>
                <PriceContainer>
                    <Title>
                        {t('proPlan')}
                    </Title>

                    <PriceLine>
                        <Price>{t('currency')}{plans?.premium?.price}</Price>
                        <Period>/ {t('monthly')}</Period>
                    </PriceLine>
                    <UnderPrice>
                        {t('under-price-basic-plan')}
                    </UnderPrice>
                    <Options>
                        <ApproveIcon src="./icons/Approve.svg" />
                        <OptionsTxt>{plans?.premium?.requests_per_day} {t('API-request')}</OptionsTxt>
                    </Options>
                    <Options>
                        <ApproveIcon src="./icons/Approve.svg" />
                        <OptionsTxt>{t('validations-tools')}</OptionsTxt>
                    </Options>
                    <Options>
                        <ApproveIcon src="./icons/Approve.svg" />
                        <OptionsTxt>{t('secure')}</OptionsTxt>
                    </Options>
                </PriceContainer>
                <Btn onClick={() => {
                    checkLogin().then((id) => {
                        if(add_to_basket){
                            add_to_basket(id, plans?.premium)?.then((res)=>{
                                toast.success(t("added_plan"), {
                                    position: "bottom-right",
                                    autoClose: 5000,
                                    hideProgressBar: false,
                                    closeOnClick: true,
                                    pauseOnHover: true,
                                    draggable: true,
                                    progress: undefined,
                                });
                            })
                        }
                    })
                }}>{t('more')}</Btn>
            </PaymentItem>
        </Container>
    )
}

export default SwiperPaymentPageMobile

const Container = styled.div`
    min-height: 100vh;
    display: none;
    flex-direction: row;
    align-items: center;
    justify-content: space-between;    
    @media screen and (max-width: 767px) {
        gap: 28px;
        flex-direction: column;
        margin-bottom:3em;
    }
`

const PaymentItem = styled.div`
    width: 270px;
    height: 550px;
    padding: 30px 15px;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: space-between;
    box-shadow: 4px 4px 20px 0 rgba(0, 0, 0, 0.15), -4px -4px 20px 0 white;
    border-radius: 5px;
    background-color: var(--BackgroundColor);
    margin: 0px 15px;
    z-index: 20;


    @media screen and (max-width: 767px) {
        /* height: 300px; */
        height: auto;
    }

`

const Title = styled.div`
    font-family: 'GothamPro';
    font-weight: bold;
    font-size: 16px;
    line-height: 24px;
    text-align: center;
    color: var(--Blue);
    text-transform: uppercase;
`
const PriceContainer = styled.div`
    width: 100%;
    height: fit-content;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: flex-start;
`

const PriceLine = styled.div`
    width: 100%;
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: center;
    margin-top: 50px;
`

const Price = styled.div`
    font-family: 'GothamPro';
    font-weight: bold;
    font-size: 36px;
    line-height: 34px;
    text-transform: uppercase;
    color: var(--TextColor);
`

const Period = styled.div`
    font-family: 'GothamPro';
    font-weight: bold;
    font-size: 20px;
    line-height: 19px;
    color: var(--TextColor);
    margin: 0px 5px;
`

const UnderPrice = styled.div`
    font-family: 'GothamPro';
    font-weight: 400;
    font-size: 12px;
    line-height: 24px;
    color: var(--TextColor);
    margin: 5px 0px  20px 0px;
`

const Options = styled.div`
    width: 100%;
    height: 55px;
    display: flex;
    flex-direction: column;
    align-items: center;
    box-shadow: inset 4px 4px 5px 0 rgba(0, 0, 0, 0.15), inset -4px -4px 5px 0 white;
    border-radius: 5px;
    margin: 10px 0px;
    padding: 10px 0px;
`
const ApproveIcon = styled.img`
    width: 24px;
    height: 24px;
`

const OptionsTxt = styled.div`
    font-family: 'GothamPro';
    font-size: 14px;
    line-height: 13px;
    margin-top: 10px;
    color: var(--TextColor);
`

const Btn = styled.button`
    width: 170px;
    height: 45px;
    border: 1px solid var(--Blue);
    border-radius: 5px;
    padding: 15px 25px 15px 25px;
    font-family: 'GothamPro';
    font-size: 14px;
    transform: scale(1);
    cursor: pointer;
    transition: all ease-in-out 100ms ;
    :hover{
        transform: scale(1.01);
    }
    :active{
        transform: scale(.99);
    }
    font-weight: 400;
    background-color: unset;
    color: var(--TextColor);
    /* margin-top: 100px; */
`
