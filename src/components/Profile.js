/* eslint-disable import/no-anonymous-default-export */
/* eslint-disable no-unused-expressions */
/* eslint-disable no-restricted-globals */
/* eslint-disable no-unused-vars */
/* eslint-disable eqeqeq */
/* eslint-disable jsx-a11y/anchor-is-valid */
/* eslint-disable react/jsx-no-comment-textnodes */
/* eslint-disable no-useless-escape */
/* eslint-disable react-hooks/exhaustive-deps */
import React from "react"
import { useTranslation } from "react-i18next";
import styled from "styled-components";
import Avatar from "./icons/avatar";
import Logout from "./icons/logout";
import PhoneInput from 'react-phone-input-2'
import 'react-phone-input-2/lib/style.css'
import { useCookies } from "react-cookie";
import { useEffect } from "react";
import { useState } from "react";
import { toast } from 'react-toastify';
import { Client, getCookie } from "../api/client";
import Minus from "./icons/minus";
import Plus from "./icons/plus";
import { useMemo } from "react";
import Settings from "./icons/settings";
import EmptyCart from "./icons/empty";
import ClipboardJS from "clipboard";

function update_plan(plan) {
    if (plan?.user_id != undefined) return new Promise((resolve, reject) => {
        Client.post("/myplans", {
            name: plan?.plan,
            action: plan?.plan_count == 0 ? "Delete" : "AddEdit",
            plan_count: plan?.plan_count,
            id: plan?.id
        }, {
            params: {
                id: plan?.user_id
            }
        }).then((res) => {
            if (res.status == 200) {
                resolve(res.data)
            } else {
                reject(res.data)
            }
        })
            .catch((err) => {
                reject(err)
            })
    })
}

export default ({ profile }) => {
    const { t } = useTranslation()
    const [basket, setBasket] = useState()
    const [plans, setPlan] = useState();
    const [error, setError] = useState();
    const [paymentData, setPaymentData] = useState({
        email: "",
        phone: ""
    });
    const [payment_pop, setPayment_pop] = useState({ status: false, index: 0 });
    const [cookies, setCookie, removeCookie] = useCookies(['____ads']);
    const [policy, setPolicy] = useState(false);
    const [paid, setPaid] = useState({ status: false, plan: {}, index: 0 });
    let re = new RegExp(/^(([^<>()\[\]\\.,;:\s@"]+(\.[^<>()\[\]\\.,;:\s@"]+)*)|(".+"))@((\[[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}])|(([a-zA-Z\-0-9]+\.)+[a-zA-Z]{2,}))$/)

    function Try_pay() {
        if (!paymentData?.phone) return setError("phone")
        if (!paymentData?.email) return setError("email")
        if (!re.test(paymentData?.email)) return setError("email")
        if (!policy) return setError("checkbox")
        Client.post("/payment", {
            id: cookies.____ads?.split(":")[1],
            plan_id: basket[payment_pop?.index]?.id,
            email: paymentData?.email,
            phone: paymentData?.phone
        }, {
            params: {
                fm: cookies.____ads?.split(":")[0]
            }
        }).then((res) => {
            if (res.status == 200 && res.data.success) {
                toast.success(t("Wait_a_second"), {
                    position: "bottom-right",
                    autoClose: 1000,
                    hideProgressBar: false,
                    onClose: () => {
                        window.open(res.data.url, "(mailvalidator.dup.company) payment", "location=0,toolbar=0,menubar=0,resizable=1,width=400,height=600");
                    },
                    closeOnClick: true,
                    pauseOnHover: true,
                    draggable: true,
                    progress: undefined,
                });
            } else {
                toast.error(t("con_not_complete"), {
                    position: "bottom-right",
                    autoClose: 5000,
                    hideProgressBar: false,
                    closeOnClick: true,
                    pauseOnHover: true,
                    draggable: true,
                    progress: undefined,
                });
            }
        })
            .catch((err) => {
                toast.error(t("con_not_complete"), {
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
    const checkLogin = () => {
        return new Promise((resolve, reject) => {
            let cookie = getCookie("____ads");
            resolve(cookie.split(":")[1])
        })
    }
    const logout = () => {
        removeCookie('____ads')
        document.location.reload()
    }
    const GET_LIST = (id) => {
        return new Promise((resolve, reject) => {
            Client.post("/myplans", {
                name: "",
                action: "GetPlans",
                plan_count: 0
            }, {
                params: {
                    id
                }
            }).then((res) => {
                if (res.status == 200) {
                    resolve(res.data)
                } else {
                    reject(res.data)
                }
            })
                .catch((err) => {
                    reject(err)
                })
        })
    }
    useEffect(() => {
        Client.post("/plans").then((res) => {
            if (res.status == 200) {
                setPlan(res.data)
            }
        })
        if (!basket) {
            if (GET_LIST) {
                checkLogin()?.then(id => {
                    GET_LIST(id)?.then((ee) => {
                        if (ee && ee.list) {
                            setBasket(ee.list)
                        } else {
                            setBasket([])
                        }
                    })
                })
            }
        }
    }, [])
    useEffect(() => {
        let el = document.querySelector("#pop-item" + paid.index)
        if (el) {
            el.style.display = paid.status ? "flex" : "none";
            el.style.tansform = paid.status ? "scale(1)" : "scale(.98)";
        }
    }, [paid])
    return (
        <ProfileWrapper>
            <Header data-aos="fade-up"
                data-aos-anchor-placement="top-bottom">
                <LogoutItm onClick={logout}>
                    <Logout width={30} />
                </LogoutItm>
                <Avatr >
                    <Avatar with={40} />
                </Avatr>
                <Infos>
                    <InfoItems>
                        <SubItem>
                            <p>
                                {t("username0")}:
                            </p>
                            <Item>
                                {profile?.username || "....."}
                            </Item>
                        </SubItem>
                        <SubItem>
                            <p>
                                {t("email")}:
                            </p>
                            <Item>
                                {profile?.email || "....."}
                            </Item>
                        </SubItem>
                        <SubItem>
                            <p>
                                {t("customer_id")}:
                            </p>
                            <Item>
                                {profile?.customer_id || "....."}
                            </Item>
                        </SubItem>
                    </InfoItems>
                </Infos>
            </Header>
            {useMemo(() => basket?.length > 0 ? <Items data-aos="fade-up"
                data-aos-anchor-placement="top-bottom">
                {
                    basket?.map((el, i) => (
                        <Product key={i} id={"ancor" + i}>
                            <PanelPaid data-aos="flip-up"
                                data-aos-anchor-placement={"#ancor" + i} id={"pop-item" + i}>
                                <Close style={{
                                    width: 20,
                                    height: 20,
                                    top: 5,
                                    right: 5
                                }}
                                    onClick={() => setPaid({ status: false, plan: {}, index: i })}>
                                    x
                                </Close>
                                <ProductBoodyLeftSetting>
                                    <ProductInfo>
                                        <ProductInfoLeft>
                                            {t("cus_id")}:
                                        </ProductInfoLeft>
                                        <ProductInfoRight>
                                            {el?.cus_id}
                                        </ProductInfoRight>
                                    </ProductInfo>
                                    <ProductInfo>
                                        <ProductInfoLeft>
                                            {t("api-key")}:
                                        </ProductInfoLeft>
                                        <ProductInfoRightCopy id="btn2" data-clipboard-text={el?.key} >
                                            {String(el?.key).slice(-30).padEnd(33, ".")}
                                        </ProductInfoRightCopy>
                                    </ProductInfo>
                                    <ProductInfo>
                                        <ProductInfoLeft>
                                            {t("Until")}:
                                        </ProductInfoLeft>
                                        <ProductInfoRight id={"planitem_" + i}>
                                            {new Date((Number(el?.last_edit || 1665526035) * 1000) + (30 * 24 * 60 * 60 * Number(el?.plan_count || 1) * 1000)).toLocaleString()}
                                        </ProductInfoRight>
                                    </ProductInfo>
                                </ProductBoodyLeftSetting>
                            </PanelPaid>
                            <ProductHeader>
                                <ProductName>
                                    {el?.plan}
                                </ProductName>
                                {(plans[String(el?.plan||"").toLowerCase()]?.price != 0 && !el?.is_paid) && <Pay onClick={() => setPayment_pop({ status: true, index: i })}>
                                    {t("pay")}
                                </Pay>}
                                {
                                    el?.is_paid && <Paid onClick={() => {
                                        setPaid({ status: true, plan: el, index: i })
                                    }}>
                                        <Settings />
                                        {t("paid")}
                                    </Paid>
                                }
                            </ProductHeader>
                            <ProductBoody>
                                <ProductBoodyLeft>
                                    <ProductInfo>
                                        <ProductInfoLeft>
                                            {t("emails_by_request")}:
                                        </ProductInfoLeft>
                                        <ProductInfoRight>
                                            {plans[String(el?.plan).toLowerCase()]?.items_per_req}
                                        </ProductInfoRight>
                                    </ProductInfo>
                                    <ProductInfo>
                                        <ProductInfoLeft>
                                            {t("API-request")}:
                                        </ProductInfoLeft>
                                        <ProductInfoRight>
                                            {plans[String(el?.plan).toLowerCase()]?.requests_per_day}
                                        </ProductInfoRight>
                                    </ProductInfo>
                                    <ProductInfo>
                                        <ProductInfoLeft>
                                            {t("total")}:
                                        </ProductInfoLeft>
                                        <ProductInfoRight id={"planitem_" + i}>
                                            {Number(plans[String(el?.plan).toLowerCase()]?.price) * el?.plan_count}
                                            {t("currency")}
                                        </ProductInfoRight>
                                    </ProductInfo>
                                </ProductBoodyLeft>
                                <ProductBoodyRight>
                                    <ProductBoodyRightTitle>
                                        {t("months")}
                                    </ProductBoodyRightTitle>
                                    {(plans[String(el?.plan).toLowerCase()]?.price != 0 && !el?.is_paid) ? <PlusMinus basketindex={i} price={Number(plans[String(el?.plan).toLowerCase()]?.price)} setBasket={setBasket} count={el?.plan_count} plan={el}></PlusMinus> : "1"}
                                </ProductBoodyRight>
                            </ProductBoody>
                        </Product>
                    ))
                }
            </Items> : <EmptyBasket>
                <EmptyCart width={150} />
                <p>
                    {t("no_items")}
                </p>
            </EmptyBasket>
                , [basket])}
            {payment_pop?.status && <Wrapper>
                <LoginModel data-aos="fade-up"
                    data-aos-anchor-placement="top-bottom">
                    <Close onClick={() => setPayment_pop({ status: false, index: 0 })}>
                        x
                    </Close>
                    <LoginContent>
                        <Title>{t("payment_title")}:</Title>
                        <Welcome>
                            {t("payment_notice")}
                        </Welcome>
                        <LoginUsername onChange={(ev) => {
                            setError("")
                            ev.preventDefault()
                            setPaymentData((s) => ({
                                ...s,
                                email: ev.target.value
                            }))
                        }} label={"email"} type="text" placeholder={t("email")} value={paymentData.email} />
                        <PhoneInput
                            inputStyle={{
                                width: 368,
                                height: 38
                            }}
                            country={'ma'}
                            value={paymentData.phone}
                            onChange={phone => {
                                setPaymentData((s) => ({
                                    ...s,
                                    phone
                                }))
                            }}
                        />

                        {
                            error &&
                            <LoginError>{t(error)} <a onClick={() => setError("")}>x</a></LoginError>
                        }
                        <Terms>
                            <input type={"checkbox"} checked={policy} onChange={(ev) => {
                                ev.preventDefault()
                                setPolicy(ev.target.checked)
                            }} />
                            <p>{t("terms")}<kbd className="tooltp">{t("policy")}</kbd></p>
                        </Terms>
                        <LoginBtn onClick={Try_pay} > {t("continue")} </LoginBtn>
                    </LoginContent>
                </LoginModel>
            </Wrapper>}
        </ProfileWrapper>
    )
}
const PlusMinus = (props) => {
    const [count, setCount] = useState(props?.count || 1)
    const { t } = useTranslation()
    useEffect(() => {
        var i;
        let item_price = document.querySelector("#planitem_" + props?.basketindex);
        setTimeout(() => {
            if (count == i) {
                update_plan({ ...props?.plan, plan_count: count })
                if (item_price?.innerText) {
                    // console.log(`${count*props?.price}${t("currency")}`);
                    item_price.innerText = `${count * props?.price}${t("currency")}`
                }
                if (count != 0) {
                    props?.setBasket(s => {
                        var f = s;
                        f[props?.basketindex].plan_count = count
                        return f
                    })
                } else {
                    props?.setBasket(s => {
                        var f = s;
                        f.filter(s => s?.id != props?.plan?.id)
                        return f
                    })
                }

            }
        }, [])
        i = count;
    }, [count])
    return (
        <PlusMinusWraper>
            <MinusStyled onClick={() => {
                setCount(s => s > 0 ? s - 1 : s)
            }} />
            <p>{count}</p>
            <PlusStyled onClick={() => {
                setCount(s => s < 12 ? s + 1 : s)
            }} />
        </PlusMinusWraper>
    )
}
let ntn2 = new ClipboardJS('#btn2');
ntn2.on("success", (e)=>{
    console.info('Text:', e.text);
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
const LoginBtn = styled.button`
    border: unset;
    padding: 1em 6em;
    border-radius: 6px;
    font-size: 15px;
    cursor: pointer;
`;
const Terms = styled.div`
    max-width: 246px;
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
        width: 250px;
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
const StyledInput = styled(PhoneInput)`
    height: 38px !important;
    width: 368px !important;
`;
const PlusStyled = styled(Plus)`
    transform: scale(1);
    cursor: pointer;
    transition: all ease-in-out 100ms ;
    :hover{
        transform: scale(1.01);
    }
    :active{
        transform: scale(.99);
    }
`;


const MinusStyled = styled(Minus)`
    transform: scale(1);
    cursor: pointer;
    transition: all ease-in-out 100ms ;
    :hover{
        transform: scale(1.01);
    }
    :active{
        transform: scale(.99);
    }
`;


const Pay = styled.div`
    position: absolute;
    width: 95px;
    height: 22px;
    right: 10px;
    top: 3px;
    background: #5ABCF3;
    color: #fff;
    font-size:12px;
    display:flex;
    align-items:center;
    justify-content:center;
    border-radius: 11px;
    transform: scale(1);
    cursor: pointer;
    transition: all ease-in-out 100ms ;
    :hover{
        transform: scale(1.01);
    }
    :active{
        transform: scale(.99);
    }
`;
const Paid = styled.div`
    position: absolute;
    width: 95px;
    height: 22px;
    right: 10px;
    top: 3px;
    background: #3FC500;
    color: #fff;
    font-size:12px;
    display:flex;
    align-items:center;
    justify-content:center;
    display:flex;
    flex-direction:row;
    justify-content:space-around;
    padding:0px 3px;
    align-items:center;
    border-radius: 11px;
    transform: scale(1);
    cursor: pointer;
    transition: all ease-in-out 100ms ;
    :hover{
        transform: scale(1.01);
    }
    :active{
        transform: scale(.99);
    }
`;

const PlusMinusWraper = styled.div`
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    align-items: center;
    padding: 8px 0px;
    gap: 10px;
    isolation: isolate;
    width: 70px;
    height: 25px;
    gap:2px;
    /* Inside auto layout */
    order: 1;
`;


const ProductBoodyRightTitle = styled.div`
    width: 42px;
    height: 12px;

    font-family: 'Inter';
    font-style: normal;
    font-weight: 400;
    font-size: 10px;
    line-height: 12px;

    color: #201F1F;
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
    z-index: 12;
    user-select: none;
    cursor: pointer;
    transition: transform  ease-in-out 100ms ;
    :active{
        transform: scale(.98)
    }
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
const Title = styled.h3`
    width: 100%;
    text-align: start;
    max-width: 360px;
    font-weight: bold;
    font-size: xx-large;
`;
const ProductInfoRight = styled.p`
    width: 81px;
    height: 14px;
    mix-blend-mode: normal;

    /* Inside auto layout */
    font-style: normal;
    font-weight: 700;
    font-size: 10px;
    line-height: 12px;
    flex: none;
    order: 1;
    flex-grow: 0;
    margin: 0px 3px 0px 0px;
    width: auto;
`;
const ProductInfoRightCopy = styled.p`
    width: 81px;
    height: 14px;
    mix-blend-mode: normal;
    background-color:#e2e2e2;
    border-radius:5px;
    overflow:hidden;
    /* Inside auto layout */
    font-style: normal;
    font-weight: 700;
    font-size: 10px;
    line-height: 12px;
    flex: none;
    order: 1;
    padding:0px 7px;
    flex-grow: 0;
    margin: 0px 3px 0px 0px;
    width: auto;
    transform: scale(1);
    transition: all ease-in-out 100ms ;
    :hover{
        transform: scale(.99);
    }
    cursor: pointer;
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
const Wrapper = styled.div`
  position: fixed;
  width: 100%;
  height: 100%;
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
        padding: 1.2em 3em;
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
        }
`;
const LoginUsername = styled.input`
    height: 39px;
    margin: 15px 0px;
    width: 335px;
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
`;
const ProductInfoLeft = styled.p`
    width: 81px;
    height: 14px;

    font-style: normal;
    font-weight: 400;
    font-size: 10px;
    line-height: 12px;
    margin: 0;
    width: auto;
    color: #000000;


    /* Inside auto layout */

    flex: none;
    order: 0;
    flex-grow: 0;
`;


const ProductInfo = styled.div`
    display: flex;
    flex-direction: row;
    align-items: center;
    padding: 0px;
    gap: 1px;

    width: 100%;
    height: 14px;


    /* Inside auto layout */

    flex: none;
    order: 0;
    align-self: stretch;
    flex-grow: 0;
`;


const ProductBoodyRight = styled.div`
   /* Frame 16 */


/* Auto layout */

display: flex;
flex-direction: column;
align-items: center;
padding: 1px 0px;
gap: 6px;

width: auto;
height: 50px;


/* Inside auto layout */

flex: none;
order: 1;
flex-grow: 2;
`;


const ProductBoodyLeft = styled.div`
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    padding: 9px 8px;
    gap: 10px;
    width: 100%;
    position:relative;
    height: 100%;
    flex:10;
    order: 0;
    align-self: stretch;
    flex-grow: 10;
`;
const ProductBoodyLeftSetting = styled.div`
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    padding: 9px 8px;
    gap: 25px;
    width: 100%;
    * {
    font-size:12px;
    }
    position:relative;
    height: 100%;
    flex:10;
    order: 0;
    align-self: stretch;
    flex-grow: 10;
`;


const ProductBoody = styled.div`
    display: flex;
    flex-direction: row;
    align-items: flex-end;
    padding: 5px 0px;
    gap: 10px;
    width: 100%;
    height: auto;
    /* Inside auto layout */
    flex: none;
    order: 1;
    align-self: stretch;
    flex-grow: 1;
`;
const Items = styled.div`
    height: 100%;
    max-height: 80vh;
    width: 100%;
    display: flex;
    flex-direction: row;
    padding: 7px 5px;
    flex-wrap: wrap;
    justify-content: space-around;
    align-items: center;
    overflow-y: scroll;
    gap: 7px;
`;

const ProductName = styled.div`
    position: absolute;
    width: 99px;
    height: 25px;
    left: 0px;
    top: 0px;
    font-weight: 400;
    font-size: 10px;
    line-height: 12px;
    text-align: center;
    display: flex;
    justify-content: center;
    align-items: center;
/* identical to box height */
    background: #9BF299;
    border-radius: 0px 0px 9px 0px;
`;


const ProductHeader = styled.div`
    /* Inside auto layout */
    width:100%;
    height: 25px;
    position:relative;
    flex: none;
    order: 0;
    flex-grow: 0;
`;


const Product = styled.div`
    display: flex;
    font-family: 'GothamPro';
    color: #201F1F;
    overflow:hidden;
    flex-direction: column;
    align-items: flex-start;
    padding: 0px;
    position: relative;
    width: 100%;
    height: 121px;
    background: #FFFFFF;
    border-radius: 6px;
    max-width: 400px;
    transform: scale(1);
    transition: all ease-in-out 100ms ;
    :hover{
        transform: scale(.99);
    }
    * {
        user-select: none;
    }
`;

const PanelPaid = styled.div`
    position:absolute;
    width:100%;
    height:100%;
    /* font-family: 'GothamPro'; */
    display:none;
    z-index:5;
    transition:transform ease-in-out 100ms;
    top:0px;
    left:0px;
    background-color:#fffffffa;
`;
const LogoutItm = styled.div`
    position: absolute;
    top: 5px;
    right: 5px;
    padding: 3px;
    display: flex ;
    justify-content:center ;
    align-items: center;
    cursor: pointer;
`;


const SubItem = styled.div`
    display: flex;
    flex-direction: row;
    flex-wrap: wrap;
    font-weight: 400;
    justify-content: flex-start;
    align-items: center;
`;


const Avatr = styled.div`
  display: flex;
  justify-content: center;
  align-items: center;
  flex: .5;
    padding: 1em;
`;


const Headline = styled.h3`
    padding: unset;
    margin: unset;
`;




const InfoItems = styled.div`
    display: flex;
    flex-direction: column;
    justify-content: space-around;
    gap: 3px;
    padding: 6px 0px;
    p {
        padding: 6px 3px;
        margin: unset;
    }
`;


const InfoHead = styled.div`

`;


const Item = styled.span`
    background-color: #e8e8e8;
    padding: 1px 7px;
    border-radius: 18px;
    user-select: auto !important;
`;


const Infos = styled.div`
    flex: 3;
    display: flex;
    flex-direction: column;
    padding: unset;
    margin: unset;
    justify-content: flex-start;
    align-items: first baseline;
    padding: 2% 2%;
    height: 92%;
    font-size: 12px;
`;


const Header = styled.div`
    align-self:center;
    width: 95%;
    border-radius: 1em;
    background-color: #fff;
    height: 100px ;
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    align-items: center;
    position:relative;
    transform:scale(1);
    transition: all ease-in-out 100ms;
    &:hover{
        transform:scale(.97);
    }
`;


const ProfileWrapper = styled.div`
    position: relative;
    background-color: #d6d6d6;
    padding:unset;
    width:100%;
    height:100%;
    display: flex;
    flex-direction: column;
    justify-content: flex-start;
    padding-top: 3em ;
    * {
        user-select: none;
    }
`;

const EmptyBasket = styled.div`
    height: 30%;
    margin: 3em 0px;
    width: 100%;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
`;
