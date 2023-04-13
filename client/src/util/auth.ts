function randomBytes(size: number) {
  return crypto.getRandomValues(new Uint8Array(size));
}

function base64url(bytes: number[]) {
  return btoa(String.fromCharCode(...bytes))
    .replace(/=/g, "")
    .replace(/\+/g, "-")
    .replace(/\//g, "_");
}

async function generateCodeChallenge(code_verifier: string) {
  const codeVerifierBytes = new TextEncoder().encode(code_verifier);
  const hashBuffer = await crypto.subtle.digest("SHA-256", codeVerifierBytes);
  return base64url(Array.from(new Uint8Array(hashBuffer)));
}

const clientId = "1ef695e7fecc4086a26b8cd329e477dc";
const redirectUri = "http://localhost:5173/callback";
const scopes =
  "user-read-currently-playing user-modify-playback-state user-read-playback-state playlist-read-private user-read-private";

export async function getSpotifyRequestUrl() {
  const codeVerifier = base64url(Array.from(randomBytes(96)));
  const state = base64url(Array.from(randomBytes(96)));

  const code_challenge = await generateCodeChallenge(codeVerifier);

  const params = new URLSearchParams({
    client_id: clientId,
    response_type: "code",
    redirect_uri: redirectUri,
    code_challenge_method: "S256",
    code_challenge,
    state,
    scope: scopes,
  });

  const requestUrl = new URL(
    `https://accounts.spotify.com/authorize?${params.toString()}`
  );
  return { requestUrl, codeVerifier, state };
}
