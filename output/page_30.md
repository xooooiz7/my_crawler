if(navigator.userAgent.match(/MSIE|Internet Explorer/i)||navigator.userAgent.match(/Trident\\/7\\..\*?rv:11/i)){var href=document.location.href;if(!href.match(/[?&]nowprocket/)){if(href.indexOf("?")==-1){if(href.indexOf("#")==-1){document.location.href=href+"?nowprocket=1"}else{document.location.href=href.replace("#","?nowprocket=1#")}}else{if(href.indexOf("#")==-1){document.location.href=href+"&nowprocket=1"}else{document.location.href=href.replace("#","&nowprocket=1#")}}}}class RocketLazyLoadScripts{constructor(){this.v="1.2.5.1",this.triggerEvents=["keydown","mousedown","mousemove","touchmove","touchstart","touchend","wheel"],this.userEventHandler=this.\_triggerListener.bind(this),this.touchStartHandler=this.\_onTouchStart.bind(this),this.touchMoveHandler=this.\_onTouchMove.bind(this),this.touchEndHandler=this.\_onTouchEnd.bind(this),this.clickHandler=this.\_onClick.bind(this),this.interceptedClicks=[],this.interceptedClickListeners=[],this.\_interceptClickListeners(this),window.addEventListener("pageshow",e=\>{this.persisted=e.persisted,this.everythingLoaded&&this.\_triggerLastFunctions()}),document.addEventListener("DOMContentLoaded",()=\>{this.\_preconnect3rdParties()}),this.delayedScripts={normal:[],async:[],defer:[]},this.trash=[],this.allJQueries=[]}\_addUserInteractionListener(e){if(document.hidden){e.\_triggerListener();return}this.triggerEvents.forEach(t=\>window.addEventListener(t,e.userEventHandler,{passive:!0})),window.addEventListener("touchstart",e.touchStartHandler,{passive:!0}),window.addEventListener("mousedown",e.touchStartHandler),document.addEventListener("visibilitychange",e.userEventHandler)}\_removeUserInteractionListener(){this.triggerEvents.forEach(e=\>window.removeEventListener(e,this.userEventHandler,{passive:!0})),document.removeEventListener("visibilitychange",this.userEventHandler)}\_onTouchStart(e){"HTML"!==e.target.tagName&&(window.addEventListener("touchend",this.touchEndHandler),window.addEventListener("mouseup",this.touchEndHandler),window.addEventListener("touchmove",this.touchMoveHandler,{passive:!0}),window.addEventListener("mousemove",this.touchMoveHandler),e.target.addEventListener("click",this.clickHandler),this.\_disableOtherEventListeners(e.target,!0),this.\_renameDOMAttribute(e.target,"onclick","rocket-onclick"),this.\_pendingClickStarted())}\_onTouchMove(e){window.removeEventListener("touchend",this.touchEndHandler),window.removeEventListener("mouseup",this.touchEndHandler),window.removeEventListener("touchmove",this.touchMoveHandler,{passive:!0}),window.removeEventListener("mousemove",this.touchMoveHandler),e.target.removeEventListener("click",this.clickHandler),this.\_disableOtherEventListeners(e.target,!1),this.\_renameDOMAttribute(e.target,"rocket-onclick","onclick"),this.\_pendingClickFinished()}\_onTouchEnd(){window.removeEventListener("touchend",this.touchEndHandler),window.removeEventListener("mouseup",this.touchEndHandler),window.removeEventListener("touchmove",this.touchMoveHandler,{passive:!0}),window.removeEventListener("mousemove",this.touchMoveHandler)}\_onClick(e){e.target.removeEventListener("click",this.clickHandler),this.\_disableOtherEventListeners(e.target,!1),this.\_renameDOMAttribute(e.target,"rocket-onclick","onclick"),this.interceptedClicks.push(e),e.preventDefault(),e.stopPropagation(),e.stopImmediatePropagation(),this.\_pendingClickFinished()}\_replayClicks(){window.removeEventListener("touchstart",this.touchStartHandler,{passive:!0}),window.removeEventListener("mousedown",this.touchStartHandler),this.interceptedClicks.forEach(e=\>{e.target.dispatchEvent(new MouseEvent("click",{view:e.view,bubbles:!0,cancelable:!0}))})}\_interceptClickListeners(e){EventTarget.prototype.addEventListenerBase=EventTarget.prototype.addEventListener,EventTarget.prototype.addEventListener=function(t,i,r){"click"!==t||e.windowLoaded||i===e.clickHandler||e.interceptedClickListeners.push({target:this,func:i,options:r}),(this||window).addEventListenerBase(t,i,r)}}\_disableOtherEventListeners(e,t){this.interceptedClickListeners.forEach(i=\>{i.target===e&&(t?e.removeEventListener("click",i.func,i.options):e.addEventListener("click",i.func,i.options))}),e.parentNode!==document.documentElement&&this.\_disableOtherEventListeners(e.parentNode,t)}\_waitForPendingClicks(){return new Promise(e=\>{this.\_isClickPending?this.\_pendingClickFinished=e:e()})}\_pendingClickStarted(){this.\_isClickPending=!0}\_pendingClickFinished(){this.\_isClickPending=!1}\_renameDOMAttribute(e,t,i){e.hasAttribute&&e.hasAttribute(t)&&(event.target.setAttribute(i,event.target.getAttribute(t)),event.target.removeAttribute(t))}\_triggerListener(){this.\_removeUserInteractionListener(this),"loading"===document.readyState?document.addEventListener("DOMContentLoaded",this.\_loadEverythingNow.bind(this)):this.\_loadEverythingNow()}\_preconnect3rdParties(){let e=[];document.querySelectorAll("script[type=rocketlazyloadscript][data-rocket-src]").forEach(t=\>{let i=t.getAttribute("data-rocket-src");if(i&&0!==i.indexOf("data:")){0===i.indexOf("//")&&(i=location.protocol+i);try{let r=new URL(i).origin;r!==location.origin&&e.push({src:r,crossOrigin:t.crossOrigin||"module"===t.getAttribute("data-rocket-type")})}catch(n){}}}),e=[...new Map(e.map(e=\>[JSON.stringify(e),e])).values()],this.\_batchInjectResourceHints(e,"preconnect")}async \_loadEverythingNow(){this.lastBreath=Date.now(),this.\_delayEventListeners(this),this.\_delayJQueryReady(this),this.\_handleDocumentWrite(),this.\_registerAllDelayedScripts(),this.\_preloadAllScripts(),await this.\_loadScriptsFromList(this.delayedScripts.normal),await this.\_loadScriptsFromList(this.delayedScripts.defer),await this.\_loadScriptsFromList(this.delayedScripts.async);try{await this.\_triggerDOMContentLoaded(),await this.\_pendingWebpackRequests(this),await this.\_triggerWindowLoad()}catch(e){console.error(e)}window.dispatchEvent(new Event("rocket-allScriptsLoaded")),this.everythingLoaded=!0,this.\_waitForPendingClicks().then(()=\>{this.\_replayClicks()}),this.\_emptyTrash()}\_registerAllDelayedScripts(){document.querySelectorAll("script[type=rocketlazyloadscript]").forEach(e=\>{e.hasAttribute("data-rocket-src")?e.hasAttribute("async")&&!1!==e.async?this.delayedScripts.async.push(e):e.hasAttribute("defer")&&!1!==e.defer||"module"===e.getAttribute("data-rocket-type")?this.delayedScripts.defer.push(e):this.delayedScripts.normal.push(e):this.delayedScripts.normal.push(e)})}async \_transformScript(e){if(await this.\_littleBreath(),!0===e.noModule&&"noModule"in HTMLScriptElement.prototype){e.setAttribute("data-rocket-status","skipped");return}return new Promise(t=\>{let i;function r(){(i||e).setAttribute("data-rocket-status","executed"),t()}try{if(navigator.userAgent.indexOf("Firefox/")\>0||""===navigator.vendor)i=document.createElement("script"),[...e.attributes].forEach(e=\>{let t=e.nodeName;"type"!==t&&("data-rocket-type"===t&&(t="type"),"data-rocket-src"===t&&(t="src"),i.setAttribute(t,e.nodeValue))}),e.text&&(i.text=e.text),i.hasAttribute("src")?(i.addEventListener("load",r),i.addEventListener("error",function(){i.setAttribute("data-rocket-status","failed"),t()}),setTimeout(()=\>{i.isConnected||t()},1)):(i.text=e.text,r()),e.parentNode.replaceChild(i,e);else{let n=e.getAttribute("data-rocket-type"),s=e.getAttribute("data-rocket-src");n?(e.type=n,e.removeAttribute("data-rocket-type")):e.removeAttribute("type"),e.addEventListener("load",r),e.addEventListener("error",function(){e.setAttribute("data-rocket-status","failed"),t()}),s?(e.removeAttribute("data-rocket-src"),e.src=s):e.src="data:text/javascript;base64,"+window.btoa(unescape(encodeURIComponent(e.text)))}}catch(a){e.setAttribute("data-rocket-status","failed"),t()}})}async \_loadScriptsFromList(e){let t=e.shift();return t&&t.isConnected?(await this.\_transformScript(t),this.\_loadScriptsFromList(e)):Promise.resolve()}\_preloadAllScripts(){this.\_batchInjectResourceHints([...this.delayedScripts.normal,...this.delayedScripts.defer,...this.delayedScripts.async],"preload")}\_batchInjectResourceHints(e,t){var i=document.createDocumentFragment();e.forEach(e=\>{let r=e.getAttribute&&e.getAttribute("data-rocket-src")||e.src;if(r){let n=document.createElement("link");n.href=r,n.rel=t,"preconnect"!==t&&(n.as="script"),e.getAttribute&&"module"===e.getAttribute("data-rocket-type")&&(n.crossOrigin=!0),e.crossOrigin&&(n.crossOrigin=e.crossOrigin),e.integrity&&(n.integrity=e.integrity),i.appendChild(n),this.trash.push(n)}}),document.head.appendChild(i)}\_delayEventListeners(e){let t={};function i(i,r){return t[r].eventsToRewrite.indexOf(i)\>=0&&!e.everythingLoaded?"rocket-"+i:i}function r(e,r){var n;!t[n=e]&&(t[n]={originalFunctions:{add:n.addEventListener,remove:n.removeEventListener},eventsToRewrite:[]},n.addEventListener=function(){arguments[0]=i(arguments[0],n),t[n].originalFunctions.add.apply(n,arguments)},n.removeEventListener=function(){arguments[0]=i(arguments[0],n),t[n].originalFunctions.remove.apply(n,arguments)}),t[e].eventsToRewrite.push(r)}function n(t,i){let r=t[i];t[i]=null,Object.defineProperty(t,i,{get:()=\>r||function(){},set(n){e.everythingLoaded?r=n:t["rocket"+i]=r=n}})}r(document,"DOMContentLoaded"),r(window,"DOMContentLoaded"),r(window,"load"),r(window,"pageshow"),r(document,"readystatechange"),n(document,"onreadystatechange"),n(window,"onload"),n(window,"onpageshow")}\_delayJQueryReady(e){let t;function i(t){return e.everythingLoaded?t:t.split(" ").map(e=\>"load"===e||0===e.indexOf("load.")?"rocket-jquery-load":e).join(" ")}function r(r){if(r&&r.fn&&!e.allJQueries.includes(r)){r.fn.ready=r.fn.init.prototype.ready=function(t){return e.domReadyFired?t.bind(document)(r):document.addEventListener("rocket-DOMContentLoaded",()=\>t.bind(document)(r)),r([])};let n=r.fn.on;r.fn.on=r.fn.init.prototype.on=function(){return this[0]===window&&("string"==typeof arguments[0]||arguments[0]instanceof String?arguments[0]=i(arguments[0]):"object"==typeof arguments[0]&&Object.keys(arguments[0]).forEach(e=\>{let t=arguments[0][e];delete arguments[0][e],arguments[0][i(e)]=t})),n.apply(this,arguments),this},e.allJQueries.push(r)}t=r}r(window.jQuery),Object.defineProperty(window,"jQuery",{get:()=\>t,set(e){r(e)}})}async \_pendingWebpackRequests(e){let t=document.querySelector("script[data-webpack]");async function i(){return new Promise(e=\>{t.addEventListener("load",e),t.addEventListener("error",e)})}t&&(await i(),await e.\_requestAnimFrame(),await e.\_pendingWebpackRequests(e))}async \_triggerDOMContentLoaded(){this.domReadyFired=!0,await this.\_littleBreath(),document.dispatchEvent(new Event("rocket-readystatechange")),await this.\_littleBreath(),document.rocketonreadystatechange&&document.rocketonreadystatechange(),await this.\_littleBreath(),document.dispatchEvent(new Event("rocket-DOMContentLoaded")),await this.\_littleBreath(),window.dispatchEvent(new Event("rocket-DOMContentLoaded"))}async \_triggerWindowLoad(){await this.\_littleBreath(),document.dispatchEvent(new Event("rocket-readystatechange")),await this.\_littleBreath(),document.rocketonreadystatechange&&document.rocketonreadystatechange(),await this.\_littleBreath(),window.dispatchEvent(new Event("rocket-load")),await this.\_littleBreath(),window.rocketonload&&window.rocketonload(),await this.\_littleBreath(),this.allJQueries.forEach(e=\>e(window).trigger("rocket-jquery-load")),await this.\_littleBreath();let e=new Event("rocket-pageshow");e.persisted=this.persisted,window.dispatchEvent(e),await this.\_littleBreath(),window.rocketonpageshow&&window.rocketonpageshow({persisted:this.persisted}),this.windowLoaded=!0}\_triggerLastFunctions(){document.onreadystatechange&&document.onreadystatechange(),window.onload&&window.onload(),window.onpageshow&&window.onpageshow({persisted:this.persisted})}\_handleDocumentWrite(){let e=new Map;document.write=document.writeln=function(t){let i=document.currentScript;i||console.error("WPRocket unable to document.write this: "+t);let r=document.createRange(),n=i.parentElement,s=e.get(i);void 0===s&&(s=i.nextSibling,e.set(i,s));let a=document.createDocumentFragment();r.setStart(a,0),a.appendChild(r.createContextualFragment(t)),n.insertBefore(a,s)}}async \_littleBreath(){Date.now()-this.lastBreath\>45&&(await this.\_requestAnimFrame(),this.lastBreath=Date.now())}async \_requestAnimFrame(){return document.hidden?new Promise(e=\>setTimeout(e)):new Promise(e=\>requestAnimationFrame(e))}\_emptyTrash(){this.trash.forEach(e=\>e.remove())}static run(){let e=new RocketLazyLoadScripts;e.\_addUserInteractionListener(e)}}RocketLazyLoadScripts.run(); ประกันลดหย่อนภาษี ลดได้เท่าไหร่? คำนวณซื้อประกันลดหย่อน 2568 :root{--wp-admin-theme-color:#007cba;--wp-admin-theme-color--rgb:0,124,186;--wp-admin-theme-color-darker-10:#006ba1;--wp-admin-theme-color-darker-10--rgb:0,107,161;--wp-admin-theme-color-darker-20:#005a87;--wp-admin-theme-color-darker-20--rgb:0,90,135;--wp-admin-border-width-focus:2px;--wp-block-synced-color:#7a00df;--wp-block-synced-color--rgb:122,0,223}@media (min-resolution:192dpi){:root{--wp-admin-border-width-focus:1.5px}}.wp-element-button{cursor:pointer}:root{--wp--preset--font-size--normal:16px;--wp--preset--font-size--huge:42px}:root .has-very-light-gray-background-color{background-color:#eee}:root .has-very-dark-gray-background-color{background-color:#313131}:root .has-very-light-gray-color{color:#eee}:root .has-very-dark-gray-color{color:#313131}:root .has-vivid-green-cyan-to-vivid-cyan-blue-gradient-background{background:linear-gradient(135deg,#00d084,#0693e3)}:root .has-purple-crush-gradient-background{background:linear-gradient(135deg,#34e2e4,#4721fb 50%,#ab1dfe)}:root .has-hazy-dawn-gradient-background{background:linear-gradient(135deg,#faaca8,#dad0ec)}:root .has-subdued-olive-gradient-background{background:linear-gradient(135deg,#fafae1,#67a671)}:root .has-atomic-cream-gradient-background{background:linear-gradient(135deg,#fdd79a,#004a59)}:root .has-nightshade-gradient-background{background:linear-gradient(135deg,#330968,#31cdcf)}:root .has-midnight-gradient-background{background:linear-gradient(135deg,#020381,#2874fc)}.has-regular-font-size{font-size:1em}.has-larger-font-size{font-size:2.625em}.has-normal-font-size{font-size:var(--wp--preset--font-size--normal)}.has-huge-font-size{font-size:var(--wp--preset--font-size--huge)}.has-text-align-center{text-align:center}.has-text-align-left{text-align:left}.has-text-align-right{text-align:right}#end-resizable-editor-section{display:none}.aligncenter{clear:both}.items-justified-left{justify-content:flex-start}.items-justified-center{justify-content:center}.items-justified-right{justify-content:flex-end}.items-justified-space-between{justify-content:space-between}.screen-reader-text{clip:rect(1px,1px,1px,1px);word-wrap:normal!important;border:0;-webkit-clip-path:inset(50%);clip-path:inset(50%);height:1px;margin:-1px;overflow:hidden;padding:0;position:absolute;width:1px}.screen-reader-text:focus{clip:auto!important;background-color:#ddd;-webkit-clip-path:none;clip-path:none;color:#444;display:block;font-size:1em;height:auto;left:5px;line-height:normal;padding:15px 23px 14px;text-decoration:none;top:5px;width:auto;z-index:100000}html :where(.has-border-color){border-style:solid}html :where([style\*=border-top-color]){border-top-style:solid}html :where([style\*=border-right-color]){border-right-style:solid}html :where([style\*=border-bottom-color]){border-bottom-style:solid}html :where([style\*=border-left-color]){border-left-style:solid}html :where([style\*=border-width]){border-style:solid}html :where([style\*=border-top-width]){border-top-style:solid}html :where([style\*=border-right-width]){border-right-style:solid}html :where([style\*=border-bottom-width]){border-bottom-style:solid}html :where([style\*=border-left-width]){border-left-style:solid}html :where(img[class\*=wp-image-]){height:auto;max-width:100%}:where(figure){margin:0 0 1em}html :where(.is-position-sticky){--wp-admin--admin-bar--position-offset:var(--wp-admin--admin-bar--height,0px)}@media screen and (max-width:600px){html :where(.is-position-sticky){--wp-admin--admin-bar--position-offset:0px}}/\*! This file is auto-generated \*/
.wp-block-button\_\_link{color:#fff;background-color:#32373c;border-radius:9999px;box-shadow:none;text-decoration:none;padding:calc(.667em + 2px) calc(1.333em + 2px);font-size:1.125em}.wp-block-file\_\_button{background:#32373c;color:#fff;text-decoration:none}body{--wp--preset--color--black: #000000;--wp--preset--color--cyan-bluish-gray: #abb8c3;--wp--preset--color--white: #ffffff;--wp--preset--color--pale-pink: #f78da7;--wp--preset--color--vivid-red: #cf2e2e;--wp--preset--color--luminous-vivid-orange: #ff6900;--wp--preset--color--luminous-vivid-amber: #fcb900;--wp--preset--color--light-green-cyan: #7bdcb5;--wp--preset--color--vivid-green-cyan: #00d084;--wp--preset--color--pale-cyan-blue: #8ed1fc;--wp--preset--color--vivid-cyan-blue: #0693e3;--wp--preset--color--vivid-purple: #9b51e0;--wp--preset--color--base: #ffffff;--wp--preset--color--contrast: #000000;--wp--preset--color--primary: #9DFF20;--wp--preset--color--secondary: #345C00;--wp--preset--color--tertiary: #F6F6F6;--wp--preset--gradient--vivid-cyan-blue-to-vivid-purple: linear-gradient(135deg,rgba(6,147,227,1) 0%,rgb(155,81,224) 100%);--wp--preset--gradient--light-green-cyan-to-vivid-green-cyan: linear-gradient(135deg,rgb(122,220,180) 0%,rgb(0,208,130) 100%);--wp--preset--gradient--luminous-vivid-amber-to-luminous-vivid-orange: linear-gradient(135deg,rgba(252,185,0,1) 0%,rgba(255,105,0,1) 100%);--wp--preset--gradient--luminous-vivid-orange-to-vivid-red: linear-gradient(135deg,rgba(255,105,0,1) 0%,rgb(207,46,46) 100%);--wp--preset--gradient--very-light-gray-to-cyan-bluish-gray: linear-gradient(135deg,rgb(238,238,238) 0%,rgb(169,184,195) 100%);--wp--preset--gradient--cool-to-warm-spectrum: linear-gradient(135deg,rgb(74,234,220) 0%,rgb(151,120,209) 20%,rgb(207,42,186) 40%,rgb(238,44,130) 60%,rgb(251,105,98) 80%,rgb(254,248,76) 100%);--wp--preset--gradient--blush-light-purple: linear-gradient(135deg,rgb(255,206,236) 0%,rgb(152,150,240) 100%);--wp--preset--gradient--blush-bordeaux: linear-gradient(135deg,rgb(254,205,165) 0%,rgb(254,45,45) 50%,rgb(107,0,62) 100%);--wp--preset--gradient--luminous-dusk: linear-gradient(135deg,rgb(255,203,112) 0%,rgb(199,81,192) 50%,rgb(65,88,208) 100%);--wp--preset--gradient--pale-ocean: linear-gradient(135deg,rgb(255,245,203) 0%,rgb(182,227,212) 50%,rgb(51,167,181) 100%);--wp--preset--gradient--electric-grass: linear-gradient(135deg,rgb(202,248,128) 0%,rgb(113,206,126) 100%);--wp--preset--gradient--midnight: linear-gradient(135deg,rgb(2,3,129) 0%,rgb(40,116,252) 100%);--wp--preset--font-size--small: clamp(0.875rem, 0.875rem + ((1vw - 0.2rem) \* 0.227), 1rem);--wp--preset--font-size--medium: clamp(1rem, 1rem + ((1vw - 0.2rem) \* 0.227), 1.125rem);--wp--preset--font-size--large: clamp(1.75rem, 1.75rem + ((1vw - 0.2rem) \* 0.227), 1.875rem);--wp--preset--font-size--x-large: 2.25rem;--wp--preset--font-size--xx-large: clamp(4rem, 4rem + ((1vw - 0.2rem) \* 10.909), 10rem);--wp--preset--font-family--dm-sans: "DM Sans", sans-serif;--wp--preset--font-family--ibm-plex-mono: 'IBM Plex Mono', monospace;--wp--preset--font-family--inter: "Inter", sans-serif;--wp--preset--font-family--system-font: -apple-system,BlinkMacSystemFont,"Segoe UI",Roboto,Oxygen-Sans,Ubuntu,Cantarell,"Helvetica Neue",sans-serif;--wp--preset--font-family--source-serif-pro: "Source Serif Pro", serif;--wp--preset--spacing--30: clamp(1.5rem, 5vw, 2rem);--wp--preset--spacing--40: clamp(1.8rem, 1.8rem + ((1vw - 0.48rem) \* 2.885), 3rem);--wp--preset--spacing--50: clamp(2.5rem, 8vw, 4.5rem);--wp--preset--spacing--60: clamp(3.75rem, 10vw, 7rem);--wp--preset--spacing--70: clamp(5rem, 5.25rem + ((1vw - 0.48rem) \* 9.096), 8rem);--wp--preset--spacing--80: clamp(7rem, 14vw, 11rem);--wp--preset--shadow--natural: 6px 6px 9px rgba(0, 0, 0, 0.2);--wp--preset--shadow--deep: 12px 12px 50px rgba(0, 0, 0, 0.4);--wp--preset--shadow--sharp: 6px 6px 0px rgba(0, 0, 0, 0.2);--wp--preset--shadow--outlined: 6px 6px 0px -3px rgba(255, 255, 255, 1), 6px 6px rgba(0, 0, 0, 1);--wp--preset--shadow--crisp: 6px 6px 0px rgba(0, 0, 0, 1);}:where(body .is-layout-flow) \> :first-child:first-child{margin-block-start: 0;}:where(body .is-layout-flow) \> :last-child:last-child{margin-block-end: 0;}:where(body .is-layout-flow) \> \*{margin-block-start: 1.5rem;margin-block-end: 0;}:where(body .is-layout-constrained) \> :first-child:first-child{margin-block-start: 0;}:where(body .is-layout-constrained) \> :last-child:last-child{margin-block-end: 0;}:where(body .is-layout-constrained) \> \*{margin-block-start: 1.5rem;margin-block-end: 0;}:where(body .is-layout-flex) {gap: 1.5rem;}:where(body .is-layout-grid) {gap: 1.5rem;}body .is-layout-flow \> .alignleft{float: left;margin-inline-start: 0;margin-inline-end: 2em;}body .is-layout-flow \> .alignright{float: right;margin-inline-start: 2em;margin-inline-end: 0;}body .is-layout-flow \> .aligncenter{margin-left: auto !important;margin-right: auto !important;}body .is-layout-constrained \> .alignleft{float: left;margin-inline-start: 0;margin-inline-end: 2em;}body .is-layout-constrained \> .alignright{float: right;margin-inline-start: 2em;margin-inline-end: 0;}body .is-layout-constrained \> .aligncenter{margin-left: auto !important;margin-right: auto !important;}body .is-layout-constrained \> :where(:not(.alignleft):not(.alignright):not(.alignfull)){max-width: var(--wp--style--global--content-size);margin-left: auto !important;margin-right: auto !important;}body .is-layout-constrained \> .alignwide{max-width: var(--wp--style--global--wide-size);}body .is-layout-flex{display: flex;}body .is-layout-flex{flex-wrap: wrap;align-items: center;}body .is-layout-flex \> \*{margin: 0;}body .is-layout-grid{display: grid;}body .is-layout-grid \> \*{margin: 0;}.has-black-color{color: var(--wp--preset--color--black) !important;}.has-cyan-bluish-gray-color{color: var(--wp--preset--color--cyan-bluish-gray) !important;}.has-white-color{color: var(--wp--preset--color--white) !important;}.has-pale-pink-color{color: var(--wp--preset--color--pale-pink) !important;}.has-vivid-red-color{color: var(--wp--preset--color--vivid-red) !important;}.has-luminous-vivid-orange-color{color: var(--wp--preset--color--luminous-vivid-orange) !important;}.has-luminous-vivid-amber-color{color: var(--wp--preset--color--luminous-vivid-amber) !important;}.has-light-green-cyan-color{color: var(--wp--preset--color--light-green-cyan) !important;}.has-vivid-green-cyan-color{color: var(--wp--preset--color--vivid-green-cyan) !important;}.has-pale-cyan-blue-color{color: var(--wp--preset--color--pale-cyan-blue) !important;}.has-vivid-cyan-blue-color{color: var(--wp--preset--color--vivid-cyan-blue) !important;}.has-vivid-purple-color{color: var(--wp--preset--color--vivid-purple) !important;}.has-black-background-color{background-color: var(--wp--preset--color--black) !important;}.has-cyan-bluish-gray-background-color{background-color: var(--wp--preset--color--cyan-bluish-gray) !important;}.has-white-background-color{background-color: var(--wp--preset--color--white) !important;}.has-pale-pink-background-color{background-color: var(--wp--preset--color--pale-pink) !important;}.has-vivid-red-background-color{background-color: var(--wp--preset--color--vivid-red) !important;}.has-luminous-vivid-orange-background-color{background-color: var(--wp--preset--color--luminous-vivid-orange) !important;}.has-luminous-vivid-amber-background-color{background-color: var(--wp--preset--color--luminous-vivid-amber) !important;}.has-light-green-cyan-background-color{background-color: var(--wp--preset--color--light-green-cyan) !important;}.has-vivid-green-cyan-background-color{background-color: var(--wp--preset--color--vivid-green-cyan) !important;}.has-pale-cyan-blue-background-color{background-color: var(--wp--preset--color--pale-cyan-blue) !important;}.has-vivid-cyan-blue-background-color{background-color: var(--wp--preset--color--vivid-cyan-blue) !important;}.has-vivid-purple-background-color{background-color: var(--wp--preset--color--vivid-purple) !important;}.has-black-border-color{border-color: var(--wp--preset--color--black) !important;}.has-cyan-bluish-gray-border-color{border-color: var(--wp--preset--color--cyan-bluish-gray) !important;}.has-white-border-color{border-color: var(--wp--preset--color--white) !important;}.has-pale-pink-border-color{border-color: var(--wp--preset--color--pale-pink) !important;}.has-vivid-red-border-color{border-color: var(--wp--preset--color--vivid-red) !important;}.has-luminous-vivid-orange-border-color{border-color: var(--wp--preset--color--luminous-vivid-orange) !important;}.has-luminous-vivid-amber-border-color{border-color: var(--wp--preset--color--luminous-vivid-amber) !important;}.has-light-green-cyan-border-color{border-color: var(--wp--preset--color--light-green-cyan) !important;}.has-vivid-green-cyan-border-color{border-color: var(--wp--preset--color--vivid-green-cyan) !important;}.has-pale-cyan-blue-border-color{border-color: var(--wp--preset--color--pale-cyan-blue) !important;}.has-vivid-cyan-blue-border-color{border-color: var(--wp--preset--color--vivid-cyan-blue) !important;}.has-vivid-purple-border-color{border-color: var(--wp--preset--color--vivid-purple) !important;}.has-vivid-cyan-blue-to-vivid-purple-gradient-background{background: var(--wp--preset--gradient--vivid-cyan-blue-to-vivid-purple) !important;}.has-light-green-cyan-to-vivid-green-cyan-gradient-background{background: var(--wp--preset--gradient--light-green-cyan-to-vivid-green-cyan) !important;}.has-luminous-vivid-amber-to-luminous-vivid-orange-gradient-background{background: var(--wp--preset--gradient--luminous-vivid-amber-to-luminous-vivid-orange) !important;}.has-luminous-vivid-orange-to-vivid-red-gradient-background{background: var(--wp--preset--gradient--luminous-vivid-orange-to-vivid-red) !important;}.has-very-light-gray-to-cyan-bluish-gray-gradient-background{background: var(--wp--preset--gradient--very-light-gray-to-cyan-bluish-gray) !important;}.has-cool-to-warm-spectrum-gradient-background{background: var(--wp--preset--gradient--cool-to-warm-spectrum) !important;}.has-blush-light-purple-gradient-background{background: var(--wp--preset--gradient--blush-light-purple) !important;}.has-blush-bordeaux-gradient-background{background: var(--wp--preset--gradient--blush-bordeaux) !important;}.has-luminous-dusk-gradient-background{background: var(--wp--preset--gradient--luminous-dusk) !important;}.has-pale-ocean-gradient-background{background: var(--wp--preset--gradient--pale-ocean) !important;}.has-electric-grass-gradient-background{background: var(--wp--preset--gradient--electric-grass) !important;}.has-midnight-gradient-background{background: var(--wp--preset--gradient--midnight) !important;}.has-small-font-size{font-size: var(--wp--preset--font-size--small) !important;}.has-medium-font-size{font-size: var(--wp--preset--font-size--medium) !important;}.has-large-font-size{font-size: var(--wp--preset--font-size--large) !important;}.has-x-large-font-size{font-size: var(--wp--preset--font-size--x-large) !important;}
:link,
:visited,
:hover,
:active,
:-webkit-any-link { text-decoration:none; color: inherit;
} #span-2174-168 \> p \> a { color: #00daaa; text-decoration: underline !important;
} div#-slide-menu-456-11 \> .oxy-slide-menu\_inner \> #menu-menu-primary-1 \> .menu-item \> .sub-menu \> .all\_insurance\_menu\_header { border-bottom: 1px solid rgba(241, 239, 227, 0.2);
}
div#-slide-menu-456-11 \> .oxy-slide-menu\_inner \> #menu-menu-primary-1 \> .menu-item \> .sub-menu { padding-right: 24px;
}
.all\_insurance\_menu\_header { border-bottom: 1px solid var(--line-4-grey);
}
.all\_insurance\_menu\_header \> a { font-weight:bold !important; padding-bottom: 16px;
}
li#menu-item-1100 \> .sub-menu \> #menu-item-1742 \> a { padding-top: 16px;
}
li#menu-item-1100 \> .sub-menu { width: 232px;
}.wp-grid-builder:not(.wpgb-template),.wpgb-facet{opacity:0.01}.wpgb-facet fieldset{margin:0;padding:0;border:none;outline:none;box-shadow:none}.wpgb-facet fieldset:last-child{margin-bottom:40px;}.wpgb-facet fieldset legend.wpgb-sr-only{height:1px;width:1px}.rll-youtube-player{position:relative;padding-bottom:56.23%;height:0;overflow:hidden;max-width:100%;}.rll-youtube-player:focus-within{outline: 2px solid currentColor;outline-offset: 5px;}.rll-youtube-player iframe{position:absolute;top:0;left:0;width:100%;height:100%;z-index:100;background:0 0}.rll-youtube-player img{bottom:0;display:block;left:0;margin:auto;max-width:100%;width:100%;position:absolute;right:0;top:0;border:none;height:auto;-webkit-transition:.4s all;-moz-transition:.4s all;transition:.4s all}.rll-youtube-player img:hover{-webkit-filter:brightness(75%)}.rll-youtube-player .play{height:100%;width:100%;left:0;top:0;position:absolute;background:url(https://www.heygoody.com/th/wp-content/plugins/wp-rocket/assets/img/youtube.png) no-repeat center;background-color: transparent !important;cursor:pointer;border:none;}.wp-embed-responsive .wp-has-aspect-ratio .rll-youtube-player{position:absolute;padding-bottom:0;width:100%;height:100%;top:0;bottom:0;left:0;right:0} (function(w,d,s,l,i){w[l]=w[l]||[];w[l].push({'gtm.start':
new Date().getTime(),event:'gtm.js'});var f=d.getElementsByTagName(s)[0],
j=d.createElement(s),dl=l!='dataLayer'?'&l='+l:'';j.async=true;j.src=
'https://www.googletagmanager.com/gtm.js?id='+i+dl;f.parentNode.insertBefore(j,f);
})(window,document,'script','dataLayer','GTM-WRLZTM2'); !function(e,r){if(void 0!==e&&!e.adbrix){var t={queue:[]},o=navigator.userAgent.toLowerCase(),n=r.createElement("script"),i="Netscape"===navigator.appName&&-1!==navigator.userAgent.search("Trident")||-1!==o.indexOf("msie")?"abx-web-sdk.ie.min.js":"abx-web-sdk.min.js";n.type="text/javascript",n.async=!0,n.src="//static.adbrix.io/web-sdk/latest/"+i,n.onload=function(){e.adbrix.runQueuedFunctions?e.adbrix.runQueuedFunctions():console.log("[adbrix] Error: could not load SDK")};var a=r.getElementsByTagName("script")[0];a.parentNode.insertBefore(n,a);["init","onInitialized","login","getUserId","logout","userProperty.get","userProperty.getAll","userProperty.addOrUpdate","userProperty.remove","userProperty.removes","common.signUp","common.invite","common.useCredit","common.purchase","event.send","debug.traceListener","commerceAttr.categories","commerceAttr.product"].forEach(function(e){var r=e.split("."),o=r.pop();r.reduce(function(e,r){return e[r]=e[r]||{}},t)[o]=function(){t.queue.push([e,arguments])}}),e.adbrix=t}}(window,document); window.adbrix.init({ appkey: "F4VJSEYm80WoKISGHbL2Dg", webSecretkey: "CXMQnacOnE6rXb9gJLVXZA", //web push push: { enable: true, serviceWorkerOptions: { file\_name: "service-worker.js", file\_path: "/", scope: "/", //example https://www.heygoody.com/scripts/service-worker.js }, }, //web pop up inWebMessage: { enable: true, openInNewWindow: true, zIndex: 9999, fetchListener: function (message) { console.log("fetch\_listener " + message); }, clickListener: function (actionId, actionType, actionArg, isClosed) { console.log( "click\_listener " + actionId + actionType + actionArg + isClosed ); }, }, }); function OptanonWrapper() { } window.addEventListener('DOMContentLoaded', function() { document.getElementById("fn-lincense").addEventListener("click",()=\>{ $("#BrokerLicense").modal("show"); }); document.getElementById("fn-lincenseOnline").addEventListener("click",()=\>{ $("#BrokerLicenseOnline").modal("show"); }); }); \<style\>.wp-grid-builder .wpgb-card.wpgb-card-hidden .wpgb-card-wrapper{opacity:1!important;visibility:visible!important;transform:none!important}.wpgb-facet {opacity:1!important;pointer-events:auto!important}.wpgb-facet \*:not(.wpgb-pagination-facet){display:none}\</style\>@font-face{font-family:"DM Sans";font-style:normal;font-weight:400;font-display:fallback;src:url('https://www.heygoody.com/th/wp-content/themes/oxygen-is-not-a-theme/assets/fonts/dm-sans/DMSans-Regular.woff2') format('woff2');font-stretch:normal;}
@font-face{font-family:"DM Sans";font-style:italic;font-weight:400;font-display:fallback;src:url('https://www.heygoody.com/th/wp-content/themes/oxygen-is-not-a-theme/assets/fonts/dm-sans/DMSans-Regular-Italic.woff2') format('woff2');font-stretch:normal;}
@font-face{font-family:"DM Sans";font-style:normal;font-weight:700;font-display:fallback;src:url('https://www.heygoody.com/th/wp-content/themes/oxygen-is-not-a-theme/assets/fonts/dm-sans/DMSans-Bold.woff2') format('woff2');font-stretch:normal;}
@font-face{font-family:"DM Sans";font-style:italic;font-weight:700;font-display:fallback;src:url('https://www.heygoody.com/th/wp-content/themes/oxygen-is-not-a-theme/assets/fonts/dm-sans/DMSans-Bold-Italic.woff2') format('woff2');font-stretch:normal;}
@font-face{font-family:"IBM Plex Mono";font-style:normal;font-weight:300;font-display:block;src:url('https://www.heygoody.com/th/wp-content/themes/oxygen-is-not-a-theme/assets/fonts/ibm-plex-mono/IBMPlexMono-Light.woff2') format('woff2');font-stretch:normal;}
@font-face{font-family:"IBM Plex Mono";font-style:normal;font-weight:400;font-display:block;src:url('https://www.heygoody.com/th/wp-content/themes/oxygen-is-not-a-theme/assets/fonts/ibm-plex-mono/IBMPlexMono-Regular.woff2') format('woff2');font-stretch:normal;}
@font-face{font-family:"IBM Plex Mono";font-style:italic;font-weight:400;font-display:block;src:url('https://www.heygoody.com/th/wp-content/themes/oxygen-is-not-a-theme/assets/fonts/ibm-plex-mono/IBMPlexMono-Italic.woff2') format('woff2');font-stretch:normal;}
@font-face{font-family:"IBM Plex Mono";font-style:normal;font-weight:700;font-display:block;src:url('https://www.heygoody.com/th/wp-content/themes/oxygen-is-not-a-theme/assets/fonts/ibm-plex-mono/IBMPlexMono-Bold.woff2') format('woff2');font-stretch:normal;}
@font-face{font-family:Inter;font-style:normal;font-weight:200 900;font-display:fallback;src:url('https://www.heygoody.com/th/wp-content/themes/oxygen-is-not-a-theme/assets/fonts/inter/Inter-VariableFont\_slnt,wght.ttf') format('truetype');font-stretch:normal;}
@font-face{font-family:"Source Serif Pro";font-style:normal;font-weight:200 900;font-display:fallback;src:url('https://www.heygoody.com/th/wp-content/themes/oxygen-is-not-a-theme/assets/fonts/source-serif-pro/SourceSerif4Variable-Roman.ttf.woff2') format('woff2');font-stretch:normal;}
@font-face{font-family:"Source Serif Pro";font-style:italic;font-weight:200 900;font-display:fallback;src:url('https://www.heygoody.com/th/wp-content/themes/oxygen-is-not-a-theme/assets/fonts/source-serif-pro/SourceSerif4Variable-Italic.ttf.woff2') format('woff2');font-stretch:normal;}\<style id="rocket-lazyload-nojs-css"\>.rll-youtube-player, [data-lazy-src]{display:none !important;}\</style\> \<iframe src="https://www.googletagmanager.com/ns.html?id=GTM-WRLZTM2"
height="0" width="0" style="display:none;visibility:hidden"\>\</iframe\> document.addEventListener("DOMContentLoaded", () =\> { createCookie("gtm-carInfo", createBase64EncodedCookie(getCarInfo()), "gtm-carInfo"); createCookie("gtm-userData", createBase64EncodedCookie(getUserData()), "gtm-userData");
});
window.onscroll = function() { /\* if(window.scrollY \> 0){ $("#onetrust-consent-sdk").fadeIn(); }else if (window.scrollY == 0){ $("#onetrust-consent-sdk").fadeOut(); } \*/ if (window.scrollY \> 0) { //console.log("show"); $("#onetrust-consent-sdk").addClass("show"); } else { $("#onetrust-consent-sdk").removeClass("show"); } };
function createBase64EncodedCookie(data) { return btoa(JSON.stringify(data));
} function createCookie(cookieName, cookieValue, existingCookieName) { if (!getCookie(existingCookieName)) { document.cookie = `${cookieName}=${cookieValue}; path=/; samesite=Lax`; }
} function getCarInfo() { return { car\_type: "", car\_brand: "", car\_model: "", car\_submodel: "", car\_year: "", car\_last\_insurance: "", car\_current\_insurance: "", car\_coverage\_date: "", car\_registered\_province: "", birthyear: "" };
} function getUserData() { const guestUserID = generateGuestUserID(); return { user\_type: "guest", user\_id: guestUserID, guest\_id: guestUserID, customer\_id: "", user\_first\_name: "", user\_last\_name: "", user\_phone: "", user\_email: "" };
} function generateGuestUserID() { const currentDate = new Date(); const year = currentDate.getFullYear(); const month = (currentDate.getMonth() + 1).toString().padStart(2, '0'); const day = currentDate.getDate().toString().padStart(2, '0'); const randomPart = (Math.floor(Math.random() \* 1000000)).toString().padStart(6, '0'); return "HGG" + year + month + day + randomPart;
} function getCookie(name) { const nameEQ = name + "="; const cookies = document.cookie.split(";"); for (let i = 0; i \< cookies.length; i++) { let cookie = cookies[i].trim(); if (cookie.startsWith(nameEQ)) { return cookie.substring(nameEQ.length); } } return null;
}

[![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img src="/Content/Images/Member/icon-other-grey-back.svg" /\> ]() [<img src="/Content/images/logo/dot-com/color-black.svg" class="fn-top-logo" alt="heygoody" width="150" height="32"> ](/)

[<img src="/Content/images/logo/dot-com/color-black.svg" class="fn-top-logo" alt="heygoody" width="150" height="32"> ](/)

* #fnSubmenu608 {
  background-image:linear-gradient(180deg, #03B2C9 0%, #9BDB2E 100%); } #fnSubmenu608::after {
  background-image:linear-gradient(180deg, #03B2C9 0%, #9BDB2E 100%); } [ ผลิตภัณฑ์ประกัน ]()
  * [<img src="data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%2032%2032'%3E%3C/svg%3E" alt="ประกันรถยนต์" width="32" height="32" data-lazy-src="/getattachment/325f384b-7bf1-4abe-bc89-4479c242c9e6/car-ins.svg">\<img src='/getattachment/325f384b-7bf1-4abe-bc89-4479c242c9e6/car-ins.svg' alt='ประกันรถยนต์' width='32' height='32'\> ประกันรถยนต์ ]()
    * [ ประกันรถยนต์ทั้งหมด ](/th/autoinsurance/all/)
    * [ ประกันรถยนต์ไฟฟ้า EV ใหม่ ](/th/autoinsurance/evcar/)
    * [ ประกันรถยนต์ชั้น 1 ](/th/autoinsurance/class1/)
    * [ ประกันรถยนต์ชั้น 2+, 2 แนะนำ ](/th/autoinsurance/class2plus-2/)
    * [ ประกันรถยนต์ชั้น 3+, 3 ](/th/autoinsurance/class3plus-3/)
    * [ ประกันรถตู้ส่วนบุคคล ](/checkinsurance/van)

  * [<img src="data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%2032%2032'%3E%3C/svg%3E" alt="ประกันเดินทาง" width="32" height="32" data-lazy-src="/getattachment/225e9d33-f798-4a04-827c-7da0c786f25a/travel-ins.svg">\<img src='/getattachment/225e9d33-f798-4a04-827c-7da0c786f25a/travel-ins.svg' alt='ประกันเดินทาง' width='32' height='32'\> ประกันเดินทาง ](/th/travelinsurance/)
  * [<img src="data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%2032%2032'%3E%3C/svg%3E" alt="ประกันบ้าน/คอนโด" width="32" height="32" data-lazy-src="/getattachment/369a3e57-e168-435a-8a49-9a36486aae13/home-ins.svg">\<img src='/getattachment/369a3e57-e168-435a-8a49-9a36486aae13/home-ins.svg' alt='ประกันบ้าน/คอนโด' width='32' height='32'\> ประกันบ้าน/คอนโด ](/th/homeinsurance/)
  * [<img src="data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%2032%2032'%3E%3C/svg%3E" alt="ประกันโรคมะเร็ง" width="32" height="32" data-lazy-src="/getattachment/b04159ca-38eb-4a8b-8956-f820787a8af6/cancer-ins.svg">\<img src='/getattachment/b04159ca-38eb-4a8b-8956-f820787a8af6/cancer-ins.svg' alt='ประกันโรคมะเร็ง' width='32' height='32'\> ประกันโรคมะเร็ง ](/th/cancer/)
  * [<img src="data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%2032%2032'%3E%3C/svg%3E" alt="" width="32" height="32" data-lazy-src="/getattachment/224e18b2-87a5-4b8c-b53c-971ec003a3f8/critical-illness-ins.svg">\<img src='/getattachment/224e18b2-87a5-4b8c-b53c-971ec003a3f8/critical-illness-ins.svg' alt='' width='32' height='32'\> ประกันโรคร้ายแรง ](/th/critical-illness/)
  * [<img src="data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%2032%2032'%3E%3C/svg%3E" alt="ประกันชีวิต (ลดหย่อนภาษี)" width="32" height="32" data-lazy-src="/getattachment/3d2d72aa-46cc-4cc6-9085-9e5917f2fbf4/annuity.svg">\<img src='/getattachment/3d2d72aa-46cc-4cc6-9085-9e5917f2fbf4/annuity.svg' alt='ประกันชีวิต (ลดหย่อนภาษี)' width='32' height='32'\> ประกันชีวิต (ลดหย่อนภาษี) ]()
    * [ คำนวณและเปรียบเทียบประกันลดหย่อนภาษี ](/th/tax-deduction/)
    * [ ประกันสะสมทรัพย์ ](/th/tax-deduction/savings-insurance/)
    * [ ประกันบำนาญ ](/th/tax-deduction/annuity-insurance/)

* [ โปรโมชั่น & กิจกรรม ](/th/promotion/)
* #fnSubmenu33 {
  background-image:linear-gradient(192deg, #36B38A 59.89%, #088B86 72.26%, #096B65 84.52%); } #fnSubmenu33::after {
  background-image:linear-gradient(192deg, #36B38A 59.89%, #088B86 72.26%, #096B65 84.52%); } [ เกี่ยวกับเรา ]()
  * [ รู้จักเรา ](/th/about-us/who-we-are/)
  * [ ทำไมต้อง heygoody? ](/th/about-us/)
  * [ รางวัลความสำเร็จ ](/th/about-us/awards-and-recognition)

* #fnSubmenu34 {
  background-image:linear-gradient(180deg, #FF8400 0%, #9600F2 100%); } #fnSubmenu34::after {
  background-image:linear-gradient(180deg, #FF8400 0%, #9600F2 100%); } [ ช่วยเหลือ ]()
  * [ ความช่วยเหลือทั้งหมด ](/th/support-info/)
  * [ การซื้อประกัน ]()
    * [ การซื้อประกันรถยนต์ ](/th/support-info/how-to-buy-auto-insurance/)
    * [ การซื้อประกันเดินทาง ](/th/support-info/how-to-buy-travel-insurance/)

  * [ การจ่ายเงิน ]()
    * [ การจ่ายเงินประกันรถยนต์ ](/th/support-info/how-to-payment-auto-insurance/)
    * [ การจ่ายค่างวดประกันรถยนต์ ](/th/support-info/how-to-billing/)
    * [ การจ่ายเงินประกันเดินทาง ](/th/support-info/how-to-payment-travel-insurance/)

  * [ การเคลมประกัน ]()
    * [ การเคลมประกันรถยนต์ ](/th/support-info/how-to-claim-auto-insurance/)
    * [ การเคลมประกันเดินทาง ](/th/support-info/how-to-claim-travel-insurance/)

  * [ การใช้งานโปรโมชั่น ](/th/support-info/how-to-promotion/)
  * [ ค้นหาอู่ซ่อม ](/th/support-info/gogogarage/)
  * [ คำถามที่พบบ่อย ](/th/faq/)

* #fnSubmenu35 {
  background-image:linear-gradient(180deg, #71E6E5 0%, #A32BFA 100%); } #fnSubmenu35::after {
  background-image:linear-gradient(180deg, #71E6E5 0%, #A32BFA 100%); } [ goodyTalks ]()
  * [ บทความ ](/th/blogs/)
  * [ ข่าวสาร ](/th/news/)

* [ ติดต่อเรา ](/th/contact-us/)
* [<img class="mr4" src="data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%2024%2024'%3E%3C/svg%3E" alt="login" width="24" height="24" data-lazy-src="/Content/images/header/profile.svg">\<img class="mr4" src="/Content/images/header/profile.svg" alt="login" width="24" height="24"\>เข้าสู่ระบบ](/member)

---
[![ค้นหา](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img class="fn-me-2" src="/Content/Images/icon-16-grey-search-dark.svg" alt="ค้นหา"\> ค้นหา ](/th/search/)

[](/th/search/) [เข้าสู่ระบบ ](/member)

window.addEventListener('DOMContentLoaded', function() { $(document).ready(function(){ //Cr Header if($(".top-menu-mh").hasClass("fn-d-lg-none")) { } else { var headerLeft = $('#id-nav-menu').slideMenu({ position: 'left' }); } //For responsive if ($(window).width() \>= 1100) { hoverMenu(); } $(window).resize(function () { toggleFunction(); if ($(window).width() \>= 1100) { $("#closeMenuNav").trigger("click"); $("#id-nav-menu").removeClass("no-transition"); } else if ($(window).width() \< 1100 && $("#id-nav-menu").hasClass("active") == false) { $("#id-nav-menu").addClass("no-transition"); } }); //GTM CR Header menu level 1 $("#navbar-section \> .nav-item \> .nav-link").click(function(){ var menuText = $(this).text().trim(); dataLayer.push({ event: "main\_menu", menu\_label: menuText, menu\_type: "header", }); }); //GTM CR Header menu level 2 $(".fn-nav-submenu \> .nav-item:not(.fn-backMenu, .fn-mobileMenuTitle) \> .nav-link").click(function(){ var menuText = $(this).closest(".fn-nav-submenu").closest(".nav-item").find("\> .nav-link").text().trim(); var menuTextLastLevel = $(this).text().trim(); dataLayer.push({ event: "main\_menu", menu\_label: menuText + " | " + menuTextLastLevel, menu\_type: "header", }); }); //GTM CR Header menu level 3 $(".fn-subMenuLVL3 \> .nav-item:not(.fn-backMenu, .fn-mobileMenuTitle) \> .nav-link").click(function(){ var menuText = $(this).closest(".fn-nav-submenu").closest(".nav-item").find("\> .nav-link").text().trim(); var menuTextLastLevel = $(this).contents().filter(function() { return this.nodeType === 3; }).text().trim(); dataLayer.push({ event: "main\_menu", menu\_label: menuText + " | " + menuTextLastLevel, menu\_type: "header", }); }); // When hamburger on mobile menu clicked $("#nav-icon3").click(function () { $("#id-nav-menu").removeClass("no-transition"); $("#backDropMenu").addClass("open"); }); // When X on mobile menu clicked $("#closeMenuNav").click(function () { $("#id-nav-menu").removeClass("active"); $("#id-nav-menu").removeClass("no-transition"); $("body").removeClass("fn-openMenuMobile minimenu-open"); $("#nav-icon3").removeClass("open"); $("#backDropMenu").removeClass("open"); $(".heygoody-menu-two").removeClass("menuopen-dark"); $(".heygoody-menu-two nav").removeClass("fn-main-menu"); $("#navbar-section .nav-item").removeClass("menu-active"); $(".menu-user-login .user-login").removeClass("d-none"); }); // When blur content page clicked $("#backDropMenu").click(function(){ $("#closeMenuNav").trigger("click"); }); // When hover menu level 2 hover it should be the same height (min-height) at menu level 3 function hoverMenu() { $('.fn-nav-submenu \> .heygoody-menu-toggle').hover( function() { $(this).addClass('hovering'); var $submenu = $(this).closest('.fn-nav-submenu'); $submenu.addClass('showed'); var menuLevel3Height = $(this).find('.fn-subMenuLVL3').outerHeight(); var navSubmenuHeight = $submenu.outerHeight(); if (menuLevel3Height \< navSubmenuHeight) { $(this).find('.fn-subMenuLVL3').css('min-height', navSubmenuHeight); } $submenu.css('min-height', menuLevel3Height); }, function() { $(this).removeClass('hovering'); var $submenu = $(this).closest('.fn-nav-submenu'); $submenu.removeClass('showed'); $submenu.css('min-height', ''); // Reset the min-height of .fn-subMenuLVL3 if needed $(this).find('.fn-subMenuLVL3').css('min-height', ''); } ); } function toggleFunction() { if ($(window).width() \<= 1099) { $('.fn-nav-submenu \> .heygoody-menu-toggle').off('mouseenter mouseleave'); } else if ($(window).width() \> 1099) { hoverMenu(); // Call hoverMenu() directly } } //เช็คเงื่อนไข renew ให้แสดง modal }); function showModalGoOut() { $("#fn-info-error5").modal("show"); //ยิงเมื่อ pop-up แสดง $.ajax({ type: "POST", url: "/RenewCheckInsurance/get\_gtm\_product\_category", data: { //type:type, req: SendObject }, //beforeSend: function(xhr) { //	xhr.setRequestHeader('X-CSRF-Token', csrfToken) //}, success: function (data) { dataLayer.push({ event: "renew\_plan", event\_category: "renew\_plan", event\_action: "display", event\_label: "pop-up\_exit", product\_category: data }); } }); } function showModalGoOut\_Mobile() { $("#fn-mini-box").addClass("d-none"); $("#fn-backdrop-login").remove(); tagAppended = false; $("#fn-info-error5").modal("show"); //ยิงเมื่อ pop-up แสดง $.ajax({ type: "POST", url: "/RenewCheckInsurance/get\_gtm\_product\_category", data: { //type:type, req: SendObject }, //beforeSend: function(xhr) { //	xhr.setRequestHeader('X-CSRF-Token', csrfToken) //}, success: function (data) { dataLayer.push({ event: "renew\_plan", event\_category: "renew\_plan", event\_action: "display", event\_label: "pop-up\_exit", product\_category: data }); } }); } });

[![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-32-11" alt="" src="/th/wp-content/uploads/2023/09/logo-horizontal-heygoody-on-black-by-ntl\_header\_.svg" class="ct-image"/\>](/)

* [ประกันรถยนต์](#)
  * [ประกันรถยนต์ทั้งหมด](https://www.heygoody.com/th/autoinsurance/all/)
  * [ประกันรถยนต์ไฟฟ้า EV](https://www.heygoody.com/th/autoinsurance/evcar/)
  * [ประกันรถยนต์ชั้น 1](https://www.heygoody.com/th/autoinsurance/class1/)
  * [ประกันรถยนต์ชั้น 2+, 2](https://www.heygoody.com/th/autoinsurance/class2plus-2/)
  * [ประกันรถยนต์ชั้น 3+, 3](https://www.heygoody.com/th/autoinsurance/class3plus-3/)

* [ทำไมต้อง heygoody?](https://www.heygoody.com/th/about-us/)
* [ช่วยเหลือ](#)
  * [การซื้อประกันและชำระเงิน](https://www.heygoody.com/th/how-to-buy/)
  * [ค้นหาอู่ซ่อม](https://www.gogo-garage.com/)

* [สาระประกันดี](#)
  * [บทความ](https://www.heygoody.com/th/blogs/)
  * [โปรโมชั่น](https://www.heygoody.com/th/promotion/)
  * [ข่าวสาร](https://www.heygoody.com/th/news/)

* [ติดต่อเรา](https://www.heygoody.com/th/contact-us/)

[![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-582-11" alt="" src="https://www.heygoody.com/th/wp-content/uploads/2023/03/icon-other-grey-search-dark.svg" class="ct-image" srcset="" sizes="(max-width: 36px) 100vw, 36px" /\>](/th/search/)[![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-585-11" alt="" src="https://www.heygoody.com/th/wp-content/uploads/2023/03/icon-other-grey-member-dark-2.svg" class="ct-image" srcset="" sizes="(max-width: 36px) 100vw, 36px" /\>](/member/)[เข้าสู่ระบบ](/member/)

[เข้าสู่ระบบ](/member/)

* [ประกันรถยนต์](#)
  * [ประกันรถยนต์ทั้งหมด](https://www.heygoody.com/th/autoinsurance/all/)
  * [ประกันรถยนต์ไฟฟ้า EV](https://www.heygoody.com/th/autoinsurance/evcar/)
  * [ประกันรถยนต์ชั้น 1](https://www.heygoody.com/th/autoinsurance/class1/)
  * [ประกันรถยนต์ชั้น 2+, 2](https://www.heygoody.com/th/autoinsurance/class2plus-2/)
  * [ประกันรถยนต์ชั้น 3+, 3](https://www.heygoody.com/th/autoinsurance/class3plus-3/)

* [ทำไมต้อง heygoody?](https://www.heygoody.com/th/about-us/)
* [ช่วยเหลือ](#)
  * [การซื้อประกันและชำระเงิน](https://www.heygoody.com/th/how-to-buy/)
  * [ค้นหาอู่ซ่อม](https://www.gogo-garage.com/)

* [สาระประกันดี](#)
  * [บทความ](https://www.heygoody.com/th/blogs/)
  * [โปรโมชั่น](https://www.heygoody.com/th/promotion/)
  * [ข่าวสาร](https://www.heygoody.com/th/news/)

* [ติดต่อเรา](https://www.heygoody.com/th/contact-us/)

[![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-586-11" alt="" src="https://www.heygoody.com/th/wp-content/uploads/2023/03/icon-16-grey-search-dark.svg" class="ct-image" srcset="" sizes="(max-width: 16px) 100vw, 16px" /\>

ค้นหา

](/th/search/)

[![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-464-11" alt="" src="/th/wp-content/uploads/2023/09/logo-horizontal-heygoody-on-black-by-ntl\_header\_.svg" class="ct-image"/\>](/)

[![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-467-11" alt="" src="https://www.heygoody.com/th/wp-content/uploads/2023/03/usericon.svg" class="ct-image" srcset="" sizes="(max-width: 36px) 100vw, 36px" /\>](/member/)

window.addEventListener('DOMContentLoaded', function() { jQuery(document).ready(function() { var selector = "#\_header-26-11", scrollval = parseInt("52"); if (!scrollval || scrollval \< 1) { jQuery("body").css("margin-top", jQuery(selector).outerHeight()); jQuery(selector).addClass("oxy-sticky-header-active"); } else { var scrollTopOld = 0; jQuery(window).scroll(function() { if (!jQuery('body').hasClass('oxy-nav-menu-prevent-overflow')) { if (jQuery(this).scrollTop() \> scrollval ) { if ( !jQuery(selector).hasClass("oxy-sticky-header-active")) { if (jQuery(selector).css('position')!='absolute') { jQuery("body").css("margin-top", jQuery(selector).outerHeight()); } jQuery(selector) .addClass("oxy-sticky-header-active") } } else { jQuery(selector) .removeClass("oxy-sticky-header-fade-in") .removeClass("oxy-sticky-header-active"); if (jQuery(selector).css('position')!='absolute') { jQuery("body").css("margin-top", ""); } } scrollTopOld = jQuery(this).scrollTop(); } }) } }); });

heygoody5

heygoody

HEY10

function dataLayerTackingLikePlan(inname,pdname,intype) { dataLayer.push({ event: "tax\_cal\_btn", event\_category: "tax\_calculator\_result", event\_action: "plancard\_buy", event\_label: inname, // e.g. RBL, TNI product\_name: pdname, //e.g. Hero 10/1 insurance\_type: intype, social\_security: document.getElementById('prakansangkomcheckbox').checked ? "N" : "Y", fund\_category: document.querySelector('input[name="allfunds"]:checked')?.value === "1" ? "Y" : "N", ssf\_fund: formatNumberForTracking(document.getElementById('FieldFund\_1').value || 0), rmf\_fund: formatNumberForTracking(document.getElementById('FieldFund\_2').value || 0), pvd\_fund: formatNumberForTracking(document.getElementById('FieldFund\_3').value || 0), pta\_fund: formatNumberForTracking(document.getElementById('FieldFund\_4').value || 0), gpf\_fund: formatNumberForTracking(document.getElementById('FieldFund\_5').value || 0), ns\_fund: formatNumberForTracking(document.getElementById('FieldFund\_6').value || 0), saving\_ins: formatNumberForTracking(document.getElementById('div\_block-804-4617').value || 0), annuity\_ins: formatNumberForTracking(document.getElementById('div\_block-837-4617').value || 0), health\_ins: formatNumberForTracking(document.getElementById('div\_block-825-4617').value || 0) });
} function dataLayerTackingBuyButton(inname,pdname,intype) { dataLayer.push({ event: "tax\_cal\_btn", event\_category: "tax\_calculator\_result", event\_action: "detailcard\_buy", event\_label: inname, product\_name: pdname, social\_security: document.getElementById('prakansangkomcheckbox').checked ? "N" : "Y", fund\_category: document.querySelector('input[name="allfunds"]:checked')?.value === "1" ? "Y" : "N", ssf\_fund: formatNumberForTracking(document.getElementById('FieldFund\_1').value || 0), rmf\_fund: formatNumberForTracking(document.getElementById('FieldFund\_2').value || 0), pvd\_fund: formatNumberForTracking(document.getElementById('FieldFund\_3').value || 0), pta\_fund: formatNumberForTracking(document.getElementById('FieldFund\_4').value || 0), gpf\_fund: formatNumberForTracking(document.getElementById('FieldFund\_5').value || 0), ns\_fund: formatNumberForTracking(document.getElementById('FieldFund\_6').value || 0), saving\_ins: formatNumberForTracking(document.getElementById('div\_block-804-4617').value || 0), annuity\_ins: formatNumberForTracking(document.getElementById('div\_block-837-4617').value || 0), health\_ins: formatNumberForTracking(document.getElementById('div\_block-825-4617').value || 0) });
}

[

หน้าหลัก

](/)

คำนวณและเปรียบเทียบประกันลดหย่อนภาษี

คำนวนและเปรียบเทียบประกันลดหย่อนภาษี
==========

ตัวช่วยวางแผนการเงินดีๆ ให้คุณประหยัดภาษีได้มากกว่าเดิม

![](/th/wp-content/uploads/2024/10/Cover_tax_calculate_page.webp)![](/th/wp-content/uploads/2024/10/Cover_tax_calculate_page_mobile.webp)

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-1722-4617" alt="" src="/th/wp-content/uploads/2024/10/Combined\_Shape\_bg\_section\_calculate\_page-scaled-1.webp" class="ct-image"/\>

ต้องซื้อเท่าไหร่? ถึงคุ้มค่าให้เราช่วยคำนวณได้เลย!

เริ่มคำนวณ

\*การคำนวณนี้เป็นการจำลองเพื่อช่วยในการวางแผนซื้อประกันลดหย่อนภาษีเท่านั้นไม่สามารถนำไปใช้ในการยื่นแบบแสดงรายการภาษีได้จริง

ผลคำนวณภาษี
----------

ยอดภาษีเดิม

฿

0

ภาษีที่ต้องจ่าย

฿

0

คุณวางแผนภาษีได้เยี่ยมมาก!

คุณประหยัดภาษีจากประกันเพิ่ม

 ฿

0

![](/th/wp-content/uploads/2024/10/coin_icon_tax_calculate_page.webp)

รวมลดหย่อนไปแล้ว

฿

0

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-787-4617" alt="" src="/th/wp-content/uploads/2024/10/Combined-\_Shape\_in\_result\_After\_calculate\_tax.webp" class="ct-image"/\>![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-1727-4617" alt="" src="/th/wp-content/uploads/2024/10/fireworks\_image\_tax\_calculate\_page\_right.webp" class="ct-image"/\>![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-1728-4617" alt="" src="/th/wp-content/uploads/2024/10/fireworks\_image\_tax\_calculate\_page\_left.webp" class="ct-image"/\>

แก้ไขรายได้และค่าลดหย่อน

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-785-4617" alt="" src="/th/wp-content/uploads/2024/10/pencil\_icon\_tax\_calculate\_page.svg" class="ct-image"/\>

ซื้อประกันเพิ่ม ประหยัดภาษีเพิ่ม

ประกันสะสมทรัพย์

รวมประกันสุขภาพลดหย่อนสูงสุด ฿100,000

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-810-4617" alt="" src="/th/wp-content/uploads/2024/10/green\_pencil\_icon\_tax\_calculate\_page.svg" class="ct-image"/\>

สิทธิลดหย่อนสูงสุดของคุณ  ฿100,000

7 แผนที่คัดมาแล้ว

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-866-4617" alt="" src="/th/wp-content/uploads/2024/09/rbl\_logo\_tax\_saving\_page.webp" class="ct-image"/\>

แรบบิท ไลฟ์ ประกันชีวิต

Hero10/1

จ่ายเบี้ยปีเดียว IRR 2.1%

เริ่มต้น ฿30,000/ ปี

ชอบแผนนี้

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-931-4617" alt="" src="/th/wp-content/uploads/2024/09/tcap\_logo\_tax\_saving\_page\_\_.webp" class="ct-image"/\>

ที ไลฟ์ ประกันชีวิต

Smart Saving 10/2

จ่ายเบี้ย 2 ปี รวมเงินคืนตลอดสัญญา 233%

เริ่มต้น ฿80,000 / ปี

ชอบแผนนี้

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-948-4617" alt="" src="/th/wp-content/uploads/2024/09/rbl\_logo\_tax\_saving\_page.webp" class="ct-image"/\>

แรบบิท ไลฟ์ ประกันชีวิต

Hero 10/3

IRR 2.25% เงินคืนระหว่างปี 20%

เริ่มต้น ฿30,000 / ปี

ชอบแผนนี้

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-965-4617" alt="" src="/th/wp-content/uploads/2024/09/mtl\_logo\_tax\_saving\_page.webp" class="ct-image"/\>

เมืองไทยประกันชีวิต

ออมจุใจ 10/3

สมัครได้ถึง 80 ปี มี promocode

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-981-4617" alt="" src="/th/wp-content/uploads/2024/09/coupon\_promotion\_copy\_.webp" class="ct-image"/\>

ลดอีก 5%

ใส่โค้ด “heygoody5”

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-1729-4617" alt="" src="/th/wp-content/uploads/2024/07/copy\_icon\_for\_cancer\_page.png" class="ct-image"/\>

เริ่มต้น ฿50,000 / ปี

ชอบแผนนี้

ผ่อน 0% 6 เดือนเฉพาะบัตรที่ร่วมรายการ

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-989-4617" alt="" src="/th/wp-content/uploads/2024/09/pru\_logo\_tax\_saving\_page.webp" class="ct-image"/\>

พรูเด็นเชียล ประกันชีวิต

PRUEasy Saver 10/4

เริ่มต้น 20,000/ปี มี promocode

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-997-4617" alt="" src="/th/wp-content/uploads/2024/09/coupon\_promotion\_copy\_.webp" class="ct-image"/\>

ลดสูงสุด 10%

ใส่โค้ด “heygoody”

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-1730-4617" alt="" src="/th/wp-content/uploads/2024/07/copy\_icon\_for\_cancer\_page.png" class="ct-image"/\>

เริ่มต้น ฿20,000 / ปี

ชอบแผนนี้

ผ่อน 0% 10 เดือนเฉพาะบัตรที่ร่วมรายการ

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-1012-4617" alt="" src="/th/wp-content/uploads/2024/09/tcap\_logo\_tax\_saving\_page\_\_.webp" class="ct-image"/\>

ที ไลฟ์ ประกันชีวิต

Smart Saving 10/4

มีแผนรายเดือนเริ่มต้น 2,000/เดือน

เริ่มต้น ฿24,000 / ปี

ชอบแผนนี้

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-1029-4617" alt="" src="/th/wp-content/uploads/2024/09/tcap\_logo\_tax\_saving\_page\_\_.webp" class="ct-image"/\>

ที ไลฟ์ ประกันชีวิต

Smart Saving 10/5

IRR 2.40% เงินคืนระหว่างปีสูงสุด 100%

เริ่มต้น ฿100,000 / ปี

ชอบแผนนี้

[

ดูประกันสะสมทรัพย์ทั้งหมด

](/th/tax-deduction/savings-insurance/)

ประกันสุขภาพ

ลดหย่อนสูงสุดไม่เกิน ฿25,000

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-827-4617" alt="" src="/th/wp-content/uploads/2024/10/green\_pencil\_icon\_tax\_calculate\_page.svg" class="ct-image"/\>

สิทธิลดหย่อนสูงสุดของคุณ  ฿25,000

ประกันมะเร็ง

ประกันโรคร้ายแรง

6

 แผนที่คัดมาแล้ว

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-1202-4617" alt="" src="/th/wp-content/uploads/2024/09/mtl\_logo\_tax\_saving\_page.webp" class="ct-image"/\>

เมืองไทยประกันชีวิต

เมืองไทย Care Plus โรคร้ายแรง

คุ้มครองมะเร็งทุกระยะ ไตวายเรื้อรังสูงสุด 10 ล้าน

เริ่มต้น ฿2,432 / ปี

ชอบแผนนี้

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-1338-4617" alt="" src="/th/wp-content/uploads/2024/09/mtl\_logo\_tax\_saving\_page.webp" class="ct-image"/\>

เมืองไทยประกันชีวิต

เมืองไทยโรคร้ายแรง D Care

เลือกซื้อกลุ่มโรคร้ายที่กังวลได้ถึง 6 กลุ่ม

เริ่มต้น ฿237 / ปี

ชอบแผนนี้

![](/th/wp-content/uploads/2024/07/mueng_thai_logo_for_cancer_section_3_edit.png)

เมืองไทยประกันภัย

Cancer 2Care

คุ้มครองทุกระยะรับ 2 เท่า 5 มะเร็ง พบบ่อย

เริ่มต้น ฿1,450 / ปี

ชอบแผนนี้

![](/th/wp-content/uploads/2024/07/mueng_thai_logo_for_cancer_section_3_edit.png)

เมืองไทยประกันภัย

เมืองไทยสู้โรคร้าย

ครอบคลุมมะเร็งทุกระยะและโรคร้ายที่พบบ่อย

เริ่มต้น ฿500 / ปี

ชอบแผนนี้

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-1386-4617" alt="" src="/th/wp-content/uploads/2024/09/pru\_logo\_tax\_saving\_page.webp" class="ct-image"/\>

พรูเด็นเชียล ประกันชีวิต

PRUCancer Care

จ่ายตั้งแต่ระยะแรก ดูแลถึงระยะลุกลาม

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-1401-4617" alt="" src="/th/wp-content/uploads/2024/09/coupon\_promotion\_copy\_.webp" class="ct-image"/\>

ลดอีก 10%

ใส่โค้ด “HEY10”

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-1738-4617" alt="" src="https://www.heygoody.com/th/wp-content/uploads/2024/07/copy\_icon\_for\_cancer\_page.png" class="ct-image"/\>

เริ่มต้น ฿178 / ปี

ชอบแผนนี้

![](/th/wp-content/uploads/2024/07/viriyah_logo_for_cancer_section_3.png)

วิริยะประกันภัย

Cancer Pro by BDMS

เบิกค่ารักษามะเร็งตามจริงสูงสุด 9 ล้าน

เริ่มต้น ฿6,240 / ปี

ชอบแผนนี้

[

ดูประกันสุขภาพทั้งหมด

](http://)

ประกันบำนาญ

รวมกองทุนลดหย่อนสูงสุดไม่เกิน ฿200,000

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-839-4617" alt="" src="/th/wp-content/uploads/2024/10/green\_pencil\_icon\_tax\_calculate\_page.svg" class="ct-image"/\>

สิทธิลดหย่อนสูงสุดของคุณ 15% ไม่เกิน ฿200,000

1 แผนที่คัดมาแล้ว

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-1055-4617" alt="" src="/th/wp-content/uploads/2024/09/mtl\_logo\_tax\_saving\_page.webp" class="ct-image"/\>

เมืองไทยประกันชีวิต

Easy Retire 99/1

จ่ายเบี้ยปีเดียว รับบำนาญปีละ 12%

เริ่มต้น ฿53,143/ ปี

ชอบแผนนี้

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-1072-4617" alt="" src="/th/wp-content/uploads/2024/09/tcap\_logo\_tax\_saving\_page\_\_.webp" class="ct-image"/\>

ที ไลฟ์ ประกันชีวิต

Smart Annuity 99/5

อายุครบ 60 รับบำนาญปีละ 20%

เริ่มซื้อได้ภายในเดือนธันวาคมนี้

ชอบแผนนี้

[

ดูประกันบำนาญทั้งหมด

](/th/tax-deduction/annuity-insurance/)

\*การคำนวณนี้เป็นการจำลองเพื่อช่วยในการวางแผนซื้อประกันลดหย่อนภาษีเท่านั้น ไม่สามารถนำไปใช้ในการยื่นแบบแสดงรายการภาษีได้จริง

ซื้อเสร็จ แอด LINE @heygoody

รับ

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-724-4617" alt="" src="/th/wp-content/uploads/2024/10/grab\_icon\_tax\_calculate\_page.svg" class="ct-image"/\>

สูงสุด

1,000

บาท

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-740-4617" alt="" src="/th/wp-content/uploads/2024/10/whisper\_icon\_toppage\_calculate\_page.webp" class="ct-image"/\>

[

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-738-4617" alt="" src="/th/wp-content/uploads/2024/09/line\_icon\_tax\_calculate\_page\_x2.webp" class="ct-image"/\>![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-741-4617" alt="" src="/th/wp-content/uploads/2024/10/good\_icon\_toppage\_calculate\_page.webp" class="ct-image"/\>

แอด LINE

](/th/addfriend/?campaign=linefriend&product=taxdeduction)

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-363-4617" alt="" src="/th/wp-content/uploads/2024/09/close-icon-for-taxcal.svg" class="ct-image" data-dismiss="modal"/\>

คำนวณภาษีต้องจ่าย

ล้าง

รายได้

รายได้รวมทั้งปี (จำเป็น)

เงินเดือนและโบนัส ก่อนถูกหักภาษี

ค่าลดหย่อนส่วนตัว

ค่าใช้จ่าย

สิทธิค่าลดหย่อนที่ทุกคนได้รับ

ค่าลดหย่อนส่วนตัว

สิทธิค่าลดหย่อนที่ทุกคนได้รับ

ประกันสังคม

ลดหย่อนสูงสุด ฿9,000

 ไม่มีประกันสังคม

ประกัน

คุณมีประกันลดหย่อนภาษีไหม?

(ประกันสะสมทรัพย์ / สุขภาพ / บำนาญ)

มี

ไม่มี

ประกันสะสมทรัพย์ รวมประกันสุขภาพลดหย่อนสูงสุด ฿100,000

ประกันสุขภาพ ลดหย่อนสูงสุด ฿25,000

ประกันบำนาญ ลดหย่อนสูงสุด 15% ไม่เกิน ฿200,000

กองทุน

คุณมีกองทุนไหม?

(กองทุน SSF / RMF / สำรองเลี้ยงชีพ /สงเคราะห์ครูเอกชน / บำนาญข้าราชการ / ออมแห่งชาติ)

มี

ไม่มี

กองทุน SSF

ลดหย่อนสูงสุด 30% และไม่เกิน ฿200,000

กองทุน RMF

ลดหย่อนสูงสุด 30% และไม่เกิน ฿500,000

กองทุนสำรองเลี้ยงชีพ

ลดหย่อนสูงสุด 15% และไม่เกิน ฿500,000

กองทุนสงเคราะห์ครูเอกชน

ลดหย่อนสูงสุดไม่เกิน ฿500,000

กองทุนบำนาญข้าราชการ

ลดหย่อนสูงสุดไม่เกิน ฿500,000

กองทุนการออมแห่งชาติ

ลดหย่อนสูงสุดไม่เกิน ฿30,000

เงินได้สุทธิ

฿

0

คุณได้สิทธิลดหย่อนแล้ว

฿

169,000

ภาษีหลังก่อนหย่อนหมวดประกัน

฿

0

ภาษีหลังลดหย่อนหมวดประกัน

฿

0

คำนวณภาษีที่ต้องจ่าย

คุณต้องการล้างค่าใช่หรือไม่

ยกเลิก

ล้างค่า

ประกันลดหย่อนภาษีแบบไหน
----------

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-1440-4617" alt="" src="/th/wp-content/uploads/2024/10/for\_you\_text\_calculate\_page.webp" class="ct-image"/\>

![](/th/wp-content/uploads/2024/10/icon_last_section_annuity_calculate_page.webp)

ประกันบำนาญ

[

ดูแผนประกัน

](/th/tax-deduction/annuity-insurance/)

![](/th/wp-content/uploads/2024/10/icon_last_section_saving_calculate_page.webp)

ประกันสะสมทรัพย์

[

ดูแผนประกัน

](/th/tax-deduction/savings-insurance/)

ต้องการลดหย่อนภาษี

สูงสุดไม่เกิน 3 แสนบาท

ประกันชีวิตแบบบำนาญ ซื้อได้สูงสุดไม่เกิน 15%ของรายได้รวมต่อปี

สูงสุดไม่เกิน 1 แสนบาทเมื่อซื้อรวมกับประกันสุขภาพ  
(โดยประกันสุขภาพสูงสุดไม่เกิน 25,000 บาท)

ประกันสะสมทรัพย์ และประกันบำนาญ เมื่อรวมกันแล้ว สามารถลดหย่อนภาษีได้สูงสุด 300,000 บาท

(กรณีไม่มีเบี้ยประกันภัยคุ้มครองชีวิต และ/หรือสุขภาพแบบอื่น)

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-1524-4617" alt="" src="/th/wp-content/uploads/2024/10/double\_quote\_right\_last\_section\_calculate\_page.svg" class="ct-image"/\>![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-1525-4617" alt="" src="/th/wp-content/uploads/2024/10/double\_quote\_left\_last\_section\_calculate\_page.svg" class="ct-image"/\>

ผู้ที่ต้องการสะสมเงินไว้ใช้หลังเกษียณ

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-1546-4617" alt="" src="/th/wp-content/uploads/2024/10/recommanded\_icon\_calculate\_page.webp" class="ct-image"/\>

แนะนำ

เริ่มจ่ายเงินบำนาญ หลังอายุ 60 ปี

มีเงินคืนระหว่างสัญญา และรับเงินก้อนเมื่อสัญญา  
กรมธรรม์สิ้นสุด ไม่จำเป็นต้องรอถึงวัยเกษียณ

ต้องการเงินปันผล / เงินคืนระหว่างปีสัญญา

ไม่มีเงินคืนระหว่างปี เพราะเน้นวางแผนเกษียณ

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-1575-4617" alt="" src="/th/wp-content/uploads/2024/10/recommanded\_icon\_calculate\_page.webp" class="ct-image"/\>

แนะนำ

มีเงินคืนระหว่างสัญญา ตามแผนประกันที่เลือกซื้อ  
บางแบบประกันให้เงินคืนระหว่างปีสูงถึง 20% - 100% ของทุนประกัน

มีรายได้แน่นอน มีเงินเดือนประจำ

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-1653-4617" alt="" src="/th/wp-content/uploads/2024/10/recommanded\_icon\_calculate\_page.webp" class="ct-image"/\>

แนะนำ

สามารถเลือกแผนจ่ายเบี้ยประกันต่อเนื่องทุกปี เช่น 5 ปี,   
10 ปี หรือจนกว่าจะถึงอายุตามที่แผนประกันกำหนด

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-1659-4617" alt="" src="/th/wp-content/uploads/2024/10/recommanded\_icon\_calculate\_page.webp" class="ct-image"/\>

แนะนำ

จ่ายเบี้ยประกันต่อเนื่องทุกปี เช่น 1 ปี, 2 ปี, 3 ปี, 4 ปี, 5 ปี ตามที่แผนประกันกำหนด

ต้องการความคล่องตัวทางการเงิน ไม่ต้องการผูกมัด

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-1620-4617" alt="" src="/th/wp-content/uploads/2024/10/recommanded\_icon\_calculate\_page.webp" class="ct-image"/\>

แนะนำ

สามารถเลือกแผนประกันที่จ่ายปีเดียวได้

ต้องการความคุ้มครองชีวิตสูง

ก่อนรับเงินบำนาญ110% ของเบี้ยประกันภัยที่ชำระมาแล้วทั้งหมด   
หรือมูลค่าเวนคืนกรมธรรม์ แล้วแต่จำนวนไหนมากกว่า  

หลังรับเงินบำนาญเบี้ยประกันภัยที่จ่ายไปแล้วทั้งหมดหักด้วยเงินบำนาญ  
ที่รับไปแล้วทั้งหมด

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-1590-4617" alt="" src="/th/wp-content/uploads/2024/10/recommanded\_icon\_calculate\_page.webp" class="ct-image"/\>

แนะนำ

บางแบบประกันให้ความคุ้มครองชีวิตในบางช่วงอายุ   
สูงถึง 515% ของทุนประกัน

ไม่อยากรับความเสี่ยง และไม่มีความรู้เรื่องการลงทุน

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-1680-4617" alt="" src="/th/wp-content/uploads/2024/10/recommanded\_icon\_calculate\_page.webp" class="ct-image"/\>

แนะนำ

จ่ายเงินคืนการันตีตามสัญญากรมธรรม์ โดยไม่นำผลประกอบการ ความผันผวนทางเศรษฐกิจ หรือการลงทุนของ  
บริษัทฯ มาเป็นปัจจัย และไม่จำเป็นต้องมีความรู้ทางการลงทุนเพราะผลประโยชน์ที่ได้รับ เป็นการการันตีตามสัญญากรมธรรม์

ซื้อเสร็จ แอด LINE @heygoody

รับ

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-1712-4617" alt="" src="/th/wp-content/uploads/2024/10/grab\_icon\_tax\_calculate\_page.svg" class="ct-image"/\>

สูงสุด

1,000

บาท

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-1716-4617" alt="" src="/th/wp-content/uploads/2024/10/whisper\_icon\_toppage\_calculate\_page.webp" class="ct-image"/\>

[

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-1719-4617" alt="" src="/th/wp-content/uploads/2024/09/line\_icon\_tax\_calculate\_page\_x2.webp" class="ct-image"/\>![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-1720-4617" alt="" src="/th/wp-content/uploads/2024/10/good\_icon\_toppage\_calculate\_page.webp" class="ct-image"/\>

แอด LINE

](/th/addfriend/?campaign=linefriend&product=taxdeduction)

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-1736-4617" alt="" src="/th/wp-content/uploads/2024/09/check-circle-solid\_copy\_success\_popup\_cancer\_page.webp" class="ct-image"/\>

คัดลอกแล้ว

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-2260-4617" alt="" src="/th/wp-content/uploads/2024/09/rbl\_logo\_tax\_saving\_page.webp" class="ct-image "/\>![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-2261-4617" alt="" src="/th/wp-content/uploads/2024/09/close\_popup\_seemore\_image\_saving\_page.svg" class="ct-image" onclick="popUpInModal(&apos;&apos;, &apos;hide&apos;);"/\>

รายละเอียดแผนประกัน

Hero10/1

จ่ายเบี้ย 1 ปี รับเงินคืนปีที่ 10

รับประกันโดย แรบบิท ไลฟ์ ประกันชีวิต

สำหรับอายุ 20 - 65 ปี

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-2276-4617" alt="" src="/th/wp-content/uploads/2024/09/rbl\_logo\_tax\_saving\_page.webp" class="ct-image"/\>

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-2278-4617" alt="" src="/th/wp-content/uploads/2024/09/Combined\_Shape\_Section\_3\_tax\_saving.webp" class="ct-image"/\>

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-2282-4617" alt="" src="/th/wp-content/uploads/2024/07/check\_small\_list\_cancer\_section\_3.png" class="ct-image"/\>

สายฟรีแลนซ์วางแผนง่าย

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-2285-4617" alt="" src="/th/wp-content/uploads/2024/07/check\_small\_list\_cancer\_section\_3.png" class="ct-image"/\>

จ่ายเบี้ยครั้งเดียวจบ ไม่มีภาระผูกพัน

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-2288-4617" alt="" src="/th/wp-content/uploads/2024/07/check\_small\_list\_cancer\_section\_3.png" class="ct-image"/\>

ผลตอบแทนเฉลี่ยต่อปี (IRR) สูง 2.3%

ตัวอย่างผลประโยชน์และความคุ้มครอง

เพศชาย อายุ 35 ปี ซื้อทุนประกัน 1แสนบาท

จ่ายเบี้ย 1 ปี 98,200 บาท

รับเงินคืนรวมตลอดสัญญา 121,000 บาท (121%)

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-2306-4617" alt="" src="/th/wp-content/uploads/2024/09/saving-hero10-1-table\_saving\_page\_.webp" class="ct-image"/\>

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-2309-4617" alt="" src="/th/wp-content/uploads/2024/09/zoom\_in\_icon\_saving\_page.svg" class="ct-image"/\>

ขยายภาพใหญ่

(1),(2)

%ของจำนวนเงินเอาประกันภัย

ปีกรมธรรม์ที่

1

2

3

4

5

6

7

8

9

10

รวมทั้งสิ้น

เบี้ยประกันชีวิตต่อปี

(ณ ต้นปีกรมธรรม์)

98,200

\-

\-

\-

\-

\-

\-

\-

\-

\-

98,200

ผลประโยชน์

(ณ สิ้นปีกรมธรรม์)

%

จำนวนเงิน

2.1%

2,100

2.1%

2,100

2.1%

2,100

2.1%

2,100

2.1%

2,100

2.1%

2,100

2.1%

2,100

2.1%

2,100

2.1%

2,100

102.1%

102,100

121%

121,000

ความคุ้มครองชีวิต

(จำนวนเงินเอาประกันภัย)

%

จำนวนเงิน

105%

105,000

105%

105,000

105%

105,000

105%

105,000

105%

105,000

105%

105,000

105%

105,000

105%

105,000

105%

105,000

105%

105,000

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-2496-4617" alt="" src="/th/wp-content/uploads/2024/09/PoseG\_hand\_coin\_right\_saving\_page.webp" class="ct-image"/\>![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-2497-4617" alt="" src="/th/wp-content/uploads/2024/09/Coin\_blur\_top\_saving\_page.webp" class="ct-image"/\>![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-2498-4617" alt="" src="/th/wp-content/uploads/2024/09/Coin\_blur\_top\_saving\_page\_.webp" class="ct-image"/\>![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-2499-4617" alt="" src="/th/wp-content/uploads/2024/09/Coin\_blur\_left\_saving\_page.webp" class="ct-image"/\>

เบี้ย 1 ปี เบี้ยเริ่มต้น

30,000 บาท/ปี

[

ซื้อเลย

](https://www.rabbitlife.co.th/th/products/hero-10-1/product-detail?sale_code=BRBKHEY.BK0001.Heygoody)

รายละเอียดแผนประกัน

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-2522-4617" alt="" src="/th/wp-content/uploads/2024/07/fire\_logo\_for\_cancer\_section\_3.png" class="ct-image"/\>

กระเป๋าเดินทางมูลค่า 4,690

[

เงื่อนไข

](/th/promotion/savinginsurance/tlife102-104-105-oct2024/)

Smart Saving 10/2

จ่ายเบี้ย 2 ปี รับเงินคืนปีที่ 10

รับประกันโดย ที ไลฟ์ ประกันชีวิต

สำหรับอายุ 1 เดือน 1 วัน - 60 ปี

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-2532-4617" alt="" src="/th/wp-content/uploads/2024/09/tcap\_logo\_tax\_saving\_page\_\_.webp" class="ct-image"/\>

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-2534-4617" alt="" src="/th/wp-content/uploads/2024/09/Combined\_Shape\_Section\_3\_tax\_saving.webp" class="ct-image"/\>

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-2538-4617" alt="" src="/th/wp-content/uploads/2024/07/check\_small\_list\_cancer\_section\_3.png" class="ct-image"/\>

สายฟรีแลนซ์วางแผนง่าย

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-2541-4617" alt="" src="/th/wp-content/uploads/2024/07/check\_small\_list\_cancer\_section\_3.png" class="ct-image"/\>

จ่ายเบี้ยสั้นแค่ 2 ปี

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-2544-4617" alt="" src="/th/wp-content/uploads/2024/07/check\_small\_list\_cancer\_section\_3.png" class="ct-image"/\>

รับเงินคืนระหว่างสัญญาปีละ 5%\*

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-5106-4617" alt="" src="/th/wp-content/uploads/2024/07/check\_small\_list\_cancer\_section\_3.png" class="ct-image"/\>

ผลตอบแทนเฉลี่ยต่อปี (IRR) 1.82%

ตัวอย่างผลประโยชน์และความคุ้มครอง

เพศชาย อายุ 35 ปี ซื้อทุนประกัน 1แสนบาท

จ่ายเบี้ย 2 ปี 200,000 บาท

รับเงินคืนรวมตลอดสัญญา 233,000 บาท (233%)

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-2562-4617" alt="" src="/th/wp-content/uploads/2024/09/saving-hero10-2-table\_saving\_page.webp" class="ct-image"/\>

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-2565-4617" alt="" src="/th/wp-content/uploads/2024/09/zoom\_in\_icon\_saving\_page.svg" class="ct-image"/\>

ขยายภาพใหญ่

(1),(2)

%ของจำนวนเงินเอาประกันภัย

ปีกรมธรรม์ที่

1

2

3

4

5

6

7

8

9

10

รวมทั้งสิ้น

เบี้ยประกันชีวิตต่อปี

(ณ ต้นปีกรมธรรม์)

100,000

100,000

\-

\-

\-

\-

\-

\-

\-

\-

200,000

ผลประโยชน์

(ณ สิ้นปีกรมธรรม์)

%

จำนวนเงิน

5%

5,000

5%

5,000

5%

5,000

5%

5,000

5%

5,000

5%

5,000

5%

5,000

5%

5,000

5%

5,000

188%

188,000

233%

233,000

ความคุ้มครองชีวิต

(จำนวนเงินเอาประกันภัย)

%

จำนวนเงิน

110%

110,000

210%

210,000

210%

210,000

210%

210,000

210%

210,000

210%

210,000

210%

210,000

210%

210,000

210%

210,000

210%

210,000

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-2752-4617" alt="" src="/th/wp-content/uploads/2024/09/PoseG\_hand\_coin\_right\_saving\_page.webp" class="ct-image"/\>![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-2753-4617" alt="" src="/th/wp-content/uploads/2024/09/Coin\_blur\_top\_saving\_page.webp" class="ct-image"/\>![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-2754-4617" alt="" src="/th/wp-content/uploads/2024/09/Coin\_blur\_top\_saving\_page\_.webp" class="ct-image"/\>![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-2755-4617" alt="" src="/th/wp-content/uploads/2024/09/Coin\_blur\_left\_saving\_page.webp" class="ct-image"/\>

จ่ายเบี้ย 2 ปี เริ่มต้นปีละ

80,000 บาท/ปี

[

ซื้อเลย

](https://online.tlife.co.th/SmartSale.aspx?SubBusiLine=613&SaleCode=90999&PlanCode=20-01-02)

รายละเอียดแผนประกัน

Hero 10/3

จ่ายเบี้ย 3 ปี รับเงินคืนปีที่ 10

รับประกันโดย แรบบิท ไลฟ์ ประกันชีวิต

สำหรับอายุ 20 - 60 ปี

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-2779-4617" alt="" src="/th/wp-content/uploads/2024/09/rbl\_logo\_tax\_saving\_page.webp" class="ct-image"/\>

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-2781-4617" alt="" src="/th/wp-content/uploads/2024/09/Combined\_Shape\_Section\_3\_tax\_saving.webp" class="ct-image"/\>

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-2785-4617" alt="" src="/th/wp-content/uploads/2024/07/check\_small\_list\_cancer\_section\_3.png" class="ct-image"/\>

รับเงินคืนระหว่างปีสูง 20%\*

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-2788-4617" alt="" src="/th/wp-content/uploads/2024/07/check\_small\_list\_cancer\_section\_3.png" class="ct-image"/\>

ผลตอบแทนเฉลี่ยต่อปี (IRR) สูง 2.25%

ตัวอย่างผลประโยชน์และความคุ้มครอง

เพศชาย อายุ 35 ปี ซื้อทุนประกัน 1แสนบาท

จ่ายเบี้ย 3 ปี 300,000 บาท

รับเงินคืนรวมตลอดสัญญา 345,000 บาท (345%)

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-2806-4617" alt="" src="/th/wp-content/uploads/2024/09/saving-hero10-3-table\_saving\_page.webp" class="ct-image"/\>

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-2809-4617" alt="" src="/th/wp-content/uploads/2024/09/zoom\_in\_icon\_saving\_page.svg" class="ct-image"/\>

ขยายภาพใหญ่

(1),(2)

%ของจำนวนเงินเอาประกันภัย

ปีกรมธรรม์ที่

1

2

3

4

5

6

7

8

9

10

รวมทั้งสิ้น

เบี้ยประกันชีวิตต่อปี

(ณ ต้นปีกรมธรรม์)

100,000

100,000

100,000

\-

\-

\-

\-

\-

\-

\-

300,000

ผลประโยชน์

(ณ สิ้นปีกรมธรรม์)

%

จำนวนเงิน

20%

20,000

20%

20,000

20%

20,000

20%

20,000

20%

20,000

20%

20,000

20%

20,000

20%

20,000

20%

20,000

165%

165,000

345%

345,000

ความคุ้มครองชีวิต

(จำนวนเงินเอาประกันภัย)

%

จำนวนเงิน

101%

101,000

202%

202,000

303%

303,000

303%

303,000

303%

303,000

303%

303,000

303%

303,000

303%

303,000

303%

303,000

303%

303,000

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-2996-4617" alt="" src="/th/wp-content/uploads/2024/09/PoseG\_hand\_coin\_right\_saving\_page.webp" class="ct-image"/\>![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-2997-4617" alt="" src="/th/wp-content/uploads/2024/09/Coin\_blur\_top\_saving\_page.webp" class="ct-image"/\>![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-2998-4617" alt="" src="/th/wp-content/uploads/2024/09/Coin\_blur\_top\_saving\_page\_.webp" class="ct-image"/\>![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-2999-4617" alt="" src="/th/wp-content/uploads/2024/09/Coin\_blur\_left\_saving\_page.webp" class="ct-image"/\>

จ่ายเบี้ย 3 ปี เริ่มต้นปีละ

30,000 บาท/ปี

[

ซื้อเลย

](https://www.rabbitlife.co.th/th/products/hero-10-3/product-detail?sale_code=BRBKHEY.BK0001.Heygoody)

รายละเอียดแผนประกัน

ออมจุใจ 10/3

จ่ายเบี้ย 3 ปี รับเงินคืนปีที่ 10

รับประกันโดย เมืองไทยประกันชีวิต

สำหรับอายุ 20 - 80 ปี

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-3023-4617" alt="" src="/th/wp-content/uploads/2024/09/mtl\_logo\_tax\_saving\_page.webp" class="ct-image"/\>

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-3025-4617" alt="" src="/th/wp-content/uploads/2024/09/Combined\_Shape\_Section\_3\_tax\_saving.webp" class="ct-image"/\>

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-3032-4617" alt="" src="/th/wp-content/uploads/2024/07/check\_small\_list\_cancer\_section\_3.png" class="ct-image"/\>

วัยเก๋าลดหย่อนสบาย เพราะสมัครได้จนถึงอายุ 80 ปี

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-3035-4617" alt="" src="/th/wp-content/uploads/2024/07/check\_small\_list\_cancer\_section\_3.png" class="ct-image"/\>

รับเงินคืนปีสุดท้ายสูงถึง 324%\*

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-5113-4617" alt="" src="/th/wp-content/uploads/2024/07/check\_small\_list\_cancer\_section\_3.png" class="ct-image"/\>

ผลตอบแทนเฉลี่ยต่อปี (IRR) 1.18%

ตัวอย่างผลประโยชน์และความคุ้มครอง

เพศชาย อายุ 35 ปี ซื้อทุนประกัน 1แสนบาท

จ่ายเบี้ย 3 ปี 300,000 บาท

รับเงินคืนรวมตลอดสัญญา 333,000 บาท (333%)

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-3053-4617" alt="" src="/th/wp-content/uploads/2024/09/saving-herojujai10-3-table\_saving\_page\_.webp" class="ct-image"/\>

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-3056-4617" alt="" src="/th/wp-content/uploads/2024/09/zoom\_in\_icon\_saving\_page.svg" class="ct-image"/\>

ขยายภาพใหญ่

(1),(2)

%ของจำนวนเงินเอาประกันภัย

ปีกรมธรรม์ที่

1

2

3

4

5

6

7

8

9

10

รวมทั้งสิ้น

เบี้ยประกันชีวิตต่อปี

(ณ ต้นปีกรมธรรม์)

100,000

100,000

100,000

\-

\-

\-

\-

\-

\-

\-

300,000

ผลประโยชน์

(ณ สิ้นปีกรมธรรม์)

%

จำนวนเงิน

1%

1,000

1%

1,000

1%

1,000

1%

1,000

1%

1,000

1%

1,000

1%

1,000

1%

1,000

1%

1,000

324%

324,000

333%

333,000

ความคุ้มครองชีวิต

(จำนวนเงินเอาประกันภัย)

%

จำนวนเงิน

101%

101,000

202%

202,000

303%

303,000

303%

303,000

303%

303,000

303%

303,000

303%

303,000

303%

303,000

303%

303,000

303%

303,000

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-3243-4617" alt="" src="/th/wp-content/uploads/2024/09/PoseG\_hand\_coin\_right\_saving\_page.webp" class="ct-image"/\>![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-3244-4617" alt="" src="/th/wp-content/uploads/2024/09/Coin\_blur\_top\_saving\_page.webp" class="ct-image"/\>![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-3245-4617" alt="" src="/th/wp-content/uploads/2024/09/Coin\_blur\_top\_saving\_page\_.webp" class="ct-image"/\>![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-3246-4617" alt="" src="/th/wp-content/uploads/2024/09/Coin\_blur\_left\_saving\_page.webp" class="ct-image"/\>

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-3250-4617" alt="" src="/th/wp-content/uploads/2024/09/coupon\_promotion\_copy\_.webp" class="ct-image"/\>

ลดอีก 5%

ใส่โค้ด “heygoody5”

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-3255-4617" alt="" src="/th/wp-content/uploads/2024/07/copy\_icon\_for\_cancer\_page.png" class="ct-image"/\>

จ่ายเบี้ย 3 ปี เริ่มต้นปีละ

50,000 บาท/ปี

[

ซื้อเลย

](https://online.muangthai.co.th/th/calculate/heygoody/savings-10-3/?cp=heygoody5)

ผ่อน 0% 6 เดือน เฉพาะบัตรที่ร่วมรายการ

รายละเอียดแผนประกัน

PRUEasy Saver 10/4

จ่ายเบี้ย 4 ปี รับเงินคืนปีที่ 10

รับประกันโดย พรูเด็นเชียล ประกันชีวิต

สำหรับอายุ 20 - 65 ปี

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-3279-4617" alt="" src="/th/wp-content/uploads/2024/09/pru\_logo\_tax\_saving\_page.webp" class="ct-image"/\>

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-3281-4617" alt="" src="/th/wp-content/uploads/2024/09/Combined\_Shape\_Section\_3\_tax\_saving.webp" class="ct-image"/\>

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-3288-4617" alt="" src="/th/wp-content/uploads/2024/07/check\_small\_list\_cancer\_section\_3.png" class="ct-image"/\>

ซื้อได้สูงสุดถึง 1 ล้านบาท

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-3291-4617" alt="" src="/th/wp-content/uploads/2024/07/check\_small\_list\_cancer\_section\_3.png" class="ct-image"/\>

รับเงินคืนปีสุดท้ายสูงถึง 404%\*

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-5117-4617" alt="" src="/th/wp-content/uploads/2024/07/check\_small\_list\_cancer\_section\_3.png" class="ct-image"/\>

ผลตอบแทนเฉลี่ยต่อปี (IRR) 1.19%

ตัวอย่างผลประโยชน์และความคุ้มครอง

เพศชาย อายุ 35 ปี ซื้อทุนประกัน 1แสนบาท

จ่ายเบี้ย 4 ปี 400,000 บาท

รับเงินคืนรวมตลอดสัญญา 440,000 บาท (440%)

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-3309-4617" alt="" src="/th/wp-content/uploads/2024/09/prusaving-plus-10-4-table\_saving\_page.webp" class="ct-image"/\>

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-3312-4617" alt="" src="/th/wp-content/uploads/2024/09/zoom\_in\_icon\_saving\_page.svg" class="ct-image"/\>

ขยายภาพใหญ่

(1),(2)

%ของจำนวนเงินเอาประกันภัย

ปีกรมธรรม์ที่

1

2

3

4

5

6

7

8

9

10

รวมทั้งสิ้น

เบี้ยประกันชีวิตต่อปี

(ณ ต้นปีกรมธรรม์)

100,000

100,000

100,000

100,000

\-

\-

\-

\-

\-

\-

400,000

ผลประโยชน์

(ณ สิ้นปีกรมธรรม์)

%

จำนวนเงิน

4%

4,000

4%

4,000

4%

4,000

4%

4,000

4%

4,000

4%

4,000

4%

4,000

4%

4,000

4%

4,000

404%

404,000

440%

440,000

ความคุ้มครองชีวิต

(จำนวนเงินเอาประกันภัย)

%

จำนวนเงิน

101%

101,000

202%

202,000

303%

303,000

404%

404,000

404%

404,000

404%

404,000

404%

404,000

404%

404,000

404%

404,000

404%

404,000

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-3499-4617" alt="" src="/th/wp-content/uploads/2024/09/PoseG\_hand\_coin\_right\_saving\_page.webp" class="ct-image"/\>![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-3500-4617" alt="" src="/th/wp-content/uploads/2024/09/Coin\_blur\_top\_saving\_page.webp" class="ct-image"/\>![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-3501-4617" alt="" src="/th/wp-content/uploads/2024/09/Coin\_blur\_top\_saving\_page\_.webp" class="ct-image"/\>![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-3502-4617" alt="" src="/th/wp-content/uploads/2024/09/Coin\_blur\_left\_saving\_page.webp" class="ct-image"/\>

จ่ายเบี้ย 4 ปี เริ่มต้นปีละ

20,000 บาท/ปี

[

ซื้อเลย

](https://online.prudential.co.th/referral/heygoodyxprueasysaver10-4)

ผ่อน 0% 10 เดือน เฉพาะบัตรที่ร่วมรายการ

รายละเอียดแผนประกัน

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-3531-4617" alt="" src="/th/wp-content/uploads/2024/07/fire\_logo\_for\_cancer\_section\_3.png" class="ct-image"/\>

Central Voucher สูงสุด 100,000

[

เงื่อนไข

](/th/promotion/savinginsurance/tlife102-104-105-oct2024/)

Smart Saving 10/4

จ่ายเบี้ย 4 ปี รับเงินคืนปีที่ 10

รับประกันโดย ที ไลฟ์ ประกันชีวิต

สำหรับอายุ 1 เดือน 1 วัน - 65 ปี

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-3541-4617" alt="" src="/th/wp-content/uploads/2024/09/tcap\_logo\_tax\_saving\_page\_\_.webp" class="ct-image"/\>

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-3543-4617" alt="" src="/th/wp-content/uploads/2024/09/Combined\_Shape\_Section\_3\_tax\_saving.webp" class="ct-image"/\>

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-3547-4617" alt="" src="/th/wp-content/uploads/2024/07/check\_small\_list\_cancer\_section\_3.png" class="ct-image"/\>

มือใหม่เริ่มลดหย่อนยิ้มได้ ด้วยเบี้ยเริ่มต้นเบาๆ

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-3550-4617" alt="" src="/th/wp-content/uploads/2024/07/check\_small\_list\_cancer\_section\_3.png" class="ct-image"/\>

ทยอยจ่ายได้แบบรายเดือน ไม่กระเทือนตอนสิ้นปี

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-3553-4617" alt="" src="/th/wp-content/uploads/2024/07/check\_small\_list\_cancer\_section\_3.png" class="ct-image"/\>

ผลตอบแทนเฉลี่ยต่อปี (IRR) สูง 1.88%

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-3556-4617" alt="" src="/th/wp-content/uploads/2024/07/check\_small\_list\_cancer\_section\_3.png" class="ct-image"/\>

รับเงินคืนปีสุดท้ายสูงถึง 410%\*

ตัวอย่างผลประโยชน์และความคุ้มครอง

เพศชาย อายุ 35 ปี ซื้อทุนประกัน 1แสนบาท

จ่ายเบี้ย 4 ปี 400,000 บาท

รับเงินคืนรวมตลอดสัญญา 460,000 บาท (460%)

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-3574-4617" alt="" src="/th/wp-content/uploads/2024/09/smart-saving10-4-table\_saving\_page.webp" class="ct-image"/\>

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-3577-4617" alt="" src="/th/wp-content/uploads/2024/09/zoom\_in\_icon\_saving\_page.svg" class="ct-image"/\>

ขยายภาพใหญ่

(1),(2)

%ของจำนวนเงินเอาประกันภัย

ปีกรมธรรม์ที่

1

2

3

4

5

6

7

8

9

10

รวมทั้งสิ้น

เบี้ยประกันชีวิตต่อปี

(ณ ต้นปีกรมธรรม์)

100,000

100,000

100,000

100,000

\-

\-

\-

\-

\-

\-

400,000

ผลประโยชน์

(ณ สิ้นปีกรมธรรม์)

%

จำนวนเงิน

5%

5,000

5%

5,000

5%

5,000

5%

5,000

6%

6,000

6%

6,000

6%

6,000

6%

6,000

6%

6,000

410%

410,000

460%

460,000

ความคุ้มครองชีวิต

(จำนวนเงินเอาประกันภัย)

%

จำนวนเงิน

110%

110,000

220%

220,000

330%

330,000

440%

440,000

440%

440,000

440%

440,000

440%

440,000

440%

440,000

440%

440,000

440%

440,000

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-3764-4617" alt="" src="/th/wp-content/uploads/2024/09/PoseG\_hand\_coin\_right\_saving\_page.webp" class="ct-image"/\>![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-3765-4617" alt="" src="/th/wp-content/uploads/2024/09/Coin\_blur\_top\_saving\_page.webp" class="ct-image"/\>![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-3766-4617" alt="" src="/th/wp-content/uploads/2024/09/Coin\_blur\_top\_saving\_page\_.webp" class="ct-image"/\>![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-3767-4617" alt="" src="/th/wp-content/uploads/2024/09/Coin\_blur\_left\_saving\_page.webp" class="ct-image"/\>

จ่ายเบี้ย 4 ปี เริ่มต้นปีละ

24,000 บาท/ปี

[

ซื้อเลย

](https://online.tlife.co.th/SmartSale.aspx?SubBusiLine=613&SaleCode=90999&PlanCode=20-09-04)

รายละเอียดแผนประกัน

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-3787-4617" alt="" src="/th/wp-content/uploads/2024/07/fire\_logo\_for\_cancer\_section\_3.png" class="ct-image"/\>

กระเป๋าเดินทางมูลค่า 4,690

[

เงื่อนไข

](/th/promotion/savinginsurance/tlife102-104-105-oct2024/)

Smart Saving 10/5

จ่ายเบี้ย 5 ปี รับเงินคืนปีที่ 10

รับประกันโดย ที ไลฟ์ ประกันชีวิต

สำหรับอายุ 1 เดือน 1 วัน - 65 ปี

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-3797-4617" alt="" src="/th/wp-content/uploads/2024/09/tcap\_logo\_tax\_saving\_page\_\_.webp" class="ct-image"/\>

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-3799-4617" alt="" src="/th/wp-content/uploads/2024/09/Combined\_Shape\_Section\_3\_tax\_saving.webp" class="ct-image"/\>

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-3803-4617" alt="" src="/th/wp-content/uploads/2024/07/check\_small\_list\_cancer\_section\_3.png" class="ct-image"/\>

สายลงทุนกดไลค์ เอาเงินคืนระหว่างสัญญาไปลงทุนต่อได้

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-3806-4617" alt="" src="/th/wp-content/uploads/2024/07/check\_small\_list\_cancer\_section\_3.png" class="ct-image"/\>

ผลตอบแทนเฉลี่ยต่อปี (IRR) สูงปรี๊ด 2.40%

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-3809-4617" alt="" src="/th/wp-content/uploads/2024/07/check\_small\_list\_cancer\_section\_3.png" class="ct-image"/\>

รับเงินคืนระหว่างสัญญาสูง 100%\*

ตัวอย่างผลประโยชน์และความคุ้มครอง

เพศชาย อายุ 35 ปี ซื้อทุนประกัน 1แสนบาท

จ่ายเบี้ย 5 ปี 500,000 บาท

รับเงินคืนรวมตลอดสัญญา 570,000 บาท (570%)

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-3827-4617" alt="" src="/th/wp-content/uploads/2024/10/smart-saving10-5-table\_saving\_page.webp" class="ct-image"/\>

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-3830-4617" alt="" src="/th/wp-content/uploads/2024/09/zoom\_in\_icon\_saving\_page.svg" class="ct-image"/\>

ขยายภาพใหญ่

(1),(2)

%ของจำนวนเงินเอาประกันภัย

ปีกรมธรรม์ที่

1

2

3

4

5

6

7

8

9

10

รวมทั้งสิ้น

เบี้ยประกันชีวิตต่อปี

(ณ ต้นปีกรมธรรม์)

100,000

100,000

100,000

100,000

100,000

\-

\-

\-

\-

\-

500,000

ผลประโยชน์

(ณ สิ้นปีกรมธรรม์)

%

จำนวนเงิน

20%

20,000

20%

20,000

20%

20,000

20%

20,000

20%

20,000

20%

20,000

100%

100,000

100%

100,000

100%

100,000

150%

150,000

570%

570,000

ความคุ้มครองชีวิต

(จำนวนเงินเอาประกันภัย)

%

จำนวนเงิน

115%

115,000

215%

215,000

315%

315,000

415%

415,000

515%

515,000

515%

515,000

415%

415,000

315%

315,000

215%

215,000

145%

145,000

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-4017-4617" alt="" src="/th/wp-content/uploads/2024/09/PoseG\_hand\_coin\_right\_saving\_page.webp" class="ct-image"/\>![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-4018-4617" alt="" src="/th/wp-content/uploads/2024/09/Coin\_blur\_top\_saving\_page.webp" class="ct-image"/\>![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-4019-4617" alt="" src="/th/wp-content/uploads/2024/09/Coin\_blur\_top\_saving\_page\_.webp" class="ct-image"/\>![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-4020-4617" alt="" src="/th/wp-content/uploads/2024/09/Coin\_blur\_left\_saving\_page.webp" class="ct-image"/\>

จ่ายเบี้ย 5 ปี เบี้ยเริ่มต้น

100,000 บาท/ปี

[

ซื้อเลย

](https://online.tlife.co.th/SmartSale.aspx?SubBusiLine=613&SaleCode=90999&PlanCode=20-03-05)

รายละเอียดแผนประกัน

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-4042-4617" alt="" src="/th/wp-content/uploads/2024/07/fire\_logo\_for\_cancer\_section\_3.png" class="ct-image"/\>

Central Voucher สูงสุด 5,500

[

เงื่อนไข

](https://www.heygoody.com/th/promotion/annuityinsurance/mtl991-nov2024/)

Easy Retire 99/1

จ่ายเบี้ย 1 ปี รับบำนาญอายุ 65 - 99 ปี

รับประกันโดย เมืองไทยประกันชีวิต

สำหรับอายุ 21 - 60 ปี

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-4052-4617" alt="" src="/th/wp-content/uploads/2024/09/mtl\_logo\_tax\_saving\_page.webp" class="ct-image"/\>

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-4054-4617" alt="" src="/th/wp-content/uploads/2024/10/combined-shape-annuity.webp" class="ct-image"/\>

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-4058-4617" alt="" src="/th/wp-content/uploads/2024/07/check\_small\_list\_cancer\_section\_3.png" class="ct-image"/\>

วางแผนง่าย จ่ายปีเดียว ไม่มีภาระผูกพัน สร้าง passive income ทันทีหลังเกษียณ

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-4061-4617" alt="" src="/th/wp-content/uploads/2024/07/check\_small\_list\_cancer\_section\_3.png" class="ct-image"/\>

รับเงินบำนาญปีละ 12%\* ทุกปี ตั้งแต่อายุครบ 65 ปี จนถึง 99 ปี

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-4064-4617" alt="" src="/th/wp-content/uploads/2024/07/check\_small\_list\_cancer\_section\_3.png" class="ct-image"/\>

รวมรับเงินบำนาญตลอดสัญญาสูงสุด 420%\*

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-4067-4617" alt="" src="/th/wp-content/uploads/2024/07/check\_small\_list\_cancer\_section\_3.png" class="ct-image"/\>

ซื้อออนไลน์ ง่าย ไม่ต้องตรวจสุขภาพ

จ่ายเบี้ย 1 ปี เบี้ยเริ่มต้น

X0,000บาท/ปี

[

ซื้อเลย

](http://)

ตัวอย่างผลประโยชน์และความคุ้มครอง

เพศชาย อายุ 35 ปี ซื้อทุนประกัน 100,000 บาท

จ่ายเบี้ย 1 ปี 135,786 บาท

ผลประโยชน์รวมเมื่ออยู่ครบสัญญา 

420,000 บาท (420%)

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-4094-4617" alt="" src="https://www.heygoody.com/th/wp-content/uploads/2024/10/table-Easy-Retire-Annuity\_edit.webp" class="ct-image"/\>

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-4097-4617" alt="" src="/th/wp-content/uploads/2024/09/zoom\_in\_icon\_saving\_page.svg" class="ct-image"/\>

ขยายภาพใหญ่

(1)

ประโยชน์ % ของจำนวนเงินเอาประกันภัย ณ วันเริ่มสัญญา

(2)

กรณีเสียชีวิตในช่วงก่อนรับเงินบำนาญ (ก่อนวันครบรอบปีกรมธรรม์ที่ผู้เอาประกันภัยครบอายุ 65 ปี) บริษัทจะจ่ายเงินให้แก่ผู้รับประโยชน์เท่ากับ 110% ของเบี้ยประกันภัยชำระครั้งเดียว หรือเงินค่าเวนคืนกรมธรรม์ประกันภัยในขณะนั้น (ตามแต่จำนวนใดจะมากกว่า)

(3)

กรณีเสียชีวิตในช่วงรับเงินบำนาญ (ตั้งแต่วันครบรอบปีกรมธรรม์ที่ผู้เอาประกันภัยครบอายุ 65 ปี) บริษัทจะจ่ายเงินให้แก่ผู้รับประโยชน์เท่ากับเบี้ยประกันภัยชำระครั้งเดียวที่ผู้เอาประกันภัยได้ชำระไว้แล้ว หักด้วยเงินบำนาญสะสมที่ผู้เอาประกันภัยได้รับไปแล้ว

อายุผู้เอา  
ประกัน

35 ปี

36-64 ปี

65 ปี

66 ปี

67 ปี

68 ปี

69 ปี

70 ปี

71 ปี

72 ปี

73 ปี

74 ปี

75 ปี

76-99 ปี

รวมทั้งสิ้น

ปีกรมธรรม์ที่

ปีที่ 1

ปีที่ 2-30

ปีที่ 31

ปีที่ 32

ปีที่ 33

ปีที่ 34

ปีที่ 35

ปีที่ 36

ปีที่ 37

ปีที่ 38

ปีที่ 39

ปีที่ 40

ปีที่ 41

 ปีที่ 42-65

 (24 รอบปีกรมธรรม์)

เบี้ยประกันชีวิตต่อปี  
(ณ ต้นปีกรมธรรม์)

135,786

\-

\-

\-

\-

\-

\-

 \-

\-

\-

\-

\-

\-

\-

135,786

รับเงินบำนาญ

(ณ ทุกสิ้นปีกรมธรรม์)

%

จำนวนเงิน

\-

\-

\-

\-

12%

12,000

12%

12,000

12%

12,000

12%

12,000

12%

12,000

12%

12,000

12%

12,000

12%

12,000

12%

12,000

12%

12,000

12%

12,000

12%

12,000

420%

420,000

ความคุ้มครองชีวิต(จำนวนเงินเอาประกันภัย)

ช่วงก่อนรับบำนาญ

(บริษัทประกันจะจ่ายผลประโยชน์ 110% ของเบี้ยประกันภัยที่ผู้เอาประกันได้ชำระมาแล้วทั้งหมดหรือจ่ายผลประโยชน์ตาม  
มูลค่าเวนคืนกรมธรรม์ แล้วแต่มูลค่าใดจะสูงกว่า)

%

จำนวนเงิน

110%

149,365

110%

149,365

\-

\-

\-

\-

\-

\-

\-

\-

\-

\-

\-

\-

\-

\-

\-

\-

\-

\-

\-

\-

\-

\-

\-

\-

ช่วงหลังรับบำนาญ

(บริษัทประกันจะจ่ายผลประโยชน์ด้วยเบี้ย  
ประกันภัยสะสมที่ผู้เอาประกันได้ชำระมาแล้วทั้งหมด หักด้วยเงินบำนาญที่ผู้เอาประกันได้รับไปแล้ว)

จำนวนเงิน

\-

\-

123,786

111,786

99,786

87,786

75,786

63,786

51,786

39,786

27,786

15,786

3,786

\-

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-4431-4617" alt="" src="/th/wp-content/uploads/2024/09/PoseG\_hand\_coin\_right\_saving\_page.webp" class="ct-image"/\>![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-4432-4617" alt="" src="/th/wp-content/uploads/2024/09/Coin\_blur\_top\_saving\_page.webp" class="ct-image"/\>![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-4433-4617" alt="" src="/th/wp-content/uploads/2024/09/Coin\_blur\_top\_saving\_page\_.webp" class="ct-image"/\>![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-4434-4617" alt="" src="/th/wp-content/uploads/2024/09/Coin\_blur\_left\_saving\_page.webp" class="ct-image"/\>

เบี้ย 1 ปี เบี้ยเริ่มต้น

53,143 บาท/ปี

[

ซื้อเลย

](https://online.muangthai.co.th/th/detail/heygoody/easy-retire/)

รายละเอียดแผนประกัน

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-4455-4617" alt="" src="/th/wp-content/uploads/2024/07/fire\_logo\_for\_cancer\_section\_3.png" class="ct-image"/\>

Central Voucher สูงสุด 3,000

[

เงื่อนไข

](http://)

Smart Annuity 95/5

จ่ายเบี้ย 5 ปี รับบำนาญอายุ 60 ปี

รับประกันโดย ที ไลฟ์ ประกันชีวิต

สำหรับอายุ 20 - 55 ปี

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-4465-4617" alt="" src="/th/wp-content/uploads/2024/09/tcap\_logo\_tax\_saving\_page\_\_.webp" class="ct-image"/\>

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-4467-4617" alt="" src="/th/wp-content/uploads/2024/10/combined-shape-annuity.webp" class="ct-image"/\>

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-4471-4617" alt="" src="/th/wp-content/uploads/2024/07/check\_small\_list\_cancer\_section\_3.png" class="ct-image"/\>

จ่ายเบี้ย 5 ปี คุ้มครองยาวจนถึงอายุ 95 ปี

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-4474-4617" alt="" src="/th/wp-content/uploads/2024/07/check\_small\_list\_cancer\_section\_3.png" class="ct-image"/\>

รับเงินบำนาญปีละ 20%\* ทุกปี ตั้งแต่อายุครบ 60 ปี จนถึง 95 ปี

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-4477-4617" alt="" src="/th/wp-content/uploads/2024/07/check\_small\_list\_cancer\_section\_3.png" class="ct-image"/\>

รวมรับเงินบำนาญตลอดสัญญาสูงสุด 720%\*

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-4480-4617" alt="" src="/th/wp-content/uploads/2024/07/check\_small\_list\_cancer\_section\_3.png" class="ct-image"/\>

ผลตอบแทนเฉลี่ยต่อปี (IRR) สูง 3.3% เมื่ออยู่ครบสัญญา

จ่ายเบี้ย 1 ปี เบี้ยเริ่มต้น

X0,000บาท/ปี

[

ซื้อเลย

](http://)

ตัวอย่างผลประโยชน์และความคุ้มครอง

เพศชาย อายุ 35 ปี ซื้อทุนประกัน 100,000 บาท

จ่ายเบี้ย 5 ปี 205,000 บาท

ผลประโยชน์รวมเมื่ออยู่ครบสัญญา 

720,000 บาท (720%)

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-4507-4617" alt="" src="/th/wp-content/uploads/2024/09/table-Easy-Smart-Annuity.webp" class="ct-image"/\>

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-4510-4617" alt="" src="/th/wp-content/uploads/2024/09/zoom\_in\_icon\_saving\_page.svg" class="ct-image"/\>

ขยายภาพใหญ่

(1)

ผลประโยชน์ % ของจำนวนเงินเอาประกันภัย ณ วันเริ่มสัญญา

(2)

กรณีเสียชีวิตในช่วงก่อนรับเงินบำนาญ (ก่อนวันครบรอบปีกรมธรรม์ที่ผู้เอาประกันภัยครบอายุ 60 ปี) บริษัทจะจ่ายเงินให้แก่ผู้รับประโยชน์เท่ากับ 110% ของเบี้ยประกันภัยที่ชำระมาแล้วทั้งหมด หรือเงินค่าเวนคืนกรมธรรม์ แล้วแต่มูลค่าใดสูงกว่า

(3)

กรณีเสียชีวิตในช่วงรับเงินบำนาญ (ตั้งแต่วันครบรอบปีกรมธรรม์ที่ผู้เอาประกันภัยครบอายุ 60 ปี) บริษัทจะจ่ายเงินให้แก่ผู้รับประโยชน์เท่ากับเบี้ยประกันภัยสะสมที่ชำระมาแล้วทั้งหมด หักด้วยเงินบำนาญสะสมที่ผู้เอาประกันภัยได้รับไปแล้ว

อายุผู้เอา  
ประกัน

35 ปี

36 ปี

37 ปี

38 ปี

39 ปี

40-59 ปี

60 ปี

61 ปี

62 ปี

63 ปี

64 ปี

65 ปี

66 ปี

67 ปี

68 ปี

69 ปี

70-95 ปี

รวมทั้งสิ้น

ปีกรมธรรม์ที่

ปีที่ 1

ปีที่ 2

ปีที่ 3

ปีที่ 4

ปีที่ 5

ปีที่ 6-25

ปีที่ 26

ปีที่ 27

ปีที่ 28

ปีที่ 29

ปีที่ 30

ปีที่ 31

ปีที่ 32

 ปีที่ 33

ปีที่ 34

 ปีที่ 35

ปีที่ 36-60

(26 รอบปีกรมธรรม์)

เบี้ยประกันชีวิตต่อปี  
(ณ ต้นปีกรมธรรม์)

41,000

41,000

41,000

41,000

41,000

\-

\-

\-

\-

\-

\-

\-

\-

\-

\-

\-

\-

205,000

รับเงินบำนาญ

(ณ ทุกสิ้นปีกรมธรรม์)

%

จำนวนเงิน

\-

\-

\-

\-

\-

\-

\-

\-

\-

\-

\-

\-

20%

20,000

20%

20,000

20%

20,000

20%

20,000

20%

20,000

20%

20,000

20%

20,000

20%

20,000

20%

20,000

20%

20,000

20%

20,000

720%

720,000

ความคุ้มครองชีวิต(จำนวนเงินเอาประกันภัย)

ช่วงก่อนรับบำนาญ

ช่วงก่อนรับบำนาญ(บริษัทประกันจะจ่ายผลประโยชน์ 110% ของเบี้ยประกันภัยที่ผู้เอาประกันได้ชำระมาแล้วทั้งหมด หรือจ่ายผลประโยชน์ตามมูลค่าเวนคืนกรมธรรม์ แล้วแต่มูลค่าใดจะสูงกว่า)

%

จำนวนเงิน

110%

45,100

110%

90,200

110%

135,300

110%

180,400

110%

225,500

110%

225,500

\-

\-

\-

\-

\-

\-

\-

\-

\-

\-

\-

\-

\-

\-

\-

\-

\-

\-

\-

\-

\-

\-

ช่วงหลังรับบำนาญ

บริษัทจะจ่ายผลประโยชน์เท่ากับเบี้ยประกันภัยสะสมที่ชำระมาแล้วทั้งหมด หักด้วยเงินบำนาญสะสมที่รับไปแล้วทั้งหมด

จำนวนเงิน

45,100

90,200

135,300

180,400

225,500

225,500

185,000

165,000

145,000

125,000

105,000

85,000

65,000

45,000

25,000

5,000

\-

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-4901-4617" alt="" src="/th/wp-content/uploads/2024/09/PoseG\_hand\_coin\_right\_saving\_page.webp" class="ct-image"/\>![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-4902-4617" alt="" src="/th/wp-content/uploads/2024/09/Coin\_blur\_top\_saving\_page.webp" class="ct-image"/\>![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-4903-4617" alt="" src="/th/wp-content/uploads/2024/09/Coin\_blur\_top\_saving\_page\_.webp" class="ct-image"/\>![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-4904-4617" alt="" src="/th/wp-content/uploads/2024/09/Coin\_blur\_left\_saving\_page.webp" class="ct-image"/\>

เริ่มซื้อได้ภายในเดือนธันวาคมนี้

เบี้ย 1 ปี เบี้ยเริ่มต้น

53,143 บาท/ปี

[

ซื้อเลย

](http://)

รายละเอียดแผนประกัน

จ่ายตั้งแต่ระยะแรก ดูแลถึงระยะลุกลาม

รับประกันโดย พรูเด็นเชียล ประกันชีวิต

สำหรับอายุ 20-64 ปี

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-5021-4617" alt="" src="/th/wp-content/uploads/2024/07/prudential\_logo\_for\_cancer\_section\_3\_edit.png" class="ct-image"/\>

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-5026-4617" alt="" src="/th/wp-content/uploads/2024/07/check\_small\_list\_cancer\_section\_3.png" class="ct-image"/\>

คุ้มครองสูงสุด 1.5 ล้าน

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-5029-4617" alt="" src="/th/wp-content/uploads/2024/07/check\_small\_list\_cancer\_section\_3.png" class="ct-image"/\>

ตรวจเจอระยะไม่ลุกลาม รับก้อนแรก

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-5032-4617" alt="" src="/th/wp-content/uploads/2024/07/check\_small\_list\_cancer\_section\_3.png" class="ct-image"/\>

รักษาต่อจนถึงระยะลุกลาม ยังได้อีกก้อน

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-5035-4617" alt="" src="/th/wp-content/uploads/2024/07/check\_small\_list\_cancer\_section\_3.png" class="ct-image"/\>

จ่ายหลักพัน คุ้มครองหลักล้าน

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-5037-4617" alt="" src="/th/wp-content/uploads/2024/07/Combined\_Shape\_smile\_popup\_modal\_cancer.png" class="ct-image"/\>

เบี้ยเริ่มต้น

178บาท/ปี

[

ซื้อเลย

](https://online.prudential.co.th/referral/heygoodyxprucancercare)

แผนประกัน

ระยะความคุ้มครอง 1ปี

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-5055-4617" alt="" src="/th/wp-content/uploads/2024/07/prudential\_logo\_for\_cancer\_section\_3\_edit.png" class="ct-image"/\>

รับประกันโดยพรูเด็นเชียล ประกันชีวิต

เสียชีวิตทุกกรณี

เมื่อตรวจพบ ระยะไม่ลุกลาม

เมื่อตรวจพบ ระยะลุกลาม

แผน 1

50,000

150,000

300,000

แผน 2

50,000

250,000

500,000

แผน 3

50,000

500,000

1,000,000

รายละเอียดแผนประกัน

คุ้มครองทุกระยะ จ่ายเบิล 5 มะเร็งฮิต

รับประกันโดย เมืองไทยประกันภัย

สำหรับอายุ 14 วัน-60 ปี

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-5133-4617" alt="" src="/th/wp-content/uploads/2024/07/mueng\_thai\_logo\_for\_cancer\_section\_3\_edit.png" class="ct-image"/\>

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-5138-4617" alt="" src="/th/wp-content/uploads/2024/07/check\_small\_list\_cancer\_section\_3.png" class="ct-image"/\>

รับเงินก้อนสูงสุด 2.2 ล้าน เมื่อตรวจพบทุกระยะ

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-5141-4617" alt="" src="/th/wp-content/uploads/2024/07/check\_small\_list\_cancer\_section\_3.png" class="ct-image"/\>

รับ 2 เท่า สำหรับ 5 มะเร็งที่พบบ่อย (ปากมดลูก, เต้านม, ตับ, ปอด, ต่อมลูกหมาก)

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-5144-4617" alt="" src="/th/wp-content/uploads/2024/07/check\_small\_list\_cancer\_section\_3.png" class="ct-image"/\>

ต่ออายุได้ถึง 70 ปี

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-5146-4617" alt="" src="/th/wp-content/uploads/2024/07/Combined\_Shape\_smile\_popup\_modal\_cancer.png" class="ct-image"/\>

เบี้ยเริ่มต้น

1,450บาท/ปี

[

ซื้อเลย

](https://mtiecommerce.muangthaiinsurance.com/product/Portal/?enckey=5VczkoFb&url_hgd=JmNhbXBhaWduX2lkPUhHRzIwMjQxMDE1MTkyMjcw)

แผนประกัน

ระยะความคุ้มครอง 1ปี

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-5164-4617" alt="" src="/th/wp-content/uploads/2024/07/mueng\_thai\_logo\_for\_cancer\_section\_3\_edit.png" class="ct-image"/\>

รับประกันโดยเมืองไทย ประกันภัย

โรคมะเร็งทุกชนิด ทุกระยะ (ไม่รวมโรคมะเร็งผิวหนัง ยกเว้น  
มะเร็งผิวหนัง ประเภท “เมลลาโนมา”)

โรคมะเร็งปากมดลูก มะเร็งเต้านม มะเร็งตับ มะเร็งต่อมลูก  
หมาก มะเร็งปอด (รับเพิ่มจากผลประโยชน์ข้อ 1)

โรคมะเร็งผิวหนังที่ไม่ใช่ “เมลลาโนมา”

แผน 1

250,000

250,000

50,000

แผน 2

500,000

500,000

100,000

แผน 3

1,000,000

1,000,000

200,000

รายละเอียดแผนประกัน

คุ้มครองมะเร็งทุกระยะ และไตวายเรื้อรังเข้าได้ทุก รพ. เบิกค่ารักษาได้ตามจริง สูงสุด 10 ล้าน

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-5217-4617" alt="" src="/th/wp-content/uploads/2024/07/mueng\_thai\_life\_logo\_for\_cancer\_section\_3\_edit.png" class="ct-image"/\>

รับประกันโดย เมืองไทย ประกันชีวิต

สำหรับอายุ 20-70 ปี

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-5224-4617" alt="" src="/th/wp-content/uploads/2024/07/check\_small\_list\_cancer\_section\_3.png" class="ct-image"/\>

คุ้มครองมะเร็งสูงสุด 5 ล้าน ไตวาย 5 ล้าน

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-5227-4617" alt="" src="/th/wp-content/uploads/2024/07/check\_small\_list\_cancer\_section\_3.png" class="ct-image"/\>

บำบัดโรคทางจิตเวชหลังป่วย

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-5230-4617" alt="" src="/th/wp-content/uploads/2024/07/check\_small\_list\_cancer\_section\_3.png" class="ct-image"/\>

การันตีต่ออายุได้ถึง 99 ปี

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-5232-4617" alt="" src="/th/wp-content/uploads/2024/07/Combined\_Shape\_smile\_popup\_modal\_cancer.png" class="ct-image"/\>

เบี้ยเริ่มต้น

2,432.50บาท/ปี

[

ซื้อเลย

](https://online.muangthai.co.th/th/calculate/heygoody/care-plus/?url_hgd=JmNhbXBhaWduX2lkPUhHRzIwMjQxMDE1MTkyMjcw)

แผนประกัน

ระยะความคุ้มครอง 1ปี

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-5250-4617" alt="" src="/th/wp-content/uploads/2024/07/mueng\_thai\_life\_logo\_for\_cancer\_section\_3\_edit.png" class="ct-image"/\>

รับประกันโดยเมืองไทยประกันชีวิต

เสียชีวิตทุกกรณี

ความคุ้มครองมะเร็ง

ผลประโยชน์สูงสุดต่อปีกรมธรรม์

ค่าบริการทางการแพทย์เพื่อรักษาโรคมะเร็ง

ค่ายากลับบ้าน (สูงสุด 30 วัน)

ค่าบริการทางการแพทย์เพื่อรักษาโรคจิตเวช

ความคุ้มครองไตวายเรื้อรัง

ผลประโยชน์สูงสุดต่อปีกรมธรรม์

ค่าบริการทางการแพทย์เพื่อรักษาโรคไตวายเรื้อรัง

ค่ายากลับบ้าน (สูงสุด 30 วัน)

ค่าบริการทางการแพทย์เพื่อรักษาโรคจิตเวช

ความคุ้มครอง

50,000

5,000,000

จ่ายตามจริง

100,000

50,000

5,000,000

จ่ายตามจริง

100,000

50,000

รายละเอียดแผนประกัน

เบิกค่ารักษามะเร็งตามจริง สูงสุด 9 ล้าน

รับประกันโดย วิริยะประกันภัย

สำหรับอายุ 21-60 ปี

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-5316-4617" alt="" src="/th/wp-content/uploads/2024/07/viriyah\_logo\_for\_cancer\_section\_3.png" class="ct-image"/\>

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-5321-4617" alt="" src="/th/wp-content/uploads/2024/07/check\_small\_list\_cancer\_section\_3.png" class="ct-image"/\>

เบิกสูงสุด 9 ล้าน ครอบคลุมครบทั้งกระบวนการรักษา (ค่าห้อง, ค่าผ่าตัด, ฉายแสง, ยากลับบ้าน)

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-5324-4617" alt="" src="/th/wp-content/uploads/2024/07/check\_small\_list\_cancer\_section\_3.png" class="ct-image"/\>

รองรับการรักษาได้ทั้งแบบ Chemotherapy และ Targeted Therapy

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-5327-4617" alt="" src="/th/wp-content/uploads/2024/07/check\_small\_list\_cancer\_section\_3.png" class="ct-image"/\>

แผนพิเศษฉพาะการรักษาใน ร.พ.ชั้นนำมาตรฐานเครือ ![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img class="logo\_bdms" src="/th/wp-content/uploads/2024/07/BDMS\_logo\_cancer\_section\_3.png" style="width: 43px;"\> (กรุงเทพ, พญาไท, สมิติ เวช, บีเอ็นเอช, เปาโล)

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-5332-4617" alt="" src="/th/wp-content/uploads/2024/07/check\_small\_list\_cancer\_section\_3.png" class="ct-image"/\>

ฟรี โปรแกรมตรวจสุขภาพเมื่อต่ออายุ

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-5335-4617" alt="" src="/th/wp-content/uploads/2024/07/check\_small\_list\_cancer\_section\_3.png" class="ct-image"/\>

การันตีต่ออายุได้ถึง 80 ปี

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-5337-4617" alt="" src="/th/wp-content/uploads/2024/07/Combined\_Shape\_smile\_popup\_modal\_cancer.png" class="ct-image"/\>

เบี้ยเริ่มต้น

6,240บาท/ปี

[

ซื้อเลย

](https://www.viriyah.com/cancer-insurance?affiliate=8V7jPBFYcEqALnim3SbwWrkTzpGM6fIK&url_hgd=JmNhbXBhaWduX2lkPUhHRzIwMjQxMDE1MTkyMjcw)

แผนประกัน

ระยะความคุ้มครอง 1ปี

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-5355-4617" alt="" src="/th/wp-content/uploads/2024/07/viriyah\_logo\_for\_cancer\_section\_3.png" class="ct-image"/\>

รับประกันโดยวิริยะ ประกันภัย

ค่ารักษาพยาบาลโรคมะเร็ง สูงสุดต่อปี

ค่าห้อง และค่าอาหารผู้ป่วยปกติ (สูงสุดไม่เกิน 365 วัน)

ค่าห้อง และค่าอาหารผู้ป่วยหนัก (สูงสุดไม่เกิน 15 วัน)

ค่ารักษาพยาบาล

* คีโม ฉายแสง
* ค่าผ่าตัด
* ค่ายาผู้ป่วยกลับบ้าน
* การรักษาแบบเฉพาะเจาะจงต่อเซลล์มะเร็ง (Targeted Therapy)
* ค่าบริการทั่วไป

ค่ารักษาพยาบาล

* ค่าธรรมเนียมแพทย์ผ่าตัด สำหรับแพทย์ หรือคณะแพทย์ผู้ช่วยศัลยกรรมแพทย์ หรือแพทย์ที่ทำการผ่าตัดหรือหัตถการทางการแพทย์
* ค่าธรรมเนียมแพทย์ วิสัญญี / พยาบาล วิสัญญี
* ค่าแพทย์เยี่ยมไข้ หรือค่าปรึกษาแพทย์ผู้เชี่ยวชาญเฉพาะโรค

ผลประโยชน์การประกันภัยอุบัติเหตุส่วนบุคคล กรณีเสียชีวิต  
หรือสูญเสียอวัยวะ สายตา หรือทุพพลภาพถาวรสิ้นเชิง  
จากอุบัติเหตุ

แผน 1

3,000,000

8,000

16,000

50,000

แผน 2

6,000,000

10,000

20,000

จ่ายตามจริง\*

( สูงสุดไม่เกินวงเงินค่ารักษาต่อปี)

จ่ายตามจริง\*

( สูงสุดไม่เกินวงเงินค่ารักษาต่อปี)

50,000

แผน 3

9,000,000

12,000

24,000

50,000

รายละเอียดแผนประกัน

โรคกรรมพันธุ์หลบไป กังวล  
โรคไหน เลือกซื้อกลุ่มโรคนั้น

รับประกันโดย เมืองไทย ประกันชีวิต

สำหรับอายุ 20-59 ปี

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-5462-4617" alt="" src="/th/wp-content/uploads/2024/07/mueng\_thai\_life\_logo\_for\_cancer\_section\_3\_edit.png" class="ct-image"/\>

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-5467-4617" alt="" src="/th/wp-content/uploads/2024/07/check\_small\_list\_cancer\_section\_3.png" class="ct-image"/\>

คุ้มครองสูงสุด 1 ล้าน

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-5470-4617" alt="" src="/th/wp-content/uploads/2024/07/check\_small\_list\_cancer\_section\_3.png" class="ct-image"/\>

เลือกซื้อตามกลุ่มโรคได้ หัวใจ ตับไต สมอง มะเร็ง

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-5473-4617" alt="" src="/th/wp-content/uploads/2024/07/check\_small\_list\_cancer\_section\_3.png" class="ct-image"/\>

การันตีต่ออายุได้ถึง 69 ปี

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-5475-4617" alt="" src="/th/wp-content/uploads/2024/07/Combined\_Shape\_smile\_popup\_modal\_cancer.png" class="ct-image"/\>

เบี้ยเริ่มต้น

237 บาท/ปี

[

ซื้อเลย

](https://online.muangthai.co.th/th/calculate/heygoody/d-care/?url_hgd=JmNhbXBhaWduX2lkPUhHRzIwMjQxMDE1MTkyMjcw)

ความคุ้มครอง 6 กลุ่มโรคร้ายแรง

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-5490-4617" alt="" src="/th/wp-content/uploads/2024/07/shield\_Critical\_illness\_Modal.png" class="ct-image"/\>

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-5492-4617" alt="" src="/th/wp-content/uploads/2024/07/check-circle-solid\_critical\_illness\_modal.png" class="ct-image"/\>

กลุ่มโรคมะเร็ง

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-5495-4617" alt="" src="/th/wp-content/uploads/2024/07/check-circle-solid\_critical\_illness\_modal.png" class="ct-image"/\>

กลุ่มโรคหลอดเลือดและหัวใจ

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-5498-4617" alt="" src="/th/wp-content/uploads/2024/07/check-circle-solid\_critical\_illness\_modal.png" class="ct-image"/\>

กลุ่มโรคการเปลี่ยนอวัยวะที่สำคัญ

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-5501-4617" alt="" src="/th/wp-content/uploads/2024/07/check-circle-solid\_critical\_illness\_modal.png" class="ct-image"/\>

กลุ่มโรคระบบประสาทและกล้ามเนื้อ

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-5504-4617" alt="" src="/th/wp-content/uploads/2024/07/check-circle-solid\_critical\_illness\_modal.png" class="ct-image"/\>

กลุ่มโรคอื่นๆ

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-5507-4617" alt="" src="/th/wp-content/uploads/2024/07/check-circle-solid\_critical\_illness\_modal.png" class="ct-image"/\>

กลุ่มโรคยอดฮิต

ความคุ้มครอง

แผน 1

กลุ่มโรคมะเร็ง

แผน 2

กลุ่มโรคเกี่ยวกับ  
หลอดเลือดและ  
หัวใจ

แผน 3

กลุ่มโรคที่เกี่ยว  
ข้องกับการเปลี่ยนอวัยวะสำคัญ

แผน 4

กลุ่มโรคที่เกี่ยว  
ข้องกับระบบประ  
สาทและกล้ามเนื้อ

แผน 5

กลุ่มโรคอื่นๆ

แผน 6

กลุ่มโรคยอดฮิต

ระยะเริ่มต้นรับ

5 แสน

โรคมะเร็งระยะไม่ลุกลาม

* การใส่เครื่องกระ ตุ้นไฟฟ้าหัวใจ
* การตัดเยื่อหุ้มหัวใจ
* การใส่เครื่องกระ ตุกไฟฟ้า หัวใจ
* โรคกล้ามเนื้อหัวใจผิดปกติระยะเริ่ม ต้น
* การซ่อมแซมตก แต่งหรือผ่าตัดลิ้นหัวใจโดยผ่าตัดผ่าน ทางเส้นเลือด
* การผ่าตัดเปลี่ยน ลิ้นหัวใจ หรือซ่อม แซมเครื่องมือโดยผ่าตัดผ่านทางเส้น เลือด
* โรคหลอดเลือด แดงหัวใจที่รักษา โดยการใช้ยา
* โรคหลอดเลือด แดงใหญ่โป่งพอง แบบไม่แสดง อาการ
* การผ่าตัดหลอด เลือดแดงใหญ่แบบ ส่องกล้อง
* โรคหลอดเลือด หัวใจตีบที่รักษา ด้วยการสวนหลอดเลือดหัวใจ

* โรคหอบหืดระดับ รุนแรง
* การผ่าตัดตับ
* การผ่าตัดแก้ไข ระบบท่อน้ำดี
* การผ่าตัดไตหนึ่ง ข้าง
* ภาวะตับอ่อนอัก เสบที่กลับเป็นซ้ำ และเรื้อรัง

* โรคไข้สมองอักเสบที่สามารถรักษาได้ หายขาด
* การผ่าตัดใส่เครื่องมือระบายน้ำโพรง สมอง
* โรคพาร์กินสันระยะแรก
* การสูญเสียแขน หรือขา
* โรคปลายประสาท อักเสบ
* โรคเยื่อหุ้มสมอง และไขสันหลัง อักเสบจากเชื้อ แบคทีเรีย
* โรคเยื่อหุ้มสมอง อักเสบจากวัณโรค
* โรคโปลิโอ
* การผ่าตัดกระดูก สันหลังคดที่ไม่ ทราบสาเหตุ

* ภาวะสูญเสียการได้ยินบางส่วน
* การเจาะคอถาวร (หรือชั่วคราว)
* แผลไหม้เล็กน้อย /ปานกลาง
* การสูญเสียการดำรงชีพอย่างอิสระ (ระยะเบื้องต้น)
* ภาวะต่อมหมวกไตบกพร่องเรื้อรัง(โรคแอดดิสัน)
* การติดเชื้ออีโบล่า
* โรคไข้เลือดออก

* โรคมะเร็งระยะไม่ ลุกลาม
* โรคหลอดเลือดหัว ใจตีบที่รักษาด้วย การสวนหลอด เลือดหัวใจ
* โรคหลอดเลือด แดงหัวใจที่รักษา โดยการใช้ยา
* การผ่าตัดใส่เครื่องมือระบายน้ำโพรง สมอง
* การผ่าตัดไตหนึ่ง ข้าง

ดูข้อมูลกลุ่มโรค

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-5552-4617" alt="" src="/th/wp-content/uploads/2024/07/chevron-down\_critical\_illness.png" class="ct-image"/\>

ระยะรุนแรง

สูงสุด 1 ล้าน\*

โรคมะเร็งระยะลุกลาม

* การผ่าตัดเส้นเลือดเลี้ยงกล้ามเนื้อ หัวใจ
* กล้ามเนื้อหัวใจตายเฉียบ พลันจากการ ขาดเลือด
* การผ่าตัดลิ้นหัวใจ โดยวิธีการเปิด หัวใจ
* โรคแรงดันใน หลอดเลือดแดง ปอดสูงแบบปฐม ภูมิ
* การผ่าตัดเส้นเลือดแดงใหญ่เอออร์ต้า

* โรคหลอดลมปอดอุดกั้นเรื้อรังขั้นรุนแรง / โรคปอด ระยะสุดท้าย
* โรคไวรัสตับอักเสบขั้นรุนแรง
* ตับวาย
* โรคโลหิตจางจาก ไขกระดูก ไม่สร้าง เม็ดโลหิต
* โรคถุงน้ำในไต
* การผ่าตัดเปลี่ยน อวัยวะหรือปลูก ถ่ายไขกระดูก
* โรคตับอักเสบเรื้อ รังจากระบบภูมิ คุ้มกันต่อต้านตนเอง
* ไตวายเรื้อรัง
* ตับแข็ง

* โรคสมองเสื่อม ชนิดอัลไซเมอร์
* ภาวะอะแพลลิก
* เนื้องอกในสมอง ชนิดที่ไม่ใช่มะเร็ง
* สมองอักเสบจาก เชื้อไวรัส
* การบาดเจ็บที่ศีรษะอย่างรุนแรง
* โรคกล้ามเนื้ออ่อนแรง
* โรคเซลล์ประสาท สั่งการเสื่อม
* โรคกล้ามเนื้อลีบ
* อัมพาตของกล้าม เนื้อแขนหรือขา
* โรคอัมพาตครึ่งซีก
* การผ่าตัดสมอง
* โรคพาร์กินสัน
* โรคหลอดเลือด สมองแตกหรือ อุดตัน
* โรคก้านสมอง เสื่อม
* โรคหลอดเลือด สมองโป่งพองที่ ต้องรักษาโดยการผ่าตัด
* โรคระบบประสาท มัลติเพิลสะเคลอ โรสิส
* การฉีกขาดของ รากประสาทต้น แขน
* โรคกล้ามเนื้อเสื่อม

* ตาบอด
* การสูญเสียการได้ยิน
* การสูญเสียความ สามารถในการพูด
* โรคเอดส์/เอชไอวี จากการถ่ายเลือด หรือจากการทำ งาน
* เชื้อเอดส์/เอชไอวี ซึ่งเป็นผลมาจาก การทำร้ายร่างกายหรือการกระทำชำ เราทางเพศ
* โรคลำไส้อักเสบ เป็นแผลรุนแรงการถูกตัดขาหรือ แขน
* แผลไหม้ฉกรรจ์
* ภาวะข้ออักเสบรู มาตอยด์ชนิดรุนแรง
* โรคเนื้อเยื่อพังผืดอักเสบติดเชื้อและ เป็นเนื้อตาย
* การทุพพลภาพ ถาวรสิ้นเชิง
* โรคกล้ามเนื้ออ่อนแรงรุนแรง
* โรคเท้าช้าง
* เนื้องอกต่อมหมวกไตชนิดฟีโอโครโมไซโตมา
* โรคระยะสุดท้าย
* ไตอักเสบลูปูส จากโรคซิสเต็มมิค ลูปูส อิริเธมาโตซูส
* โรคครอยตส์เฟลดต์-จาค็อบ
* โรคหนังแข็งชนิด ลุกลาม

* โรคมะเร็งระยะลุกลาม
* การผ่าตัดเส้นเลือดเลี้ยงกล้ามเนื้อหัวใจ
* กล้ามเนื้อหัวใจตายเฉียบพลันจากการขาดเลือด
* โรคหลอดเลือด สมองแตกหรือ อุดตัน
* ไตวายเรื้อรัง
* การทุพพลภาพ ถาวรสิ้นเชิง
* โรคหลอดเลือด สมองโป่งพองที่ ต้องรักษาโดยการ ผ่าตัด

ดูข้อมูลกลุ่มโรค

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-5574-4617" alt="" src="/th/wp-content/uploads/2024/07/chevron-down\_critical\_illness.png" class="ct-image"/\>

กรณีเสียชีวิตคุ้มครอง 50,000 บาท

ความคุ้มครองแผน 1

กลุ่มโรคมะเร็ง

ระยะเริ่มต้น

ระยะรุนแรง

5 แสน

สูงสุด 1 ล้าน\*

โรคมะเร็งระยะไม่ลุกลาม

โรคมะเร็งระยะลุกลาม

กรณีเสียชีวิต คุ้มครอง 50,000 บาท

ความคุ้มครองแผน 2

กลุ่มโรคมะเร็ง

ระยะเริ่มต้น

ระยะรุนแรง

5 แสน

สูงสุด 1 ล้าน\*

* การใส่เครื่องกระ ตุ้นไฟฟ้าหัวใจ
* การตัดเยื่อหุ้มหัวใจ
* การใส่เครื่องกระ ตุกไฟฟ้า หัวใจ
* โรคกล้ามเนื้อหัวใจผิดปกติระยะเริ่ม ต้น
* การซ่อมแซมตก แต่งหรือผ่าตัดลิ้นหัวใจโดยผ่าตัดผ่าน ทางเส้นเลือด
* การผ่าตัดเปลี่ยน ลิ้นหัวใจ หรือซ่อม แซมเครื่องมือโดยผ่าตัดผ่านทางเส้น เลือด
* โรคหลอดเลือด แดงหัวใจที่รักษา โดยการใช้ยา
* โรคหลอดเลือด แดงใหญ่โป่งพอง แบบไม่แสดง อาการ
* การผ่าตัดหลอด เลือดแดงใหญ่แบบ ส่องกล้อง
* โรคหลอดเลือด หัวใจตีบที่รักษา ด้วยการสวนหลอดเลือดหัวใจ

* การผ่าตัดเส้นเลือดเลี้ยงกล้ามเนื้อ หัวใจ
* กล้ามเนื้อหัวใจตายเฉียบ พลันจากการ ขาดเลือด
* การผ่าตัดลิ้นหัวใจ โดยวิธีการเปิด หัวใจ
* โรคแรงดันใน หลอดเลือดแดง ปอดสูงแบบปฐม ภูมิ
* การผ่าตัดเส้นเลือดแดงใหญ่เอออร์ต้า

ดูข้อมูลกลุ่มโรค

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-5621-4617" alt="" src="/th/wp-content/uploads/2024/07/chevron-down\_critical\_illness.png" class="ct-image"/\>

กรณีเสียชีวิต คุ้มครอง 50,000 บาท

ความคุ้มครองแผน 3

กลุ่มโรคที่เกี่ยวข้องกับการเปลี่ยนอวัยวะสำคัญ

ระยะเริ่มต้น

ระยะรุนแรง

5 แสน

สูงสุด 1 ล้าน\*

* โรคหอบหืดระดับรุนแรง
* การผ่าตัดตับ
* การผ่าตัดแก้ไขระบบท่อน้ำด
* การผ่าตัดไตหนึ่งข้าง
* ภาวะตับอ่อนอักเสบที่กลับ เป็นซ้ำและเรื้อรัง

* โรคหลอดลมปอดอุดกั้นเรื้อรังขั้นรุนแรง / โรคปอดระยะสุดท้าย
* โรคไวรัสตับอักเสบขั้นรุน แรง
* ตับวาย
* โรคโลหิตจางจากไขกระดูก ไม่สร้างเม็ดโลหิต
* โรคถุงน้ำในไต
* การผ่าตัดเปลี่ยนอวัยวะหรือปลูกถ่ายไขกระดูก
* โรคตับอักเสบเรื้อรังจาก ระบบภูมิคุ้มกันต่อต้านตน เอง
* ไตวายเรื้อรัง
* ตับแข็ง

ดูข้อมูลกลุ่มโรค

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-5646-4617" alt="" src="/th/wp-content/uploads/2024/07/chevron-down\_critical\_illness.png" class="ct-image"/\>

กรณีเสียชีวิต คุ้มครอง 50,000 บาท

ความคุ้มครองแผน 4

กลุ่มโรคที่เกี่ยวข้องกับระบบประสาทและกล้ามเนื้อ

ระยะเริ่มต้น

ระยะรุนแรง

5 แสน

สูงสุด 1 ล้าน\*

* โรคไข้สมองอักเสบที่สามารถรักษาได้ หายขาด
* การผ่าตัดใส่เครื่องมือระบายน้ำโพรง สมอง
* โรคพาร์กินสันระยะแรก
* การสูญเสียแขน หรือขา
* โรคปลายประสาท อักเสบ
* โรคเยื่อหุ้มสมอง และไขสันหลัง อักเสบจากเชื้อ แบคทีเรีย
* โรคเยื่อหุ้มสมอง อักเสบจากวัณโรค
* โรคโปลิโอ
* การผ่าตัดกระดูก สันหลังคดที่ไม่ ทราบสาเหตุ

* โรคสมองเสื่อม ชนิดอัลไซเมอร์
* ภาวะอะแพลลิก
* เนื้องอกในสมอง ชนิดที่ไม่ใช่มะเร็ง
* สมองอักเสบจาก เชื้อไวรัส
* การบาดเจ็บที่ศีรษะอย่างรุนแรง
* โรคกล้ามเนื้ออ่อนแรง
* โรคเซลล์ประสาท สั่งการเสื่อม
* โรคกล้ามเนื้อลีบ
* อัมพาตของกล้าม เนื้อแขนหรือขา
* โรคอัมพาตครึ่งซีก
* การผ่าตัดสมอง
* โรคพาร์กินสัน
* โรคหลอดเลือด สมองแตกหรือ อุดตัน
* โรคก้านสมอง เสื่อม
* โรคหลอดเลือด สมองโป่งพองที่ ต้องรักษาโดยการผ่าตัด
* โรคระบบประสาท มัลติเพิลสะเคลอ โรสิส
* การฉีกขาดของ รากประสาทต้น แขน
* โรคกล้ามเนื้อเสื่อม

ดูข้อมูลกลุ่มโรค

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-5671-4617" alt="" src="/th/wp-content/uploads/2024/07/chevron-down\_critical\_illness.png" class="ct-image"/\>

กรณีเสียชีวิต คุ้มครอง 50,000 บาท

ความคุ้มครองแผน 5

กลุ่มโรคอื่นๆ

ระยะเริ่มต้น

ระยะรุนแรง

5 แสน

สูงสุด 1 ล้าน\*

* ภาวะสูญเสียการได้ยินบางส่วน
* การเจาะคอถาวร (หรือชั่วคราว)
* แผลไหม้เล็กน้อย /ปานกลาง
* การสูญเสียการดำรงชีพอย่างอิสระ (ระยะเบื้องต้น)
* ภาวะต่อมหมวกไตบกพร่องเรื้อรัง(โรคแอดดิสัน)
* การติดเชื้ออีโบล่า
* โรคไข้เลือดออก

* ตาบอด
* การสูญเสียการได้ยิน
* การสูญเสียความ สามารถในการพูด
* โรคเอดส์/เอชไอวี จากการถ่ายเลือด หรือจากการทำ งาน
* เชื้อเอดส์/เอชไอวี ซึ่งเป็นผลมาจาก การทำร้ายร่างกายหรือการกระทำชำ เราทางเพศ
* โรคลำไส้อักเสบ เป็นแผลรุนแรงการถูกตัดขาหรือ แขน
* แผลไหม้ฉกรรจ์
* ภาวะข้ออักเสบรู มาตอยด์ชนิดรุนแรง
* โรคเนื้อเยื่อพังผืดอักเสบติดเชื้อและ เป็นเนื้อตาย
* การทุพพลภาพ ถาวรสิ้นเชิง
* โรคกล้ามเนื้ออ่อนแรงรุนแรง
* โรคเท้าช้าง
* เนื้องอกต่อมหมวกไตชนิดฟีโอโครโมไซโตมา
* โรคระยะสุดท้าย
* ไตอักเสบลูปูส จากโรคซิสเต็มมิค ลูปูส อิริเธมาโตซูส
* โรคครอยตส์เฟลดต์-จาค็อบ
* โรคหนังแข็งชนิด ลุกลาม

ดูข้อมูลกลุ่มโรค

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-5696-4617" alt="" src="/th/wp-content/uploads/2024/07/chevron-down\_critical\_illness.png" class="ct-image"/\>

กรณีเสียชีวิต คุ้มครอง 50,000 บาท

ความคุ้มครองแผน 6

กลุ่มโรคยอดฮิต

ระยะเริ่มต้น

ระยะรุนแรง

5 แสน

สูงสุด 1 ล้าน\*

* โรคมะเร็งระยะไม่ ลุกลาม
* โรคหลอดเลือดหัว ใจตีบที่รักษาด้วย การสวนหลอด เลือดหัวใจ
* โรคหลอดเลือด แดงหัวใจที่รักษา โดยการใช้ยา
* การผ่าตัดใส่เครื่องมือระบายน้ำโพรง สมอง
* การผ่าตัดไตหนึ่ง ข้าง

* โรคมะเร็งระยะลุกลาม
* การผ่าตัดเส้นเลือดเลี้ยงกล้ามเนื้อหัวใจ
* กล้ามเนื้อหัวใจตายเฉียบพลันจากการขาดเลือด
* โรคหลอดเลือด สมองแตกหรือ อุดตัน
* ไตวายเรื้อรัง
* การทุพพลภาพ ถาวรสิ้นเชิง
* โรคหลอดเลือด สมองโป่งพองที่ ต้องรักษาโดยการ ผ่าตัด

ดูข้อมูลกลุ่มโรค

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-5721-4617" alt="" src="/th/wp-content/uploads/2024/07/chevron-down\_critical\_illness.png" class="ct-image"/\>

กรณีเสียชีวิต คุ้มครอง 50,000 บาท

รายละเอียดแผนประกัน

ครอบคลุมทั้งมะเร็งทุกระยะ และโรคร้ายที่พบบ่อย

รับประกันโดย เมืองไทยประกันภัย

สำหรับอายุ 16-60 ปี

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-5739-4617" alt="" src="/th/wp-content/uploads/2024/07/mueng\_thai\_logo\_for\_cancer\_section\_3\_edit.png" class="ct-image"/\>

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-5744-4617" alt="" src="/th/wp-content/uploads/2024/07/check\_small\_list\_cancer\_section\_3.png" class="ct-image"/\>

รับเงินก้อนสูงสุด 5 แสน

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-5747-4617" alt="" src="/th/wp-content/uploads/2024/07/check\_small\_list\_cancer\_section\_3.png" class="ct-image"/\>

คุ้มครองครบทั้งมะเร็ง หัวใจ สมอง ปอด

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-5750-4617" alt="" src="/th/wp-content/uploads/2024/07/check\_small\_list\_cancer\_section\_3.png" class="ct-image"/\>

การันตีต่ออายุได้ถึง 65 ปี

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-5752-4617" alt="" src="/th/wp-content/uploads/2024/07/Combined\_Shape\_smile\_popup\_modal\_cancer.png" class="ct-image"/\>

เบี้ยเริ่มต้น

500บาท/ปี

[

ซื้อเลย

](https://mtiecommerce.muangthaiinsurance.com/product/Portal/?enckey=8BybA82f&url_hgd=JmNhbXBhaWduX2lkPUhHRzIwMjQxMDE1MTkyMjcw)

คุ้มครอง 4 โรคร้าย

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-5767-4617" alt="" src="/th/wp-content/uploads/2024/07/shield\_Critical\_illness\_Modal.png" class="ct-image"/\>

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-5769-4617" alt="" src="/th/wp-content/uploads/2024/07/check-circle-solid\_critical\_illness\_modal.png" class="ct-image"/\>

โรคมะเร็งทุกระยะ (ไม่รวมโรคมะเร็งผิวหนัง ยกเว้นมะเร็วผิวหนังประเภท "เมลลาโนมา")

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-5772-4617" alt="" src="/th/wp-content/uploads/2024/07/check-circle-solid\_critical\_illness\_modal.png" class="ct-image"/\>

โรคกล้ามเนื้อหัวใจตายเฉียบพลัน

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-5775-4617" alt="" src="/th/wp-content/uploads/2024/07/check-circle-solid\_critical\_illness\_modal.png" class="ct-image"/\>

โรคหลอดเลือดสมองแตกหรืออุดตัน

![](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%200'%3E%3C/svg%3E)\<img id="image-5778-4617" alt="" src="/th/wp-content/uploads/2024/07/check-circle-solid\_critical\_illness\_modal.png" class="ct-image"/\>

โรคปอดระยะสุดท้าย

รับเงินก้อน เมื่อตรวจเจอโรคใดโรคหนึ่งเป็นครั้งแรก

แผน 1

100,000

แผน 2

200,000

แผน 3

500,000

เลขที่ใบอนุญาตเสนอขายประกันภัยผ่านช่องทางอิเล็กทรอนิกส์

<img src="data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%20752%20532'%3E%3C/svg%3E" alt="เลขที่ใบอนุญาตเสนอขายประกันภัยผ่านช่องทางอิเล็กทรอนิกส์" class="img-fluid" oncontextmenu="return false;" width="752" height="532" data-lazy-src="/heygoody/media/source/license/image-footer-heygoody-broker-license-online.jpg">\<img src="/heygoody/media/source/license/image-footer-heygoody-broker-license-online.jpg" alt="เลขที่ใบอนุญาตเสนอขายประกันภัยผ่านช่องทางอิเล็กทรอนิกส์" class="img-fluid" onContextMenu="return false;" width="752" height="532" /\>

window.addEventListener('DOMContentLoaded', function() { function modalLicenseOnline() { $("#BrokerLicenseOnline").modal("show"); }
});

เลขที่ใบอนุญาตประกันวินาศภัย

<img src="data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%20752%20532'%3E%3C/svg%3E" alt="เลขที่ใบอนุญาตประกันวินาศภัย" class="img-fluid" oncontextmenu="return false;" width="752" height="532" data-lazy-src="/heygoody/media/source/license/image-footer-heygoody-broker-license.jpg">\<img src="/heygoody/media/source/license/image-footer-heygoody-broker-license.jpg" alt="เลขที่ใบอนุญาตประกันวินาศภัย" class="img-fluid" onContextMenu="return false;" width="752" height="532" /\>

window.addEventListener('DOMContentLoaded', function() { function modalLicense() { $("#BrokerLicense").modal("show"); }
});

16 รางวัล

การันตีความสำเร็จจากเวทีระดับโลก [ดูรางวัลทั้งหมด](/th/about-us/awards-and-recognition)@media (min-width: 1200px){ .award-container { max-width: calc(100% - 20px); margin-right: 0; }
.award-description { max-width: 160px; margin: auto;
}
}
@media (min-width: 1400px) { .award-container { max-width: calc(100% - 120px); }
} @media (min-width: 2560px) { .award-container { max-width: calc(100% - 240px); }
}

<img src="data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%20114%200'%3E%3C/svg%3E" width="114" data-lazy-src="/content/images/home-new/awards/trophy.png">\<img src="/content/images/home-new/awards/trophy.png" width="114"\>

<img src="data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%2048'%3E%3C/svg%3E" alt="Insure Tech Connect Asia" height="48" class="swp-img" data-lazy-src="/getattachment/b614ccd8-ce25-4721-be2f-75dbc480f8f6/itc.png">\<img src="/getattachment/b614ccd8-ce25-4721-be2f-75dbc480f8f6/itc.png" alt="Insure Tech Connect Asia" height="48" class="swp-img" /\>

Insure Tech Connect Asia

Brokerage Breakthrough · Data Analytics Master Awards - 2024

<img src="data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%2048'%3E%3C/svg%3E" alt="Global Retail Banking Innovation" height="48" class="swp-img" data-lazy-src="/getattachment/70102a11-9c7e-49f1-97c2-4c95bb54a1df/tdb.png">\<img src="/getattachment/70102a11-9c7e-49f1-97c2-4c95bb54a1df/tdb.png" alt="Global Retail Banking Innovation" height="48" class="swp-img" /\>

Global Retail Banking Innovation

Best Customer Centric Business Model - 2024

<img src="data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%2048'%3E%3C/svg%3E" alt="New York Festivals Awards 2024" height="48" class="swp-img" data-lazy-src="/getattachment/f245e48c-0afd-40d6-a0bd-6872cdf6a641/nyf.webp">\<img src="/getattachment/f245e48c-0afd-40d6-a0bd-6872cdf6a641/nyf.webp" alt="New York Festivals Awards 2024" height="48" class="swp-img" /\>

New York Festivals Awards 2024

Best Customer Centric Business Model - 2024

<img src="data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%2048'%3E%3C/svg%3E" alt="The Work 2024" height="48" class="swp-img" data-lazy-src="/getattachment/e936879e-55c8-4a59-91f5-580a2888867b/thework.png">\<img src="/getattachment/e936879e-55c8-4a59-91f5-580a2888867b/thework.png" alt="The Work 2024" height="48" class="swp-img" /\>

The Work 2024

Film/TV Craft · Film/Web Film · Culture · Work for Good · Branded Content+Entertainment - 2024

<img src="data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%2048'%3E%3C/svg%3E" alt="Thailand Influencer Awards 2024 by Tellscore" height="48" class="swp-img" data-lazy-src="/getattachment/b8250f6e-ef4f-4af4-af66-888a1b4a2d30/influencer.png">\<img src="/getattachment/b8250f6e-ef4f-4af4-af66-888a1b4a2d30/influencer.png" alt="Thailand Influencer Awards 2024 by Tellscore" height="48" class="swp-img" /\>

Thailand Influencer Awards 2024 by Tellscore

Best Financial & Investment Influencer Campaign - 2024

<img src="data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%2048'%3E%3C/svg%3E" alt="AdPeople Awards &amp; Symposium 2024" height="48" class="swp-img" data-lazy-src="/getattachment/2fd0fc28-55d6-4142-b245-13cd584c7b05/ad-people-v1.png">\<img src="/getattachment/2fd0fc28-55d6-4142-b245-13cd584c7b05/ad-people-v1.png" alt="AdPeople Awards &amp; Symposium 2024" height="48" class="swp-img" /\>

AdPeople Awards & Symposium 2024

•Silver หมวดหมู่ Craft  
•Bronze หมวดหมู่ Craft  
•Bronze หมวดหมู่ Film

<img src="data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%200%2048'%3E%3C/svg%3E" alt="Marketing Award of Thailand 2024" height="48" class="swp-img" data-lazy-src="/getattachment/45e2ce0f-e6cc-4e97-874e-53f55cb9120f/mat.png">\<img src="/getattachment/45e2ce0f-e6cc-4e97-874e-53f55cb9120f/mat.png" alt="Marketing Award of Thailand 2024" height="48" class="swp-img" /\>

Marketing Award of Thailand 2024

Silver -Brand Experience & Communication

<img src="data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%20147%2046'%3E%3C/svg%3E" alt="heygoody" width="147" height="46" data-lazy-src="/heygoody/media/footersection/color-white.svg?ext=.svg">\<img src="/heygoody/media/footersection/color-white.svg?ext=.svg" alt="heygoody" width="147" height="46" /\>

ช่องทางติดต่อ heygoody

[<img class="fn-icon-circle fn-line" src="/heygoody/media/footersection/image-social-white-line.svg?ext=.svg" title="line" width="36" height="36"> ](/th/addfriend/?campaign=linefriend) [<img class="fn-icon-circle fn-fb" src="/heygoody/media/footersection/image-social-white-facebook.svg?ext=.svg" title="facebook" width="36" height="36"> ](https://www.facebook.com/heygoodyTH)

[เลขที่ใบอนุญาตประกันวินาศภัย ว00015/2556]() [เลขที่ใบอนุญาตเสนอขายประกันภัยผ่านช่องทาง  
อิเล็กทรอนิกส์   
อลว 015521000/2563]()   
บริษัท เงินติดล้อ จำกัด (มหาชน)

<img class="oic" src="data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%2083%2036'%3E%3C/svg%3E" alt="oic" width="83" height="36" data-lazy-src="/heygoody/media/footersection/image-logo-oic.svg?ext=.svg">\<img class="oic" src="/heygoody/media/footersection/image-logo-oic.svg?ext=.svg" alt="oic" width="83" height="36" /\> <img class="dbd" src="data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%2083%2036'%3E%3C/svg%3E" alt="dbd" width="83" height="36" data-lazy-src="/heygoody/media/footersection/image-logo-dbd.svg?ext=.svg">\<img class="dbd" src="/heygoody/media/footersection/image-logo-dbd.svg?ext=.svg" alt="dbd" width="83" height="36" /\>

ประกันรถยนต์

[ ประกันรถยนต์ทั้งหมด ](/th/autoinsurance/all/)

[ ประกันรถยนต์ไฟฟ้า EV ](/th/autoinsurance/evcar/)

[ ประกันรถตู้ส่วนบุคคล ](/checkinsurance/?cartype=210)

[ ประกันรถยนต์ชั้น 1 ](/th/autoinsurance/class1/)

[ ประกันรถยนต์ชั้น 2+, 2 ](/th/autoinsurance/class2plus-2/)

[ ประกันรถยนต์ชั้น 3+, 3 ](/th/autoinsurance/class3plus-3/)

ประกันเดินทาง

[ ประกันเดินทางต่างประเทศ ](/th/travelinsurance/)

ประกันสุขภาพ

[ ประกันโรคร้ายแรง ](/th/critical-illness/)

[ ประกันโรคมะเร็ง ](/th/cancer/)

ประกันชีวิต (ลดหย่อนภาษี)

[ คำนวณและเปรียบเทียบประกันลดหย่อนภาษี ](/th/tax-deduction/)

[ ประกันสะสมทรัพย์ ](/th/tax-deduction/savings-insurance/)

[ ประกันบำนาญ ](/th/tax-deduction/annuity-insurance/)

ประกันอื่นๆ

[ ประกันบ้าน/คอนโด ](/th/homeinsurance/)

เกี่ยวกับเรา

[ รู้จักเรา ](/th/about-us/who-we-are/)

[ ทำไมต้อง heygoody? ](/th/about-us/)

[ รางวัลความสำเร็จ ](/th/about-us/awards-and-recognition)

ศูนย์ช่วยเหลือ

[ ความช่วยเหลือทั้งหมด ](/th/support-info/)

[ การใช้งานโปรโมชั่น ](/th/support-info/how-to-promotion/)

[ ค้นหาอู่ซ่อม ](/th/support-info/gogogarage/)

[ คำถามที่พบบ่อย ](/th/faq/)

อื่นๆ

[ โปรโมชั่น & กิจกรรม ](/th/promotion/)

[ บทความ ](/th/blogs/)

[ ข่าวสาร ](/th/news/)

[ ติดต่อเรา ](/th/contact-us/)

 © 2568 บริษัท เงินติดล้อ จำกัด (มหาชน)

[นโยบายความเป็นส่วนตัว](/privacypolicy) | [นโยบายส่วนบุคคลเกี่ยวกับคุกกี้](/cookiepolicy)

 (function(h,o,t,j,a,r){ h.hj=h.hj||function(){(h.hj.q=h.hj.q||[]).push(arguments)}; h.\_hjSettings={hjid:3132820,hjsv:6}; a=o.getElementsByTagName('head')[0]; r=o.createElement('script');r.async=1; r.src=t+h.\_hjSettings.hjid+j+h.\_hjSettings.hjsv; a.appendChild(r); })(window,document,'https://static.hotjar.com/c/hotjar-','.js?sv=');chevron-down  setTimeout(() =\> { ;window.NREUM||(NREUM={});NREUM.init={distributed\_tracing:{enabled:true},privacy:{cookies\_enabled:true},ajax:{deny\_list:["bam.nr-data.net"]}}; ;NREUM.loader\_config={accountID:"6392919",trustKey:"6392919",agentID:"1134553161",licenseKey:"NRJS-450b5396455f75c0f1a",applicationID:"1002484837"};
;NREUM.info={beacon:"bam.nr-data.net",errorBeacon:"bam.nr-data.net",licenseKey:"NRJS-450b5396455f75c0f1a",applicationID:"1002484837",sa:1};
;/\*! For license information please see nr-loader-spa-1.283.2.min.js.LICENSE.txt \*/
(()=\>{var e,t,r={8122:(e,t,r)=\>{"use strict";r.d(t,{a:()=\>i});var n=r(944);function i(e,t){try{if(!e||"object"!=typeof e)return(0,n.R)(3);if(!t||"object"!=typeof t)return(0,n.R)(4);const r=Object.create(Object.getPrototypeOf(t),Object.getOwnPropertyDescriptors(t)),o=0===Object.keys(r).length?e:r;for(let a in o)if(void 0!==e[a])try{if(null===e[a]){r[a]=null;continue}Array.isArray(e[a])&&Array.isArray(t[a])?r[a]=Array.from(new Set([...e[a],...t[a]])):"object"==typeof e[a]&&"object"==typeof t[a]?r[a]=i(e[a],t[a]):r[a]=e[a]}catch(e){(0,n.R)(1,e)}return r}catch(e){(0,n.R)(2,e)}}},2555:(e,t,r)=\>{"use strict";r.d(t,{Vp:()=\>c,fn:()=\>s,x1:()=\>u});var n=r(384),i=r(8122);const o={beacon:n.NT.beacon,errorBeacon:n.NT.errorBeacon,licenseKey:void 0,applicationID:void 0,sa:void 0,queueTime:void 0,applicationTime:void 0,ttGuid:void 0,user:void 0,account:void 0,product:void 0,extra:void 0,jsAttributes:{},userAttributes:void 0,atts:void 0,transactionName:void 0,tNamePlain:void 0},a={};function s(e){try{const t=c(e);return!!t.licenseKey&&!!t.errorBeacon&&!!t.applicationID}catch(e){return!1}}function c(e){if(!e)throw new Error("All info objects require an agent identifier!");if(!a[e])throw new Error("Info for ".concat(e," was never set"));return a[e]}function u(e,t){if(!e)throw new Error("All info objects require an agent identifier!");a[e]=(0,i.a)(t,o);const r=(0,n.nY)(e);r&&(r.info=a[e])}},9417:(e,t,r)=\>{"use strict";r.d(t,{D0:()=\>h,gD:()=\>g,xN:()=\>p});var n=r(3333);const i=e=\>{if(!e||"string"!=typeof e)return!1;try{document.createDocumentFragment().querySelector(e)}catch{return!1}return!0};var o=r(2614),a=r(944),s=r(384),c=r(8122);const u="[data-nr-mask]",d=()=\>{const e={feature\_flags:[],experimental:{marks:!1,measures:!1,resources:!1},mask\_selector:"\*",block\_selector:"[data-nr-block]",mask\_input\_options:{color:!1,date:!1,"datetime-local":!1,email:!1,month:!1,number:!1,range:!1,search:!1,tel:!1,text:!1,time:!1,url:!1,week:!1,textarea:!1,select:!1,password:!0}};return{ajax:{deny\_list:void 0,block\_internal:!0,enabled:!0,autoStart:!0},distributed\_tracing:{enabled:void 0,exclude\_newrelic\_header:void 0,cors\_use\_newrelic\_header:void 0,cors\_use\_tracecontext\_headers:void 0,allowed\_origins:void 0},get feature\_flags(){return e.feature\_flags},set feature\_flags(t){e.feature\_flags=t},generic\_events:{enabled:!0,autoStart:!0},harvest:{interval:30},jserrors:{enabled:!0,autoStart:!0},logging:{enabled:!0,autoStart:!0},metrics:{enabled:!0,autoStart:!0},obfuscate:void 0,page\_action:{enabled:!0},page\_view\_event:{enabled:!0,autoStart:!0},page\_view\_timing:{enabled:!0,autoStart:!0},performance:{get capture\_marks(){return e.feature\_flags.includes(n.$v.MARKS)||e.experimental.marks},set capture\_marks(t){e.experimental.marks=t},get capture\_measures(){return e.feature\_flags.includes(n.$v.MEASURES)||e.experimental.measures},set capture\_measures(t){e.experimental.measures=t},capture\_detail:!0,resources:{get enabled(){return e.feature\_flags.includes(n.$v.RESOURCES)||e.experimental.resources},set enabled(t){e.experimental.resources=t},asset\_types:[],first\_party\_domains:[],ignore\_newrelic:!0}},privacy:{cookies\_enabled:!0},proxy:{assets:void 0,beacon:void 0},session:{expiresMs:o.wk,inactiveMs:o.BB},session\_replay:{autoStart:!0,enabled:!1,preload:!1,sampling\_rate:10,error\_sampling\_rate:100,collect\_fonts:!1,inline\_images:!1,fix\_stylesheets:!0,mask\_all\_inputs:!0,get mask\_text\_selector(){return e.mask\_selector},set mask\_text\_selector(t){i(t)?e.mask\_selector="".concat(t,",").concat(u):""===t||null===t?e.mask\_selector=u:(0,a.R)(5,t)},get block\_class(){return"nr-block"},get ignore\_class(){return"nr-ignore"},get mask\_text\_class(){return"nr-mask"},get block\_selector(){return e.block\_selector},set block\_selector(t){i(t)?e.block\_selector+=",".concat(t):""!==t&&(0,a.R)(6,t)},get mask\_input\_options(){return e.mask\_input\_options},set mask\_input\_options(t){t&&"object"==typeof t?e.mask\_input\_options={...t,password:!0}:(0,a.R)(7,t)}},session\_trace:{enabled:!0,autoStart:!0},soft\_navigations:{enabled:!0,autoStart:!0},spa:{enabled:!0,autoStart:!0},ssl:void 0,user\_actions:{enabled:!0,elementAttributes:["id","className","tagName","type"]}}},l={},f="All configuration objects require an agent identifier!";function h(e){if(!e)throw new Error(f);if(!l[e])throw new Error("Configuration for ".concat(e," was never set"));return l[e]}function p(e,t){if(!e)throw new Error(f);l[e]=(0,c.a)(t,d());const r=(0,s.nY)(e);r&&(r.init=l[e])}function g(e,t){if(!e)throw new Error(f);var r=h(e);if(r){for(var n=t.split("."),i=0;i\<n.length-1;i++)if("object"!=typeof(r=r[n[i]]))return;r=r[n[n.length-1]]}return r}},5603:(e,t,r)=\>{"use strict";r.d(t,{a:()=\>c,o:()=\>s});var n=r(384),i=r(8122);const o={accountID:void 0,trustKey:void 0,agentID:void 0,licenseKey:void 0,applicationID:void 0,xpid:void 0},a={};function s(e){if(!e)throw new Error("All loader-config objects require an agent identifier!");if(!a[e])throw new Error("LoaderConfig for ".concat(e," was never set"));return a[e]}function c(e,t){if(!e)throw new Error("All loader-config objects require an agent identifier!");a[e]=(0,i.a)(t,o);const r=(0,n.nY)(e);r&&(r.loader\_config=a[e])}},3371:(e,t,r)=\>{"use strict";r.d(t,{V:()=\>f,f:()=\>l});var n=r(8122),i=r(384),o=r(6154),a=r(9324);let s=0;const c={buildEnv:a.F3,distMethod:a.Xs,version:a.xv,originTime:o.WN},u={customTransaction:void 0,disabled:!1,isolatedBacklog:!1,loaderType:void 0,maxBytes:3e4,onerror:void 0,ptid:void 0,releaseIds:{},appMetadata:{},session:void 0,denyList:void 0,timeKeeper:void 0,obfuscator:void 0,harvester:void 0},d={};function l(e){if(!e)throw new Error("All runtime objects require an agent identifier!");if(!d[e])throw new Error("Runtime for ".concat(e," was never set"));return d[e]}function f(e,t){if(!e)throw new Error("All runtime objects require an agent identifier!");d[e]={...(0,n.a)(t,u),...c},Object.hasOwnProperty.call(d[e],"harvestCount")||Object.defineProperty(d[e],"harvestCount",{get:()=\>++s});const r=(0,i.nY)(e);r&&(r.runtime=d[e])}},9324:(e,t,r)=\>{"use strict";r.d(t,{F3:()=\>i,Xs:()=\>o,Yq:()=\>a,xv:()=\>n});const n="1.283.2",i="PROD",o="CDN",a="^2.0.0-alpha.17"},6154:(e,t,r)=\>{"use strict";r.d(t,{A4:()=\>s,OF:()=\>d,RI:()=\>i,WN:()=\>h,bv:()=\>o,gm:()=\>a,lR:()=\>f,m:()=\>u,mw:()=\>c,sb:()=\>l});var n=r(1863);const i="undefined"!=typeof window&&!!window.document,o="undefined"!=typeof WorkerGlobalScope&&("undefined"!=typeof self&&self instanceof WorkerGlobalScope&&self.navigator instanceof WorkerNavigator||"undefined"!=typeof globalThis&&globalThis instanceof WorkerGlobalScope&&globalThis.navigator instanceof WorkerNavigator),a=i?window:"undefined"!=typeof WorkerGlobalScope&&("undefined"!=typeof self&&self instanceof WorkerGlobalScope&&self||"undefined"!=typeof globalThis&&globalThis instanceof WorkerGlobalScope&&globalThis),s="complete"===a?.document?.readyState,c=Boolean("hidden"===a?.document?.visibilityState),u=""+a?.location,d=/iPad|iPhone|iPod/.test(a.navigator?.userAgent),l=d&&"undefined"==typeof SharedWorker,f=(()=\>{const e=a.navigator?.userAgent?.match(/Firefox[/\\s](\\d+\\.\\d+)/);return Array.isArray(e)&&e.length\>=2?+e[1]:0})(),h=Date.now()-(0,n.t)()},7295:(e,t,r)=\>{"use strict";r.d(t,{Xv:()=\>a,gX:()=\>i,iW:()=\>o});var n=[];function i(e){if(!e||o(e))return!1;if(0===n.length)return!0;for(var t=0;t\<n.length;t++){var r=n[t];if("\*"===r.hostname)return!1;if(s(r.hostname,e.hostname)&&c(r.pathname,e.pathname))return!1}return!0}function o(e){return void 0===e.hostname}function a(e){if(n=[],e&&e.length)for(var t=0;t\<e.length;t++){let r=e[t];if(!r)continue;0===r.indexOf("http://")?r=r.substring(7):0===r.indexOf("https://")&&(r=r.substring(8));const i=r.indexOf("/");let o,a;i\>0?(o=r.substring(0,i),a=r.substring(i)):(o=r,a="");let[s]=o.split(":");n.push({hostname:s,pathname:a})}}function s(e,t){return!(e.length\>t.length)&&t.indexOf(e)===t.length-e.length}function c(e,t){return 0===e.indexOf("/")&&(e=e.substring(1)),0===t.indexOf("/")&&(t=t.substring(1)),""===e||e===t}},1687:(e,t,r)=\>{"use strict";r.d(t,{Ak:()=\>c,Ze:()=\>l,x3:()=\>u});var n=r(7836),i=r(3606),o=r(860),a=r(2646);const s={};function c(e,t){const r={staged:!1,priority:o.P3[t]||0};d(e),s[e].get(t)||s[e].set(t,r)}function u(e,t){e&&s[e]&&(s[e].get(t)&&s[e].delete(t),h(e,t,!1),s[e].size&&f(e))}function d(e){if(!e)throw new Error("agentIdentifier required");s[e]||(s[e]=new Map)}function l(e="",t="feature",r=!1){if(d(e),!e||!s[e].get(t)||r)return h(e,t);s[e].get(t).staged=!0,f(e)}function f(e){const t=Array.from(s[e]);t.every((([e,t])=\>t.staged))&&(t.sort(((e,t)=\>e[1].priority-t[1].priority)),t.forEach((([t])=\>{s[e].delete(t),h(e,t)})))}function h(e,t,r=!0){const o=e?n.ee.get(e):n.ee,s=i.i.handlers;if(!o.aborted&&o.backlog&&s){if(r){const e=o.backlog[t],r=s[t];if(r){for(let t=0;e&&t\<e.length;++t)p(e[t],r);Object.entries(r).forEach((([e,t])=\>{Object.values(t||{}).forEach((t=\>{t[0]?.on&&t[0]?.context()instanceof a.y&&t[0].on(e,t[1])}))}))}}o.isolatedBacklog||delete s[t],o.backlog[t]=null,o.emit("drain-"+t,[])}}function p(e,t){var r=e[1];Object.values(t[r]||{}).forEach((t=\>{var r=e[0];if(t[0]===r){var n=t[1],i=e[3],o=e[2];n.apply(i,o)}}))}},7836:(e,t,r)=\>{"use strict";r.d(t,{P:()=\>c,ee:()=\>u});var n=r(384),i=r(8990),o=r(3371),a=r(2646),s=r(5607);const c="nr@context:".concat(s.W),u=function e(t,r){var n={},s={},d={},l=!1;try{l=16===r.length&&(0,o.f)(r).isolatedBacklog}catch(e){}var f={on:p,addEventListener:p,removeEventListener:function(e,t){var r=n[e];if(!r)return;for(var i=0;i\<r.length;i++)r[i]===t&&r.splice(i,1)},emit:function(e,r,n,i,o){!1!==o&&(o=!0);if(u.aborted&&!i)return;t&&o&&t.emit(e,r,n);for(var a=h(n),c=g(e),d=c.length,l=0;l\<d;l++)c[l].apply(a,r);var p=v()[s[e]];p&&p.push([f,e,r,a]);return a},get:m,listeners:g,context:h,buffer:function(e,t){const r=v();if(t=t||"feature",f.aborted)return;Object.entries(e||{}).forEach((([e,n])=\>{s[n]=t,t in r||(r[t]=[])}))},abort:function(){f.\_aborted=!0,Object.keys(f.backlog).forEach((e=\>{delete f.backlog[e]}))},isBuffering:function(e){return!!v()[s[e]]},debugId:r,backlog:l?{}:t&&"object"==typeof t.backlog?t.backlog:{},isolatedBacklog:l};return Object.defineProperty(f,"aborted",{get:()=\>{let e=f.\_aborted||!1;return e||(t&&(e=t.aborted),e)}}),f;function h(e){return e&&e instanceof a.y?e:e?(0,i.I)(e,c,(()=\>new a.y(c))):new a.y(c)}function p(e,t){n[e]=g(e).concat(t)}function g(e){return n[e]||[]}function m(t){return d[t]=d[t]||e(f,t)}function v(){return f.backlog}}(void 0,"globalEE"),d=(0,n.Zm)();d.ee||(d.ee=u)},2646:(e,t,r)=\>{"use strict";r.d(t,{y:()=\>n});class n{constructor(e){this.contextId=e}}},9908:(e,t,r)=\>{"use strict";r.d(t,{d:()=\>n,p:()=\>i});var n=r(7836).ee.get("handle");function i(e,t,r,i,o){o?(o.buffer([e],i),o.emit(e,t,r)):(n.buffer([e],i),n.emit(e,t,r))}},3606:(e,t,r)=\>{"use strict";r.d(t,{i:()=\>o});var n=r(9908);o.on=a;var i=o.handlers={};function o(e,t,r,o){a(o||n.d,i,e,t,r)}function a(e,t,r,i,o){o||(o="feature"),e||(e=n.d);var a=t[o]=t[o]||{};(a[r]=a[r]||[]).push([e,i])}},3878:(e,t,r)=\>{"use strict";function n(e,t){return{capture:e,passive:!1,signal:t}}function i(e,t,r=!1,i){window.addEventListener(e,t,n(r,i))}function o(e,t,r=!1,i){document.addEventListener(e,t,n(r,i))}r.d(t,{DD:()=\>o,jT:()=\>n,sp:()=\>i})},5607:(e,t,r)=\>{"use strict";r.d(t,{W:()=\>n});const n=(0,r(9566).bz)()},9566:(e,t,r)=\>{"use strict";r.d(t,{LA:()=\>s,ZF:()=\>c,bz:()=\>a,el:()=\>u});var n=r(6154);const i="xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx";function o(e,t){return e?15&e[t]:16\*Math.random()|0}function a(){const e=n.gm?.crypto||n.gm?.msCrypto;let t,r=0;return e&&e.getRandomValues&&(t=e.getRandomValues(new Uint8Array(30))),i.split("").map((e=\>"x"===e?o(t,r++).toString(16):"y"===e?(3&o()|8).toString(16):e)).join("")}function s(e){const t=n.gm?.crypto||n.gm?.msCrypto;let r,i=0;t&&t.getRandomValues&&(r=t.getRandomValues(new Uint8Array(e)));const a=[];for(var s=0;s\<e;s++)a.push(o(r,i++).toString(16));return a.join("")}function c(){return s(16)}function u(){return s(32)}},2614:(e,t,r)=\>{"use strict";r.d(t,{BB:()=\>a,H3:()=\>n,g:()=\>u,iL:()=\>c,tS:()=\>s,uh:()=\>i,wk:()=\>o});const n="NRBA",i="SESSION",o=144e5,a=18e5,s={STARTED:"session-started",PAUSE:"session-pause",RESET:"session-reset",RESUME:"session-resume",UPDATE:"session-update"},c={SAME\_TAB:"same-tab",CROSS\_TAB:"cross-tab"},u={OFF:0,FULL:1,ERROR:2}},1863:(e,t,r)=\>{"use strict";function n(){return Math.floor(performance.now())}r.d(t,{t:()=\>n})},7485:(e,t,r)=\>{"use strict";r.d(t,{D:()=\>i});var n=r(6154);function i(e){if(0===(e||"").indexOf("data:"))return{protocol:"data"};try{const t=new URL(e,location.href),r={port:t.port,hostname:t.hostname,pathname:t.pathname,search:t.search,protocol:t.protocol.slice(0,t.protocol.indexOf(":")),sameOrigin:t.protocol===n.gm?.location?.protocol&&t.host===n.gm?.location?.host};return r.port&&""!==r.port||("http:"===t.protocol&&(r.port="80"),"https:"===t.protocol&&(r.port="443")),r.pathname&&""!==r.pathname?r.pathname.startsWith("/")||(r.pathname="/".concat(r.pathname)):r.pathname="/",r}catch(e){return{}}}},944:(e,t,r)=\>{"use strict";function n(e,t){"function"==typeof console.debug&&console.debug("New Relic Warning: https://github.com/newrelic/newrelic-browser-agent/blob/main/docs/warning-codes.md#".concat(e),t)}r.d(t,{R:()=\>n})},5284:(e,t,r)=\>{"use strict";r.d(t,{t:()=\>c,B:()=\>s});var n=r(7836),i=r(6154);const o="newrelic";const a=new Set,s={};function c(e,t){const r=n.ee.get(t);s[t]??={},e&&"object"==typeof e&&(a.has(t)||(r.emit("rumresp",[e]),s[t]=e,a.add(t),function(e={}){try{i.gm.dispatchEvent(new CustomEvent(o,{detail:e}))}catch(e){}}({loaded:!0})))}},8990:(e,t,r)=\>{"use strict";r.d(t,{I:()=\>i});var n=Object.prototype.hasOwnProperty;function i(e,t,r){if(n.call(e,t))return e[t];var i=r();if(Object.defineProperty&&Object.keys)try{return Object.defineProperty(e,t,{value:i,writable:!0,enumerable:!1}),i}catch(e){}return e[t]=i,i}},6389:(e,t,r)=\>{"use strict";function n(e,t=500,r={}){const n=r?.leading||!1;let i;return(...r)=\>{n&&void 0===i&&(e.apply(this,r),i=setTimeout((()=\>{i=clearTimeout(i)}),t)),n||(clearTimeout(i),i=setTimeout((()=\>{e.apply(this,r)}),t))}}function i(e){let t=!1;return(...r)=\>{t||(t=!0,e.apply(this,r))}}r.d(t,{J:()=\>i,s:()=\>n})},3304:(e,t,r)=\>{"use strict";r.d(t,{A:()=\>o});var n=r(7836);const i=()=\>{const e=new WeakSet;return(t,r)=\>{if("object"==typeof r&&null!==r){if(e.has(r))return;e.add(r)}return r}};function o(e){try{return JSON.stringify(e,i())??""}catch(e){try{n.ee.emit("internal-error",[e])}catch(e){}return""}}},5289:(e,t,r)=\>{"use strict";r.d(t,{GG:()=\>o,sB:()=\>a});var n=r(3878);function i(){return"undefined"==typeof document||"complete"===document.readyState}function o(e,t){if(i())return e();(0,n.sp)("load",e,t)}function a(e){if(i())return e();(0,n.DD)("DOMContentLoaded",e)}},384:(e,t,r)=\>{"use strict";r.d(t,{NT:()=\>o,US:()=\>d,Zm:()=\>a,bQ:()=\>c,dV:()=\>s,nY:()=\>u,pV:()=\>l});var n=r(6154),i=r(1863);const o={beacon:"bam.nr-data.net",errorBeacon:"bam.nr-data.net"};function a(){return n.gm.NREUM||(n.gm.NREUM={}),void 0===n.gm.newrelic&&(n.gm.newrelic=n.gm.NREUM),n.gm.NREUM}function s(){let e=a();return e.o||(e.o={ST:n.gm.setTimeout,SI:n.gm.setImmediate,CT:n.gm.clearTimeout,XHR:n.gm.XMLHttpRequest,REQ:n.gm.Request,EV:n.gm.Event,PR:n.gm.Promise,MO:n.gm.MutationObserver,FETCH:n.gm.fetch,WS:n.gm.WebSocket}),e}function c(e,t){let r=a();r.initializedAgents??={},t.initializedAt={ms:(0,i.t)(),date:new Date},r.initializedAgents[e]=t}function u(e){let t=a();return t.initializedAgents?.[e]}function d(e,t){a()[e]=t}function l(){return function(){let e=a();const t=e.info||{};e.info={beacon:o.beacon,errorBeacon:o.errorBeacon,...t}}(),function(){let e=a();const t=e.init||{};e.init={...t}}(),s(),function(){let e=a();const t=e.loader\_config||{};e.loader\_config={...t}}(),a()}},2843:(e,t,r)=\>{"use strict";r.d(t,{u:()=\>i});var n=r(3878);function i(e,t=!1,r,i){(0,n.DD)("visibilitychange",(function(){if(t)return void("hidden"===document.visibilityState&&e());e(document.visibilityState)}),r,i)}},8139:(e,t,r)=\>{"use strict";r.d(t,{u:()=\>f});var n=r(7836),i=r(3434),o=r(8990),a=r(6154);const s={},c=a.gm.XMLHttpRequest,u="addEventListener",d="removeEventListener",l="nr@wrapped:".concat(n.P);function f(e){var t=function(e){return(e||n.ee).get("events")}(e);if(s[t.debugId]++)return t;s[t.debugId]=1;var r=(0,i.YM)(t,!0);function f(e){r.inPlace(e,[u,d],"-",p)}function p(e,t){return e[1]}return"getPrototypeOf"in Object&&(a.RI&&h(document,f),c&&h(c.prototype,f),h(a.gm,f)),t.on(u+"-start",(function(e,t){var n=e[1];if(null!==n&&("function"==typeof n||"object"==typeof n)){var i=(0,o.I)(n,l,(function(){var e={object:function(){if("function"!=typeof n.handleEvent)return;return n.handleEvent.apply(n,arguments)},function:n}[typeof n];return e?r(e,"fn-",null,e.name||"anonymous"):n}));this.wrapped=e[1]=i}})),t.on(d+"-start",(function(e){e[1]=this.wrapped||e[1]})),t}function h(e,t,...r){let n=e;for(;"object"==typeof n&&!Object.prototype.hasOwnProperty.call(n,u);)n=Object.getPrototypeOf(n);n&&t(n,...r)}},3434:(e,t,r)=\>{"use strict";r.d(t,{Jt:()=\>o,YM:()=\>c});var n=r(7836),i=r(5607);const o="nr@original:".concat(i.W);var a=Object.prototype.hasOwnProperty,s=!1;function c(e,t){return e||(e=n.ee),r.inPlace=function(e,t,n,i,o){n||(n="");const a="-"===n.charAt(0);for(let s=0;s\<t.length;s++){const c=t[s],u=e[c];d(u)||(e[c]=r(u,a?c+n:n,i,c,o))}},r.flag=o,r;function r(t,r,n,s,c){return d(t)?t:(r||(r=""),nrWrapper[o]=t,function(e,t,r){if(Object.defineProperty&&Object.keys)try{return Object.keys(e).forEach((function(r){Object.defineProperty(t,r,{get:function(){return e[r]},set:function(t){return e[r]=t,t}})})),t}catch(e){u([e],r)}for(var n in e)a.call(e,n)&&(t[n]=e[n])}(t,nrWrapper,e),nrWrapper);function nrWrapper(){var o,a,d,l;try{a=this,o=[...arguments],d="function"==typeof n?n(o,a):n||{}}catch(t){u([t,"",[o,a,s],d],e)}i(r+"start",[o,a,s],d,c);try{return l=t.apply(a,o)}catch(e){throw i(r+"err",[o,a,e],d,c),e}finally{i(r+"end",[o,a,l],d,c)}}}function i(r,n,i,o){if(!s||t){var a=s;s=!0;try{e.emit(r,n,i,t,o)}catch(t){u([t,r,n,i],e)}s=a}}}function u(e,t){t||(t=n.ee);try{t.emit("internal-error",e)}catch(e){}}function d(e){return!(e&&"function"==typeof e&&e.apply&&!e[o])}},9414:(e,t,r)=\>{"use strict";r.d(t,{J:()=\>c});var n=r(7836),i=r(2646),o=r(944),a=r(3434);const s=new Map;function c(e,t,r,c){if("object"!=typeof t||!t||"string"!=typeof r||!r||"function"!=typeof t[r])return(0,o.R)(29);const u=function(e){return(e||n.ee).get("logger")}(e),d=(0,a.YM)(u),l=new i.y(n.P);l.level=c.level,l.customAttributes=c.customAttributes;const f=t[r]?.[a.Jt]||t[r];return s.set(f,l),d.inPlace(t,[r],"wrap-logger-",(()=\>s.get(f))),u}},9300:(e,t,r)=\>{"use strict";r.d(t,{T:()=\>n});const n=r(860).K7.ajax},3333:(e,t,r)=\>{"use strict";r.d(t,{$v:()=\>u,TZ:()=\>n,Zp:()=\>i,kd:()=\>c,mq:()=\>s,nf:()=\>a,qN:()=\>o});const n=r(860).K7.genericEvents,i=["auxclick","click","copy","keydown","paste","scrollend"],o=["focus","blur"],a=4,s=1e3,c=["PageAction","UserAction","BrowserPerformance"],u={MARKS:"experimental.marks",MEASURES:"experimental.measures",RESOURCES:"experimental.resources"}},6774:(e,t,r)=\>{"use strict";r.d(t,{T:()=\>n});const n=r(860).K7.jserrors},993:(e,t,r)=\>{"use strict";r.d(t,{A$:()=\>o,ET:()=\>a,TZ:()=\>s,p\_:()=\>i});var n=r(860);const i={ERROR:"ERROR",WARN:"WARN",INFO:"INFO",DEBUG:"DEBUG",TRACE:"TRACE"},o={OFF:0,ERROR:1,WARN:2,INFO:3,DEBUG:4,TRACE:5},a="log",s=n.K7.logging},3785:(e,t,r)=\>{"use strict";r.d(t,{R:()=\>c,b:()=\>u});var n=r(9908),i=r(1863),o=r(860),a=r(8154),s=r(993);function c(e,t,r={},c=s.p\_.INFO){(0,n.p)(a.xV,["API/logging/".concat(c.toLowerCase(),"/called")],void 0,o.K7.metrics,e),(0,n.p)(s.ET,[(0,i.t)(),t,r,c],void 0,o.K7.logging,e)}function u(e){return"string"==typeof e&&Object.values(s.p\_).some((t=\>t===e.toUpperCase().trim()))}},8154:(e,t,r)=\>{"use strict";r.d(t,{z\_:()=\>o,XG:()=\>s,TZ:()=\>n,rs:()=\>i,xV:()=\>a});r(6154),r(9566),r(384);const n=r(860).K7.metrics,i="sm",o="cm",a="storeSupportabilityMetrics",s="storeEventMetrics"},6630:(e,t,r)=\>{"use strict";r.d(t,{T:()=\>n});const n=r(860).K7.pageViewEvent},782:(e,t,r)=\>{"use strict";r.d(t,{T:()=\>n});const n=r(860).K7.pageViewTiming},6344:(e,t,r)=\>{"use strict";r.d(t,{BB:()=\>d,G4:()=\>o,Qb:()=\>l,TZ:()=\>i,Ug:()=\>a,\_s:()=\>s,bc:()=\>u,yP:()=\>c});var n=r(2614);const i=r(860).K7.sessionReplay,o={RECORD:"recordReplay",PAUSE:"pauseReplay",REPLAY\_RUNNING:"replayRunning",ERROR\_DURING\_REPLAY:"errorDuringReplay"},a=.12,s={DomContentLoaded:0,Load:1,FullSnapshot:2,IncrementalSnapshot:3,Meta:4,Custom:5},c={[n.g.ERROR]:15e3,[n.g.FULL]:3e5,[n.g.OFF]:0},u={RESET:{message:"Session was reset",sm:"Reset"},IMPORT:{message:"Recorder failed to import",sm:"Import"},TOO\_MANY:{message:"429: Too Many Requests",sm:"Too-Many"},TOO\_BIG:{message:"Payload was too large",sm:"Too-Big"},CROSS\_TAB:{message:"Session Entity was set to OFF on another tab",sm:"Cross-Tab"},ENTITLEMENTS:{message:"Session Replay is not allowed and will not be started",sm:"Entitlement"}},d=5e3,l={API:"api"}},5270:(e,t,r)=\>{"use strict";r.d(t,{Aw:()=\>c,CT:()=\>u,SR:()=\>s});var n=r(384),i=r(9417),o=r(7767),a=r(6154);function s(e){return!!(0,n.dV)().o.MO&&(0,o.V)(e)&&!0===(0,i.gD)(e,"session\_trace.enabled")}function c(e){return!0===(0,i.gD)(e,"session\_replay.preload")&&s(e)}function u(e,t){const r=t.correctAbsoluteTimestamp(e);return{originalTimestamp:e,correctedTimestamp:r,timestampDiff:e-r,originTime:a.WN,correctedOriginTime:t.correctedOriginTime,originTimeDiff:Math.floor(a.WN-t.correctedOriginTime)}}},3738:(e,t,r)=\>{"use strict";r.d(t,{He:()=\>i,Kp:()=\>s,Lc:()=\>u,Rz:()=\>d,TZ:()=\>n,bD:()=\>o,d3:()=\>a,jx:()=\>l,uP:()=\>c});const n=r(860).K7.sessionTrace,i="bstResource",o="resource",a="-start",s="-end",c="fn"+a,u="fn"+s,d="pushState",l=1e3},3962:(e,t,r)=\>{"use strict";r.d(t,{AM:()=\>o,O2:()=\>c,Qu:()=\>u,TZ:()=\>s,ih:()=\>d,pP:()=\>a,tC:()=\>i});var n=r(860);const i=["click","keydown","submit","popstate"],o="api",a="initialPageLoad",s=n.K7.softNav,c={INITIAL\_PAGE\_LOAD:"",ROUTE\_CHANGE:1,UNSPECIFIED:2},u={INTERACTION:1,AJAX:2,CUSTOM\_END:3,CUSTOM\_TRACER:4},d={IP:"in progress",FIN:"finished",CAN:"cancelled"}},7378:(e,t,r)=\>{"use strict";r.d(t,{$p:()=\>x,BR:()=\>b,Kp:()=\>R,L3:()=\>y,Lc:()=\>c,NC:()=\>o,SG:()=\>d,TZ:()=\>i,U6:()=\>p,UT:()=\>m,d3:()=\>w,dT:()=\>f,e5:()=\>A,gx:()=\>v,l9:()=\>l,oW:()=\>h,op:()=\>g,rw:()=\>u,tH:()=\>T,uP:()=\>s,wW:()=\>E,xq:()=\>a});var n=r(384);const i=r(860).K7.spa,o=["click","submit","keypress","keydown","keyup","change"],a=999,s="fn-start",c="fn-end",u="cb-start",d="api-ixn-",l="remaining",f="interaction",h="spaNode",p="jsonpNode",g="fetch-start",m="fetch-done",v="fetch-body-",b="jsonp-end",y=(0,n.dV)().o.ST,w="-start",R="-end",x="-body",E="cb"+R,A="jsTime",T="fetch"},4234:(e,t,r)=\>{"use strict";r.d(t,{W:()=\>o});var n=r(7836),i=r(1687);class o{constructor(e,t){this.agentIdentifier=e,this.ee=n.ee.get(e),this.featureName=t,this.blocked=!1}deregisterDrain(){(0,i.x3)(this.agentIdentifier,this.featureName)}}},7767:(e,t,r)=\>{"use strict";r.d(t,{V:()=\>o});var n=r(9417),i=r(6154);const o=e=\>i.RI&&!0===(0,n.gD)(e,"privacy.cookies\_enabled")},8969:(e,t,r)=\>{"use strict";r.d(t,{j:()=\>O});var n=r(860),i=r(2555),o=r(3371),a=r(9908),s=r(7836),c=r(1687),u=r(5289),d=r(6154),l=r(944),f=r(8154),h=r(384),p=r(6344);const g=["setErrorHandler","finished","addToTrace","addRelease","recordCustomEvent","addPageAction","setCurrentRouteName","setPageViewName","setCustomAttribute","interaction","noticeError","setUserId","setApplicationVersion","start",p.G4.RECORD,p.G4.PAUSE,"log","wrapLogger"],m=["setErrorHandler","finished","addToTrace","addRelease"];var v=r(1863),b=r(2614),y=r(993),w=r(3785),R=r(9414);function x(){const e=(0,h.pV)();g.forEach((t=\>{e[t]=(...r)=\>function(t,...r){let n=[];return Object.values(e.initializedAgents).forEach((e=\>{e&&e.api?e.exposed&&e.api[t]&&n.push(e.api[t](...r)):(0,l.R)(38,t)})),n.length\>1?n:n[0]}(t,...r)}))}const E={};var A=r(9417),T=r(5603),N=r(5284);const S=e=\>{const t=e.startsWith("http");e+="/",r.p=t?e:"https://"+e};let \_=!1;function O(e,t={},g,O){let{init:I,info:P,loader\_config:j,runtime:C={},exposed:k=!0}=t;C.loaderType=g;const L=(0,h.pV)();P||(I=L.init,P=L.info,j=L.loader\_config),(0,A.xN)(e.agentIdentifier,I||{}),(0,T.a)(e.agentIdentifier,j||{}),P.jsAttributes??={},d.bv&&(P.jsAttributes.isWorker=!0),(0,i.x1)(e.agentIdentifier,P);const H=(0,A.D0)(e.agentIdentifier),M=[P.beacon,P.errorBeacon];\_||(H.proxy.assets&&(S(H.proxy.assets),M.push(H.proxy.assets)),H.proxy.beacon&&M.push(H.proxy.beacon),x(),(0,h.US)("activatedFeatures",N.B),e.runSoftNavOverSpa&&=!0===H.soft\_navigations.enabled&&H.feature\_flags.includes("soft\_nav")),C.denyList=[...H.ajax.deny\_list||[],...H.ajax.block\_internal?M:[]],C.ptid=e.agentIdentifier,(0,o.V)(e.agentIdentifier,C),e.ee=s.ee.get(e.agentIdentifier),void 0===e.api&&(e.api=function(e,t,h=!1){t||(0,c.Ak)(e,"api");const g={};var x=s.ee.get(e),A=x.get("tracer");E[e]=b.g.OFF,x.on(p.G4.REPLAY\_RUNNING,(t=\>{E[e]=t}));var T="api-",N=T+"ixn-";function S(t,r,n,o){const a=(0,i.Vp)(e);return null===r?delete a.jsAttributes[t]:(0,i.x1)(e,{...a,jsAttributes:{...a.jsAttributes,[t]:r}}),I(T,n,!0,o||null===r?"session":void 0)(t,r)}function \_(){}g.log=function(e,{customAttributes:t={},level:r=y.p\_.INFO}={}){(0,a.p)(f.xV,["API/log/called"],void 0,n.K7.metrics,x),(0,w.R)(x,e,t,r)},g.wrapLogger=(e,t,{customAttributes:r={},level:i=y.p\_.INFO}={})=\>{(0,a.p)(f.xV,["API/wrapLogger/called"],void 0,n.K7.metrics,x),(0,R.J)(x,e,t,{customAttributes:r,level:i})},m.forEach((e=\>{g[e]=I(T,e,!0,"api")})),g.addPageAction=I(T,"addPageAction",!0,n.K7.genericEvents),g.recordCustomEvent=I(T,"recordCustomEvent",!0,n.K7.genericEvents),g.setPageViewName=function(t,r){if("string"==typeof t)return"/"!==t.charAt(0)&&(t="/"+t),(0,o.f)(e).customTransaction=(r||"http://custom.transaction")+t,I(T,"setPageViewName",!0)()},g.setCustomAttribute=function(e,t,r=!1){if("string"==typeof e){if(["string","number","boolean"].includes(typeof t)||null===t)return S(e,t,"setCustomAttribute",r);(0,l.R)(40,typeof t)}else(0,l.R)(39,typeof e)},g.setUserId=function(e){if("string"==typeof e||null===e)return S("enduser.id",e,"setUserId",!0);(0,l.R)(41,typeof e)},g.setApplicationVersion=function(e){if("string"==typeof e||null===e)return S("application.version",e,"setApplicationVersion",!1);(0,l.R)(42,typeof e)},g.start=()=\>{try{(0,a.p)(f.xV,["API/start/called"],void 0,n.K7.metrics,x),x.emit("manual-start-all")}catch(e){(0,l.R)(23,e)}},g[p.G4.RECORD]=function(){(0,a.p)(f.xV,["API/recordReplay/called"],void 0,n.K7.metrics,x),(0,a.p)(p.G4.RECORD,[],void 0,n.K7.sessionReplay,x)},g[p.G4.PAUSE]=function(){(0,a.p)(f.xV,["API/pauseReplay/called"],void 0,n.K7.metrics,x),(0,a.p)(p.G4.PAUSE,[],void 0,n.K7.sessionReplay,x)},g.interaction=function(e){return(new \_).get("object"==typeof e?e:{})};const O=\_.prototype={createTracer:function(e,t){var r={},i=this,o="function"==typeof t;return(0,a.p)(f.xV,["API/createTracer/called"],void 0,n.K7.metrics,x),h||(0,a.p)(N+"tracer",[(0,v.t)(),e,r],i,n.K7.spa,x),function(){if(A.emit((o?"":"no-")+"fn-start",[(0,v.t)(),i,o],r),o)try{return t.apply(this,arguments)}catch(e){const t="string"==typeof e?new Error(e):e;throw A.emit("fn-err",[arguments,this,t],r),t}finally{A.emit("fn-end",[(0,v.t)()],r)}}}};function I(e,t,r,i){return function(){return(0,a.p)(f.xV,["API/"+t+"/called"],void 0,n.K7.metrics,x),i&&(0,a.p)(e+t,[r?(0,v.t)():performance.now(),...arguments],r?null:this,i,x),r?void 0:this}}function P(){r.e(478).then(r.bind(r,8778)).then((({setAPI:t})=\>{t(e),(0,c.Ze)(e,"api")})).catch((e=\>{(0,l.R)(27,e),x.abort()}))}return["actionText","setName","setAttribute","save","ignore","onEnd","getContext","end","get"].forEach((e=\>{O[e]=I(N,e,void 0,h?n.K7.softNav:n.K7.spa)})),g.setCurrentRouteName=h?I(N,"routeName",void 0,n.K7.softNav):I(T,"routeName",!0,n.K7.spa),g.noticeError=function(t,r){"string"==typeof t&&(t=new Error(t)),(0,a.p)(f.xV,["API/noticeError/called"],void 0,n.K7.metrics,x),(0,a.p)("err",[t,(0,v.t)(),!1,r,!!E[e]],void 0,n.K7.jserrors,x)},d.RI?(0,u.GG)((()=\>P()),!0):P(),g}(e.agentIdentifier,O,e.runSoftNavOverSpa)),void 0===e.exposed&&(e.exposed=k),\_=!0}},8374:(e,t,r)=\>{r.nc=(()=\>{try{return document?.currentScript?.nonce}catch(e){}return""})()},860:(e,t,r)=\>{"use strict";r.d(t,{$J:()=\>u,K7:()=\>s,P3:()=\>c,XX:()=\>i,qY:()=\>n,v4:()=\>a});const n="events",i="jserrors",o="browser/blobs",a="rum",s={ajax:"ajax",genericEvents:"generic\_events",jserrors:i,logging:"logging",metrics:"metrics",pageAction:"page\_action",pageViewEvent:"page\_view\_event",pageViewTiming:"page\_view\_timing",sessionReplay:"session\_replay",sessionTrace:"session\_trace",softNav:"soft\_navigations",spa:"spa"},c={[s.pageViewEvent]:1,[s.pageViewTiming]:2,[s.metrics]:3,[s.jserrors]:4,[s.spa]:5,[s.ajax]:6,[s.sessionTrace]:7,[s.softNav]:8,[s.sessionReplay]:9,[s.logging]:10,[s.genericEvents]:11},u={[s.pageViewEvent]:a,[s.pageViewTiming]:n,[s.ajax]:n,[s.spa]:n,[s.softNav]:n,[s.metrics]:i,[s.jserrors]:i,[s.sessionTrace]:o,[s.sessionReplay]:o,[s.logging]:"browser/logs",[s.genericEvents]:"ins"}}},n={};function i(e){var t=n[e];if(void 0!==t)return t.exports;var o=n[e]={exports:{}};return r[e](o,o.exports,i),o.exports}i.m=r,i.d=(e,t)=\>{for(var r in t)i.o(t,r)&&!i.o(e,r)&&Object.defineProperty(e,r,{enumerable:!0,get:t[r]})},i.f={},i.e=e=\>Promise.all(Object.keys(i.f).reduce(((t,r)=\>(i.f[r](e,t),t)),[])),i.u=e=\>({212:"nr-spa-compressor",249:"nr-spa-recorder",478:"nr-spa"}[e]+"-1.283.2.min.js"),i.o=(e,t)=\>Object.prototype.hasOwnProperty.call(e,t),e={},t="NRBA-1.283.2.PROD:",i.l=(r,n,o,a)=\>{if(e[r])e[r].push(n);else{var s,c;if(void 0!==o)for(var u=document.getElementsByTagName("script"),d=0;d\<u.length;d++){var l=u[d];if(l.getAttribute("src")==r||l.getAttribute("data-webpack")==t+o){s=l;break}}if(!s){c=!0;var f={478:"sha512-2oN05BjxuObKuOX8E0vq/zS51M+2HokmNPBRUrIC1fw3hpJqoI18/nckSFiqV11KxT7ag3C+FunKrR8n0PD9Ig==",249:"sha512-Zs5nIHr/khH6G8IhAEdnngg+P7y/IfmjU0PQmXABpCEtSTeKV22OYdaa9lENrW9uxI0lZ6O5e5dCnEMsTS0onA==",212:"sha512-LPKde7A1ZxIHzoSqWKxn5uWVhM9u76Vtmp9DMBf+Ry3mnn2jpsfyfigMYD5Yka2RG3NeIBqOwNYuPrWL39qn6w=="};(s=document.createElement("script")).charset="utf-8",s.timeout=120,i.nc&&s.setAttribute("nonce",i.nc),s.setAttribute("data-webpack",t+o),s.src=r,0!==s.src.indexOf(window.location.origin+"/")&&(s.crossOrigin="anonymous"),f[a]&&(s.integrity=f[a])}e[r]=[n];var h=(t,n)=\>{s.onerror=s.onload=null,clearTimeout(p);var i=e[r];if(delete e[r],s.parentNode&&s.parentNode.removeChild(s),i&&i.forEach((e=\>e(n))),t)return t(n)},p=setTimeout(h.bind(null,void 0,{type:"timeout",target:s}),12e4);s.onerror=h.bind(null,s.onerror),s.onload=h.bind(null,s.onload),c&&document.head.appendChild(s)}},i.r=e=\>{"undefined"!=typeof Symbol&&Symbol.toStringTag&&Object.defineProperty(e,Symbol.toStringTag,{value:"Module"}),Object.defineProperty(e,"\_\_esModule",{value:!0})},i.p="https://js-agent.newrelic.com/",(()=\>{var e={38:0,788:0};i.f.j=(t,r)=\>{var n=i.o(e,t)?e[t]:void 0;if(0!==n)if(n)r.push(n[2]);else{var o=new Promise(((r,i)=\>n=e[t]=[r,i]));r.push(n[2]=o);var a=i.p+i.u(t),s=new Error;i.l(a,(r=\>{if(i.o(e,t)&&(0!==(n=e[t])&&(e[t]=void 0),n)){var o=r&&("load"===r.type?"missing":r.type),a=r&&r.target&&r.target.src;s.message="Loading chunk "+t+" failed.\\n("+o+": "+a+")",s.name="ChunkLoadError",s.type=o,s.request=a,n[1](s)}}),"chunk-"+t,t)}};var t=(t,r)=\>{var n,o,[a,s,c]=r,u=0;if(a.some((t=\>0!==e[t]))){for(n in s)i.o(s,n)&&(i.m[n]=s[n]);if(c)c(i)}for(t&&t(r);u\<a.length;u++)o=a[u],i.o(e,o)&&e[o]&&e[o][0](),e[o]=0},r=self["webpackChunk:NRBA-1.283.2.PROD"]=self["webpackChunk:NRBA-1.283.2.PROD"]||[];r.forEach(t.bind(null,0)),r.push=t.bind(null,r.push.bind(r))})(),(()=\>{"use strict";i(8374);var e=i(944),t=i(6344),r=i(9566);class n{agentIdentifier;constructor(){this.agentIdentifier=(0,r.LA)(16)}#e(t,...r){if("function"==typeof this.api?.[t])return this.api[t](...r);(0,e.R)(35,t)}addPageAction(e,t){return this.#e("addPageAction",e,t)}recordCustomEvent(e,t){return this.#e("recordCustomEvent",e,t)}setPageViewName(e,t){return this.#e("setPageViewName",e,t)}setCustomAttribute(e,t,r){return this.#e("setCustomAttribute",e,t,r)}noticeError(e,t){return this.#e("noticeError",e,t)}setUserId(e){return this.#e("setUserId",e)}setApplicationVersion(e){return this.#e("setApplicationVersion",e)}setErrorHandler(e){return this.#e("setErrorHandler",e)}addRelease(e,t){return this.#e("addRelease",e,t)}log(e,t){return this.#e("log",e,t)}}class o extends n{#e(t,...r){if("function"==typeof this.api?.[t])return this.api[t](...r);(0,e.R)(35,t)}start(){return this.#e("start")}finished(e){return this.#e("finished",e)}recordReplay(){return this.#e(t.G4.RECORD)}pauseReplay(){return this.#e(t.G4.PAUSE)}addToTrace(e){return this.#e("addToTrace",e)}setCurrentRouteName(e){return this.#e("setCurrentRouteName",e)}interaction(){return this.#e("interaction")}wrapLogger(e,t,r){return this.#e("wrapLogger",e,t,r)}}var a=i(860),s=i(9417);const c=Object.values(a.K7);function u(e){const t={};return c.forEach((r=\>{t[r]=function(e,t){return!0===(0,s.gD)(t,"".concat(e,".enabled"))}(r,e)})),t}var d=i(8969);var l=i(1687),f=i(4234),h=i(5289),p=i(6154),g=i(5270),m=i(7767),v=i(6389);class b extends f.W{constructor(e,t,r=!0){super(e.agentIdentifier,t),this.auto=r,this.abortHandler=void 0,this.featAggregate=void 0,this.onAggregateImported=void 0,!1===e.init[this.featureName].autoStart&&(this.auto=!1),this.auto?(0,l.Ak)(e.agentIdentifier,t):this.ee.on("manual-start-all",(0,v.J)((()=\>{(0,l.Ak)(e.agentIdentifier,this.featureName),this.auto=!0,this.importAggregator(e)})))}importAggregator(t,r={}){if(this.featAggregate||!this.auto)return;let n;this.onAggregateImported=new Promise((e=\>{n=e}));const o=async()=\>{let o;try{if((0,m.V)(this.agentIdentifier)){const{setupAgentSession:e}=await i.e(478).then(i.bind(i,6526));o=e(t)}}catch(t){(0,e.R)(20,t),this.ee.emit("internal-error",[t]),this.featureName===a.K7.sessionReplay&&this.abortHandler?.()}try{if(!this.#t(this.featureName,o))return(0,l.Ze)(this.agentIdentifier,this.featureName),void n(!1);const{lazyFeatureLoader:e}=await i.e(478).then(i.bind(i,6103)),{Aggregate:a}=await e(this.featureName,"aggregate");this.featAggregate=new a(t,r),t.runtime.harvester.initializedAggregates.push(this.featAggregate),n(!0)}catch(t){(0,e.R)(34,t),this.abortHandler?.(),(0,l.Ze)(this.agentIdentifier,this.featureName,!0),n(!1),this.ee&&this.ee.abort()}};p.RI?(0,h.GG)((()=\>o()),!0):o()}#t(e,t){switch(e){case a.K7.sessionReplay:return(0,g.SR)(this.agentIdentifier)&&!!t;case a.K7.sessionTrace:return!!t;default:return!0}}}var y=i(6630);class w extends b{static featureName=y.T;constructor(e,t=!0){super(e,y.T,t),this.importAggregator(e)}}var R=i(384);var x=i(9908),E=i(2843),A=i(3878),T=i(782),N=i(1863);class S extends b{static featureName=T.T;constructor(e,t=!0){super(e,T.T,t),p.RI&&((0,E.u)((()=\>(0,x.p)("docHidden",[(0,N.t)()],void 0,T.T,this.ee)),!0),(0,A.sp)("pagehide",(()=\>(0,x.p)("winPagehide",[(0,N.t)()],void 0,T.T,this.ee))),this.importAggregator(e))}}var \_=i(8154);class O extends b{static featureName=\_.TZ;constructor(e,t=!0){super(e,\_.TZ,t),this.importAggregator(e)}}var I=i(6774),P=i(3304);class j{constructor(e,t,r,n,i){this.name="UncaughtError",this.message="string"==typeof e?e:(0,P.A)(e),this.sourceURL=t,this.line=r,this.column=n,this.\_\_newrelic=i}}function C(e){return H(e)?e:new j(void 0!==e?.message?e.message:e,e?.filename||e?.sourceURL,e?.lineno||e?.line,e?.colno||e?.col,e?.\_\_newrelic)}function k(e){const t="Unhandled Promise Rejection";if(!e?.reason)return;if(H(e.reason))try{return e.reason.message=t+": "+e.reason.message,C(e.reason)}catch(t){return C(e.reason)}const r=C(e.reason);return r.message=t+": "+r?.message,r}function L(e){if(e.error instanceof SyntaxError&&!/:\\d+$/.test(e.error.stack?.trim())){const t=new j(e.message,e.filename,e.lineno,e.colno,e.error.\_\_newrelic);return t.name=SyntaxError.name,t}return H(e.error)?e.error:C(e)}function H(e){return e instanceof Error&&!!e.stack}class M extends b{static featureName=I.T;#r=!1;constructor(e,r=!0){super(e,I.T,r);try{this.removeOnAbort=new AbortController}catch(e){}this.ee.on("internal-error",((e,t)=\>{this.abortHandler&&(0,x.p)("ierr",[C(e),(0,N.t)(),!0,{},this.#r,t],void 0,this.featureName,this.ee)})),this.ee.on(t.G4.REPLAY\_RUNNING,(e=\>{this.#r=e})),p.gm.addEventListener("unhandledrejection",(e=\>{this.abortHandler&&(0,x.p)("err",[k(e),(0,N.t)(),!1,{unhandledPromiseRejection:1},this.#r],void 0,this.featureName,this.ee)}),(0,A.jT)(!1,this.removeOnAbort?.signal)),p.gm.addEventListener("error",(e=\>{this.abortHandler&&(0,x.p)("err",[L(e),(0,N.t)(),!1,{},this.#r],void 0,this.featureName,this.ee)}),(0,A.jT)(!1,this.removeOnAbort?.signal)),this.abortHandler=this.#n,this.importAggregator(e)}#n(){this.removeOnAbort?.abort(),this.abortHandler=void 0}}var D=i(8990);let K=1;const U="nr@id";function V(e){const t=typeof e;return!e||"object"!==t&&"function"!==t?-1:e===p.gm?0:(0,D.I)(e,U,(function(){return K++}))}function G(e){if("string"==typeof e&&e.length)return e.length;if("object"==typeof e){if("undefined"!=typeof ArrayBuffer&&e instanceof ArrayBuffer&&e.byteLength)return e.byteLength;if("undefined"!=typeof Blob&&e instanceof Blob&&e.size)return e.size;if(!("undefined"!=typeof FormData&&e instanceof FormData))try{return(0,P.A)(e).length}catch(e){return}}}var F=i(8139),B=i(7836),W=i(3434);const z={},q=["open","send"];function Z(t){var r=t||B.ee;const n=function(e){return(e||B.ee).get("xhr")}(r);if(void 0===p.gm.XMLHttpRequest)return n;if(z[n.debugId]++)return n;z[n.debugId]=1,(0,F.u)(r);var i=(0,W.YM)(n),o=p.gm.XMLHttpRequest,a=p.gm.MutationObserver,s=p.gm.Promise,c=p.gm.setInterval,u="readystatechange",d=["onload","onerror","onabort","onloadstart","onloadend","onprogress","ontimeout"],l=[],f=p.gm.XMLHttpRequest=function(t){const r=new o(t),a=n.context(r);try{n.emit("new-xhr",[r],a),r.addEventListener(u,(s=a,function(){var e=this;e.readyState\>3&&!s.resolved&&(s.resolved=!0,n.emit("xhr-resolved",[],e)),i.inPlace(e,d,"fn-",y)}),(0,A.jT)(!1))}catch(t){(0,e.R)(15,t);try{n.emit("internal-error",[t])}catch(e){}}var s;return r};function h(e,t){i.inPlace(t,["onreadystatechange"],"fn-",y)}if(function(e,t){for(var r in e)t[r]=e[r]}(o,f),f.prototype=o.prototype,i.inPlace(f.prototype,q,"-xhr-",y),n.on("send-xhr-start",(function(e,t){h(e,t),function(e){l.push(e),a&&(g?g.then(b):c?c(b):(m=-m,v.data=m))}(t)})),n.on("open-xhr-start",h),a){var g=s&&s.resolve();if(!c&&!s){var m=1,v=document.createTextNode(m);new a(b).observe(v,{characterData:!0})}}else r.on("fn-end",(function(e){e[0]&&e[0].type===u||b()}));function b(){for(var e=0;e\<l.length;e++)h(0,l[e]);l.length&&(l=[])}function y(e,t){return t}return n}var Y="fetch-",J=Y+"body-",X=["arrayBuffer","blob","json","text","formData"],Q=p.gm.Request,ee=p.gm.Response,te="prototype";const re={};function ne(e){const t=function(e){return(e||B.ee).get("fetch")}(e);if(!(Q&&ee&&p.gm.fetch))return t;if(re[t.debugId]++)return t;function r(e,r,n){var i=e[r];"function"==typeof i&&(e[r]=function(){var e,r=[...arguments],o={};t.emit(n+"before-start",[r],o),o[B.P]&&o[B.P].dt&&(e=o[B.P].dt);var a=i.apply(this,r);return t.emit(n+"start",[r,e],a),a.then((function(e){return t.emit(n+"end",[null,e],a),e}),(function(e){throw t.emit(n+"end",[e],a),e}))})}return re[t.debugId]=1,X.forEach((e=\>{r(Q[te],e,J),r(ee[te],e,J)})),r(p.gm,"fetch",Y),t.on(Y+"end",(function(e,r){var n=this;if(r){var i=r.headers.get("content-length");null!==i&&(n.rxSize=i),t.emit(Y+"done",[null,r],n)}else t.emit(Y+"done",[e],n)})),t}var ie=i(7485),oe=i(5603);class ae{constructor(e){this.agentIdentifier=e}generateTracePayload(e){if(!this.shouldGenerateTrace(e))return null;var t=(0,oe.o)(this.agentIdentifier);if(!t)return null;var n=(t.accountID||"").toString()||null,i=(t.agentID||"").toString()||null,o=(t.trustKey||"").toString()||null;if(!n||!i)return null;var a=(0,r.ZF)(),s=(0,r.el)(),c=Date.now(),u={spanId:a,traceId:s,timestamp:c};return(e.sameOrigin||this.isAllowedOrigin(e)&&this.useTraceContextHeadersForCors())&&(u.traceContextParentHeader=this.generateTraceContextParentHeader(a,s),u.traceContextStateHeader=this.generateTraceContextStateHeader(a,c,n,i,o)),(e.sameOrigin&&!this.excludeNewrelicHeader()||!e.sameOrigin&&this.isAllowedOrigin(e)&&this.useNewrelicHeaderForCors())&&(u.newrelicHeader=this.generateTraceHeader(a,s,c,n,i,o)),u}generateTraceContextParentHeader(e,t){return"00-"+t+"-"+e+"-01"}generateTraceContextStateHeader(e,t,r,n,i){return i+"@nr=0-1-"+r+"-"+n+"-"+e+"----"+t}generateTraceHeader(e,t,r,n,i,o){if(!("function"==typeof p.gm?.btoa))return null;var a={v:[0,1],d:{ty:"Browser",ac:n,ap:i,id:e,tr:t,ti:r}};return o&&n!==o&&(a.d.tk=o),btoa((0,P.A)(a))}shouldGenerateTrace(e){return this.isDtEnabled()&&this.isAllowedOrigin(e)}isAllowedOrigin(e){var t=!1,r={};if((0,s.gD)(this.agentIdentifier,"distributed\_tracing")&&(r=(0,s.D0)(this.agentIdentifier).distributed\_tracing),e.sameOrigin)t=!0;else if(r.allowed\_origins instanceof Array)for(var n=0;n\<r.allowed\_origins.length;n++){var i=(0,ie.D)(r.allowed\_origins[n]);if(e.hostname===i.hostname&&e.protocol===i.protocol&&e.port===i.port){t=!0;break}}return t}isDtEnabled(){var e=(0,s.gD)(this.agentIdentifier,"distributed\_tracing");return!!e&&!!e.enabled}excludeNewrelicHeader(){var e=(0,s.gD)(this.agentIdentifier,"distributed\_tracing");return!!e&&!!e.exclude\_newrelic\_header}useNewrelicHeaderForCors(){var e=(0,s.gD)(this.agentIdentifier,"distributed\_tracing");return!!e&&!1!==e.cors\_use\_newrelic\_header}useTraceContextHeadersForCors(){var e=(0,s.gD)(this.agentIdentifier,"distributed\_tracing");return!!e&&!!e.cors\_use\_tracecontext\_headers}}var se=i(9300),ce=i(7295),ue=["load","error","abort","timeout"],de=ue.length,le=(0,R.dV)().o.REQ,fe=(0,R.dV)().o.XHR;class he extends b{static featureName=se.T;constructor(e,t=!0){super(e,se.T,t),this.dt=new ae(e.agentIdentifier),this.handler=(e,t,r,n)=\>(0,x.p)(e,t,r,n,this.ee);try{const e={xmlhttprequest:"xhr",fetch:"fetch",beacon:"beacon"};p.gm?.performance?.getEntriesByType("resource").forEach((t=\>{if(t.initiatorType in e&&0!==t.responseStatus){const r={status:t.responseStatus},n={rxSize:t.transferSize,duration:Math.floor(t.duration),cbTime:0};pe(r,t.name),this.handler("xhr",[r,n,t.startTime,t.responseEnd,e[t.initiatorType]],void 0,a.K7.ajax)}}))}catch(e){}ne(this.ee),Z(this.ee),function(e,t,r,n){function i(e){var t=this;t.totalCbs=0,t.called=0,t.cbTime=0,t.end=R,t.ended=!1,t.xhrGuids={},t.lastSize=null,t.loadCaptureCalled=!1,t.params=this.params||{},t.metrics=this.metrics||{},e.addEventListener("load",(function(r){E(t,e)}),(0,A.jT)(!1)),p.lR||e.addEventListener("progress",(function(e){t.lastSize=e.loaded}),(0,A.jT)(!1))}function o(e){this.params={method:e[0]},pe(this,e[1]),this.metrics={}}function s(t,r){e.loader\_config.xpid&&this.sameOrigin&&r.setRequestHeader("X-NewRelic-ID",e.loader\_config.xpid);var i=n.generateTracePayload(this.parsedOrigin);if(i){var o=!1;i.newrelicHeader&&(r.setRequestHeader("newrelic",i.newrelicHeader),o=!0),i.traceContextParentHeader&&(r.setRequestHeader("traceparent",i.traceContextParentHeader),i.traceContextStateHeader&&r.setRequestHeader("tracestate",i.traceContextStateHeader),o=!0),o&&(this.dt=i)}}function c(e,r){var n=this.metrics,i=e[0],o=this;if(n&&i){var a=G(i);a&&(n.txSize=a)}this.startTime=(0,N.t)(),this.body=i,this.listener=function(e){try{"abort"!==e.type||o.loadCaptureCalled||(o.params.aborted=!0),("load"!==e.type||o.called===o.totalCbs&&(o.onloadCalled||"function"!=typeof r.onload)&&"function"==typeof o.end)&&o.end(r)}catch(e){try{t.emit("internal-error",[e])}catch(e){}}};for(var s=0;s\<de;s++)r.addEventListener(ue[s],this.listener,(0,A.jT)(!1))}function u(e,t,r){this.cbTime+=e,t?this.onloadCalled=!0:this.called+=1,this.called!==this.totalCbs||!this.onloadCalled&&"function"==typeof r.onload||"function"!=typeof this.end||this.end(r)}function d(e,t){var r=""+V(e)+!!t;this.xhrGuids&&!this.xhrGuids[r]&&(this.xhrGuids[r]=!0,this.totalCbs+=1)}function l(e,t){var r=""+V(e)+!!t;this.xhrGuids&&this.xhrGuids[r]&&(delete this.xhrGuids[r],this.totalCbs-=1)}function f(){this.endTime=(0,N.t)()}function h(e,r){r instanceof fe&&"load"===e[0]&&t.emit("xhr-load-added",[e[1],e[2]],r)}function g(e,r){r instanceof fe&&"load"===e[0]&&t.emit("xhr-load-removed",[e[1],e[2]],r)}function m(e,t,r){t instanceof fe&&("onload"===r&&(this.onload=!0),("load"===(e[0]&&e[0].type)||this.onload)&&(this.xhrCbStart=(0,N.t)()))}function v(e,r){this.xhrCbStart&&t.emit("xhr-cb-time",[(0,N.t)()-this.xhrCbStart,this.onload,r],r)}function b(e){var t,r=e[1]||{};if("string"==typeof e[0]?0===(t=e[0]).length&&p.RI&&(t=""+p.gm.location.href):e[0]&&e[0].url?t=e[0].url:p.gm?.URL&&e[0]&&e[0]instanceof URL?t=e[0].href:"function"==typeof e[0].toString&&(t=e[0].toString()),"string"==typeof t&&0!==t.length){t&&(this.parsedOrigin=(0,ie.D)(t),this.sameOrigin=this.parsedOrigin.sameOrigin);var i=n.generateTracePayload(this.parsedOrigin);if(i&&(i.newrelicHeader||i.traceContextParentHeader))if(e[0]&&e[0].headers)s(e[0].headers,i)&&(this.dt=i);else{var o={};for(var a in r)o[a]=r[a];o.headers=new Headers(r.headers||{}),s(o.headers,i)&&(this.dt=i),e.length\>1?e[1]=o:e.push(o)}}function s(e,t){var r=!1;return t.newrelicHeader&&(e.set("newrelic",t.newrelicHeader),r=!0),t.traceContextParentHeader&&(e.set("traceparent",t.traceContextParentHeader),t.traceContextStateHeader&&e.set("tracestate",t.traceContextStateHeader),r=!0),r}}function y(e,t){this.params={},this.metrics={},this.startTime=(0,N.t)(),this.dt=t,e.length\>=1&&(this.target=e[0]),e.length\>=2&&(this.opts=e[1]);var r,n=this.opts||{},i=this.target;"string"==typeof i?r=i:"object"==typeof i&&i instanceof le?r=i.url:p.gm?.URL&&"object"==typeof i&&i instanceof URL&&(r=i.href),pe(this,r);var o=(""+(i&&i instanceof le&&i.method||n.method||"GET")).toUpperCase();this.params.method=o,this.body=n.body,this.txSize=G(n.body)||0}function w(e,t){if(this.endTime=(0,N.t)(),this.params||(this.params={}),(0,ce.iW)(this.params))return;let n;this.params.status=t?t.status:0,"string"==typeof this.rxSize&&this.rxSize.length\>0&&(n=+this.rxSize);const i={txSize:this.txSize,rxSize:n,duration:(0,N.t)()-this.startTime};r("xhr",[this.params,i,this.startTime,this.endTime,"fetch"],this,a.K7.ajax)}function R(e){const t=this.params,n=this.metrics;if(!this.ended){this.ended=!0;for(let t=0;t\<de;t++)e.removeEventListener(ue[t],this.listener,!1);t.aborted||(0,ce.iW)(t)||(n.duration=(0,N.t)()-this.startTime,this.loadCaptureCalled||4!==e.readyState?null==t.status&&(t.status=0):E(this,e),n.cbTime=this.cbTime,r("xhr",[t,n,this.startTime,this.endTime,"xhr"],this,a.K7.ajax))}}function E(e,r){e.params.status=r.status;var n=function(e,t){var r=e.responseType;return"json"===r&&null!==t?t:"arraybuffer"===r||"blob"===r||"json"===r?G(e.response):"text"===r||""===r||void 0===r?G(e.responseText):void 0}(r,e.lastSize);if(n&&(e.metrics.rxSize=n),e.sameOrigin){var i=r.getResponseHeader("X-NewRelic-App-Data");i&&((0,x.p)(\_.rs,["Ajax/CrossApplicationTracing/Header/Seen"],void 0,a.K7.metrics,t),e.params.cat=i.split(", ").pop())}e.loadCaptureCalled=!0}t.on("new-xhr",i),t.on("open-xhr-start",o),t.on("open-xhr-end",s),t.on("send-xhr-start",c),t.on("xhr-cb-time",u),t.on("xhr-load-added",d),t.on("xhr-load-removed",l),t.on("xhr-resolved",f),t.on("addEventListener-end",h),t.on("removeEventListener-end",g),t.on("fn-end",v),t.on("fetch-before-start",b),t.on("fetch-start",y),t.on("fn-start",m),t.on("fetch-done",w)}(e,this.ee,this.handler,this.dt),this.importAggregator(e)}}function pe(e,t){var r=(0,ie.D)(t),n=e.params||e;n.hostname=r.hostname,n.port=r.port,n.protocol=r.protocol,n.host=r.hostname+":"+r.port,n.pathname=r.pathname,e.parsedOrigin=r,e.sameOrigin=r.sameOrigin}const ge={},me=["pushState","replaceState"];function ve(e){const t=function(e){return(e||B.ee).get("history")}(e);return!p.RI||ge[t.debugId]++||(ge[t.debugId]=1,(0,W.YM)(t).inPlace(window.history,me,"-")),t}var be=i(3738);const{He:ye,bD:we,d3:Re,Kp:xe,TZ:Ee,Lc:Ae,uP:Te,Rz:Ne}=be;class Se extends b{static featureName=Ee;constructor(e,t=!0){super(e,Ee,t);if(!(0,m.V)(this.agentIdentifier))return void this.deregisterDrain();const r=this.ee;let n;ve(r),this.eventsEE=(0,F.u)(r),this.eventsEE.on(Te,(function(e,t){this.bstStart=(0,N.t)()})),this.eventsEE.on(Ae,(function(e,t){(0,x.p)("bst",[e[0],t,this.bstStart,(0,N.t)()],void 0,a.K7.sessionTrace,r)})),r.on(Ne+Re,(function(e){this.time=(0,N.t)(),this.startPath=location.pathname+location.hash})),r.on(Ne+xe,(function(e){(0,x.p)("bstHist",[location.pathname+location.hash,this.startPath,this.time],void 0,a.K7.sessionTrace,r)}));try{n=new PerformanceObserver((e=\>{const t=e.getEntries();(0,x.p)(ye,[t],void 0,a.K7.sessionTrace,r)})),n.observe({type:we,buffered:!0})}catch(e){}this.importAggregator(e,{resourceObserver:n})}}var \_e=i(2614);class Oe extends b{static featureName=t.TZ;#i;#o;constructor(e,r=!0){let n;super(e,t.TZ,r),this.replayRunning=!1,this.#o=e;try{n=JSON.parse(localStorage.getItem("".concat(\_e.H3,"\_").concat(\_e.uh)))}catch(e){}(0,g.SR)(e.agentIdentifier)&&this.ee.on(t.G4.RECORD,(()=\>this.#a())),this.#s(n)?(this.#i=n?.sessionReplayMode,this.#c()):this.importAggregator(e),this.ee.on("err",(e=\>{this.replayRunning&&(this.errorNoticed=!0,(0,x.p)(t.G4.ERROR\_DURING\_REPLAY,[e],void 0,this.featureName,this.ee))})),this.ee.on(t.G4.REPLAY\_RUNNING,(e=\>{this.replayRunning=e}))}#s(e){return e&&(e.sessionReplayMode===\_e.g.FULL||e.sessionReplayMode===\_e.g.ERROR)||(0,g.Aw)(this.agentIdentifier)}#u=!1;async#c(e){if(!this.#u){this.#u=!0;try{const{Recorder:t}=await Promise.all([i.e(478),i.e(249)]).then(i.bind(i,8589));this.recorder??=new t({mode:this.#i,agentIdentifier:this.agentIdentifier,trigger:e,ee:this.ee,agentRef:this.#o}),this.recorder.startRecording(),this.abortHandler=this.recorder.stopRecording}catch(e){}this.importAggregator(this.#o,{recorder:this.recorder,errorNoticed:this.errorNoticed})}}#a(){this.featAggregate?this.featAggregate.mode!==\_e.g.FULL&&this.featAggregate.initializeRecording(\_e.g.FULL,!0):(this.#i=\_e.g.FULL,this.#c(t.Qb.API),this.recorder&&this.recorder.parent.mode!==\_e.g.FULL&&(this.recorder.parent.mode=\_e.g.FULL,this.recorder.stopRecording(),this.recorder.startRecording(),this.abortHandler=this.recorder.stopRecording))}}var Ie=i(3962);class Pe extends b{static featureName=Ie.TZ;constructor(e,t=!0){if(super(e,Ie.TZ,t),!p.RI||!(0,R.dV)().o.MO)return;const r=ve(this.ee);Ie.tC.forEach((e=\>{(0,A.sp)(e,(e=\>{a(e)}),!0)}));const n=()=\>(0,x.p)("newURL",[(0,N.t)(),""+window.location],void 0,this.featureName,this.ee);r.on("pushState-end",n),r.on("replaceState-end",n);try{this.removeOnAbort=new AbortController}catch(e){}(0,A.sp)("popstate",(e=\>(0,x.p)("newURL",[e.timeStamp,""+window.location],void 0,this.featureName,this.ee)),!0,this.removeOnAbort?.signal);let i=!1;const o=new((0,R.dV)().o.MO)(((e,t)=\>{i||(i=!0,requestAnimationFrame((()=\>{(0,x.p)("newDom",[(0,N.t)()],void 0,this.featureName,this.ee),i=!1})))})),a=(0,v.s)((e=\>{(0,x.p)("newUIEvent",[e],void 0,this.featureName,this.ee),o.observe(document.body,{attributes:!0,childList:!0,subtree:!0,characterData:!0})}),100,{leading:!0});this.abortHandler=function(){this.removeOnAbort?.abort(),o.disconnect(),this.abortHandler=void 0},this.importAggregator(e,{domObserver:o})}}var je=i(7378);const Ce={},ke=["appendChild","insertBefore","replaceChild"];function Le(e){const t=function(e){return(e||B.ee).get("jsonp")}(e);if(!p.RI||Ce[t.debugId])return t;Ce[t.debugId]=!0;var r=(0,W.YM)(t),n=/[?&](?:callback|cb)=([^&#]+)/,i=/(.\*)\\.([^.]+)/,o=/^(\\w+)(\\.|$)(.\*)$/;function a(e,t){if(!e)return t;const r=e.match(o),n=r[1];return a(r[3],t[n])}return r.inPlace(Node.prototype,ke,"dom-"),t.on("dom-start",(function(e){!function(e){if(!e||"string"!=typeof e.nodeName||"script"!==e.nodeName.toLowerCase())return;if("function"!=typeof e.addEventListener)return;var o=(s=e.src,c=s.match(n),c?c[1]:null);var s,c;if(!o)return;var u=function(e){var t=e.match(i);if(t&&t.length\>=3)return{key:t[2],parent:a(t[1],window)};return{key:e,parent:window}}(o);if("function"!=typeof u.parent[u.key])return;var d={};function l(){t.emit("jsonp-end",[],d),e.removeEventListener("load",l,(0,A.jT)(!1)),e.removeEventListener("error",f,(0,A.jT)(!1))}function f(){t.emit("jsonp-error",[],d),t.emit("jsonp-end",[],d),e.removeEventListener("load",l,(0,A.jT)(!1)),e.removeEventListener("error",f,(0,A.jT)(!1))}r.inPlace(u.parent,[u.key],"cb-",d),e.addEventListener("load",l,(0,A.jT)(!1)),e.addEventListener("error",f,(0,A.jT)(!1)),t.emit("new-jsonp",[e.src],d)}(e[0])})),t}const He={};function Me(e){const t=function(e){return(e||B.ee).get("promise")}(e);if(He[t.debugId])return t;He[t.debugId]=!0;var r=t.context,n=(0,W.YM)(t),i=p.gm.Promise;return i&&function(){function e(r){var o=t.context(),a=n(r,"executor-",o,null,!1);const s=Reflect.construct(i,[a],e);return t.context(s).getCtx=function(){return o},s}p.gm.Promise=e,Object.defineProperty(e,"name",{value:"Promise"}),e.toString=function(){return i.toString()},Object.setPrototypeOf(e,i),["all","race"].forEach((function(r){const n=i[r];e[r]=function(e){let i=!1;[...e||[]].forEach((e=\>{this.resolve(e).then(a("all"===r),a(!1))}));const o=n.apply(this,arguments);return o;function a(e){return function(){t.emit("propagate",[null,!i],o,!1,!1),i=i||!e}}}})),["resolve","reject"].forEach((function(r){const n=i[r];e[r]=function(e){const r=n.apply(this,arguments);return e!==r&&t.emit("propagate",[e,!0],r,!1,!1),r}})),e.prototype=i.prototype;const o=i.prototype.then;i.prototype.then=function(...e){var i=this,a=r(i);a.promise=i,e[0]=n(e[0],"cb-",a,null,!1),e[1]=n(e[1],"cb-",a,null,!1);const s=o.apply(this,e);return a.nextPromise=s,t.emit("propagate",[i,!0],s,!1,!1),s},i.prototype.then[W.Jt]=o,t.on("executor-start",(function(e){e[0]=n(e[0],"resolve-",this,null,!1),e[1]=n(e[1],"resolve-",this,null,!1)})),t.on("executor-err",(function(e,t,r){e[1](r)})),t.on("cb-end",(function(e,r,n){t.emit("propagate",[n,!0],this.nextPromise,!1,!1)})),t.on("propagate",(function(e,r,n){this.getCtx&&!r||(this.getCtx=function(){if(e instanceof Promise)var r=t.context(e);return r&&r.getCtx?r.getCtx():this})}))}(),t}const De={},Ke="setTimeout",Ue="setInterval",Ve="clearTimeout",Ge="-start",Fe=[Ke,"setImmediate",Ue,Ve,"clearImmediate"];function Be(e){const t=function(e){return(e||B.ee).get("timer")}(e);if(De[t.debugId]++)return t;De[t.debugId]=1;var r=(0,W.YM)(t);return r.inPlace(p.gm,Fe.slice(0,2),Ke+"-"),r.inPlace(p.gm,Fe.slice(2,3),Ue+"-"),r.inPlace(p.gm,Fe.slice(3),Ve+"-"),t.on(Ue+Ge,(function(e,t,n){e[0]=r(e[0],"fn-",null,n)})),t.on(Ke+Ge,(function(e,t,n){this.method=n,this.timerDuration=isNaN(e[1])?0:+e[1],e[0]=r(e[0],"fn-",this,n)})),t}const We={};function ze(e){const t=function(e){return(e||B.ee).get("mutation")}(e);if(!p.RI||We[t.debugId])return t;We[t.debugId]=!0;var r=(0,W.YM)(t),n=p.gm.MutationObserver;return n&&(window.MutationObserver=function(e){return this instanceof n?new n(r(e,"fn-")):n.apply(this,arguments)},MutationObserver.prototype=n.prototype),t}const{TZ:qe,d3:Ze,Kp:Ye,$p:Je,wW:Xe,e5:$e,tH:Qe,uP:et,rw:tt,Lc:rt}=je;class nt extends b{static featureName=qe;constructor(e,t=!0){if(super(e,qe,t),!p.RI)return;try{this.removeOnAbort=new AbortController}catch(e){}let r,n=0;const i=this.ee.get("tracer"),o=Le(this.ee),a=Me(this.ee),s=Be(this.ee),c=Z(this.ee),u=this.ee.get("events"),d=ne(this.ee),l=ve(this.ee),f=ze(this.ee);function h(e,t){l.emit("newURL",[""+window.location,t])}function g(){n++,r=window.location.hash,this[et]=(0,N.t)()}function m(){n--,window.location.hash!==r&&h(0,!0);var e=(0,N.t)();this[$e]=\~\~this[$e]+e-this[et],this[rt]=e}function v(e,t){e.on(t,(function(){this[t]=(0,N.t)()}))}this.ee.on(et,g),a.on(tt,g),o.on(tt,g),this.ee.on(rt,m),a.on(Xe,m),o.on(Xe,m),this.ee.on("fn-err",((...t)=\>{t[2]?.\_\_newrelic?.[e.agentIdentifier]||(0,x.p)("function-err",[...t],void 0,this.featureName,this.ee)})),this.ee.buffer([et,rt,"xhr-resolved"],this.featureName),u.buffer([et],this.featureName),s.buffer(["setTimeout"+Ye,"clearTimeout"+Ze,et],this.featureName),c.buffer([et,"new-xhr","send-xhr"+Ze],this.featureName),d.buffer([Qe+Ze,Qe+"-done",Qe+Je+Ze,Qe+Je+Ye],this.featureName),l.buffer(["newURL"],this.featureName),f.buffer([et],this.featureName),a.buffer(["propagate",tt,Xe,"executor-err","resolve"+Ze],this.featureName),i.buffer([et,"no-"+et],this.featureName),o.buffer(["new-jsonp","cb-start","jsonp-error","jsonp-end"],this.featureName),v(d,Qe+Ze),v(d,Qe+"-done"),v(o,"new-jsonp"),v(o,"jsonp-end"),v(o,"cb-start"),l.on("pushState-end",h),l.on("replaceState-end",h),window.addEventListener("hashchange",h,(0,A.jT)(!0,this.removeOnAbort?.signal)),window.addEventListener("load",h,(0,A.jT)(!0,this.removeOnAbort?.signal)),window.addEventListener("popstate",(function(){h(0,n\>1)}),(0,A.jT)(!0,this.removeOnAbort?.signal)),this.abortHandler=this.#n,this.importAggregator(e)}#n(){this.removeOnAbort?.abort(),this.abortHandler=void 0}}var it=i(3333);class ot extends b{static featureName=it.TZ;constructor(e,t=!0){super(e,it.TZ,t);const r=[e.init.page\_action.enabled,e.init.performance.capture\_marks,e.init.performance.capture\_measures,e.init.user\_actions.enabled,e.init.performance.resources.enabled];if(p.RI&&(e.init.user\_actions.enabled&&(it.Zp.forEach((e=\>(0,A.sp)(e,(e=\>(0,x.p)("ua",[e],void 0,this.featureName,this.ee)),!0))),it.qN.forEach((e=\>{const t=(0,v.s)((e=\>{(0,x.p)("ua",[e],void 0,this.featureName,this.ee)}),500,{leading:!0});(0,A.sp)(e,t)}))),e.init.performance.resources.enabled&&p.gm.PerformanceObserver?.supportedEntryTypes.includes("resource"))){new PerformanceObserver((e=\>{e.getEntries().forEach((e=\>{(0,x.p)("browserPerformance.resource",[e],void 0,this.featureName,this.ee)}))})).observe({type:"resource",buffered:!0})}r.some((e=\>e))?this.importAggregator(e):this.deregisterDrain()}}var at=i(993),st=i(3785),ct=i(9414);class ut extends b{static featureName=at.TZ;constructor(e,t=!0){super(e,at.TZ,t);const r=this.ee;(0,ct.J)(r,p.gm.console,"log",{level:"info"}),(0,ct.J)(r,p.gm.console,"error",{level:"error"}),(0,ct.J)(r,p.gm.console,"warn",{level:"warn"}),(0,ct.J)(r,p.gm.console,"info",{level:"info"}),(0,ct.J)(r,p.gm.console,"debug",{level:"debug"}),(0,ct.J)(r,p.gm.console,"trace",{level:"trace"}),this.ee.on("wrap-logger-end",(function([e]){const{level:t,customAttributes:n}=this;(0,st.R)(r,e,n,t)})),this.importAggregator(e)}}new class extends o{constructor(t){super(),p.gm?(this.features={},(0,R.bQ)(this.agentIdentifier,this),this.desiredFeatures=new Set(t.features||[]),this.desiredFeatures.add(w),this.runSoftNavOverSpa=[...this.desiredFeatures].some((e=\>e.featureName===a.K7.softNav)),(0,d.j)(this,t,t.loaderType||"agent"),this.run()):(0,e.R)(21)}get config(){return{info:this.info,init:this.init,loader\_config:this.loader\_config,runtime:this.runtime}}run(){try{const t=u(this.agentIdentifier),r=[...this.desiredFeatures];r.sort(((e,t)=\>a.P3[e.featureName]-a.P3[t.featureName])),r.forEach((r=\>{if(!t[r.featureName]&&r.featureName!==a.K7.pageViewEvent)return;if(this.runSoftNavOverSpa&&r.featureName===a.K7.spa)return;if(!this.runSoftNavOverSpa&&r.featureName===a.K7.softNav)return;const n=function(e){switch(e){case a.K7.ajax:return[a.K7.jserrors];case a.K7.sessionTrace:return[a.K7.ajax,a.K7.pageViewEvent];case a.K7.sessionReplay:return[a.K7.sessionTrace];case a.K7.pageViewTiming:return[a.K7.pageViewEvent];default:return[]}}(r.featureName).filter((e=\>!(e in this.features)));n.length\>0&&(0,e.R)(36,{targetFeature:r.featureName,missingDependencies:n}),this.features[r.featureName]=new r(this)}))}catch(t){(0,e.R)(22,t);for(const e in this.features)this.features[e].abortHandler?.();const r=(0,R.Zm)();delete r.initializedAgents[this.agentIdentifier]?.api,delete r.initializedAgents[this.agentIdentifier]?.features,delete this.sharedAggregator;return r.ee.get(this.agentIdentifier).abort(),!1}}}({features:[he,w,S,Se,Oe,O,M,ot,ut,Pe,nt],loaderType:"spa"})})()})(); }, 5000); window.addEventListener('DOMContentLoaded', function() { jQuery(document).ready(function() { jQuery('body').on('click', '.oxy-menu-toggle', function() { jQuery(this).parent('.oxy-nav-menu').toggleClass('oxy-nav-menu-open'); jQuery('body').toggleClass('oxy-nav-menu-prevent-overflow'); jQuery('html').toggleClass('oxy-nav-menu-prevent-overflow'); }); var selector = '.oxy-nav-menu-open .menu-item a[href\*="#"]'; jQuery('body').on('click', selector, function(){ jQuery('.oxy-nav-menu-open').removeClass('oxy-nav-menu-open'); jQuery('body').removeClass('oxy-nav-menu-prevent-overflow'); jQuery('html').removeClass('oxy-nav-menu-prevent-overflow'); jQuery(this).click(); }); }); }); window.addEventListener('DOMContentLoaded', function() { jQuery(document).ready(oxygen\_init\_slide\_menu); function oxygen\_init\_slide\_menu($) { // check if supports touch, otherwise it's click: let touchEvent = 'ontouchstart' in window ? 'click' : 'click'; $('.oxy-slide-menu').each(function(){ let slide\_menu = $(this); let slide\_start = slide\_menu.children( '.oxy-slide-menu\_inner' ).data( 'start' ); let slide\_duration = slide\_menu.children( '.oxy-slide-menu\_inner' ).data( 'duration' ); let slideClickArea = '.menu-item-has-children \> a \> .oxy-slide-menu\_dropdown-icon-click-area'; let dropdownIcon = slide\_menu.children( '.oxy-slide-menu\_inner' ).data( 'icon' ); slide\_menu.find('.menu-item-has-children \> a').append('\<button aria-expanded=\\"false\\" aria-pressed=\\"false\\" class=\\"oxy-slide-menu\_dropdown-icon-click-area\\"\>\<svg class=\\"oxy-slide-menu\_dropdown-icon\\"\>\<use xlink:href=\\"#'+ dropdownIcon +'\\"\>\</use\>\</svg\>\<span class=\\"screen-reader-text\\"\>Submenu\</span\>\</button\>'); // If being hidden as starting position, for use as mobile menu if ( slide\_start == 'hidden' ) { let slide\_trigger\_selector = $( slide\_menu.children( '.oxy-slide-menu\_inner' ).data( 'trigger-selector' ) ); //slide\_trigger\_selector.click( function( event ) { slide\_trigger\_selector.on( touchEvent, function(e) { slide\_menu.slideToggle(slide\_duration); } ); } if ('enable' === slide\_menu.children( '.oxy-slide-menu\_inner' ).data( 'currentopen' )) { let currentAncestorButton = slide\_menu.find('.current-menu-ancestor').children('a').children('.oxy-slide-menu\_dropdown-icon-click-area'); currentAncestorButton.attr('aria-expanded', 'true'); currentAncestorButton.attr('aria-pressed', 'true'); currentAncestorButton.addClass('oxy-slide-menu\_open'); currentAncestorButton.closest('.current-menu-ancestor').children('.sub-menu').slideDown(0); } }); // Sub menu icon being clicked $('.oxy-slide-menu, .oxygen-builder-body').on( touchEvent, '.menu-item-has-children \> a \> .oxy-slide-menu\_dropdown-icon-click-area', function(e) { e.stopPropagation(); e.preventDefault(); oxy\_slide\_menu\_toggle(this); } ); function oxy\_slide\_menu\_toggle(trigger) { var durationData = $(trigger).closest('.oxy-slide-menu\_inner').data( 'duration' ); var othermenus = $(trigger).closest( '.menu-item-has-children' ).siblings('.menu-item-has-children'); othermenus.find( '.sub-menu' ).slideUp( durationData ); othermenus.find( '.oxy-slide-menu\_open' ).removeClass( 'oxy-slide-menu\_open' ); othermenus.find( '.oxy-slide-menu\_open' ).attr('aria-expanded', function (i, attr) { return attr == 'true' ? 'false' : 'true' }); othermenus.find( '.oxy-slide-menu\_open' ).attr('aria-pressed', function (i, attr) { return attr == 'true' ? 'false' : 'true' }); $(trigger).closest('.menu-item-has-children').children('.sub-menu').slideToggle( durationData ); $(trigger).attr('aria-expanded', function (i, attr) { return attr == 'true' ? 'false' : 'true' }); $(trigger).attr('aria-pressed', function (i, attr) { return attr == 'true' ? 'false' : 'true' }); $(trigger).toggleClass('oxy-slide-menu\_open'); } let selector = '.oxy-slide-menu .menu-item a[href\*="#"]'; $(selector).on('click', function(event){ if ($(event.target).closest('.oxy-slide-menu\_dropdown-icon-click-area').length \> 0) { // toggle icon clicked, no need to trigger it return; } else if ($(event.target).attr("href") === "#" && $(this).parent().hasClass('menu-item-has-children')) { // prevent browser folllowing link event.preventDefault(); // empty href don't lead anywhere, use it as toggle icon click area var hasklinkIcon = $(this).find('.oxy-slide-menu\_dropdown-icon-click-area'); oxy\_slide\_menu\_toggle(hasklinkIcon); } }); }; }); window.addEventListener('DOMContentLoaded', function() { jQuery(document).ready(oxygen\_init\_burger); function oxygen\_init\_burger($) { $('.oxy-burger-trigger').each(function( i, OxyBurgerTrigger ) { let touchEventOption = $( OxyBurgerTrigger ).children('.hamburger').data('touch'); let touchEvent = 'ontouchstart' in window ? touchEventOption : 'click'; // Close hamburger when element clicked $( OxyBurgerTrigger ).on( touchEvent, function(e) { e.stopPropagation(); // Check user wants animations if ($(this).children( '.hamburger' ).data('animation') !== 'disable') { $(this).children( '.hamburger' ).toggleClass('is-active'); } } ); } ); // For listening for modals closing to close the hamburger var className = 'live'; var target = document.querySelectorAll(".oxy-modal-backdrop[data-trigger='user\_clicks\_element']"); for (var i = 0; i \< target.length; i++) { // create an observer instance var observer = new MutationObserver(function(mutations) { mutations.forEach(function(mutation) { // When the style changes on modal backdrop if (mutation.attributeName === 'style') { // If the modal is live and is closing if(!mutation.target.classList.contains(className)){ // Close the toggle closeToggle(mutation.target); } } }); }); // configuration of the observer var config = { attributes: true, attributeFilter: ['style'], subtree: false }; // pass in the target node, as well as the observer options observer.observe(target[i], config); } // Helper function to close hamburger if modal closed. function closeToggle(elem) { var triggerSelector = $($(elem).data('trigger-selector')); // Abort if burger not being used as the trigger or animations not turned on if ((!triggerSelector.hasClass('oxy-burger-trigger')) || (triggerSelector.children( '.hamburger' ).data('animation') === 'disable') ) { return; } // Close that particular burger triggerSelector.children('.hamburger').removeClass('is-active'); } } }); "use strict";var \_createClass=function(){function defineProperties(target,props){for(var i=0;i\<props.length;i++){var descriptor=props[i];descriptor.enumerable=descriptor.enumerable||!1,descriptor.configurable=!0,"value"in descriptor&&(descriptor.writable=!0),Object.defineProperty(target,descriptor.key,descriptor)}}return function(Constructor,protoProps,staticProps){return protoProps&&defineProperties(Constructor.prototype,protoProps),staticProps&&defineProperties(Constructor,staticProps),Constructor}}();function \_classCallCheck(instance,Constructor){if(!(instance instanceof Constructor))throw new TypeError("Cannot call a class as a function")}var RocketBrowserCompatibilityChecker=function(){function RocketBrowserCompatibilityChecker(options){\_classCallCheck(this,RocketBrowserCompatibilityChecker),this.passiveSupported=!1,this.\_checkPassiveOption(this),this.options=!!this.passiveSupported&&options}return \_createClass(RocketBrowserCompatibilityChecker,[{key:"\_checkPassiveOption",value:function(self){try{var options={get passive(){return!(self.passiveSupported=!0)}};window.addEventListener("test",null,options),window.removeEventListener("test",null,options)}catch(err){self.passiveSupported=!1}}},{key:"initRequestIdleCallback",value:function(){!1 in window&&(window.requestIdleCallback=function(cb){var start=Date.now();return setTimeout(function(){cb({didTimeout:!1,timeRemaining:function(){return Math.max(0,50-(Date.now()-start))}})},1)}),!1 in window&&(window.cancelIdleCallback=function(id){return clearTimeout(id)})}},{key:"isDataSaverModeOn",value:function(){return"connection"in navigator&&!0===navigator.connection.saveData}},{key:"supportsLinkPrefetch",value:function(){var elem=document.createElement("link");return elem.relList&&elem.relList.supports&&elem.relList.supports("prefetch")&&window.IntersectionObserver&&"isIntersecting"in IntersectionObserverEntry.prototype}},{key:"isSlowConnection",value:function(){return"connection"in navigator&&"effectiveType"in navigator.connection&&("2g"===navigator.connection.effectiveType||"slow-2g"===navigator.connection.effectiveType)}}]),RocketBrowserCompatibilityChecker}();var RocketPreloadLinksConfig = {"excludeUris":"\\/th(\\/wp-admin\\/|\\/about-us\\/|\\/search\\/|\\/faq\\/|\\/(?:.+\\/)?feed(?:\\/(?:.+\\/?)?)?$|\\/(?:.+\\/)?embed\\/|\\/(index.php\\/)?(.\*)wp-json(\\/.\*|$))|\\/refer\\/|\\/go\\/|\\/recommend\\/|\\/recommends\\/","usesTrailingSlash":"1","imageExt":"jpg|jpeg|gif|png|tiff|bmp|webp|avif|pdf|doc|docx|xls|xlsx|php","fileExt":"jpg|jpeg|gif|png|tiff|bmp|webp|avif|pdf|doc|docx|xls|xlsx|php|html|htm","siteUrl":"https:\\/\\/www.heygoody.com\\/th","onHoverDelay":"100","rateThrottle":"3"};(function() {
"use strict";var r="function"==typeof Symbol&&"symbol"==typeof Symbol.iterator?function(e){return typeof e}:function(e){return e&&"function"==typeof Symbol&&e.constructor===Symbol&&e!==Symbol.prototype?"symbol":typeof e},e=function(){function i(e,t){for(var n=0;n\<t.length;n++){var i=t[n];i.enumerable=i.enumerable||!1,i.configurable=!0,"value"in i&&(i.writable=!0),Object.defineProperty(e,i.key,i)}}return function(e,t,n){return t&&i(e.prototype,t),n&&i(e,n),e}}();function i(e,t){if(!(e instanceof t))throw new TypeError("Cannot call a class as a function")}var t=function(){function n(e,t){i(this,n),this.browser=e,this.config=t,this.options=this.browser.options,this.prefetched=new Set,this.eventTime=null,this.threshold=1111,this.numOnHover=0}return e(n,[{key:"init",value:function(){!this.browser.supportsLinkPrefetch()||this.browser.isDataSaverModeOn()||this.browser.isSlowConnection()||(this.regex={excludeUris:RegExp(this.config.excludeUris,"i"),images:RegExp(".("+this.config.imageExt+")$","i"),fileExt:RegExp(".("+this.config.fileExt+")$","i")},this.\_initListeners(this))}},{key:"\_initListeners",value:function(e){-1\<this.config.onHoverDelay&&document.addEventListener("mouseover",e.listener.bind(e),e.listenerOptions),document.addEventListener("mousedown",e.listener.bind(e),e.listenerOptions),document.addEventListener("touchstart",e.listener.bind(e),e.listenerOptions)}},{key:"listener",value:function(e){var t=e.target.closest("a"),n=this.\_prepareUrl(t);if(null!==n)switch(e.type){case"mousedown":case"touchstart":this.\_addPrefetchLink(n);break;case"mouseover":this.\_earlyPrefetch(t,n,"mouseout")}}},{key:"\_earlyPrefetch",value:function(t,e,n){var i=this,r=setTimeout(function(){if(r=null,0===i.numOnHover)setTimeout(function(){return i.numOnHover=0},1e3);else if(i.numOnHover\>i.config.rateThrottle)return;i.numOnHover++,i.\_addPrefetchLink(e)},this.config.onHoverDelay);t.addEventListener(n,function e(){t.removeEventListener(n,e,{passive:!0}),null!==r&&(clearTimeout(r),r=null)},{passive:!0})}},{key:"\_addPrefetchLink",value:function(i){return this.prefetched.add(i.href),new Promise(function(e,t){var n=document.createElement("link");n.rel="prefetch",n.href=i.href,n.onload=e,n.onerror=t,document.head.appendChild(n)}).catch(function(){})}},{key:"\_prepareUrl",value:function(e){if(null===e||"object"!==(void 0===e?"undefined":r(e))||!1 in e||-1===["http:","https:"].indexOf(e.protocol))return null;var t=e.href.substring(0,this.config.siteUrl.length),n=this.\_getPathname(e.href,t),i={original:e.href,protocol:e.protocol,origin:t,pathname:n,href:t+n};return this.\_isLinkOk(i)?i:null}},{key:"\_getPathname",value:function(e,t){var n=t?e.substring(this.config.siteUrl.length):e;return n.startsWith("/")||(n="/"+n),this.\_shouldAddTrailingSlash(n)?n+"/":n}},{key:"\_shouldAddTrailingSlash",value:function(e){return this.config.usesTrailingSlash&&!e.endsWith("/")&&!this.regex.fileExt.test(e)}},{key:"\_isLinkOk",value:function(e){return null!==e&&"object"===(void 0===e?"undefined":r(e))&&(!this.prefetched.has(e.href)&&e.origin===this.config.siteUrl&&-1===e.href.indexOf("?")&&-1===e.href.indexOf("#")&&!this.regex.excludeUris.test(e.href)&&!this.regex.images.test(e.href))}}],[{key:"run",value:function(){"undefined"!=typeof RocketPreloadLinksConfig&&new n(new RocketBrowserCompatibilityChecker({capture:!0,passive:!0}),RocketPreloadLinksConfig).init()}}]),n}();t.run();
}());  document.addEventListener('DOMContentLoaded', function() { // หน่วงเวลาโหลด jQuery (10 วินาที) setTimeout(function() { var script = document.createElement('script'); script.src = 'https://www.heygoody.com/th/wp-includes/js/jquery/jquery.min.js'; document.body.appendChild(script); }, 10000); // หน่วงเวลาโหลด New Relic (10 วินาที) setTimeout(function() { var script = document.createElement('script'); script.src = 'https://js-agent.newrelic.com/nr-spa-1.281.0.min.js'; document.body.appendChild(script); }, 10000); }); function copyTextToClipboard(text) { // Create a temporary hidden textarea element to hold the text const textArea = document.createElement("textarea"); textArea.value = text; textArea.style.position = "fixed"; // Avoid scrolling to bottom textArea.style.left = "-9999px"; // Move element out of the screen document.body.appendChild(textArea); textArea.focus(); textArea.select(); try { // Use the Clipboard API to copy the text document.execCommand('copy'); var copysuccess = document.getElementById("div\_block-1735-4617"); console.log('Text successfully copied'); copysuccess.style.display = "flex"; setTimeout( function() { copysuccess.style.display = "none"; }, 1200 ); } catch (err) { console.error('Unable to copy text', err); } // Remove the temporary element document.body.removeChild(textArea);
} document.querySelectorAll(".copyButtonOomJujai").forEach(function(button) { button.addEventListener("click", function() { const text = document.querySelector("#textToCopyOomJujai").innerText; copyTextToClipboard(text); });
}); document.querySelectorAll(".copyButtonPRUEasy").forEach(function(button) { button.addEventListener("click", function() { const text = document.querySelector("#textToCopyPRUEasy").innerText; copyTextToClipboard(text); });
}); document.querySelectorAll(".copyButtonPRUCancer").forEach(function(button) { button.addEventListener("click", function() { const text = document.querySelector("#textToCopyPRUCancer").innerText; copyTextToClipboard(text); });
});:root{scroll-behavior: unset !important;}
body { text-size-adjust: none; -webkit-text-size-adjust: none; -ms-text-size-adjust: none; -moz-text-size-adjust: none;
}
@media (max-width: 768px) { body { overflow: auto !important; }
}
.disable\_input { border-radius: 8px; border: 1px solid var(--border-text-field, #C9C9C9); background: var(--background-bg-disable, #EBEBEB) !important; pointer-events: none;
}
.cleansection { color: #3E3E3E !important; fill: #3E3E3E !important;
} .enableButtonCal { background: var(--button-primary, #13875D) !important; color: #F9F9F4 !important;
} .enbleTextCal { color: #F9F9F4 !important;
} @media (max-width: 1199px) { .slider-container { overflow-x: auto; }
}
.next-btn, .prev-btn { cursor: pointer;
} .active\_filter { background: #0075FF !important;
}
.active\_filter div { color: #ffffff !important;
}
.prakansangkom { accent-color: #1DA578;
}
.oxy-rich-text \> ul { list-style-type: none;
} .oxy-rich-text \> ul \> li { position: relative; padding-left: 16px; /\* Adjust as needed \*/ line-height: normal;
}
.oxy-rich-text \> ul \> li::marker { list-style: none; content: unset; font-size: 14px;
}
.oxy-rich-text \> ul \> li:before { content: '•'; /\* Unicode character for bullet \*/ position: absolute; left: 0; color: #000000; /\* Desired color \*/ font-size: 16px; /\* Desired size \*/ line-height: 1;
}body { font-family: 'notosansthai'!important; background-color: #1d1d1d !important; text-size-adjust: none; -webkit-text-size-adjust: none; -ms-text-size-adjust: none; -moz-text-size-adjust: none;
}
.ct-section { overflow-x: unset !important;
}border-radius: 8px;
box-shadow: 0px 2px 6px 0px rgba(0, 57, 126, 0.30);#prakansangkomcheckbox { width: 20px; height: 20px; border-radius: 4px; border: 1px solid var(--border-text-field, #C9C9C9); background: var(--background-bg-white, #FFF);
}input[type="radio"] { width: 24px; height: 24px; accent-color: #1DA578;
}document.addEventListener("DOMContentLoaded", function() { let inputField = document.getElementById("income"); let len = inputField.length; trcikerButtonCal(len);
}); document.addEventListener("visibilitychange", function() { if (document.visibilityState === 'visible') { let inputField = document.getElementById("income"); let len = inputField.value.length; console.log(len); trcikerButtonCal(len); }
});window.lazyLoadOptions=[{elements\_selector:"img[data-lazy-src],.rocket-lazyload,iframe[data-lazy-src]",data\_src:"lazy-src",data\_srcset:"lazy-srcset",data\_sizes:"lazy-sizes",class\_loading:"lazyloading",class\_loaded:"lazyloaded",threshold:300,callback\_loaded:function(element){if(element.tagName==="IFRAME"&&element.dataset.rocketLazyload=="fitvidscompatible"){if(element.classList.contains("lazyloaded")){if(typeof window.jQuery!="undefined"){if(jQuery.fn.fitVids){jQuery(element).parent().fitVids()}}}}}},{elements\_selector:".rocket-lazyload",data\_src:"lazy-src",data\_srcset:"lazy-srcset",data\_sizes:"lazy-sizes",class\_loading:"lazyloading",class\_loaded:"lazyloaded",threshold:300,}];window.addEventListener('LazyLoad::Initialized',function(e){var lazyLoadInstance=e.detail.instance;if(window.MutationObserver){var observer=new MutationObserver(function(mutations){var image\_count=0;var iframe\_count=0;var rocketlazy\_count=0;mutations.forEach(function(mutation){for(var i=0;i\<mutation.addedNodes.length;i++){if(typeof mutation.addedNodes[i].getElementsByTagName!=='function'){continue}
if(typeof mutation.addedNodes[i].getElementsByClassName!=='function'){continue}
images=mutation.addedNodes[i].getElementsByTagName('img');is\_image=mutation.addedNodes[i].tagName=="IMG";iframes=mutation.addedNodes[i].getElementsByTagName('iframe');is\_iframe=mutation.addedNodes[i].tagName=="IFRAME";rocket\_lazy=mutation.addedNodes[i].getElementsByClassName('rocket-lazyload');image\_count+=images.length;iframe\_count+=iframes.length;rocketlazy\_count+=rocket\_lazy.length;if(is\_image){image\_count+=1}
if(is\_iframe){iframe\_count+=1}}});if(image\_count\>0||iframe\_count\>0||rocketlazy\_count\>0){lazyLoadInstance.update()}});var b=document.getElementsByTagName("body")[0];var config={childList:!0,subtree:!0};observer.observe(b,config)}},!1)function lazyLoadThumb(e,alt){var t='\<img data-lazy-src="https://i.ytimg.com/vi\_webp/ID/hqdefault.webp" alt="" width="480" height="360"\>\<noscript\>\<img src="https://i.ytimg.com/vi\_webp/ID/hqdefault.webp" alt="" width="480" height="360"\>\</noscript\>',a='\<button class="play" aria-label="play Youtube video"\>\</button\>';t=t.replace('alt=""','alt="'+alt+'"');return t.replace("ID",e)+a}function lazyLoadYoutubeIframe(){var e=document.createElement("iframe"),t="ID?autoplay=1";t+=0===this.parentNode.dataset.query.length?'':'&'+this.parentNode.dataset.query;e.setAttribute("src",t.replace("ID",this.parentNode.dataset.src)),e.setAttribute("frameborder","0"),e.setAttribute("allowfullscreen","1"),e.setAttribute("allow", "accelerometer; autoplay; encrypted-media; gyroscope; picture-in-picture"),this.parentNode.parentNode.replaceChild(e,this.parentNode)}document.addEventListener("DOMContentLoaded",function(){var e,t,p,a=document.getElementsByClassName("rll-youtube-player");for(t=0;t\<a.length;t++)e=document.createElement("div"),e.setAttribute("data-id",a[t].dataset.id),e.setAttribute("data-query", a[t].dataset.query),e.setAttribute("data-src", a[t].dataset.src),e.innerHTML=lazyLoadThumb(a[t].dataset.id,a[t].dataset.alt),a[t].appendChild(e),p=e.querySelector('.play'),p.onclick=lazyLoadYoutubeIframe});