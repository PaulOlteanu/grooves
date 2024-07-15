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

    async function finishAuth() {
      try {
        if (!code) {
          failLogin();
          return;
        }

        const apiResponse = await fetch(
          import.meta.env.VITE_API_URL + "/auth",
          {
            method: "POST",
            headers: {
              "Content-Type": "application/json",
            },
            body: JSON.stringify({ code }),
          },
        );

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
      } catch {
        failLogin();
      }
    }

    localStorage.removeItem("state");
    void finishAuth();
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
