import type { AppProps } from 'next/app'
import { ThemeProvider, DefaultTheme } from 'styled-components'
import GlobalStyle from '../components/globalstyles'

const theme: DefaultTheme = {
    // discord branding
    colors: {
        blurple: "#5865F2",
        green: "#57F287",
        yellow: "#FEE75C",
        fuchsia: "#EB459E",
        red: "#ED4245",
        white: "#F2EFEA",
        // black: "#1b1b1b",
        black: "#000",
    },
};

export default function App({ Component, pageProps }: AppProps) {
  return (
    <>
      <ThemeProvider theme={theme}>
        <GlobalStyle />
        <Component {...pageProps} />
      </ThemeProvider>
    </>
  )
}
