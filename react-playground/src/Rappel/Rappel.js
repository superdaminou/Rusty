import { addRappel, deleteRappel } from './RappelsService';
import { useState } from 'react';



export function Rappel({rappel}) {

  function handleClick() {
    deleteRappel(rappel.id)
  }

    return(
      <li>
        <p>{rappel.nom}</p> 
        <ul>
          <li>{rappel.criticite}</li>
          <li>
          <button onClick={handleClick}>Delete</button>
          </li>
        </ul>
      </li>
    )
  }


export function Rappels({rappels}) {
  return (
    <ul>{rappels.map(rappel => <Rappel rappel={rappel}/>)}</ul>
  )
}



export function AddRappel() {
  const [rappel, setRappel] = useState({
      nom: "String",
      criticite: "String",
      repetition: 5,
      date_limite: "2023-01-01"
  })

  const handleChange = (event) => {
    console.log(event)
    const name = event.target.name;
    const value = event.target.value;
    setRappel(values => ({...values, [name]: value}))
  }

  const handleSubmit = (event) => {
    event.preventDefault();
    addRappel(rappel)
  }


  return (
    <form onSubmit={handleSubmit}>
      <label>Nom:
      <input 
        type="text" 
        name="nom" 
        value={rappel.nom || ""} 
        onChange={handleChange}
      />
      </label>
      <label>Criticite:
        <input 
          type="text" 
          name="criticite" 
          value={rappel.criticite || ""} 
          onChange={handleChange}
        />
        </label>
        <label>Date limite:
        <input 
          type="text" 
          name="date_limite" 
          value={rappel.date_limite || ""} 
          onChange={handleChange}
        />
        </label>
        <label>Repetition:
        <input 
          type="number" 
          name="repetition" 
          value={rappel.repetition || 0} 
          onChange={handleChange}
        />
        </label>
        <input type="submit" />
    </form>
  );
}