import { Playlist, PlaylistElement, Song } from "../types";

export function addElement(
  playlist: Playlist,
  element: PlaylistElement
): Playlist {
  return { ...playlist, elements: [...playlist.elements, element] };
}

export function updateElement(
  playlist: Playlist,
  elementIndex: number,
  updatedElement: PlaylistElement
) {
  return {
    ...playlist,
    elements: playlist.elements.map((e, i) =>
      i === elementIndex ? updatedElement : e
    ),
  };
}

export function removeElement(
  playlist: Playlist,
  elementIndex: number
): Playlist {
  return {
    ...playlist,
    elements: playlist.elements.filter((_, i) => i !== elementIndex),
  };
}

export function addSong(element: PlaylistElement, song: Song): PlaylistElement {
  return { ...element, songs: [...element.songs, song] };
}

export function removeSong(
  element: PlaylistElement,
  songIndex: number
): PlaylistElement {
  return {
    ...element,
    songs: element.songs.filter((_, i) => i !== songIndex),
  };
}
