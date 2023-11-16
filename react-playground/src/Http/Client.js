const client = "http://localhost:7878"


export async function post(path, params, body) {
    return await fetch(client + path , {
      method: "POST",
      body: JSON.stringify(body)
    })
    .then(response =>response.json())
    .catch(error => {throw new Error(error)});
  }

  export async function dele(path, params, body) {
    return await fetch(client + path + "/" + params, {
      method: "DELETE",
      body: JSON.stringify(body)
    })
    .then(response =>response.json())
    .catch(error => {"ouii"});
  }


