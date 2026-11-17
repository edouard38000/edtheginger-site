// Playbook pages: the Copy button of each prompt box.
// The button is in the page with `hidden`; this script shows it and copies the exact text of the
// prompt (the <pre> next to it, without the example under it). Without this script the buttons
// stay hidden and the text can still be selected. No third-party code.
(function () {
  "use strict";

  function copyWithSelection(text) {
    var area = document.createElement("textarea");
    area.value = text;
    area.setAttribute("readonly", "");
    area.style.position = "fixed";
    area.style.top = "-1000px";
    document.body.appendChild(area);
    area.select();
    var copied = false;
    try {
      copied = document.execCommand("copy");
    } catch (error) {
      copied = false;
    }
    document.body.removeChild(area);
    return copied ? Promise.resolve() : Promise.reject(new Error("copy failed"));
  }

  function copy(text) {
    if (navigator.clipboard && window.isSecureContext) {
      return navigator.clipboard.writeText(text).catch(function () {
        return copyWithSelection(text);
      });
    }
    return copyWithSelection(text);
  }

  function setUp(button) {
    var box = button.closest(".pb-prompt__box");
    var source = box && box.querySelector(".pb-prompt__text");
    if (!source) {
      return;
    }
    var timer = 0;
    button.setAttribute("aria-live", "polite");
    button.hidden = false;
    button.addEventListener("click", function () {
      copy(source.textContent).then(
        function () {
          button.textContent = "Copied";
          button.classList.add("is-copied");
        },
        function () {
          button.textContent = "Select and copy";
        }
      ).then(function () {
        window.clearTimeout(timer);
        timer = window.setTimeout(function () {
          button.textContent = "Copy";
          button.classList.remove("is-copied");
        }, 2000);
      });
    });
  }

  document.querySelectorAll("[data-copy]").forEach(setUp);
})();
