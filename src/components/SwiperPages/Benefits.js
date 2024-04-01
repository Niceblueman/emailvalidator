import { useTranslation } from "react-i18next"
import styled from "styled-components"

const Benefits = () => {
    const { t } = useTranslation();
    return (
        <Container>
            <Box>
                <TitleLine>
                    <Img src="https://res.cloudinary.com/dxjubrqnd/image/upload/v1663767467/Email-API-Front/syntax_f8o7cz.svg" />
                    <Title>{t('syntax')}</Title>
                </TitleLine>
                <Txt>
                    {t('syntax-box-txt')}
                </Txt>
            </Box>
            <Box>
                <TitleLine>
                    <Img src="https://res.cloudinary.com/dxjubrqnd/image/upload/v1663767445/Email-API-Front/smtp_u9llx6.svg" />
                    <Title>{t('smtp')}</Title>
                </TitleLine>
                <Txt>
                    {t('smtp-box-txt')}
                </Txt>
            </Box>
            <Box>
                <TitleLine>
                    <Img src="https://res.cloudinary.com/dxjubrqnd/image/upload/v1663767416/Email-API-Front/role_l5q0wo.svg" />
                    <Title>{t('role-check')}</Title>
                </TitleLine>
                <Txt>
                    {t('role-check-box-txt')}
                </Txt>
            </Box>
            <Box>
                <TitleLine>
                    <Img src="https://res.cloudinary.com/dxjubrqnd/image/upload/v1663767447/Email-API-Front/providers_is72dq.svg" />
                    <Title>{t('free-providers')}</Title>
                </TitleLine>
                <Txt>
                    {t('free-providers-txt')}
                </Txt>
            </Box>
            <Box>
                <TitleLine>
                    <Img src="https://res.cloudinary.com/dxjubrqnd/image/upload/v1663767402/Email-API-Front/providers_maydii.svg" />
                    <Title>{t('disposable-providers')}</Title>
                </TitleLine>
                <Txt>
                    {t('disposable-providers-txt')}
                </Txt>
            </Box>
            <Box>
                <TitleLine>
                    <Img src="https://res.cloudinary.com/dxjubrqnd/image/upload/v1663767378/Email-API-Front/score_bk9epr.svg" />
                    <Title>{t('score')}</Title>
                </TitleLine>
                <Txt>
                    {t('score-box-txt')}
                </Txt>
            </Box>
        </Container>
    )
}

export default Benefits

const Container = styled.div`
    min-height: 100vh;
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: space-between;    
    flex-wrap: wrap;
`

const Box = styled.div`
    width: 25%;
    height: fit-content;
    padding: 30px;
    /* margin: 15px 0px 15px 30px; */
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    justify-content: flex-start;
    box-shadow: 4px 4px 20px 0 rgba(0, 0, 0, 0.15), -4px -4px 20px 0 white;
    border-radius: 5px;
    background-color: var(--BackgroundColor);
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
    line-height: 24px;
    color: var(--TextColor);
    margin: 0px 10px;
`

const Img = styled.img`
    width: 46px;
    height: 46px;
`

const Txt = styled.div`
    font-family: 'GothamPro';
    font-weight: 400;
    font-size: 16px;
    line-height: 24px;
    color: var(--TextColor);
    margin-top: 20px;
`

