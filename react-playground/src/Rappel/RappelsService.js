import {post, dele} from "./../Http/Client"

export function getRappels() {

    return fetch("http://localhost:7878/rappels")
    .then(response => response.json())
    .catch(error => alert("Erreur : " + error));
  }

export function getRappel(id) {
    return fetch("http://localhost:7878/rappel/"+id)
    .then(response => response.json())
    .catch(error => alert("Erreur : " + error));
}

export function deleteRappel(id) {
  try {
    dele("/rappel", id, null);
  } catch (e) {
    console.log(e.name + " : " + e.message)
  }
}

export function addRappel(rappel) {
  try {
    post("/rappel", null, rappel);
  } catch (e) {
    console.log(e.name + " : " + e.message)
  }
}