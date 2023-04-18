import { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import { useAuth } from "../contexts/auth";

enum LoginState {
  InProgress,
  Succeeded,
  Failed,
}

export default function LoginCallback() {
  const navigate = useNavigate();
  const { setToken } = useAuth();
  const [loginState, setLoginState] = useState(LoginState.InProgress);

  const failLogin = () => setLoginState(LoginState.Failed);

  useEffect(() => {
    const url = new URL(window.location.href);
    const params = new URLSearchParams(url.search);
    const code = params.get("code");

    if (!code) {
      failLogin();
      return;
    }

    const clientId = "1ef695e7fecc4086a26b8cd329e477dc";

    async function finishAuth(codeVerifier: string) {
      if (!code) {
        failLogin();
        return;
      }

      const tokenResponse = await fetch(
        "https://accounts.spotify.com/api/token",
        {
          method: "POST",
          body: new URLSearchParams({
            client_id: clientId,
            grant_type: "authorization_code",
            code,
            redirect_uri: `${import.meta.env.VITE_FRONTEND_URL}/callback`,
            code_verifier: codeVerifier,
          }),
        }
      );

      if (tokenResponse.status !== 200) {
        failLogin();
        return;
      }

      // eslint-disable-next-line @typescript-eslint/no-unsafe-assignment
      const tokenData = await tokenResponse.json();
      // eslint-disable-next-line @typescript-eslint/no-unsafe-assignment, @typescript-eslint/no-unsafe-member-access
      const spotifyToken = tokenData.access_token;
      // eslint-disable-next-line @typescript-eslint/no-unsafe-assignment, @typescript-eslint/no-unsafe-member-access
      const spotifyRefreshToken = tokenData.refresh_token;

      if (!spotifyToken || !spotifyRefreshToken) {
        failLogin();
        return;
      }

      const apiResponse = await fetch(import.meta.env.VITE_API_URL + "/auth", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          access_token: spotifyToken,
          refresh_token: spotifyRefreshToken,
        }),
      });

      if (apiResponse.status !== 200) {
        failLogin();
        return;
      }

      const apiData = await apiResponse.json();
      // eslint-disable-next-line @typescript-eslint/no-unsafe-assignment, @typescript-eslint/no-unsafe-member-access
      const apiToken: string = apiData.token;
      if (apiToken) {
        localStorage.setItem("apiToken", apiToken);
        setToken(apiToken);
        setLoginState(LoginState.Succeeded);

        navigate("/");
      }
    }

    const codeVerifier = localStorage.getItem("codeVerifier");
    localStorage.removeItem("state");
    localStorage.removeItem("codeVerifier");

    if (codeVerifier) {
      void finishAuth(codeVerifier);
    } else {
      failLogin();
    }
  }, [navigate, setToken]);

  let displayText = "";
  switch (loginState) {
    case LoginState.InProgress:
      displayText = "Login In Progress";
      break;

    case LoginState.Failed:
      displayText = "Login Failed D:";
      break;
  }

  return (
    <main>
      <div
        style={{
          display: "flex",
          justifyContent: "center",
          alignItems: "center",
        }}
      >
        <p>{displayText}</p>
      </div>
    </main>
  );
}
