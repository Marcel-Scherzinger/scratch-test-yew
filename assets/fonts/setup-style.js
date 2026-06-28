function onBaseVisit() {
    const base = document.querySelector('base');
    const href = base ? base.getAttribute('href') : '';

    const cssText = `
        @font-face {
            font-family: 'Material Symbols Outlined';
            font-style: normal;
            font-weight: 400;
            src: url(${href}/MaterialIconsOutlined-Regular.woff2) format('woff2');
        }

        .material-symbols-outlined {
            font-family: 'Material Symbols Outlined';
            font-weight: normal;
            font-style: normal;
            font-size: 24px;
            line-height: 1;
            letter-spacing: normal;
            text-transform: none;
            display: inline-block;
            white-space: nowrap;
            word-wrap: normal;
            direction: ltr;
        }`;
    console.log(cssText);

    const style = document.createElement('style');
    // style.type = 'text/css';
    style.appendChild(document.createTextNode(cssText));
    document.head.appendChild(style);
}

window.addEventListener('DOMContentLoaded', onBaseVisit);
