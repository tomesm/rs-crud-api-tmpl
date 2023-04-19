type WebMethod = "GET" | "POST" | "DELETE" | "PATCH";

const API_BASE_PATH = "/api";

export async function apiGet(path: string, data?: any) {
  return execute("GET", path, data);
}
export async function apiPost(path: string, data: any) {
  return execute("POST", path, data);
}
export async function apiPatch(path: string, data: any) {
  return execute("PATCH", path, data);
}
export async function apiDelete(path: string, data?: any) {
  return execute("DELETE", path, data);
}


async function execute(httpMethod: WebMethod, path: string, data?: any) {
  const url = `${API_BASE_PATH}/${path}`;

  const response = await fetch(url, {
    method: httpMethod,
    mode: 'same-origin',
    cache: 'no-cache',
    headers: {
      'Content-Type': 'application/json',
      'X-Auth-Token': '123'
    },
    body: JSON.stringify(data)
  });

  let res = await response.json();
  return res.data;
}
