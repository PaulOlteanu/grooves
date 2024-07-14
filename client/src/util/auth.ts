function randomBytes(size: number) {
  return crypto.getRandomValues(new Uint8Array(size));
}

function base64url(bytes: number[]) {
  return btoa(String.fromCharCode(...bytes))
    .replace(/=/g, "")
    .replace(/\+/g, "-")
    .replace(/\//g, "_");
}

const clientId = "1ef695e7fecc4086a26b8cd329e477dc";
const scopes =
  "user-read-currently-playing user-modify-playback-state user-read-playback-state playlist-read-private user-read-private user-read-email";

export function getSpotifyRequestUrl() {
  const state = base64url(Array.from(randomBytes(96)));

  const params = new URLSearchParams({
    client_id: clientId,
    response_type: "code",
    redirect_uri: `${import.meta.env.VITE_FRONTEND_URL}/callback`,
    state,
    scope: scopes,
  });

  const requestUrl = new URL(
    `https://accounts.spotify.com/authorize?${params.toString()}`,
  );
  return { requestUrl, state };
}
