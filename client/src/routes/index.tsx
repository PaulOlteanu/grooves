import { useEffect, useState } from "react";
import { Outlet } from "react-router-dom";
import Navbar from "../components/navbar";
import { AuthProvider, useAuth } from "../contexts/auth";
import { getSpotifyRequestUrl } from "../util/auth";

function LoginButton({ loginLink }: { loginLink: URL }) {
  return (
    <a href={loginLink.toString()}>
      <p className="text-center underline">Sign In with Spotify</p>
    </a>
  );
}

export default function Index() {
  const { token } = useAuth();
  const [loginLink, setLoginLink] = useState<URL | null>(null);

  useEffect(() => {
    if (!token) {
      const getUrl = async () => {
        const { codeVerifier, state, requestUrl } =
          await getSpotifyRequestUrl();

        window.localStorage.setItem("codeVerifier", codeVerifier);
        window.localStorage.setItem("state", state);
        setLoginLink(requestUrl);
      };

      void getUrl();
    }
  }, [token]);

  let loginButton = null;
  if (!token && loginLink) {
    loginButton = <LoginButton loginLink={loginLink} />;
  }
  return <div>{!token ? loginButton : <Outlet />}</div>;
}
