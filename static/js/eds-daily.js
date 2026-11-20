// The Ed's Daily signup form. Self-hosted, no dependency.
// Without this script the form posts to the endpoint and the browser follows its redirect. With
// it, the form is sent with fetch (Accept: application/json, no cookies) and the endpoint's answer
// shows in the form's status line (aria-live), so the visitor never leaves the page. The messages
// are the form's data-message-* attributes (templates/partials/eds-daily-form.html), picked by the
// endpoint's answer, never taken from the answer itself.
// On /daily/, the endpoint sends a visitor whose plain post failed back with ?error=...: the
// script shows the matching message there too.
(function () {
  "use strict";

  var forms = document.querySelectorAll(".eds-daily-form");
  if (!forms.length) return;

  // The answers the endpoint can give: "pending" for 202, or one of its error names.
  var KNOWN = ["pending", "invalid_email", "rate_limited", "unavailable"];

  function show(form, answer) {
    if (KNOWN.indexOf(answer) < 0) answer = "unavailable";
    var status = form.querySelector(".eds-daily-form__status");
    status.textContent = form.getAttribute("data-message-" + answer);
    form.setAttribute("data-answer", answer);
  }

  function answerFor(response) {
    if (response.status === 202) return "pending";
    if (response.status === 400) return "invalid_email";
    if (response.status === 429) return "rate_limited";
    return "unavailable";
  }

  if (!window.fetch || !window.FormData || !window.URLSearchParams) return;

  Array.prototype.forEach.call(forms, function (form) {
    var button = form.querySelector("button[type=submit]");
    form.addEventListener("submit", function (event) {
      event.preventDefault();
      if (button.disabled) return;
      button.disabled = true;
      form.setAttribute("aria-busy", "true");
      // A simple CORS request (form encoding, no custom headers but Accept): no preflight.
      fetch(form.action, {
        method: "POST",
        headers: { Accept: "application/json" },
        body: new URLSearchParams(new FormData(form)),
        credentials: "omit",
      })
        .then(answerFor, function () {
          return "unavailable";
        })
        .then(function (answer) {
          show(form, answer);
          if (answer === "pending") form.reset();
        })
        .then(function () {
          button.disabled = false;
          form.removeAttribute("aria-busy");
        });
    });
  });

  // /daily/?error=...: the endpoint's answer to a post sent without this script.
  var error = new URLSearchParams(window.location.search).get("error");
  if (error && KNOWN.indexOf(error) > 0) show(forms[0], error);
})();
