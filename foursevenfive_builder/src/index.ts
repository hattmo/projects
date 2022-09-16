import fillPdf from "fill-pdf";
import { join as p } from "path";
import fs from "fs";
import csv from "csv-parser";
import progressbar from "progress";

const pdfTemplatePath = p(__dirname, "../af475.pdf");

//These variables are used in the construction of the PDF object adjust as the verbiage changes.
//Thes can be added and removed as necessary

const pq = [
  "- Proven leader--demonstrated effective leadership guiding a diverse group of peers through challenging environments",
  "- Contributor to flight's cohesion--applied interpersonal skills to resolve conflicts and foster collaborative relationships",
  "- Critical thinker--employed intellectual skill when analyzing and evaluating information during team decision making",
  "- Effective communicator--executed proficiency across all facets of both formal and informal communication methods",
  "- Strategic focus--comprehended the roles of national security strategy through global, regional and cultural situations",
  "- Professional--displayed values of integrity, service and excellence through a demanding, multi-discipline curriculum",
];

const aa = [
  "***SOS Class taught in Virtual In-Residence Remote format due to ongoing COVID-19 pandemic***",
  "",
];

const thinkTankBullet = [
  "- Competitively selected as one of 24 SOS Think Tank students among 836 peers to analyze a critical DoD Arctic issue",
  "  -- Displayed advanced critical thinking; fused 350+ hrs of tm research--presented actionable COAs/fixes to Sr Ldrsp",
].join("\n");
const auarBullet =
  "- Selected as one of 84 SOS Researchers among 836 peers to conduct advanced research on strategic topic of interest";
const ideasBullet =
  "  -- Displayed advanced critical thinking; ID'd emergent technologies for future fight; briefed results to Sr Leadership";
const adwarUIBullet =
  "  -- Displayed advanced critical thinking; collected empirical data; improved wargame to support 4K students annually";
const afmakerBullet =
  "  -- Advanced 3D-printing practice & tools; developed use cases/prototypes ISO AF ops; results shared w/ Sr Leaders";
const arAppBullet =
  "  -- Displayed critical thinking; created proof-of-concept for augmented reality app in military education to SOS/CC";
const sosAppBullet =
  "  -- Displayed critical thinking, proj management; applied modern agile practices ISO SOS/CC feedback tool/edu enhance";
const mafBullet =
  "  -- Displayed advanced critical thinking; ID'd key issues to solve logistics under fire; briefed results to Sr Leadership";
const isrBullet =
  "  -- Pioneered ISR readiness innovation; ID'd issues/proposed COAs--advanced HAF A2/6 ISR Dominance 2030 vision";
const cafBullet =
  "  -- Displayed advanced critical thinking; ID'd issues/bolstered C2 TTPs for 2030 fight; briefed results to Sr Leadership";
const sofBullet =
  "  -- Displayed advanced critical thinking; developed novel SOF operating concepts-- solved future security challenges";
const diversityBullet =
  "  -- Displayed advanced critical thinking; ID'd diversity/inclusion solutions to remove barriers--briefed HAF/A1";
const spaceBullet =
  "  -- Displayed critical thinking; ID'd sources, networks & SMEs; provided Space Force recommendations to SAF/SP";
const raceBullet =
  "  -- Displayed advanced critical thinking; ID'd issues to address racial injustice in the AF--briefed Sr Leadership";
const uxBullet =
  "  -- Displayed critical thinking; ID'd UX issues, dev'd COAs/prototype ISO common AF apps--briefed SAF/CXO";
const aiBullet =
  "  -- Displayed critical thinking; applied AI to solve problems; delivered products to JAIC, HQ USSF/A5, & AFRL";
const interBullet =
  "  -- Applied critical thinking; researched PACAF int'l partnerships; briefed security cooperation plan to IOS & SOS/CC";

//These array will store 1 PDFFields object for each row in the input.csv.
const inputVals: PDFFields[] = [];

//This function generates the PDFs for each item in the input array.  this should not need to be edited
const genPDF = (input: PDFFields): Promise<void> =>
  new Promise((res, rej) => {
    fillPdf.generatePdf(input, pdfTemplatePath, [], (err: any, data: any) => {
      if (err) {
        rej(err);
      } else {
        fs.writeFileSync(
          p(__dirname, "../out", `${input.studentName}_${input.SSN}.pdf`),
          data
        );
        res();
      }
    });
  });

//This function reads the pdf one line at a time.
fs.createReadStream(p(__dirname, "../input.csv"))
  .pipe(csv())
  .on("data", (data) => {

//Extract each column of data into a variable for this row.
    const {
      studentName,
      SSN,
      rank,
      thinktank,
      auar,
      ideas,
      adwarUI,
      afmaker,
      arApp,
      sosApp,
      maf,
      isr,
      caf,
      sof,
      diversity,
      space,
      race,
      ux,
      ai,
      inter,
    } = data;

//thisAA is a array of strings that represents the Academic accomplishments box.
//push a string to it to make a new bullet in the box
    const thisAA = [...aa];

    if (thinktank) {
      thisAA.push(thinkTankBullet);
    }
    if (auar) {
      thisAA.push(auarBullet);
    }
    if (inter) {
      thisAA.push(interBullet);
    }
    if (ideas) {
      thisAA.push(ideasBullet);
    }
    if (adwarUI) {
      thisAA.push(adwarUIBullet);
    }
    if (afmaker) {
      thisAA.push(afmakerBullet);
    }
    if (arApp) {
      thisAA.push(arAppBullet);
    }
    if (sosApp) {
      thisAA.push(sosAppBullet);
    }
    if (maf) {
      thisAA.push(mafBullet);
    }
    if (isr) {
      thisAA.push(isrBullet);
    }
    if (caf) {
      thisAA.push(cafBullet);
    }
    if (sof) {
      thisAA.push(sofBullet);
    }
    if (diversity) {
      thisAA.push(diversityBullet);
    }
    if (space) {
      thisAA.push(spaceBullet);
    }
    if (race) {
      thisAA.push(raceBullet);
    }
    if (ux) {
      thisAA.push(uxBullet);
    }
    if (ai) {
      thisAA.push(aiBullet);
    }

//after the AA array is built.  build an "input" object.  this object needs to be
//type PDFFields.  the keys of this object match the input fields of the PDF.
    const input: PDFFields = {
      studentName,
      SSN,
      rank,
      afsc: "92S0",
      org: "Squadron Officer School (AETC), Maxwell AFB AL",
      from: "16 Nov 2020",
      to: "18 Dec 2020",
      weeks: "5",
      annual: "Off",
      final: "Yes",
      directed: "Off",
      schoolName: "Squadron Officer School (AETC), Maxwell AFB AL",
      courseName: "Squadron Officer School (Resident Course) - PDS Code Q",
      degree: "",
      notCompleted: "Off",
      dg: "Off",
      noDg: "Yes",
      dgCriteria: "",
      academicAccomplishments: thisAA.join("\n"),
      professionalQualities: pq.join("\n"),
      otherComments: "",
      evalName: [
        "Bob Johnson, Colonel, USAF",
        "Squadron Officer School (AETC)",
        "Maxwell AFB AL",
      ].join("\n"),
      evalTitle: "Commandant",
      evalSSN: "1234",
      evalDate: "18 Dec 2020",
      evalSig: "",
    };

//add the pdf input object to an arry
    inputVals.push(input);
  })
//once every row of the csv is read call genPDF for each item in input vals
//I added a progress bar because this takes a long time
  .on("end", () => {
    var bar = new progressbar("[:bar] :current/:total", {
      total: inputVals.length,
      width: 30,
      complete: "#",
      incomplete: " ",
    });
    (async () => {
      for (let input of inputVals) {
        bar.tick();
        await genPDF(input);
      }
    })();
  });


//type definition for the inputs of the pdf.  if the input names change, change it here.

type PDFFields = {
  studentName: string;
  SSN: string;
  rank: string;
  afsc: string;
  org: string;
  from: string;
  to: string;
  weeks: string;
  annual: "Yes" | "Off";
  final: "Yes" | "Off";
  directed: "Yes" | "Off";
  schoolName: string;
  courseName: string;
  degree: string;
  notCompleted: "Yes" | "Off";
  dg: "Yes" | "Off";
  noDg: "Yes" | "Off";
  dgCriteria: string;
  academicAccomplishments: string;
  professionalQualities: string;
  otherComments: string;
  evalName: string;
  evalTitle: string;
  evalSSN: string;
  evalDate: string;
  evalSig: string;
};
