pub const ANIM: &str = r#"
.hoverScale {
  @apply hover:scale-[1.05]
}

main{
  width: min(1200px, 90vw);
  margin: auto;
}
.slider{
  width: 100%;
  height: var(--height);
  overflow: hidden;
  mask-image: linear-gradient(
      to right,
      transparent,
      #000 10% 90%,
      transparent
  );
}
.slider .list{
  display: flex;
  width: 100%;
  min-width: calc(var(--width) * var(--quantity));
  position: relative;
}
.slider .list .item{
  width: var(--width);
  height: var(--height);
  position: absolute;
  left: 100%;
  animation: autoRun 28s linear infinite;
  transition: filter 0.5s;
  animation-delay: calc( (28s / var(--quantity)) * (var(--position) - 1) - 10s)!important;
}
.slider .list .item img{
  width: 100%;
}
@keyframes autoRun{
  from{
      left: 100%;
  }to{
      left: calc(var(--width) * -1);
  }
}
.slider:hover .item{
  animation-play-state: paused!important;
  filter: grayscale(1);
}
.slider .item:hover{
  filter: grayscale(0);
}
.slider[reverse="true"] .item{
  animation: reversePlay 10s linear infinite;
}
@keyframes reversePlay{
  from{
      left: calc(var(--width) * -1);
  }to{
      left: 100%;
  }
}

.pixel-corners,
.pixel-corners--wrapper {
  clip-path: polygon(0px calc(100% - 12px),
    3px calc(100% - 12px),
    3px calc(100% - 6px),
    6px calc(100% - 6px),
    6px calc(100% - 3px),
    12px calc(100% - 3px),
    12px 100%,
    calc(100% - 12px) 100%,
    calc(100% - 12px) calc(100% - 3px),
    calc(100% - 6px) calc(100% - 3px),
    calc(100% - 6px) calc(100% - 6px),
    calc(100% - 3px) calc(100% - 6px),
    calc(100% - 3px) calc(100% - 12px),
    100% calc(100% - 12px),
    100% 12px,
    calc(100% - 3px) 12px,
    calc(100% - 3px) 6px,
    calc(100% - 6px) 6px,
    calc(100% - 6px) 3px,
    calc(100% - 12px) 3px,
    calc(100% - 12px) 0px,
    12px 0px,
    12px 3px,
    6px 3px,
    6px 6px,
    3px 6px,
    3px 12px,
    0px 12px);
  position: relative;
}
.pixel-corners {
  border: 3px solid transparent;
}
.pixel-corners--wrapper {
  width: fit-content;
  height: fit-content;
}
.pixel-corners--wrapper .pixel-corners {
  display: block;
  clip-path: polygon(3px 12px,
    6px 12px,
    6px 6px,
    12px 6px,
    12px 3px,
    calc(100% - 12px) 3px,
    calc(100% - 12px) 6px,
    calc(100% - 6px) 6px,
    calc(100% - 6px) 12px,
    calc(100% - 3px) 12px,
    calc(100% - 3px) calc(100% - 12px),
    calc(100% - 6px) calc(100% - 12px),
    calc(100% - 6px) calc(100% - 6px),
    calc(100% - 12px) calc(100% - 6px),
    calc(100% - 12px) calc(100% - 3px),
    12px calc(100% - 3px),
    12px calc(100% - 6px),
    6px calc(100% - 6px),
    6px calc(100% - 12px),
    3px calc(100% - 12px));
}
.pixel-corners::after,
.pixel-corners--wrapper::after {
  content: "";
  position: absolute;
  clip-path: polygon(0px calc(100% - 12px),
    3px calc(100% - 12px),
    3px calc(100% - 6px),
    6px calc(100% - 6px),
    6px calc(100% - 3px),
    12px calc(100% - 3px),
    12px 100%,
    calc(100% - 12px) 100%,
    calc(100% - 12px) calc(100% - 3px),
    calc(100% - 6px) calc(100% - 3px),
    calc(100% - 6px) calc(100% - 6px),
    calc(100% - 3px) calc(100% - 6px),
    calc(100% - 3px) calc(100% - 12px),
    100% calc(100% - 12px),
    100% 12px,
    calc(100% - 3px) 12px,
    calc(100% - 3px) 6px,
    calc(100% - 6px) 6px,
    calc(100% - 6px) 3px,
    calc(100% - 12px) 3px,
    calc(100% - 12px) 0px,
    12px 0px,
    12px 3px,
    6px 3px,
    6px 6px,
    3px 6px,
    3px 12px,
    0px 12px,
    0px 50%,
    3px 50%,
    3px 12px,
    6px 12px,
    6px 6px,
    12px 6px,
    12px 3px,
    calc(100% - 12px) 3px,
    calc(100% - 12px) 6px,
    calc(100% - 6px) 6px,
    calc(100% - 6px) 12px,
    calc(100% - 3px) 12px,
    calc(100% - 3px) calc(100% - 12px),
    calc(100% - 6px) calc(100% - 12px),
    calc(100% - 6px) calc(100% - 6px),
    calc(100% - 12px) calc(100% - 6px),
    calc(100% - 12px) calc(100% - 3px),
    12px calc(100% - 3px),
    12px calc(100% - 6px),
    6px calc(100% - 6px),
    6px calc(100% - 12px),
    3px calc(100% - 12px),
    3px 50%,
    0px 50%);
  top: 0;
  bottom: 0;
  left: 0;
  right: 0;
  background: #935cd1;
  display: block;
  pointer-events: none;
}
.pixel-corners::after {
  margin: -3px;
}

.font-outline-2 {
  text-shadow: 1px 1px black;
}

.bg-slideshow {
  background-position: center;
  height: 100%;
  @apply px-10 py-[15px] lg:bg-bg-pc bg-bg-mobile lg:bg-no-repeat lg:text-[24px] text-[21px] sm:text-[21px] hover:scale-[0.95] active:scale-[1]
}


#links {
    width: 400px;
    text-align: left;
    font-size: x-large;

    @apply text-slate-900;

    display: flex;
    flex-direction: column;
}

#links a {
    @apply text-slate-900;
    text-decoration: none;
    margin-top: 20px;
    margin: 10px;
    border: white 1px solid;
    border-radius: 5px;
    padding: 10px;
}

#links a:hover {
    background-color: #1f1f1f;
    cursor: pointer;
}

#header {
    max-width: 1200px;
}

"#;