import { Hero } from "../components/sections";
import Nav from "../components/nav";
import { IParallax, Parallax, ParallaxLayer } from "@react-spring/parallax";
import styled from "styled-components";
import { useEffect, useState } from "react";
import useIsMobile from "../components/hooks/is-mobile";
import dynamic from "next/dynamic";

const Uptime = dynamic(() => import("../components/sections/Uptime"), {
  ssr: false,
});

const Canvas = styled.canvas`
  display: flex;
  align-items: center;
  justify-content: center;
  /* white border 1px away  rounded on the edges */
  border: 2px solid ${({ theme }) => theme.colors.white};
  border-radius: calc(5% + 15px);
  // make border 4px away from the edge
  padding: 4px;
`;

const CardWrapper = styled.div`
  position: relative;
  /* align to the center */
  margin: 0 auto;
  justify-content: center;
  margin-left: 5vw;
  width: 300px;

  backdrop-filter: blur(5px);
`;

const Title = styled.h2`
  font-size: 20px;
  font-weight: bold;
  margin: 0 0 8px;
`;

const Content = styled.p`
  font-size: 16px;
  margin: 0;
`;

const HandwrittenNoteCard = ({ title, content }) => {
  return (
    <CardWrapper>
      <Title>{title}</Title>
      <Content>{content}</Content>
    </CardWrapper>
  );
};

// phone image source: /public/frames/frame-N.png
const FRAME_COUNT = 549; // Number of frames extracted from video
const FRAME_WIDTH = 300; // Width of each frame
const FRAME_HEIGHT = 600;
const Home = () => {
  const [parallaxRef, setParallaxRef] = useState<IParallax | null>(null);
  const mobile = useIsMobile();
  const loadFrame = async (frameNumber) => { 
     console.log("frame: ", frameNumber);
    const frameUrl = `/frames/frame-${frameNumber.toString().padStart(4, "0")}.png`;
      //  @ts-ignore
    const img: { width: number; height: number; } = await loadImage(frameUrl); // Load the image using canvas API
     const canvas = document.querySelector("canvas"); // Get the canvas element
     if (!canvas) return;
    const ctx = canvas.getContext("2d"); // Get the canvas context

    // @ts-expect-error ts-migrate(2531) FIXME: Object is possibly 'null'.
    ctx.clearRect(0, 0, FRAME_WIDTH, FRAME_HEIGHT); // Clear the canvas
     // get the scale
     // it is the min of the 2 ratios
     let scale_factor = Math.min(canvas.width / img.width, canvas.height / img.height);

     // Lets get the new width and height based on the scale factor
     let newWidth = img.width * scale_factor;
     let newHeight = img.height * scale_factor;

     // get the top left position of the image
     // in order to center the image within the canvas
     let x = (canvas.width / 2) - (newWidth / 2);
     let y = (canvas.height / 2) - (newHeight / 2);

     // When drawing the image, we have to scale down the image
     // width and height in order to fit within the canvas
    // @ts-expect-error ts-migrate(2531) FIXME: Object is possibly 'null'.
     ctx.drawImage(img, x, y, newWidth, newHeight);
  };

  const loadImage = (url) => {
    return new Promise((resolve, reject) => {
      const img = new Image();
      img.onload = () => resolve(img);
      img.onerror = reject;
      img.src = url;
    });
  };
  useEffect(() => {
    // preload images 1-142
    const src = "/frames/frame-{}.png";

    for (let i = 1; i <= 142; i++) {
      const img = new Image();
      img.src = src.replace("{}", i.toString().padStart(4, "0"));
    }
    if (!parallaxRef) return;
    // get the position of the canvas
    
    parallaxRef.container.current?.addEventListener("scroll", () => {
      const scrollY = parallaxRef.container.current.scrollTop;
      
      const canvas = document.querySelector("canvas");
      let frameNumber = 1;
      if (scrollY > 0) {
        frameNumber = Math.floor(scrollY / 10) + 1;
      }
      console.log(`ScrollY: ${scrollY} Frame: ${frameNumber}`)
      
      // console.log(`ScrollY: ${scrollY} Offset: ${offset}`, canvas);
      frameNumber -= 50; //offset

      console.log(`Frame: ${frameNumber}`)
      if (frameNumber > 0 && frameNumber <= FRAME_COUNT) {
        loadFrame(frameNumber);
      };
    });
    loadFrame(1);
    return () => {
      parallaxRef.container.current?.removeEventListener("scroll", () => {
        console.log("removed");
      });
    }
  }, [parallaxRef?.container?.current]);

  const alignCenter = { display: 'flex', alignItems: mobile ? 'top' : 'center', position: "relative" }

  return (
    <Parallax pages={10} ref={setParallaxRef}>
      <ParallaxLayer offset={0} speed={0.5}>
        <Nav />
        <Hero />
      </ParallaxLayer>
      {/* @ts-ignore */}
      <ParallaxLayer sticky={{ start: 1, end: 5.5 }} style={{ ...alignCenter, justifyContent: (mobile ? 'right' : 'center' )}}>
          <Canvas width={FRAME_WIDTH} height={FRAME_HEIGHT} />
      </ParallaxLayer>

      <ParallaxLayer
        sticky={{ start: 1, end: 3.5 }}
        speed={3}
        // @ts-ignore
        style={{ ...alignCenter, justifyContent: mobile ? "left" : "flex-end" }}
      >
        <HandwrittenNoteCard
            title={"Bookmark any message you want to save"}
            content={"Just tap the bookmark icon to save a message to your bookmarks"}
        />
      </ParallaxLayer>

      <ParallaxLayer
        sticky={{ start: 4, end: 5 }}
        speed={3}
        // @ts-ignore
        style={{ ...alignCenter, justifyContent: mobile ? "left" : "flex-end" }}
      >
        <HandwrittenNoteCard
          title={"Group Embeds with colors"}
          content={"Click on the paint brush icon to change the color of the embed"}
        />
      </ParallaxLayer>

      <Uptime/>
    </Parallax>
  );
};

export default Home;
