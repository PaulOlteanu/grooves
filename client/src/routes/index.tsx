import { useEffect, useState } from "react";
import { useAuth } from "../contexts/auth";
import { getSpotifyRequestUrl } from "../util/auth";

function LoginButton({ loginLink }: { loginLink: URL }) {
  return (
    <a href={loginLink.toString()}>
      <p className="underline">Sign In with Spotify</p>
    </a>
  );
}

function LoggedOutPage() {
  const { token } = useAuth();
  const [loginLink, setLoginLink] = useState<URL | null>(null);

  useEffect(() => {
    if (!token) {
      const { state, requestUrl } = getSpotifyRequestUrl();
      localStorage.setItem("state", state);
      setLoginLink(requestUrl);
    }
  }, [token]);

  if (!token && loginLink) {
    return (
      <div>
        <p>Welcome to Grooves!</p>
        <LoginButton loginLink={loginLink} />
        <br />
        <p>You'll need to ask Paul to add your email before you can login</p>
      </div>
    );
  }

  return (
    <div>
      <p>Welcome to Grooves!</p>
    </div>
  );
}

function LoggedInPage() {
  return (
    <div>
      <p>Welcome to Grooves!</p>
      <p>Head on over to the playlists page, create a playlist and enjoy!</p>
      <p>
        <b>Note: </b> Before clicking play on a playlist, you will need to play{" "}
        <em>something</em> on your Spotify client so that grooves knows where to
        start playing
      </p>
      <br />
      <p>A few limitations that will be fixed soon&trade;:</p>
      <ul className="list-disc pl-5">
        <li>
          If you pause your Spotify, Grooves will stop controlling playback
        </li>
        <li>
          There's no playlist renaming or re-ordering, you can't add individual
          songs, etc.
        </li>
        <li>
          There's no error messages for most things, so if you run into a
          problem try again, and if it still doesn't work, message Paul
        </li>
        <li>
          And surely many more! If you don't know how to do something, or have
          any other problems, message Paul
        </li>
      </ul>
    </div>
  );
}

export default function Index() {
  const { token } = useAuth();

  return <div>{!token ? <LoggedOutPage /> : <LoggedInPage />}</div>;
}
