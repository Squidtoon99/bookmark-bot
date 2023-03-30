import styled from "styled-components";
import { Caveat } from "next/font/google";
import { Parallax, ParallaxLayer } from "@react-spring/parallax";

const font = Caveat({
  weight: "400",
  style: "normal",
  subsets: ["latin"],
});

const TitleWrapper = styled.div`
  display: inline-flex;
  flex-direction: row;
  align-items: center;
  justify-content: center;
  width: 100%;
  height: calc(90vh - 64px);
  gap: 16px;
  padding: 0 16px;
  background-color: ${({ theme }) => theme.colors.black};

  @media (max-width: 768px) {
    flex-direction: column;
    gap: 1px;
    height: 75vh;
  }
`;

const Title = styled.div`
  color: ${({ theme }) => theme.colors.white};
  font-size: 48px;
  padding: 4px 4px;
  ${font.style};

  @media (min-width: 768px) {
    font-size: 120px;
  }

  @media (min-width: 480px) {
    font-size: 80px;
  }
`;

const RedTitle = styled(Title)`
  color: ${({ theme }) => theme.colors.red};
`;

const Hero = () => {
  return (
    <TitleWrapper>
      <Title>bookmark</Title> <RedTitle>anything</RedTitle>
    </TitleWrapper>
  );
};

export default Hero;
