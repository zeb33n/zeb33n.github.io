export function print_to_html(s){
  stdout = document.getElementById("stdout"); 
  stdout.innerHTML += "<div>"
  stdout.innerHTML += s;
  stdout.innerHTML += "</div>";
}
