export function appendStringToBody(value) {
  const text = document.createTextNode(value);
  document.body.appendChild(text);
}

export const alert = window.alert;
