import axios from "axios";
import { API_URL } from "./constants";
import { ApiToken, Playlist, Song } from "./types";

export type ApiError = {
  message: string;
};

export type ApiResponse<T> = T | ApiError;

function isApiError(x: unknown): x is ApiError {
  return (x as ApiError).message !== undefined;
}

async function getPlaylists(token: ApiToken) {
  console.log("Getting playlists with token: " + token);
  const { data } = await axios.get<Playlist[]>(`${API_URL}/playlists`, {
    headers: {
      Authorization: `Bearer ${token}`,
    },
  });

  return data;
}

// TODO: Make this take a playlist
async function createPlaylist(name: string, token: ApiToken) {
  const { data } = await axios.post<Playlist>(
    `${API_URL}/playlists`,
    { name, elements: [] },
    {
      headers: {
        Authorization: `Bearer ${token}`,
      },
    }
  );

  return data;
}

async function getPlaylist(playlistId: number, token: ApiToken) {
  console.log("Getting playlist with token: " + token);
  const { data } = await axios.get<Playlist>(
    `${API_URL}/playlists/${playlistId}`,
    {
      headers: {
        Authorization: `Bearer ${token}`,
      },
    }
  );

  return data;
}

async function updatePlaylist(playlist: Playlist, token: ApiToken) {
  const { data } = await axios.put<Playlist>(
    `${API_URL}/playlists/${playlist.id}`,
    playlist,
    {
      headers: {
        Authorization: `Bearer ${token}`,
      },
    }
  );

  return data;
}

async function deletePlaylist(playlistId: number, token: ApiToken) {
  return axios.delete(`${API_URL}/playlists/${playlistId}`, {
    headers: {
      Authorization: `Bearer ${token}`,
    },
  });
}

export type SongSearchResult = {
  name: string;
  spotify_id: string;
  image_url: string;
};

export type AlbumSearchResult = {
  name: string;
  spotify_id: string;
  image_url: string;
  songs: Song[];
};

export type SearchResults = {
  songs: SongSearchResult[];
  albums: AlbumSearchResult[];
};

async function search(query: string, token: ApiToken) {
  const { data } = await axios.get<SearchResults>(
    `${API_URL}/spotify/search?q=${query}`,
    {
      headers: {
        Authorization: `Bearer ${token}`,
      },
    }
  );

  return data;
}

async function albumSongs(albumId: string, token: ApiToken) {
  const { data } = await axios.get<Song[]>(
    `${API_URL}/spotify/album_songs/${albumId}`,
    {
      headers: {
        Authorization: `Bearer ${token}`,
      },
    }
  );

  return data;
}

async function sendPlayerCommand(command: object, token: ApiToken) {
  const { data } = await axios.post(`${API_URL}/player`, command, {
    headers: {
      Authorization: `Bearer ${token}`,
    },
  });

  return data;
}

export default {
  isApiError,
  getPlaylists,
  createPlaylist,
  getPlaylist,
  updatePlaylist,
  deletePlaylist,
  search,
  albumSongs,
  sendPlayerCommand,
};
