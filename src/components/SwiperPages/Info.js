import { useTranslation } from "react-i18next"
import styled from "styled-components"

const Info = () => {
    const { t } = useTranslation();
    return (
        <Container>
            <LongBox>
                <TitleLine>
                    <Img src="https://res.cloudinary.com/dxjubrqnd/image/upload/v1663773866/Email-API-Front/validation_jaqcqp.svg" />
                    <Title>{t('validation')}</Title>
                </TitleLine>
                <Txt>
                    {t('validation-txt')}
                </Txt>
            </LongBox>
            <LongBox>
                <TitleLine>
                    <Img src="https://res.cloudinary.com/dxjubrqnd/image/upload/v1663773894/Email-API-Front/SecureAPI_kw5m6f.svg" />
                    <Title>{t('secure-API')}</Title>
                </TitleLine>
                <Txt>
                    {t('secure-API-txt')}
                </Txt>
            </LongBox>
            <LongBox>
                <TitleLine>
                    <Img src="https://res.cloudinary.com/dxjubrqnd/image/upload/v1663773961/Email-API-Front/advancedTools_wjjbi5.svg" />
                    <Title>{t('advanced-tools')}</Title>
                </TitleLine>
                <Txt>
                    {t('advanced-tools-txt')}
                </Txt>
            </LongBox>
            <ShortBox>
                <Img src="https://res.cloudinary.com/dxjubrqnd/image/upload/v1663773961/Email-API-Front/advancedTools_wjjbi5.svg" />
                <ShortBoxTitle>{t('advanced-tools')}</ShortBoxTitle>
                <TxtShortBox>
                    {t('advanced-tools-txt')}
                </TxtShortBox>
            </ShortBox>
            <ShortBox>
                <Img src="./icons/Approve.svg" />
                <ShortBoxTitle>{t('advanced-tools')}</ShortBoxTitle>
                <TxtShortBox>
                    {t('advanced-tools-txt')}
                </TxtShortBox>
            </ShortBox>
            <ShortBox>
                <Img src="https://res.cloudinary.com/dxjubrqnd/image/upload/v1663774765/Email-API-Front/database_bfflgz.svg" />
                <ShortBoxTitle>{t('database')}</ShortBoxTitle>
                <TxtShortBox>
                    {t('database-txt')}
                </TxtShortBox>
            </ShortBox>
            <ShortBox>
                <Img src="https://res.cloudinary.com/dxjubrqnd/image/upload/v1663774783/Email-API-Front/https_nfhpwm.svg" />
                <ShortBoxTitle>{t('https')}</ShortBoxTitle>
                <TxtShortBox>
                    {t('https-txt')}
                </TxtShortBox>
            </ShortBox>
            <LongBox>
                <TitleLine>
                    <Img src="https://res.cloudinary.com/dxjubrqnd/image/upload/v1663774805/Email-API-Front/documentation_ifeqlc.svg" />
                    <Title>{t('interactive-documentation')}</Title>
                </TitleLine>
                <Txt>
                    {t('interactive-documentation-txt')}
                </Txt>
            </LongBox>
            <LongBox>
                <TitleLine>
                    <Img src="https://res.cloudinary.com/dxjubrqnd/image/upload/v1663774826/Email-API-Front/TEchSupport_ovittg.svg" />
                    <Title>{t('tech-support')}</Title>
                </TitleLine>
                <Txt>
                    {t('tech-support-txt')}
                </Txt>
            </LongBox>
            <LongBox>
                <TitleLine>
                    <Img src="https://res.cloudinary.com/dxjubrqnd/image/upload/v1663774871/Email-API-Front/statistic_jziktb.svg" />
                    <Title>{t('statistics')}</Title>
                </TitleLine>
                <Txt>
                    {t('tech-support-txt')}
                </Txt>
            </LongBox>
        </Container>
    )
}

export default Info

const Container = styled.div`
    min-height: 100vh;
    display: flex;
    flex-direction: row;
    flex-wrap: wrap;
    align-items: center;
    justify-content: space-between;    
    flex-wrap: wrap;
    @media screen and (max-width: 667px) {
        flex-direction: column;
    }
`

const LongBox = styled.div`
    width: 25%;
    height: fit-content;
    padding: 20px;
    /* margin: 15px 0px 15px 30px; */
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    justify-content: flex-start;
    box-shadow: 4px 4px 20px 0 rgba(0, 0, 0, 0.15), -4px -4px 20px 0 white;
    border-radius: 5px;
    background-color: var(--BackgroundColor);
    @media screen and (max-width: 667px) {
        width: 77%;
        margin: 9px 0px 0px 0px;
    }
`

const ShortBox = styled.div`
    width: 18%;
    height: fit-content;
    padding: 20px;
    /* margin: 15px 0px 15px 30px; */
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: flex-start;
    box-shadow: 4px 4px 20px 0 rgba(0, 0, 0, 0.15), -4px -4px 20px 0 white;
    border-radius: 5px;
    background-color: var(--BackgroundColor);
    @media screen and (max-width: 667px) {
        width: 77%;
        margin: 9px 0px 0px 0px;
    }
`

const TitleLine = styled.div`
    width: 100%;
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: flex-start;
`

const Title = styled.div`
    font-family: 'GothamPro';
    font-weight: bold;
    font-size: 16px;
    line-height: 18px;
    color: var(--TextColor);
    margin: 0px 10px;
    text-align: center;
`

const ShortBoxTitle = styled.div`
    font-family: 'GothamPro';
    font-weight: bold;
    font-size: 16px;
    line-height: 24px;
    color: var(--TextColor);
    margin: 0px 10px;
    text-align: center;
    margin-top: 10px;
`

const Img = styled.img`
    width: 34px;
    height: 34px;
`

const Txt = styled.div`
    font-family: 'GothamPro';
    font-weight: 400;
    font-size: 16px;
    line-height: 18px;
    color: var(--TextColor);
    margin-top: 20px;
`

const TxtShortBox = styled.div`
    font-family: 'GothamPro';
    font-weight: 400;
    font-size: 16px;
    line-height: 18px;
    color: var(--TextColor);
    margin-top: 20px;
    text-align: center;
`
