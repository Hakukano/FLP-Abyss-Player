(self.webpackChunk_N_E=self.webpackChunk_N_E||[]).push([[405],{8312:function(e,t,l){(window.__NEXT_P=window.__NEXT_P||[]).push(["/",function(){return l(7756)}])},7756:function(e,t,l){"use strict";l.r(t),l.d(t,{default:function(){return P}});var a=l(5893),s=l(7294),i=l(2004),o=l(1415),n=l(44),c=l(8680),r=l(4033);function d(e){let[t,l]=(0,s.useState)(""),[i,o]=(0,s.useState)(1),[d,u]=(0,s.useState)(5),[h,m]=(0,s.useState)([]),[f,p]=(0,s.useState)(0),[y,w]=(0,s.useState)(!1),[x,g]=(0,s.useState)(new Map),_=async t=>{w(!0);try{let l=await e.playlistRemote.read({id:t});e.setPlaylistData(l)}finally{w(!1)}},j=async(l,a)=>{w(!0);try{let s=await e.playlistRemote.list({search:t,filter:[],offset:(l-1)*a,length:a});m(s.data),p(s.count);let i=new Map;for(let t of s.data)try{let l=await e.playlistLocal.read({remote_id:t.id});i.set(t.id,l.id)}catch(e){}g(i)}finally{w(!1)}},b=e=>{o(e),j(e,d)},v=(e,t)=>{u(e),o(t),j(t,e)};return(0,s.useEffect)(()=>{j(i,d)},[]),(0,a.jsxs)("div",{className:"w-full h-full flex flex-col space-y-1",children:[(0,a.jsx)("input",{className:"w-full",type:"text",placeholder:"Press ENTER to search...",value:t,style:{color:"#222"},onChange:e=>l(e.target.value),onKeyDown:e=>{"Enter"===e.key&&j(i,d)}}),(0,a.jsx)("div",{id:"scrollable-pl-remote",className:"h-full flex overflow-auto flex-col",children:(0,a.jsx)("div",{className:"w-full h-auto",children:(0,a.jsx)(n.ZP,{columns:[{omit:!0,name:"ID",selector:e=>e.id},{name:"Full Path",selector:e=>e.path,button:!0,width:"95%",cell:e=>(0,a.jsx)("button",{onClick:async()=>{await _(e.id)},children:e.path})},{omit:!0,name:"MIME Type",selector:e=>e.mime_type},{name:"",button:!0,width:"5%",cell:t=>x.has(t.id)?(0,a.jsx)(r.Z,{className:"w-full h-full",onClick:async()=>{try{await e.playlistLocal.delete({remote_id:t.id}),await j(i,d)}catch(e){}}}):(0,a.jsx)(c.Z,{className:"w-full h-full",onClick:async()=>{try{await e.playlistLocal.create({path:t.path,mime_type:t.mime_type,remote_id:t.id}),await j(i,d)}catch(e){}}})}],data:h,progressPending:y,pagination:!0,paginationServer:!0,paginationPerPage:d,paginationTotalRows:f,paginationRowsPerPageOptions:[5,10,15,20],onChangeRowsPerPage:v,onChangePage:b})})})]})}var u=l(8533);function h(e){let[t,l]=(0,s.useState)(""),[i,o]=(0,s.useState)([]),[n,c]=(0,s.useState)(!0),d=async t=>{{let l=await e.playlistLocal.read({id:t});e.setPlaylistData({id:l.remote_id,path:l.path,mime_type:l.mime_type})}},h=async l=>{let a=await e.playlistLocal.list({search:t,filter:[],offset:0,length:l});o(a.data),c(l<a.count)};return(0,s.useEffect)(()=>{h(i.length+20)},[]),(0,a.jsxs)("div",{className:"h-full flex flex-col space-y-1",children:[(0,a.jsx)("input",{className:"w-full",type:"text",placeholder:"Press ENTER to search...",value:t,style:{color:"#222"},onChange:e=>l(e.target.value),onKeyDown:e=>{"Enter"===e.key&&h(i.length+20)}}),(0,a.jsx)("div",{id:"scrollable-pl-local",className:"h-full flex overflow-auto flex-col",children:(0,a.jsx)(u.Z,{className:"flex flex-col",next:()=>{h(i.length+20)},hasMore:n,dataLength:i.length,loader:(0,a.jsx)("h4",{children:"Loading..."}),scrollableTarget:"scrollable-pl-local",children:i.map((t,l)=>(0,a.jsxs)("div",{className:"grid grid-rows-1 grid-cols-12 gap-1",children:[(0,a.jsx)("button",{className:"m-0.5 border border-solid border-black text-left col-span-11",style:{color:"#222"},onClick:async()=>{await d(t.id)},children:t.path}),(0,a.jsx)(r.Z,{className:"w-full col-span-1",style:{color:"#222"},onClick:async()=>{try{await e.playlistLocal.delete({id:t.id}),await h(i.length)}catch(e){}}})]},l))})})]})}var m=l(2913),f=l(7292),p=l(5073),y=l(682),w=l(6249);function x(e){return(0,a.jsx)("nav",{className:"w-full bg-white border-gray-200 dark:bg-gray-900",style:{height:"5vh"},children:(0,a.jsxs)("div",{className:"h-full flex items-center justify-between p-1",children:[(0,a.jsx)(m.Z,{className:"h-full",onClick:async()=>{let t=e.playlistData;if(t){let l=await e.playlistLocal.step({current:t.id,step:-1});e.setPlaylistData({id:l.remote_id,path:l.path,mime_type:l.mime_type})}}}),(0,a.jsx)(p.Z,{className:"h-full",onClick:()=>e.setShowPlaylistRemote(!e.showPlaylistRemote)}),(0,a.jsx)(y.Z,{className:"h-full",onClick:()=>e.setShowPlaylistLocal(!e.showPlaylistLocal)}),(0,a.jsx)(w.Z,{className:"h-full",onClick:()=>{!0===confirm("Purge local playlist?")&&e.playlistLocal.purge()}}),(0,a.jsx)(f.Z,{className:"h-full",onClick:async()=>{let t=e.playlistData;if(t){let l=await e.playlistLocal.step({current:t.id,step:1});e.setPlaylistData({id:l.remote_id,path:l.path,mime_type:l.mime_type})}}})]})})}class g{async read(e){let t=await fetch("/playlists/".concat(e.id));if(t.ok)return await t.json();throw{status:t.status,message:await t.text()}}async list(e){let t=await fetch("/playlists?".concat(new URLSearchParams({filter:e.filter.join(","),search:e.search,offset:e.offset.toString(),length:e.length.toString()})));if(t.ok)return await t.json();throw{status:t.status,message:await t.text()}}}var _=l(9632),j=l(9520);class b extends j.ZP{constructor(){super("flp-abyss-player-client"),this.version(1).stores({playlist_locals:"++id, data"})}}let v=new b;class N{async create(e){let t=await v.playlist_locals.get(1);if(!t)throw{cause:"db",message:"not found"};let l=t.data.length;return t.data.push(e),await v.playlist_locals.put(t,1),l}async read(e){let t=await v.playlist_locals.get(1);if(!t)throw{cause:"db",message:"not found"};if(void 0!==e.id){let l=t.data[e.id];return{id:e.id,path:l.path,mime_type:l.mime_type,remote_id:l.remote_id}}if(void 0!==e.remote_id){let l=t.data.map((e,t)=>({id:t,path:e.path,mime_type:e.mime_type,remote_id:e.remote_id})).find(t=>t.remote_id===e.remote_id);if(!l)throw{cause:"query",message:"not found"};return l}throw{cause:"query",message:"bad request"}}async delete(e){let t=await v.playlist_locals.get(1);if(!t)throw{cause:"db",message:"not found"};if(void 0!==e.id)t.data.splice(e.id,1);else if(void 0!==e.remote_id){let l=t.data.findIndex(t=>t.remote_id===e.remote_id);if(l<0)throw{cause:"query",message:"not found"};t.data.splice(l,1)}else throw{cause:"query",message:"bad request"};await v.playlist_locals.put(t,1)}async list(e){let t=await v.playlist_locals.get(1);if(!t)throw{cause:"db",message:"not found"};let l=t.data.map((e,t)=>({id:t,path:e.path,mime_type:e.mime_type,remote_id:e.remote_id}));if(e.search){let t=new _.Z(l,{keys:["path"]}).search(e.search),a=t.length,s=t.slice(e.offset,e.offset+e.length).map(e=>e.item);return{count:a,data:s}}{let t=l.length,a=l.slice(e.offset,e.offset+e.length);return{count:t,data:a}}}async count(){let e=await v.playlist_locals.get(1);if(!e)throw{cause:"db",message:"not found"};return e.data.length}async purge(){await v.playlist_locals.put({id:1,data:[]},1)}async step(e){let t=await v.playlist_locals.get(1);if(!t)throw{cause:"db",message:"not found"};let l=t.data.findIndex(t=>t.remote_id===e.current)||0,a=l+e.step,s=Math.min(Math.max(a,0),t.data.length-1),i=t.data[s];return{id:s,path:i.path,mime_type:i.mime_type,remote_id:i.remote_id}}constructor(){v.playlist_locals.get(1).then(e=>{e||v.playlist_locals.put({id:1,data:[]},1)})}}function P(){let e=new g,t=new N,[l,n]=(0,s.useState)(null),[c,r]=(0,s.useState)(!1),[u,m]=(0,s.useState)(!1);return(0,a.jsxs)("main",{className:"flex flex-col w-full h-full items-center justify-between p-2 space-y-2",children:[(0,a.jsx)(x,{playlistLocal:t,playlistData:l,setPlaylistData:n,showPlaylistRemote:c,setShowPlaylistRemote:r,showPlaylistLocal:u,setShowPlaylistLocal:m}),(0,a.jsx)("div",{className:"w-full h-full",children:l?l.mime_type.startsWith("image/")?(0,a.jsx)("div",{className:"w-full text-center",style:{height:"88vh"},children:(0,a.jsx)("img",{className:"w-full h-full align-middle",style:{objectFit:"contain",overflow:"hidden"},src:"/playlists/".concat(l.id,"/stream"),alt:l.path})}):(0,a.jsx)(i.Z,{width:"100%",height:"88vh",url:"/playlists/".concat(l.id,"/stream"),controls:!0}):(0,a.jsx)(a.Fragment,{})}),c?(0,a.jsxs)(a.Fragment,{children:[(0,a.jsx)("div",{className:"absolute justify-center items-center fixed inset-0 z-50 outline-none focus:outline-none",style:{width:"90%",height:"95vh",left:"5%"},children:(0,a.jsx)("div",{className:"relative w-full h-full",children:(0,a.jsxs)("div",{className:"h-full border-0 rounded-lg shadow-lg relative flex flex-col w-full bg-white outline-none focus:outline-none",children:[(0,a.jsxs)("div",{className:"flex items-start justify-between p-1 border-b border-solid border-slate-200 rounded-t",children:[(0,a.jsx)("h3",{className:"text-2xl text-black font-semibold",children:"Remote Playlist"}),(0,a.jsx)(o.Z,{className:"h-8 p-1 ml-auto bg-transparent border-0 text-black float-right text-3xl leading-none font-semibold outline-none focus:outline-none",onClick:()=>r(!1)})]}),(0,a.jsx)("div",{className:"relative p-2 w-full flex min-h-0",children:(0,a.jsx)(d,{playlistRemote:e,setPlaylistData:n,playlistLocal:t})})]})})}),(0,a.jsx)("div",{className:"opacity-25 fixed inset-0 z-40 bg-black"})]}):(0,a.jsx)(a.Fragment,{}),u?(0,a.jsxs)(a.Fragment,{children:[(0,a.jsx)("div",{className:"absolute justify-center items-center fixed inset-0 z-50 outline-none focus:outline-none",style:{width:"90%",height:"95vh",left:"5%"},children:(0,a.jsx)("div",{className:"relative w-full h-full",children:(0,a.jsxs)("div",{className:"h-full border-0 rounded-lg shadow-lg relative flex flex-col w-full bg-white outline-none focus:outline-none",children:[(0,a.jsxs)("div",{className:"flex items-start justify-between p-1 border-b border-solid border-slate-200 rounded-t",children:[(0,a.jsx)("h3",{className:"text-2xl text-black font-semibold",children:"Local Playlist"}),(0,a.jsx)(o.Z,{className:"h-8 p-1 ml-auto bg-transparent border-0 text-black float-right text-3xl leading-none font-semibold outline-none focus:outline-none",onClick:()=>m(!1)})]}),(0,a.jsx)("div",{className:"relative p-2 w-full flex min-h-0",children:(0,a.jsx)(h,{playlistLocal:t,setPlaylistData:n})})]})})}),(0,a.jsx)("div",{className:"opacity-25 fixed inset-0 z-40 bg-black"})]}):(0,a.jsx)(a.Fragment,{})]})}}},function(e){e.O(0,[872,774,888,179],function(){return e(e.s=8312)}),_N_E=e.O()}]);