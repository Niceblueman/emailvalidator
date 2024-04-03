
import { useTranslation } from "react-i18next";
import styled from "styled-components";

const DerrivedFooter = () => {
    const { t } = useTranslation()
    return (
        <Wrappwer id="partners">
            <Container>
                <Btn>{t('trust')}</Btn>
                <LogoRow>
                    <Logo width={50} src="./icons/dup.svg" />
                </LogoRow>
            </Container>
        </Wrappwer>
    )
}
export default (DerrivedFooter);

const Wrappwer = styled.div`
    width: 100%;
    box-shadow:  -4px -4px 10px 0 rgba(0, 0, 0, 0.15),  -4px -4px 10px 0 white;
    background-color: var(--BackgroundColor);
`

const Container = styled.div`
    width: 100%;
    max-width: 1200px;
    margin: 0px auto;
    height: fit-content;
    padding: 37px 0px;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: space-between;

    @media screen and (max-width: 767px) {
        width: calc(100% - 32px);
        padding: 40px 0px 20px 0px;
    }
`
const Btn = styled.button`
    width: 170px;
    height: 45px;
    border: 1px solid var(--Blue);
    border-radius: 5px;
    padding: 15px 25px 15px 25px;
    font-family: 'GothamPro';
    font-size: 14px;
    font-weight: 400;
    background-color: unset;
    color: var(--TextColor);
`

const LogoRow = styled.div`
    margin: 10px 0px;   
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: space-between;

    @media screen and (max-width: 767px) {
        flex-wrap: wrap;
        justify-content: center;

    }

`

const Logo = styled.img`
    width: 120px;
    height: 51px;
    object-fit: contain;
    margin: 0px 15px; 

    @media screen and (max-width: 767px) {
        margin: 20px 20px;
    }
`