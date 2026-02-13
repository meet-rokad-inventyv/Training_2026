// @ts-check
//
// The line above enables type checking for this file. Various IDEs interpret
// the @ts-check directive. It will give you helpful autocompletion when
// implementing this exercise.

/**
 * Removes duplicate tracks from a playlist.
 *
 * @param {string[]} playlist
 * @returns {string[]} new playlist with unique entries
 */
export function removeDuplicates(playlist) {
  let a = new Set(playlist)
  let b = new Array
  for (let name of a) {
    b.push(name);
  }
  return b;
}

/**
 * Checks whether a playlist includes a track.
 *
 * @param {string[]} playlist
 * @param {string} track
 * @returns {boolean} whether the track is in the playlist
 */
export function hasTrack(playlist, track) {
    for (let name of playlist) {
        if(name === track){
          return true;
        }
    }
  return false;
}

/**
 * Adds a track to a playlist.
 *
 * @param {string[]} playlist
 * @param {string} track
 * @returns {string[]} new playlist
 */
export function addTrack(playlist, track) {
  let a = new Set(playlist)
  a.add(track)
  let b = new Array
  for (let name of a) {
    b.push(name);
  }
  return b;
}

/**
 * Deletes a track from a playlist.
 *
 * @param {string[]} playlist
 * @param {string} track
 * @returns {string[]} new playlist
 */
export function deleteTrack(playlist, track) {
    let a = new Set(playlist)
    a.delete(track)
    let b = new Array
    for (let name of a) {
      b.push(name);
    }
    return b;
}

/**
 * Lists the unique artists in a playlist.
 *
 * @param {string[]} playlist
 * @returns {string[]} list of artists
 */
export function listArtists(playlist) {
    const artists = new Set();
    for (const i of playlist) {
      const [, artist] = i.split(' - ');
      artists.add(artist);
    }
    return Array.from(artists);
}