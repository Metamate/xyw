let container = document.querySelector("#container");
let wrapper = document.querySelector("#wrapper");
let canvas;

let canvasWidth;
let canvasHeight;

document
  .querySelector("#fullscreen")
  .addEventListener("click", requestFullScreen, false);

function requestFullScreen() {
  if (!document.fullscreenElement) {
    canvas.requestFullscreen();
  }
}

window.addEventListener(
  "load",
  function () {
    const interval = setInterval(() => {
      canvas = document.querySelector("canvas");

      if (canvas) {
        clearInterval(interval);
        canvas.onselectstart = () => {
          return false;
        };

        canvasWidth = canvas.getAttribute("width");
        canvasHeight = canvas.getAttribute("height");

        screen.orientation.addEventListener("change", (event) => {
          if (document.fullscreenElement) {
            document.exitFullscreen();
          }
        });

        container.style.maxWidth = canvasWidth + "px";
        container.style.maxHeight = canvasHeight + "px";

        container.appendChild(canvas);
        document.querySelector("#loading").style.display = "none";
        canvas.style.display = "block";
        wrapper.style.display = "block";
      }
    }, 50);
  },
  false
);
