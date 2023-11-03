import{o as we,t as ye}from"../chunks/scheduler.eff79e75.js";import{S as Ge,a as Je,I as M,g as Ce,f as Me,b as _e,c as le,s as ee,i as ve,d as F,e as J,P as Ve,h as Xe}from"../chunks/singletons.5f4df5e1.js";function Ze(t,r){return t==="/"||r==="ignore"?t:r==="never"?t.endsWith("/")?t.slice(0,-1):t:r==="always"&&!t.endsWith("/")?t+"/":t}function Qe(t){return t.split("%25").map(decodeURI).join("%25")}function et(t){for(const r in t)t[r]=decodeURIComponent(t[r]);return t}const tt=["href","pathname","search","searchParams","toString","toJSON"];function nt(t,r){const f=new URL(t);for(const s of tt)Object.defineProperty(f,s,{get(){return r(),t[s]},enumerable:!0,configurable:!0});return at(f),f}function at(t){Object.defineProperty(t,"hash",{get(){throw new Error("Cannot access event.url.hash. Consider using `$page.url.hash` inside a component instead")}})}const rt="/__data.json";function ot(t){return t.replace(/\/$/,"")+rt}function it(...t){let r=5381;for(const f of t)if(typeof f=="string"){let s=f.length;for(;s;)r=r*33^f.charCodeAt(--s)}else if(ArrayBuffer.isView(f)){const s=new Uint8Array(f.buffer,f.byteOffset,f.byteLength);let d=s.length;for(;d;)r=r*33^s[--d]}else throw new TypeError("value must be a string or TypedArray");return(r>>>0).toString(36)}const fe=window.fetch;window.fetch=(t,r)=>((t instanceof Request?t.method:(r==null?void 0:r.method)||"GET")!=="GET"&&ne.delete(ke(t)),fe(t,r));const ne=new Map;function st(t,r){const f=ke(t,r),s=document.querySelector(f);if(s!=null&&s.textContent){const{body:d,...u}=JSON.parse(s.textContent),E=s.getAttribute("data-ttl");return E&&ne.set(f,{body:d,init:u,ttl:1e3*Number(E)}),Promise.resolve(new Response(d,u))}return fe(t,r)}function ct(t,r,f){if(ne.size>0){const s=ke(t,f),d=ne.get(s);if(d){if(performance.now()<d.ttl&&["default","force-cache","only-if-cached",void 0].includes(f==null?void 0:f.cache))return new Response(d.body,d.init);ne.delete(s)}}return fe(r,f)}function ke(t,r){let s=`script[data-sveltekit-fetched][data-url=${JSON.stringify(t instanceof Request?t.url:t)}]`;if(r!=null&&r.headers||r!=null&&r.body){const d=[];r.headers&&d.push([...new Headers(r.headers)].join(",")),r.body&&(typeof r.body=="string"||ArrayBuffer.isView(r.body))&&d.push(r.body),s+=`[data-hash="${it(...d)}"]`}return s}const lt=/^(\[)?(\.\.\.)?(\w+)(?:=(\w+))?(\])?$/;function ft(t){const r=[];return{pattern:t==="/"?/^\/$/:new RegExp(`^${dt(t).map(s=>{const d=/^\[\.\.\.(\w+)(?:=(\w+))?\]$/.exec(s);if(d)return r.push({name:d[1],matcher:d[2],optional:!1,rest:!0,chained:!0}),"(?:/(.*))?";const u=/^\[\[(\w+)(?:=(\w+))?\]\]$/.exec(s);if(u)return r.push({name:u[1],matcher:u[2],optional:!0,rest:!1,chained:!0}),"(?:/([^/]+))?";if(!s)return;const E=s.split(/\[(.+?)\](?!\])/);return"/"+E.map((h,g)=>{if(g%2){if(h.startsWith("x+"))return be(String.fromCharCode(parseInt(h.slice(2),16)));if(h.startsWith("u+"))return be(String.fromCharCode(...h.slice(2).split("-").map(U=>parseInt(U,16))));const p=lt.exec(h);if(!p)throw new Error(`Invalid param: ${h}. Params and matcher names can only have underscores and alphanumeric characters.`);const[,x,j,k,N]=p;return r.push({name:k,matcher:N,optional:!!x,rest:!!j,chained:j?g===1&&E[0]==="":!1}),j?"(.*?)":x?"([^/]*)?":"([^/]+?)"}return be(h)}).join("")}).join("")}/?$`),params:r}}function ut(t){return!/^\([^)]+\)$/.test(t)}function dt(t){return t.slice(1).split("/").filter(ut)}function pt(t,r,f){const s={},d=t.slice(1),u=d.filter(l=>l!==void 0);let E=0;for(let l=0;l<r.length;l+=1){const h=r[l];let g=d[l-E];if(h.chained&&h.rest&&E&&(g=d.slice(l-E,l+1).filter(p=>p).join("/"),E=0),g===void 0){h.rest&&(s[h.name]="");continue}if(!h.matcher||f[h.matcher](g)){s[h.name]=g;const p=r[l+1],x=d[l+1];p&&!p.rest&&p.optional&&x&&h.chained&&(E=0),!p&&!x&&Object.keys(s).length===u.length&&(E=0);continue}if(h.optional&&h.chained){E++;continue}return}if(!E)return s}function be(t){return t.normalize().replace(/[[\]]/g,"\\$&").replace(/%/g,"%25").replace(/\//g,"%2[Ff]").replace(/\?/g,"%3[Ff]").replace(/#/g,"%23").replace(/[.*+?^${}()|\\]/g,"\\$&")}function ht({nodes:t,server_loads:r,dictionary:f,matchers:s}){const d=new Set(r);return Object.entries(f).map(([l,[h,g,p]])=>{const{pattern:x,params:j}=ft(l),k={id:l,exec:N=>{const U=x.exec(N);if(U)return pt(U,j,s)},errors:[1,...p||[]].map(N=>t[N]),layouts:[0,...g||[]].map(E),leaf:u(h)};return k.errors.length=k.layouts.length=Math.max(k.errors.length,k.layouts.length),k});function u(l){const h=l<0;return h&&(l=~l),[h,t[l]]}function E(l){return l===void 0?l:[d.has(l),t[l]]}}function Ke(t){try{return JSON.parse(sessionStorage[t])}catch{}}function qe(t,r){const f=JSON.stringify(r);try{sessionStorage[t]=f}catch{}}const gt=-1,mt=-2,wt=-3,yt=-4,_t=-5,vt=-6;function bt(t,r){if(typeof t=="number")return d(t,!0);if(!Array.isArray(t)||t.length===0)throw new Error("Invalid input");const f=t,s=Array(f.length);function d(u,E=!1){if(u===gt)return;if(u===wt)return NaN;if(u===yt)return 1/0;if(u===_t)return-1/0;if(u===vt)return-0;if(E)throw new Error("Invalid input");if(u in s)return s[u];const l=f[u];if(!l||typeof l!="object")s[u]=l;else if(Array.isArray(l))if(typeof l[0]=="string"){const h=l[0],g=r==null?void 0:r[h];if(g)return s[u]=g(d(l[1]));switch(h){case"Date":s[u]=new Date(l[1]);break;case"Set":const p=new Set;s[u]=p;for(let k=1;k<l.length;k+=1)p.add(d(l[k]));break;case"Map":const x=new Map;s[u]=x;for(let k=1;k<l.length;k+=2)x.set(d(l[k]),d(l[k+1]));break;case"RegExp":s[u]=new RegExp(l[1],l[2]);break;case"Object":s[u]=Object(l[1]);break;case"BigInt":s[u]=BigInt(l[1]);break;case"null":const j=Object.create(null);s[u]=j;for(let k=1;k<l.length;k+=2)j[l[k]]=d(l[k+1]);break;default:throw new Error(`Unknown type ${h}`)}}else{const h=new Array(l.length);s[u]=h;for(let g=0;g<l.length;g+=1){const p=l[g];p!==mt&&(h[g]=d(p))}}else{const h={};s[u]=h;for(const g in l){const p=l[g];h[g]=d(p)}}return s[u]}return d(0)}function Et(t){return t.filter(r=>r!=null)}const ze=new Set(["load","prerender","csr","ssr","trailingSlash","config"]);[...ze];const St=new Set([...ze]);[...St];async function kt(t){var r;for(const f in t)if(typeof((r=t[f])==null?void 0:r.then)=="function")return Object.fromEntries(await Promise.all(Object.entries(t).map(async([s,d])=>[s,await d])));return t}class te{constructor(r,f){this.status=r,typeof f=="string"?this.body={message:f}:f?this.body=f:this.body={message:`Error: ${r}`}}toString(){return JSON.stringify(this.body)}}class Fe{constructor(r,f){this.status=r,this.location=f}}const Rt="x-sveltekit-invalidated",At="x-sveltekit-trailing-slash",K=Ke(Ge)??{},Q=Ke(Je)??{};function Ee(t){K[t]=ee()}function It(t,r){var $e;const f=ht(t),s=t.nodes[0],d=t.nodes[1];s(),d();const u=document.documentElement,E=[],l=[];let h=null;const g={before_navigate:[],on_navigate:[],after_navigate:[]};let p={branch:[],error:null,url:null},x=!1,j=!1,k=!0,N=!1,U=!1,H=!1,B=!1,V,D=($e=history.state)==null?void 0:$e[M];D||(D=Date.now(),history.replaceState({...history.state,[M]:D},"",location.href));const ue=K[D];ue&&(history.scrollRestoration="manual",scrollTo(ue.x,ue.y));let q,ae,W;async function Re(){if(W=W||Promise.resolve(),await W,!W)return;W=null;const e=new URL(location.href),i=X(e,!0);h=null;const n=ae={},o=i&&await he(i);if(n===ae&&o){if(o.type==="redirect")return re(new URL(o.location,e).href,{},[e.pathname],n);o.props.page!==void 0&&(q=o.props.page),V.$set(o.props)}}function Ae(e){l.some(i=>i==null?void 0:i.snapshot)&&(Q[e]=l.map(i=>{var n;return(n=i==null?void 0:i.snapshot)==null?void 0:n.capture()}))}function Ie(e){var i;(i=Q[e])==null||i.forEach((n,o)=>{var a,c;(c=(a=l[o])==null?void 0:a.snapshot)==null||c.restore(n)})}function Le(){Ee(D),qe(Ge,K),Ae(D),qe(Je,Q)}async function re(e,{noScroll:i=!1,replaceState:n=!1,keepFocus:o=!1,state:a={},invalidateAll:c=!1},m,v){return typeof e=="string"&&(e=new URL(e,Ce(document))),ce({url:e,scroll:i?ee():null,keepfocus:o,redirect_chain:m,details:{state:a,replaceState:n},nav_token:v,accepted:()=>{c&&(B=!0)},blocked:()=>{},type:"goto"})}async function Pe(e){return h={id:e.id,promise:he(e).then(i=>(i.type==="loaded"&&i.state.error&&(h=null),i))},h.promise}async function oe(...e){const n=f.filter(o=>e.some(a=>o.exec(a))).map(o=>Promise.all([...o.layouts,o.leaf].map(a=>a==null?void 0:a[1]())));await Promise.all(n)}function Oe(e){var o;p=e.state;const i=document.querySelector("style[data-sveltekit]");i&&i.remove(),q=e.props.page,V=new t.root({target:r,props:{...e.props,stores:F,components:l},hydrate:!0}),Ie(D);const n={from:null,to:{params:p.params,route:{id:((o=p.route)==null?void 0:o.id)??null},url:new URL(location.href)},willUnload:!1,type:"enter",complete:Promise.resolve()};g.after_navigate.forEach(a=>a(n)),j=!0}async function Y({url:e,params:i,branch:n,status:o,error:a,route:c,form:m}){let v="never";for(const y of n)(y==null?void 0:y.slash)!==void 0&&(v=y.slash);e.pathname=Ze(e.pathname,v),e.search=e.search;const b={type:"loaded",state:{url:e,params:i,branch:n,error:a,route:c},props:{constructors:Et(n).map(y=>y.node.component)}};m!==void 0&&(b.props.form=m);let _={},L=!q,A=0;for(let y=0;y<Math.max(n.length,p.branch.length);y+=1){const w=n[y],O=p.branch[y];(w==null?void 0:w.data)!==(O==null?void 0:O.data)&&(L=!0),w&&(_={..._,...w.data},L&&(b.props[`data_${A}`]=_),A+=1)}return(!p.url||e.href!==p.url.href||p.error!==a||m!==void 0&&m!==q.form||L)&&(b.props.page={error:a,params:i,route:{id:(c==null?void 0:c.id)??null},status:o,url:new URL(e),form:m??null,data:L?_:q.data}),b}async function de({loader:e,parent:i,url:n,params:o,route:a,server_data_node:c}){var _,L,A;let m=null;const v={dependencies:new Set,params:new Set,parent:!1,route:!1,url:!1},b=await e();if((_=b.universal)!=null&&_.load){let P=function(...w){for(const O of w){const{href:$}=new URL(O,n);v.dependencies.add($)}};const y={route:new Proxy(a,{get:(w,O)=>(v.route=!0,w[O])}),params:new Proxy(o,{get:(w,O)=>(v.params.add(O),w[O])}),data:(c==null?void 0:c.data)??null,url:nt(n,()=>{v.url=!0}),async fetch(w,O){let $;w instanceof Request?($=w.url,O={body:w.method==="GET"||w.method==="HEAD"?void 0:await w.blob(),cache:w.cache,credentials:w.credentials,headers:w.headers,integrity:w.integrity,keepalive:w.keepalive,method:w.method,mode:w.mode,redirect:w.redirect,referrer:w.referrer,referrerPolicy:w.referrerPolicy,signal:w.signal,...O}):$=w;const C=new URL($,n);return P(C.href),C.origin===n.origin&&($=C.href.slice(n.origin.length)),j?ct($,C.href,O):st($,O)},setHeaders:()=>{},depends:P,parent(){return v.parent=!0,i()}};m=await b.universal.load.call(null,y)??null,m=m?await kt(m):null}return{node:b,loader:e,server:c,universal:(L=b.universal)!=null&&L.load?{type:"data",data:m,uses:v}:null,data:m??(c==null?void 0:c.data)??null,slash:((A=b.universal)==null?void 0:A.trailingSlash)??(c==null?void 0:c.slash)}}function Ue(e,i,n,o,a){if(B)return!0;if(!o)return!1;if(o.parent&&e||o.route&&i||o.url&&n)return!0;for(const c of o.params)if(a[c]!==p.params[c])return!0;for(const c of o.dependencies)if(E.some(m=>m(new URL(c))))return!0;return!1}function pe(e,i){return(e==null?void 0:e.type)==="data"?e:(e==null?void 0:e.type)==="skip"?i??null:null}async function he({id:e,invalidating:i,url:n,params:o,route:a}){if((h==null?void 0:h.id)===e)return h.promise;const{errors:c,layouts:m,leaf:v}=a,b=[...m,v];c.forEach(S=>S==null?void 0:S().catch(()=>{})),b.forEach(S=>S==null?void 0:S[1]().catch(()=>{}));let _=null;const L=p.url?e!==p.url.pathname+p.url.search:!1,A=p.route?a.id!==p.route.id:!1;let P=!1;const y=b.map((S,I)=>{var G;const R=p.branch[I],T=!!(S!=null&&S[0])&&((R==null?void 0:R.loader)!==S[1]||Ue(P,A,L,(G=R.server)==null?void 0:G.uses,o));return T&&(P=!0),T});if(y.some(Boolean)){try{_=await He(n,y)}catch(S){return ie({status:S instanceof te?S.status:500,error:await Z(S,{url:n,params:o,route:{id:a.id}}),url:n,route:a})}if(_.type==="redirect")return _}const w=_==null?void 0:_.nodes;let O=!1;const $=b.map(async(S,I)=>{var ge;if(!S)return;const R=p.branch[I],T=w==null?void 0:w[I];if((!T||T.type==="skip")&&S[1]===(R==null?void 0:R.loader)&&!Ue(O,A,L,(ge=R.universal)==null?void 0:ge.uses,o))return R;if(O=!0,(T==null?void 0:T.type)==="error")throw T;return de({loader:S[1],url:n,params:o,route:a,parent:async()=>{var De;const Te={};for(let me=0;me<I;me+=1)Object.assign(Te,(De=await $[me])==null?void 0:De.data);return Te},server_data_node:pe(T===void 0&&S[0]?{type:"skip"}:T??null,S[0]?R==null?void 0:R.server:void 0)})});for(const S of $)S.catch(()=>{});const C=[];for(let S=0;S<b.length;S+=1)if(b[S])try{C.push(await $[S])}catch(I){if(I instanceof Fe)return{type:"redirect",location:I.location};let R=500,T;if(w!=null&&w.includes(I))R=I.status??R,T=I.error;else if(I instanceof te)R=I.status,T=I.body;else{if(await F.updated.check())return await z(n);T=await Z(I,{params:o,url:n,route:{id:a.id}})}const G=await xe(S,C,c);return G?await Y({url:n,params:o,branch:C.slice(0,G.idx).concat(G.node),status:R,error:T,route:a}):await Ne(n,{id:a.id},T,R)}else C.push(void 0);return await Y({url:n,params:o,branch:C,status:200,error:null,route:a,form:i?void 0:null})}async function xe(e,i,n){for(;e--;)if(n[e]){let o=e;for(;!i[o];)o-=1;try{return{idx:o+1,node:{node:await n[e](),loader:n[e],data:{},server:null,universal:null}}}catch{continue}}}async function ie({status:e,error:i,url:n,route:o}){const a={};let c=null;if(t.server_loads[0]===0)try{const _=await He(n,[!0]);if(_.type!=="data"||_.nodes[0]&&_.nodes[0].type!=="data")throw 0;c=_.nodes[0]??null}catch{(n.origin!==location.origin||n.pathname!==location.pathname||x)&&await z(n)}const v=await de({loader:s,url:n,params:a,route:o,parent:()=>Promise.resolve({}),server_data_node:pe(c)}),b={node:await d(),loader:d,universal:null,server:null,data:null};return await Y({url:n,params:a,branch:[v,b],status:e,error:i,route:null})}function X(e,i){if(ve(e,J))return;const n=se(e);for(const o of f){const a=o.exec(n);if(a)return{id:e.pathname+e.search,invalidating:i,route:o,params:et(a),url:e}}}function se(e){return Qe(e.pathname.slice(J.length)||"/")}function je({url:e,type:i,intent:n,delta:o}){let a=!1;const c=Be(p,n,e,i);o!==void 0&&(c.navigation.delta=o);const m={...c.navigation,cancel:()=>{a=!0,c.reject(new Error("navigation was cancelled"))}};return U||g.before_navigate.forEach(v=>v(m)),a?null:c}async function ce({url:e,scroll:i,keepfocus:n,redirect_chain:o,details:a,type:c,delta:m,nav_token:v={},accepted:b,blocked:_}){var $,C,S;const L=X(e,!1),A=je({url:e,type:c,delta:m,intent:L});if(!A){_();return}const P=D;b(),U=!0,j&&F.navigating.set(A.navigation),ae=v;let y=L&&await he(L);if(!y){if(ve(e,J))return await z(e);y=await Ne(e,{id:null},await Z(new Error(`Not found: ${e.pathname}`),{url:e,params:{},route:{id:null}}),404)}if(e=(L==null?void 0:L.url)||e,ae!==v)return A.reject(new Error("navigation was aborted")),!1;if(y.type==="redirect")if(o.length>10||o.includes(e.pathname))y=await ie({status:500,error:await Z(new Error("Redirect loop"),{url:e,params:{},route:{id:null}}),url:e,route:{id:null}});else return re(new URL(y.location,e).href,{},[...o,e.pathname],v),!1;else(($=y.props.page)==null?void 0:$.status)>=400&&await F.updated.check()&&await z(e);if(E.length=0,B=!1,N=!0,Ee(P),Ae(P),(C=y.props.page)!=null&&C.url&&y.props.page.url.pathname!==e.pathname&&(e.pathname=(S=y.props.page)==null?void 0:S.url.pathname),a){const I=a.replaceState?0:1;if(a.state[M]=D+=I,history[a.replaceState?"replaceState":"pushState"](a.state,"",e),!a.replaceState){let R=D+1;for(;Q[R]||K[R];)delete Q[R],delete K[R],R+=1}}if(h=null,j){p=y.state,y.props.page&&(y.props.page.url=e);const I=(await Promise.all(g.on_navigate.map(R=>R(A.navigation)))).filter(R=>typeof R=="function");if(I.length>0){let R=function(){g.after_navigate=g.after_navigate.filter(T=>!I.includes(T))};I.push(R),g.after_navigate.push(...I)}V.$set(y.props)}else Oe(y);const{activeElement:w}=document;if(await ye(),k){const I=e.hash&&document.getElementById(decodeURIComponent(e.hash.slice(1)));i?scrollTo(i.x,i.y):I?I.scrollIntoView():scrollTo(0,0)}const O=document.activeElement!==w&&document.activeElement!==document.body;!n&&!O&&Se(),k=!0,y.props.page&&(q=y.props.page),U=!1,c==="popstate"&&Ie(D),A.fulfil(void 0),g.after_navigate.forEach(I=>I(A.navigation)),F.navigating.set(null),N=!1}async function Ne(e,i,n,o){return e.origin===location.origin&&e.pathname===location.pathname&&!x?await ie({status:o,error:n,url:e,route:i}):await z(e)}function z(e){return location.href=e.href,new Promise(()=>{})}function Ye(){let e;u.addEventListener("mousemove",c=>{const m=c.target;clearTimeout(e),e=setTimeout(()=>{o(m,2)},20)});function i(c){o(c.composedPath()[0],1)}u.addEventListener("mousedown",i),u.addEventListener("touchstart",i,{passive:!0});const n=new IntersectionObserver(c=>{for(const m of c)m.isIntersecting&&(oe(se(new URL(m.target.href))),n.unobserve(m.target))},{threshold:0});function o(c,m){const v=Me(c,u);if(!v)return;const{url:b,external:_,download:L}=_e(v,J);if(_||L)return;const A=le(v);if(!A.reload)if(m<=A.preload_data){const P=X(b,!1);P&&Pe(P)}else m<=A.preload_code&&oe(se(b))}function a(){n.disconnect();for(const c of u.querySelectorAll("a")){const{url:m,external:v,download:b}=_e(c,J);if(v||b)continue;const _=le(c);_.reload||(_.preload_code===Ve.viewport&&n.observe(c),_.preload_code===Ve.eager&&oe(se(m)))}}g.after_navigate.push(a),a()}function Z(e,i){return e instanceof te?e.body:t.hooks.handleError({error:e,event:i})??{message:i.route.id!=null?"Internal Error":"Not Found"}}return{after_navigate:e=>{we(()=>(g.after_navigate.push(e),()=>{const i=g.after_navigate.indexOf(e);g.after_navigate.splice(i,1)}))},before_navigate:e=>{we(()=>(g.before_navigate.push(e),()=>{const i=g.before_navigate.indexOf(e);g.before_navigate.splice(i,1)}))},on_navigate:e=>{we(()=>(g.on_navigate.push(e),()=>{const i=g.on_navigate.indexOf(e);g.on_navigate.splice(i,1)}))},disable_scroll_handling:()=>{(N||!j)&&(k=!1)},goto:(e,i={})=>re(e,i,[]),invalidate:e=>{if(typeof e=="function")E.push(e);else{const{href:i}=new URL(e,location.href);E.push(n=>n.href===i)}return Re()},invalidate_all:()=>(B=!0,Re()),preload_data:async e=>{const i=new URL(e,Ce(document)),n=X(i,!1);if(!n)throw new Error(`Attempted to preload a URL that does not belong to this app: ${i}`);await Pe(n)},preload_code:oe,apply_action:async e=>{if(e.type==="error"){const i=new URL(location.href),{branch:n,route:o}=p;if(!o)return;const a=await xe(p.branch.length,n,o.errors);if(a){const c=await Y({url:i,params:p.params,branch:n.slice(0,a.idx).concat(a.node),status:e.status??500,error:e.error,route:o});p=c.state,V.$set(c.props),ye().then(Se)}}else e.type==="redirect"?re(e.location,{invalidateAll:!0},[]):(V.$set({form:null,page:{...q,form:e.data,status:e.status}}),await ye(),V.$set({form:e.data}),e.type==="success"&&Se())},_start_router:()=>{var i;history.scrollRestoration="manual",addEventListener("beforeunload",n=>{let o=!1;if(Le(),!U){const a=Be(p,void 0,null,"leave"),c={...a.navigation,cancel:()=>{o=!0,a.reject(new Error("navigation was cancelled"))}};g.before_navigate.forEach(m=>m(c))}o?(n.preventDefault(),n.returnValue=""):history.scrollRestoration="auto"}),addEventListener("visibilitychange",()=>{document.visibilityState==="hidden"&&Le()}),(i=navigator.connection)!=null&&i.saveData||Ye(),u.addEventListener("click",n=>{var P;if(n.button||n.which!==1||n.metaKey||n.ctrlKey||n.shiftKey||n.altKey||n.defaultPrevented)return;const o=Me(n.composedPath()[0],u);if(!o)return;const{url:a,external:c,target:m,download:v}=_e(o,J);if(!a)return;if(m==="_parent"||m==="_top"){if(window.parent!==window)return}else if(m&&m!=="_self")return;const b=le(o);if(!(o instanceof SVGAElement)&&a.protocol!==location.protocol&&!(a.protocol==="https:"||a.protocol==="http:")||v)return;if(c||b.reload){je({url:a,type:"link"})?U=!0:n.preventDefault();return}const[L,A]=a.href.split("#");if(A!==void 0&&L===location.href.split("#")[0]){if(p.url.hash===a.hash){n.preventDefault(),(P=o.ownerDocument.getElementById(A))==null||P.scrollIntoView();return}if(H=!0,Ee(D),e(a),!b.replace_state)return;H=!1,n.preventDefault()}ce({url:a,scroll:b.noscroll?ee():null,keepfocus:b.keep_focus??!1,redirect_chain:[],details:{state:{},replaceState:b.replace_state??a.href===location.href},accepted:()=>n.preventDefault(),blocked:()=>n.preventDefault(),type:"link"})}),u.addEventListener("submit",n=>{if(n.defaultPrevented)return;const o=HTMLFormElement.prototype.cloneNode.call(n.target),a=n.submitter;if(((a==null?void 0:a.formMethod)||o.method)!=="get")return;const m=new URL((a==null?void 0:a.hasAttribute("formaction"))&&(a==null?void 0:a.formAction)||o.action);if(ve(m,J))return;const v=n.target,{keep_focus:b,noscroll:_,reload:L,replace_state:A}=le(v);if(L)return;n.preventDefault(),n.stopPropagation();const P=new FormData(v),y=a==null?void 0:a.getAttribute("name");y&&P.append(y,(a==null?void 0:a.getAttribute("value"))??""),m.search=new URLSearchParams(P).toString(),ce({url:m,scroll:_?ee():null,keepfocus:b??!1,redirect_chain:[],details:{state:{},replaceState:A??m.href===location.href},nav_token:{},accepted:()=>{},blocked:()=>{},type:"form"})}),addEventListener("popstate",async n=>{var o;if((o=n.state)!=null&&o[M]){if(n.state[M]===D)return;const a=K[n.state[M]];if(p.url.href.split("#")[0]===location.href.split("#")[0]){K[D]=ee(),D=n.state[M],scrollTo(a.x,a.y);return}const c=n.state[M]-D;await ce({url:new URL(location.href),scroll:a,keepfocus:!1,redirect_chain:[],details:null,accepted:()=>{D=n.state[M]},blocked:()=>{history.go(-c)},type:"popstate",delta:c})}else if(!H){const a=new URL(location.href);e(a)}}),addEventListener("hashchange",()=>{H&&(H=!1,history.replaceState({...history.state,[M]:++D},"",location.href))});for(const n of document.querySelectorAll("link"))n.rel==="icon"&&(n.href=n.href);addEventListener("pageshow",n=>{n.persisted&&F.navigating.set(null)});function e(n){p.url=n,F.page.set({...q,url:n}),F.page.notify()}},_hydrate:async({status:e=200,error:i,node_ids:n,params:o,route:a,data:c,form:m})=>{x=!0;const v=new URL(location.href);({params:o={},route:a={id:null}}=X(v,!1)||{});let b;try{const _=n.map(async(P,y)=>{const w=c[y];return w!=null&&w.uses&&(w.uses=We(w.uses)),de({loader:t.nodes[P],url:v,params:o,route:a,parent:async()=>{const O={};for(let $=0;$<y;$+=1)Object.assign(O,(await _[$]).data);return O},server_data_node:pe(w)})}),L=await Promise.all(_),A=f.find(({id:P})=>P===a.id);if(A){const P=A.layouts;for(let y=0;y<P.length;y++)P[y]||L.splice(y,0,void 0)}b=await Y({url:v,params:o,branch:L,status:e,error:i,form:m,route:A??null})}catch(_){if(_ instanceof Fe){await z(new URL(_.location,location.href));return}b=await ie({status:_ instanceof te?_.status:500,error:await Z(_,{url:v,params:o,route:a}),url:v,route:a})}Oe(b)}}}async function He(t,r){const f=new URL(t);f.pathname=ot(t.pathname),t.pathname.endsWith("/")&&f.searchParams.append(At,"1"),f.searchParams.append(Rt,r.map(d=>d?"1":"0").join(""));const s=await fe(f.href);if(!s.ok)throw new te(s.status,await s.json());return new Promise(async d=>{var p;const u=new Map,E=s.body.getReader(),l=new TextDecoder;function h(x){return bt(x,{Promise:j=>new Promise((k,N)=>{u.set(j,{fulfil:k,reject:N})})})}let g="";for(;;){const{done:x,value:j}=await E.read();if(x&&!g)break;for(g+=!j&&g?`
`:l.decode(j);;){const k=g.indexOf(`
`);if(k===-1)break;const N=JSON.parse(g.slice(0,k));if(g=g.slice(k+1),N.type==="redirect")return d(N);if(N.type==="data")(p=N.nodes)==null||p.forEach(U=>{(U==null?void 0:U.type)==="data"&&(U.uses=We(U.uses),U.data=h(U.data))}),d(N);else if(N.type==="chunk"){const{id:U,data:H,error:B}=N,V=u.get(U);u.delete(U),B?V.reject(h(B)):V.fulfil(h(H))}}}})}function We(t){return{dependencies:new Set((t==null?void 0:t.dependencies)??[]),params:new Set((t==null?void 0:t.params)??[]),parent:!!(t!=null&&t.parent),route:!!(t!=null&&t.route),url:!!(t!=null&&t.url)}}function Se(){const t=document.querySelector("[autofocus]");if(t)t.focus();else{const r=document.body,f=r.getAttribute("tabindex");r.tabIndex=-1,r.focus({preventScroll:!0,focusVisible:!1}),f!==null?r.setAttribute("tabindex",f):r.removeAttribute("tabindex");const s=getSelection();if(s&&s.type!=="None"){const d=[];for(let u=0;u<s.rangeCount;u+=1)d.push(s.getRangeAt(u));setTimeout(()=>{if(s.rangeCount===d.length){for(let u=0;u<s.rangeCount;u+=1){const E=d[u],l=s.getRangeAt(u);if(E.commonAncestorContainer!==l.commonAncestorContainer||E.startContainer!==l.startContainer||E.endContainer!==l.endContainer||E.startOffset!==l.startOffset||E.endOffset!==l.endOffset)return}s.removeAllRanges()}})}}}function Be(t,r,f,s){var h,g;let d,u;const E=new Promise((p,x)=>{d=p,u=x});return E.catch(()=>{}),{navigation:{from:{params:t.params,route:{id:((h=t.route)==null?void 0:h.id)??null},url:t.url},to:f&&{params:(r==null?void 0:r.params)??null,route:{id:((g=r==null?void 0:r.route)==null?void 0:g.id)??null},url:f},willUnload:!r,type:s,complete:E},fulfil:d,reject:u}}async function Ot(t,r,f){const s=It(t,r);Xe({client:s}),f?await s._hydrate(f):s.goto(location.href,{replaceState:!0}),s._start_router()}export{Ot as start};
