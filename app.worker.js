(()=>{"use strict";var e,t,r,n,o,a,i,s,c,u={846:(e,t,r)=>{var n=self;r.e(622).then(r.bind(r,622)).then((function(e){e.init(),n.addEventListener("message",(function(t){if("test"===t.data.type){console.log("Generating image data for...",t.data);var r=new ImageData(new Uint8ClampedArray(e.get_buffer(t.data.width,t.data.height,t.data.row0,t.data.rows).buffer),t.data.width,t.data.rows);n.postMessage({type:"image",width:t.data.width,height:t.data.height,row0:t.data.row0,rows:t.data.rows,data:r})}})),n.postMessage({type:"ready"})}))}},f={};function d(e){var t=f[e];if(void 0!==t)return t.exports;var r=f[e]={id:e,loaded:!1,exports:{}};return u[e](r,r.exports,d),r.loaded=!0,r.exports}d.m=u,d.c=f,d.d=(e,t)=>{for(var r in t)d.o(t,r)&&!d.o(e,r)&&Object.defineProperty(e,r,{enumerable:!0,get:t[r]})},d.f={},d.e=e=>Promise.all(Object.keys(d.f).reduce(((t,r)=>(d.f[r](e,t),t)),[])),d.u=e=>e+".app.worker.js",d.g=function(){if("object"==typeof globalThis)return globalThis;try{return this||new Function("return this")()}catch(e){if("object"==typeof window)return window}}(),d.hmd=e=>((e=Object.create(e)).children||(e.children=[]),Object.defineProperty(e,"exports",{enumerable:!0,set:()=>{throw new Error("ES Modules may not assign module.exports or exports.*, Use ESM export syntax, instead: "+e.id)}}),e),d.o=(e,t)=>Object.prototype.hasOwnProperty.call(e,t),d.r=e=>{"undefined"!=typeof Symbol&&Symbol.toStringTag&&Object.defineProperty(e,Symbol.toStringTag,{value:"Module"}),Object.defineProperty(e,"__esModule",{value:!0})},(()=>{var e;d.g.importScripts&&(e=d.g.location+"");var t=d.g.document;if(!e&&t&&(t.currentScript&&(e=t.currentScript.src),!e)){var r=t.getElementsByTagName("script");if(r.length)for(var n=r.length-1;n>-1&&(!e||!/^http(s?):/.test(e));)e=r[n--].src}if(!e)throw new Error("Automatic publicPath is not supported in this browser");e=e.replace(/#.*$/,"").replace(/\?.*$/,"").replace(/\/[^\/]+$/,"/"),d.p=e})(),(()=>{var e={478:1};d.f.i=(t,r)=>{e[t]||importScripts(d.p+d.u(t))};var t=self.webpackChunklucifer=self.webpackChunklucifer||[],r=t.push.bind(t);t.push=t=>{var[n,o,a]=t;for(var i in o)d.o(o,i)&&(d.m[i]=o[i]);for(a&&a(d);n.length;)e[n.pop()]=1;r(t)}})(),i={},s={409:function(){return{"./index_bg.js":{__wbg_new_abda76e883ba8a5f:function(){return void 0===e&&(e=d.c[298].exports),e.V5()},__wbg_stack_658279fe44541cf6:function(e,r){return void 0===t&&(t=d.c[298].exports),t.u$(e,r)},__wbg_error_f851667af71bcfc6:function(e,t){return void 0===r&&(r=d.c[298].exports),r.Xu(e,t)},__wbindgen_object_drop_ref:function(e){return void 0===n&&(n=d.c[298].exports),n.bk(e)},__wbg_random_26e2d782b541ca6b:function(){return void 0===o&&(o=d.c[298].exports),o.gK()},__wbindgen_throw:function(e,t){return void 0===a&&(a=d.c[298].exports),a.Qn(e,t)}}}}},c={622:[409]},d.w={},d.f.wasm=function(e,t){(c[e]||[]).forEach((function(r,n){var o=i[r];if(o)t.push(o);else{var a,c=s[r](),u=fetch(d.p+""+{622:{409:"1d5775cf3bd2168b375a"}}[e][r]+".module.wasm");a=c&&"function"==typeof c.then&&"function"==typeof WebAssembly.compileStreaming?Promise.all([WebAssembly.compileStreaming(u),c]).then((function(e){return WebAssembly.instantiate(e[0],e[1])})):"function"==typeof WebAssembly.instantiateStreaming?WebAssembly.instantiateStreaming(u,c):u.then((function(e){return e.arrayBuffer()})).then((function(e){return WebAssembly.instantiate(e,c)})),t.push(i[r]=a.then((function(e){return d.w[r]=(e.instance||e).exports})))}}))},d(846)})();