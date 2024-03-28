function switch_language(lang) {
  var items = window.location.pathname.split("/");
  items[1] = lang;
  const url = items.join("/");
  //   console.log(url);
  window.open(url, "_blank").focus();
}
