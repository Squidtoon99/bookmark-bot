import { ParallaxLayer } from "@react-spring/parallax";
import arrow from "../../public/svg/angles-up-solid.svg";
import Image from "next/image";
import styled, { keyframes } from "styled-components";
import { Questrial } from "next/font/google";
import seedrandom from "seedrandom";

const font = Questrial({
    weight: "400",
    style: "normal",
    subsets: ["latin"],
});

const Arrow = styled.svg < {
    position: {
        top: string;
left: string;
    }
}>`
        --distance: 50vw;
    z-index: -1;
    position: absolute;
    top: ${props => props.position.top};
    left: ${props => props.position.left};
    width: 2.5rem;
    height: 2.5rem;
    border-radius: 50%;
    background: ${props => props.theme.colors.black};
    fill: ${props => props.theme.colors.red};
    /* background blur */
    backdrop-filter: blur(10px);
    // set the width and height to 0.5 if the user is on mobile
    @media (max-width: 768px) {
        width: 0.5px;
        height: 0.5px;
        --distance: 
        
    }
    @media (prefers-reduced-motion: reduce) {
        animation-play-state: paused;
    }
`;

const ArrowIcon = ({ ...props }) => {
    // @ts-ignore
    return <Arrow xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" {...props}>
        <path fillRule="evenodd" d="M11.47 4.72a.75.75 0 011.06 0l7.5 7.5a.75.75 0 11-1.06 1.06L12 6.31l-6.97 6.97a.75.75 0 01-1.06-1.06l7.5-7.5zm.53 7.59l-6.97 6.97a.75.75 0 01-1.06-1.06l7.5-7.5a.75.75 0 011.06 0l7.5 7.5a.75.75 0 11-1.06 1.06L12 12.31z" clipRule="evenodd" />
    </Arrow>
}

const Content = styled.div`
    color: ${({ theme }) => theme.colors.white};
    font-size: 48px;
    padding: 4px 4px;
    ${font.style};

    /* centered */
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;

    backdrop-filter: blur(10px) saturate(180%);
    @media (min-width: 768px) {
        font-size: 120px;
    }
    @media (min-width: 480px) {
        font-size: 80px;
    }

    p {
        margin: 0;
        font-size: 12px;
        padding: 4px 4px;
        @media (min-width: 768px) {
            font-size: 36px;
            max-width: 30vw;
        }
        @media (min-width: 480px) {
            font-size: 24px;
            max-width: 55vw;
        }
    }
`;

const Uptime = () => {
    let visited: { top: number[]; left: number[]; } = {
        top: [],
        left: [],
    }
    // how good the uptime is 
    return <>
        {
            Array.from(Array(25), (e, i) => {
                var rng = seedrandom(i);
                // @ts-ignore // This is really bad and also the most efficient way to do this.
                let top = rng() * 100;
                let left = rng() * 100;

                while (visited.top.indexOf(top) !== -1) {
                    top = rng() * 100;
                }
                while (visited.left.indexOf(left) !== -1) {
                    left = rng() * 100;
                }
                return <ParallaxLayer speed={rng() * 0.5 + 0.25} offset={rng() * 0.5 + 6.5} key={i}>
                    <ArrowIcon position={{ top: `${rng() * 100}vh`, left: `${rng() * 100}vw` }} />
                </ParallaxLayer>
            })
        }
        <ParallaxLayer speed={0.5} sticky={{start: 7, end: 8}} offset={6.5}>
            <Content>
                100% Uptime
                <p>
                    Run on the same infrastructure as discord, so you can rest assured that your bot will be online 24/7.
                </p>
            </Content>
        </ParallaxLayer>
    </>;
}

export default Uptime;