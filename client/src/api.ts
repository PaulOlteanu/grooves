import axios from "axios";
import { API_TOKEN, API_URL } from "./constants";
import { Playlist, Song } from "./types";

export type ApiError = {
  message: string;
};

export type ApiResponse<T> = T | ApiError;

function isApiError(x: unknown): x is ApiError {
  return (x as ApiError).message !== undefined;
}

async function getPlaylists() {
  const { data } = await axios.get<Playlist[]>(`${API_URL}/playlists`, {
    headers: {
      Authorization: `Bearer ${API_TOKEN}`,
    },
  });

  return data;
}

async function createPlaylist(name: string) {
  const { data } = await axios.post<Playlist>(
    `${API_URL}/playlists`,
    { name, elements: [] },
    {
      headers: {
        Authorization: `Bearer ${API_TOKEN}`,
      },
    }
  );

  return data;
}

async function getPlaylist(playlistId: number) {
  const { data } = await axios.get<Playlist>(
    `${API_URL}/playlists/${playlistId}`,
    {
      headers: {
        Authorization: `Bearer ${API_TOKEN}`,
      },
    }
  );

  return data;
}

async function updatePlaylist(playlist: Playlist) {
  const { data } = await axios.put<Playlist>(
    `${API_URL}/playlists/${playlist.id}`,
    playlist,
    {
      headers: {
        Authorization: `Bearer ${API_TOKEN}`,
      },
    }
  );

  return data;
}

async function deletePlaylist(playlistId: number) {
  return axios.delete(`${API_URL}/playlists/${playlistId}`, {
    headers: {
      Authorization: `Bearer ${API_TOKEN}`,
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

async function search(query: string) {
  const { data } = await axios.get<SearchResults>(
    `${API_URL}/spotify/search?q=${query}`,
    {
      headers: {
        Authorization: `Bearer ${API_TOKEN}`,
      },
    }
  );

  return data;
}

async function albumSongs(albumId: string) {
  const { data } = await axios.get<Song[]>(
    `${API_URL}/spotify/album_songs/${albumId}`,
    {
      headers: {
        Authorization: `Bearer ${API_TOKEN}`,
      },
    }
  );

  return data;
}

async function sendPlayerCommand(command: object) {
  const { data } = await axios.post(`${API_URL}/player`, command, {
    headers: {
      Authorization: `Bearer ${API_TOKEN}`,
    },
  });

  console.log(data);

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
