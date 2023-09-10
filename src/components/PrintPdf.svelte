<script lang="ts">
	import type { personalId } from '../types/personalId.type';

	let customData = 'This is custom data to be printed.';
	export let info: personalId;

	function printCustomData() {
		const printWindow = window.open(' ', ' ', 'width=600,height=400');
		printWindow!.document.open();
		printWindow!.document.write(`
<head>
    <title>Lična Karta</title>
</head>
<style>
@page { margin: 53pt 53pt 0 53pt }
body {
  font-family: Arial, sans-serif;
  font-size: 11pt;
  font-weight: lighter;
}
hr {
  size: 0;
  border-top: 0.2pt solid #000;
  margin: 0;
  padding: 0;
}
h1 {
  font-size: 13.2pt;
  line-height: 14pt;
  font-weight: normal;
  padding: 0 0 0 9.6pt;
  margin: 6.5pt 0 7pt 0;
}
h2 {
  font-size: 11pt;
  padding: 0 0 0 8.5pt;
  font-weight: normal;
  margin: 4pt 0 4pt 0;
}
table#top {
  margin: 3.5pt 0 5pt 7pt;
  padding: 0;
  border: 0;
}
table#bottom {
  margin: 4pt 0 4pt 7pt;
  padding: 0;
  border: 0;
}
tr {
  padding: 0;
  margin: 0;
}
td {
  margin: 0;
  padding: 0 0 11.1pt 0;
  vertical-align:top;
  font-size: 11pt;
  line-height: 12pt;
}
td.left {
  width: 127pt;
}
td.last {
  padding: 0;
}
p {
  padding: 0;
  margin: 0;
}
hr#second {
  margin-top: 2pt;
}
p.block {
  margin: 3.5pt 0 0 8pt;
}
hr.small {
  margin-top: 52pt;
}
p#small1 {
  font-size: 9pt;
  line-height: 9.8pt;
  margin-top: 7pt;
}
p#small2 {
  font-size: 9pt;
  line-height: 9.8pt;
  margin-bottom: 7pt;
}
img.profile {
  margin: 14pt 0 10pt 0;
  width: 158px;
  height: 211px;
  border: 1px solid #333;
}
</style>
<body>
    <hr/>
    <h1>ЧИТАЧ ЕЛЕКТРОНСКЕ ЛИЧНЕ КАРТЕ: ШТАМПА ПОДАТАКА</h1>
    <hr/>
    <img class="profile" src="data:image/png;base64,${info.Image}"/>
    <hr/>
    <h2>Подаци о грађанину</h2>
    <hr/>
    <table id="top">
        <tr>
            <td class="left">Презиме:</td>
            <td>${info.Surname}</td>
        </tr>
        <tr>
            <td class="left">Име:</td>
            <td>${info.GivenName}</td>
        </tr>
        <tr>
            <td class="left">Име једног родитеља:</td>
            <td>${info.ParentGivenName}</td>
        </tr>
        <tr>
            <td class="left">Датум рођења:</td>
            <td>${info.DateOfBirth}</td>
        </tr>
        <tr>
            <td class="left">Место рођења,<br/>општина i држава:</td>
            <td>${info.PlaceOfBirth}, ${info.CommunityOfBirth ? info.CommunityOfBirth : ''}, ${
			info.StateOfBirth
		}</td>
        </tr>
        <tr>
            <td class="left">Пребивалиште:</td>
            <td>${info.Place}, ${info.Community}, ${info.Street} ${info.HouseNumber}${
			info.Floor ? '/' + info.Floor : ''
		}${info.AppartmentNumber ? '/' + info.AppartmentNumber : ''}</td>
        </tr>
        <tr>
            <td class="left">Датум промене адресе:</td>
            <td>${info.AddressDate}</td>
        </tr>
        <tr>
            <td class="left">ЈМБГ:</td>
            <td>${info.PersonalNumber}</td>
        </tr>
        <tr>
            <td class="left last">Пол:</td>
            <td class="last">${info.Sex}</td>
        </tr>
    </table>
    <hr/>
    <h2>Подаци о документу</h2>
    <hr/>
    <table id="bottom">
        <tr>
            <td class="left">Документ издаје:</td>
            <td>${info.IssuingAuthority}</td>
        </tr>
        <tr>
            <td class="left">Број документа:</td>
            <td>${info.DocRegNo}</td>
        </tr>
        <tr>
            <td class="left">Датум издавања:</td>
            <td>${info.IssuingDate}</td>
        </tr>
        <tr>
            <td class="left last">Важи до:</td>
            <td class="last">${info.ExpiryDate}</td>
        </tr>
    </table>
    <hr/>
    <hr id="second"/>
    <p class="block">Датум штампе: ${new Date()
			.toLocaleDateString('en-GB')
			.replaceAll('/', '.')}.</p>
    <hr class="small"/>
    <p id="small1">
    1. У чипу личне карте, подаци о имену и презимену имаоца личне карте исписани су на
			националном писму онако како су исписани на самом обрасцу личне карте, док су остали подаци
			исписани латиничким писмом.
    </p>
    <p id="small2">
    2. Ако се име лица састоји од две речи чија је укупна дужина између 20 и 30 карактера или презимена
			од две речи чија је укупна дужина између 30 и 36 карактера, у чипу личне карте издате пре 18.08.2014.
			године, друга реч у имену или презимену скраћује се на прва два карактера.
    </p>
    <hr/>
</body>
</html>
	`);
		setTimeout(function () {
			printWindow!.print();
		}, 500);
		printWindow!.onfocus = function () {
			setTimeout(function () {
				printWindow!.close();
			}, 500);
		};
	}
</script>

<div>
	<button on:click={printCustomData} class="shadow-md p-1 shadow-gray-600">Штампа података</button>
</div>
