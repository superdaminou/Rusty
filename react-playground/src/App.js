import { useState } from 'react';
import {Rappels, AddRappel} from './Rappel/Rappel';
import { getRappels, getRappel } from './Rappel/RappelsService';

export default function App() {
  const [rappels, setRappels] = useState([])
  
  function handleClick() {
    getRappels().then((value) => setRappels(value));
  }
  
  return(
    <div>
      <Formulaire onClick={handleClick} reallySetRappel={setRappels}></Formulaire>
      <Rappels rappels={rappels}></Rappels>
      <AddRappel></AddRappel>
    </div>
  )
}

function Formulaire({onClick, reallySetRappel}) {
  return(
    <div>
      <SearchOne setRappels={reallySetRappel}></SearchOne>
      <Bouton onBoutonClick={onClick} children={"all"}></Bouton>
    </div>
  )
}

function SearchOne({setRappels}) {
  const [id, setId] = useState(1)
  
  function handleClick() {
    getRappel(id).then((value) => setRappels([value]))
  }

  function handleChange(event) {
    setId(event.target.value)
  }

  return (
    <div>
      <Input onChange={handleChange}></Input>
      <Bouton onBoutonClick={handleClick} children={"search"}></Bouton>
    </div>
  )
}

function Bouton({onBoutonClick, children}) {
  return (
    <button onClick={onBoutonClick}>
      {children}
    </button>
  )
}

function Input({onChange}) {
  return (
    <input type="text" placeholder='Rappel' onChange={onChange}></input>
  )
}