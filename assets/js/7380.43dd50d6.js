"use strict";(self.webpackChunknebula_docs=self.webpackChunknebula_docs||[]).push([[7380],{7380:(t,e,a)=>{a.d(e,{diagram:()=>y});var r=a(6141),i=a(4094),n=a(9982),d=a(9219),o=a(6979);a(6287),a(8488),a(2829);let s=0;const l=function(t){let e=t.id;return t.type&&(e+="<"+(0,o.x)(t.type)+">"),e},p=function(t,e,a,r){const{displayText:i,cssStyle:n}=e.getDisplayDetails(),d=t.append("tspan").attr("x",r.padding).text(i);""!==n&&d.attr("style",e.cssStyle),a||d.attr("dy",r.textHeight)},c=function(t,e,a,r){o.l.debug("Rendering class ",e,a);const i=e.id,n={id:i,label:e.id,width:0,height:0},d=t.append("g").attr("id",r.db.lookUpDomId(i)).attr("class","classGroup");let s;s=e.link?d.append("svg:a").attr("xlink:href",e.link).attr("target",e.linkTarget).append("text").attr("y",a.textHeight+a.padding).attr("x",0):d.append("text").attr("y",a.textHeight+a.padding).attr("x",0);let c=!0;e.annotations.forEach((function(t){const e=s.append("tspan").text("\xab"+t+"\xbb");c||e.attr("dy",a.textHeight),c=!1}));let g=l(e);const h=s.append("tspan").text(g).attr("class","title");c||h.attr("dy",a.textHeight);const f=s.node().getBBox().height;let x,u,y;if(e.members.length>0){x=d.append("line").attr("x1",0).attr("y1",a.padding+f+a.dividerMargin/2).attr("y2",a.padding+f+a.dividerMargin/2);const t=d.append("text").attr("x",a.padding).attr("y",f+a.dividerMargin+a.textHeight).attr("fill","white").attr("class","classText");c=!0,e.members.forEach((function(e){p(t,e,c,a),c=!1})),u=t.node().getBBox()}if(e.methods.length>0){y=d.append("line").attr("x1",0).attr("y1",a.padding+f+a.dividerMargin+u.height).attr("y2",a.padding+f+a.dividerMargin+u.height);const t=d.append("text").attr("x",a.padding).attr("y",f+2*a.dividerMargin+u.height+a.textHeight).attr("fill","white").attr("class","classText");c=!0,e.methods.forEach((function(e){p(t,e,c,a),c=!1}))}const b=d.node().getBBox();var m=" ";e.cssClasses.length>0&&(m+=e.cssClasses.join(" "));const k=d.insert("rect",":first-child").attr("x",0).attr("y",0).attr("width",b.width+2*a.padding).attr("height",b.height+a.padding+.5*a.dividerMargin).attr("class",m).node().getBBox().width;return s.node().childNodes.forEach((function(t){t.setAttribute("x",(k-t.getBBox().width)/2)})),e.tooltip&&s.insert("title").text(e.tooltip),x&&x.attr("x2",k),y&&y.attr("x2",k),n.width=k,n.height=b.height+a.padding+.5*a.dividerMargin,n},g=function(t,e,a,r,n){const d=function(t){switch(t){case n.db.relationType.AGGREGATION:return"aggregation";case n.db.relationType.EXTENSION:return"extension";case n.db.relationType.COMPOSITION:return"composition";case n.db.relationType.DEPENDENCY:return"dependency";case n.db.relationType.LOLLIPOP:return"lollipop"}};e.points=e.points.filter((t=>!Number.isNaN(t.y)));const l=e.points,p=(0,i.n8j)().x((function(t){return t.x})).y((function(t){return t.y})).curve(i.qrM),c=t.append("path").attr("d",p(l)).attr("id","edge"+s).attr("class","relation");let g,h,f="";r.arrowMarkerAbsolute&&(f=window.location.protocol+"//"+window.location.host+window.location.pathname+window.location.search,f=f.replace(/\(/g,"\\("),f=f.replace(/\)/g,"\\)")),1==a.relation.lineType&&c.attr("class","relation dashed-line"),10==a.relation.lineType&&c.attr("class","relation dotted-line"),"none"!==a.relation.type1&&c.attr("marker-start","url("+f+"#"+d(a.relation.type1)+"Start)"),"none"!==a.relation.type2&&c.attr("marker-end","url("+f+"#"+d(a.relation.type2)+"End)");const x=e.points.length;let u,y,b,m,k=o.u.calcLabelPosition(e.points);if(g=k.x,h=k.y,x%2!=0&&x>1){let t=o.u.calcCardinalityPosition("none"!==a.relation.type1,e.points,e.points[0]),r=o.u.calcCardinalityPosition("none"!==a.relation.type2,e.points,e.points[x-1]);o.l.debug("cardinality_1_point "+JSON.stringify(t)),o.l.debug("cardinality_2_point "+JSON.stringify(r)),u=t.x,y=t.y,b=r.x,m=r.y}if(void 0!==a.title){const e=t.append("g").attr("class","classLabel"),i=e.append("text").attr("class","label").attr("x",g).attr("y",h).attr("fill","red").attr("text-anchor","middle").text(a.title);window.label=i;const n=i.node().getBBox();e.insert("rect",":first-child").attr("class","box").attr("x",n.x-r.padding/2).attr("y",n.y-r.padding/2).attr("width",n.width+r.padding).attr("height",n.height+r.padding)}if(o.l.info("Rendering relation "+JSON.stringify(a)),void 0!==a.relationTitle1&&"none"!==a.relationTitle1){t.append("g").attr("class","cardinality").append("text").attr("class","type1").attr("x",u).attr("y",y).attr("fill","black").attr("font-size","6").text(a.relationTitle1)}if(void 0!==a.relationTitle2&&"none"!==a.relationTitle2){t.append("g").attr("class","cardinality").append("text").attr("class","type2").attr("x",b).attr("y",m).attr("fill","black").attr("font-size","6").text(a.relationTitle2)}s++},h=function(t,e,a,r){o.l.debug("Rendering note ",e,a);const i=e.id,n={id:i,text:e.text,width:0,height:0},d=t.append("g").attr("id",i).attr("class","classGroup");let s=d.append("text").attr("y",a.textHeight+a.padding).attr("x",0);const l=JSON.parse(`"${e.text}"`).split("\n");l.forEach((function(t){o.l.debug(`Adding line: ${t}`),s.append("tspan").text(t).attr("class","title").attr("dy",a.textHeight)}));const p=d.node().getBBox(),c=d.insert("rect",":first-child").attr("x",0).attr("y",0).attr("width",p.width+2*a.padding).attr("height",p.height+l.length*a.textHeight+a.padding+.5*a.dividerMargin).node().getBBox().width;return s.node().childNodes.forEach((function(t){t.setAttribute("x",(c-t.getBBox().width)/2)})),n.width=c,n.height=p.height+l.length*a.textHeight+a.padding+.5*a.dividerMargin,n};let f={};const x=function(t){const e=Object.entries(f).find((e=>e[1].label===t));if(e)return e[0]},u={draw:function(t,e,a,r){const s=(0,o.c)().class;f={},o.l.info("Rendering diagram "+t);const l=(0,o.c)().securityLevel;let p;"sandbox"===l&&(p=(0,i.Ltv)("#i"+e));const u="sandbox"===l?(0,i.Ltv)(p.nodes()[0].contentDocument.body):(0,i.Ltv)("body"),y=u.select(`[id='${e}']`);var b;(b=y).append("defs").append("marker").attr("id","extensionStart").attr("class","extension").attr("refX",0).attr("refY",7).attr("markerWidth",190).attr("markerHeight",240).attr("orient","auto").append("path").attr("d","M 1,7 L18,13 V 1 Z"),b.append("defs").append("marker").attr("id","extensionEnd").attr("refX",19).attr("refY",7).attr("markerWidth",20).attr("markerHeight",28).attr("orient","auto").append("path").attr("d","M 1,1 V 13 L18,7 Z"),b.append("defs").append("marker").attr("id","compositionStart").attr("class","extension").attr("refX",0).attr("refY",7).attr("markerWidth",190).attr("markerHeight",240).attr("orient","auto").append("path").attr("d","M 18,7 L9,13 L1,7 L9,1 Z"),b.append("defs").append("marker").attr("id","compositionEnd").attr("refX",19).attr("refY",7).attr("markerWidth",20).attr("markerHeight",28).attr("orient","auto").append("path").attr("d","M 18,7 L9,13 L1,7 L9,1 Z"),b.append("defs").append("marker").attr("id","aggregationStart").attr("class","extension").attr("refX",0).attr("refY",7).attr("markerWidth",190).attr("markerHeight",240).attr("orient","auto").append("path").attr("d","M 18,7 L9,13 L1,7 L9,1 Z"),b.append("defs").append("marker").attr("id","aggregationEnd").attr("refX",19).attr("refY",7).attr("markerWidth",20).attr("markerHeight",28).attr("orient","auto").append("path").attr("d","M 18,7 L9,13 L1,7 L9,1 Z"),b.append("defs").append("marker").attr("id","dependencyStart").attr("class","extension").attr("refX",0).attr("refY",7).attr("markerWidth",190).attr("markerHeight",240).attr("orient","auto").append("path").attr("d","M 5,7 L9,13 L1,7 L9,1 Z"),b.append("defs").append("marker").attr("id","dependencyEnd").attr("refX",19).attr("refY",7).attr("markerWidth",20).attr("markerHeight",28).attr("orient","auto").append("path").attr("d","M 18,7 L9,13 L14,7 L9,1 Z");const m=new d.T({multigraph:!0});m.setGraph({isMultiGraph:!0}),m.setDefaultEdgeLabel((function(){return{}}));const k=r.db.getClasses(),w=Object.keys(k);for(const i of w){const t=k[i],e=c(y,t,s,r);f[e.id]=e,m.setNode(e.id,e),o.l.info("Org height: "+e.height)}r.db.getRelations().forEach((function(t){o.l.info("tjoho"+x(t.id1)+x(t.id2)+JSON.stringify(t)),m.setEdge(x(t.id1),x(t.id2),{relation:t},t.title||"DEFAULT")}));r.db.getNotes().forEach((function(t){o.l.debug(`Adding note: ${JSON.stringify(t)}`);const e=h(y,t,s,r);f[e.id]=e,m.setNode(e.id,e),t.class&&t.class in k&&m.setEdge(t.id,x(t.class),{relation:{id1:t.id,id2:t.class,relation:{type1:"none",type2:"none",lineType:10}}},"DEFAULT")})),(0,n.Zp)(m),m.nodes().forEach((function(t){void 0!==t&&void 0!==m.node(t)&&(o.l.debug("Node "+t+": "+JSON.stringify(m.node(t))),u.select("#"+(r.db.lookUpDomId(t)||t)).attr("transform","translate("+(m.node(t).x-m.node(t).width/2)+","+(m.node(t).y-m.node(t).height/2)+" )"))})),m.edges().forEach((function(t){void 0!==t&&void 0!==m.edge(t)&&(o.l.debug("Edge "+t.v+" -> "+t.w+": "+JSON.stringify(m.edge(t))),g(y,m.edge(t),m.edge(t).relation,s,r))}));const L=y.node().getBBox(),v=L.width+40,E=L.height+40;(0,o.i)(y,E,v,s.useMaxWidth);const M=`${L.x-20} ${L.y-20} ${v} ${E}`;o.l.debug(`viewBox ${M}`),y.attr("viewBox",M)}},y={parser:r.p,db:r.d,renderer:u,styles:r.s,init:t=>{t.class||(t.class={}),t.class.arrowMarkerAbsolute=t.arrowMarkerAbsolute,r.d.clear()}}}}]);