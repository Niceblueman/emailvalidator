/* eslint-disable no-unused-expressions */
/* eslint-disable no-restricted-globals */
/* eslint-disable no-unused-vars */
/* eslint-disable eqeqeq */
/* eslint-disable jsx-a11y/anchor-is-valid */
/* eslint-disable react/jsx-no-comment-textnodes */
/* eslint-disable no-useless-escape */
/* eslint-disable react-hooks/exhaustive-deps */
import { useTranslation } from "react-i18next"
import styled from "styled-components"

const VerifyPage = () => {

    const { t } = useTranslation();
    return (
        <Wrapper>
            <Container>
                <Box>
                    <TitleLine>
                        <Img src="https://res.cloudinary.com/dxjubrqnd/image/upload/v1663931006/Email-API-Front/world_mpdbou.svg" />
                        <Title>{t('power-application')}</Title>
                    </TitleLine>
                    <Txt>
                        {t('power-application-txt')}
                    </Txt>
                </Box>
                <Box>
                    <TitleLine>
                        <Img src="https://res.cloudinary.com/dxjubrqnd/image/upload/v1663773894/Email-API-Front/SecureAPI_kw5m6f.svg" />
                        <Title>
                            {t('integrate')}
                        </Title>
                    </TitleLine>
                    <Txt>
                        {t('integrate-txt')}
                    </Txt>
                </Box>
                <DocumentationBtn >
                    {t('documentation')}
                </DocumentationBtn>
            </Container>
            <ContainerLeft>
                <TitleBox>
                    {t('verify')}
                </TitleBox>
                <FormContainer>
                    <InputLine>
                        <InputTitle>// verify any email address:</InputTitle>
                        <Input />
                    </InputLine>
                    <InputLine>
                        <InputTitle>/https://apilayer.net/api/check</InputTitle>
                        <Input />
                    </InputLine>
                    <InputLine>
                        <InputTitle>? access_key = YOUR_ACCESS_KEY</InputTitle>
                        <Input />
                    </InputLine>
                    <InputLine>
                        <Input />
                    </InputLine>
                    <InputLine>
                        <InputTitle>// API returns simple JSON response </InputTitle>
                        <Input />
                    </InputLine>
                </FormContainer>
            </ContainerLeft>
        </Wrapper>
    )
}

export default VerifyPage

const Wrapper = styled.div`
    min-height: 100vh;
    width: 99%;
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: space-between;    
    flex-wrap: wrap;
`

const Container = styled.div`
    width: 45%;
    max-width: 370px;
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    justify-content: space-between;
    height: 80vh;
`

const ContainerLeft = styled.div`
    width: 45%;
    /* max-width: 370px; */
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    justify-content: space-between;
    height: 80vh;
`


const Box = styled.div`
    width: 100%;
    max-width: 370px;
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

const BtnLine = styled.div`
    width: 100%;
    /* align-self: center */
`

const DocumentationBtn = styled.button`
    font-family: 'GothamPro';
    font-weight: bold;
    font-size: 14px;
    line-height: 13px;
    border-radius: 5px;
    color: white;
    background-color: #FF6C6C;
    padding: 25px;
    align-self: center;
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
    text-align: center;
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

const TitleBox = styled.div`
    width: calc(100% - 60px);
    padding: 20px 30px;
    font-family: 'GothamPro';
    font-weight: bold;
    font-size: 16px;
    line-height: 24px;
    box-shadow: 4px 4px 20px 0 rgba(0, 0, 0, 0.15), -4px -4px 20px 0 white;
    text-transform: uppercase;
    color: var(--TextColor);
    background-color: var(--BackgroundColor);
`

const FormContainer = styled.div`
    width: calc(100% - 60px);
    padding: 30px;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: space-between;
    box-shadow: 4px 4px 20px 0 rgba(0, 0, 0, 0.15), -4px -4px 20px 0 white;
    background-color: var(--BackgroundColor);
    border-radius: 5px;
`

const InputLine = styled.div`
    width: 100%;
    margin: 15px 0px;
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    justify-content: space-between;
`

const InputTitle = styled.div`
    font-family: 'GothamPro';
    font-weight: 400;
    font-size: 14px;
    line-height: 13px;
    color: rgba(129, 129, 129, 1);
    margin-bottom: 5px;
`

const Input = styled.div`
    width: 100%;
    height: 40px;
    box-shadow: inset 4px 4px 20px 0 rgba(0, 0, 0, 0.15), inset -4px -4px 20px 0 white;
    border-radius: 5px;
`
