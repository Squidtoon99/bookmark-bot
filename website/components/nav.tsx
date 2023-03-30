import styled from "styled-components";
import Link from "next/link";

const NavWrapper = styled.nav`
  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: space-between;
  width: 100%;
  height: 64px;
  padding: 0 16px;
  background-color: ${({ theme }) => theme.colors.black};
`;

const NavItems = styled.div`
  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: space-between;
  margin-left: 16px;
  gap: 16px;
`;

const NavLink = styled(Link)`
  color: ${({ theme }) => theme.colors.white};
  font-size: 16px;
  font-weight: 400;
  text-decoration: none;
  font-weight: 600;
  transition: color 0.15s ease;

  &:hover {
    color: ${({ theme }) => theme.colors.red};
  }
`;

const Logo = styled.svg`
  width: 32px;
  height: 32px;
  fill: ${({ theme }) => theme.colors.red};
`;

const Nav = () => {
  return (
    <NavWrapper>
      <Logo viewBox="0 0 24 24">
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" strokeWidth={0} stroke="currentColor" className="w-6 h-6">
            <path strokeLinecap="round" strokeLinejoin="round" d="M17.593 3.322c1.1.128 1.907 1.077 1.907 2.185V21L12 17.25 4.5 21V5.507c0-1.108.806-2.057 1.907-2.185a48.507 48.507 0 0111.186 0z" />
        </svg>
      </Logo>
      <NavItems>
        <NavLink href="/#features">Features</NavLink>
        <NavLink href={process.env.NEXT_PUBLIC_DOCS_LINK as string}>Documentation</NavLink>
        <NavLink href={process.env.NEXT_PUBLIC_BOT_INVITE as string}>Invite</NavLink>
      </NavItems>
    </NavWrapper>
  );
};

export default Nav;
