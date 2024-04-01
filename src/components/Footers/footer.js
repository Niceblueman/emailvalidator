import styled from "styled-components"

const Footer = () => {
    return (
        <Wrapper >
            <Container>
                <LookUp>
                    <Title>
                        Other Api's (soon)
                    </Title>
                    <ContentContainer>
                        <ContentItem>Ip score</ContentItem>
                        <ContentItem>Email template validator</ContentItem>
                    </ContentContainer>
                </LookUp>
                <LookUp>
                    <Title>
                        Bulk SDK's (soon)
                    </Title>
                    <ContentContainer>
                        <ContentItem>Python</ContentItem>
                        <ContentItem>Javascript</ContentItem>
                        <ContentItem>Rust</ContentItem>
                        <ContentItem>Php</ContentItem>
                    </ContentContainer>
                </LookUp>
                <LookUp>
                    <Title>
                        CONVERSION
                    </Title>
                    <ContentContainer>
                        <ContentItem onClick={()=>window.open("https://discord.gg/cGbrcBrr")} >Discord</ContentItem>
                    </ContentContainer>
                </LookUp>
                <LookUp>
                    <Logo src="./icons/logo-blue.svg" />
                    <ContentContainer>

                        <ContentItem>© 2024 DUP Inc. All Rights Reserved. Terms
                            & Conditions and Privacy Policy.</ContentItem>
                    </ContentContainer>
                </LookUp>
            </Container>
        </Wrapper>
    )
}

export default (Footer)

const Wrapper = styled.footer`
    width: 100%;
    box-shadow:  4px 4px 10px 0 rgba(0, 0, 0, 0.15),  -4px -4px 10px 0 white;
    position: relative;
    top: auto;
    bottom: 0;
    left: 0;
    background-color: var(--BackgroundColor);
`

const Container = styled.div`
    max-width: 1000px;
    width: 100%;
    margin: 0px auto;
    height: fit-content;
    padding: 30px 0px;
    display: flex;
    flex-direction: row;
    align-items: flex-start;
    justify-content: space-between;

    @media screen and (max-width: 767px) {
        width: calc(100% - 32px);
        flex-direction: column;
    }
`

const LookUp = styled.div`
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    justify-content: flex-start;
    margin: 0px 10px;
`

const Title = styled.div`
    font-family: 'GothamPro';
    font-weight: 800;
    font-size: 16px;
    line-height: 24px;
    color: var(--TextColor);
    text-transform: uppercase;
`

const ContentContainer = styled.div`
    display: flex; 
    flex-direction: column; 
    align-items: flex-start;
    justify-content: flex-start;
    margin: 20px 0px;

    @media screen and (max-width: 767px) {
        margin: 10px 0px;
    }
`

const ContentItem = styled.div`
    font-family: 'GothamPro';
    font-style: normal;
    font-weight: 400;
    font-size: 14px;
    line-height: 24px;
    color: var(--TextColor);
    margin: 5px 0px;
    max-width: 300px;
    cursor:pointer;
`

const Logo = styled.img`
    width: 82px;
    height: 60px;

    @media screen and (max-width: 767px) {
        width: 100%;
    }
`
