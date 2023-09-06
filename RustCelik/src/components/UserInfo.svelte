<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { onMount } from 'svelte';
	import type { personalId } from '../types/personalId.type';

	export let info: personalId;
</script>

<div class="h-[100%] w-full text-sm p-2">
	<div class="flex h-[20%] w-full m-1 col-span-3">
		<div>
			<img
				src="https://www.egmo.org/countries/country14/flag2.png"
				width="100px"
				height="30px"
				alt="Slika"
				class="p-1 shadow-sm shadow-gray-400"
			/>
		</div>
		<div class="p-2 flex flex-1 space-x-4 text-left">
			<div>
				<p>Република Србија</p>
				<p>ЛИЧНА КАРТА</p>
			</div>
			<div>
				<p>Republika Srbija</p>
				<p>LIČNA KARTA</p>
			</div>
			<div>
				<p>Republic of Serbia</p>
				<p>IDENTITY CARD</p>
			</div>
		</div>
		<div class="p-3 pr-3">
			<button class="shadow-md p-1 shadow-gray-600">Stampa podataka</button>
		</div>
	</div>
	<div class="h-[50%] flex w-full p-2 space-x-2">
		<div class="p-1 w-[660px] h-[340px]">
			{#if info.Image !== ''}
				<img
					src="data:image/png;base64,{info.Image}"
					alt="Slika"
					class="w-full h-full rounded-sm"
				/>
			{:else}
				<div class="w-full h-full rounded-sm bg-gray-300" />
			{/if}
		</div>
		<div class="m-1 p-2 border border-gray-400 rounded-lg w-[900px] space-y-1">
			<h3>Podaci o građaninu</h3>
			<div class="flex w-fulls text-left space-x-2">
				<label for="first_name" class="block mb-1 text-sm text-gray-900 my-auto w-[25%]"
					>First name</label
				>
				<input
					type="text"
					id="first_name"
					class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg block w-[70%] p-0.5 pl-2 cursor-text"
					value={info.GivenName}
					disabled
				/>
			</div>
			<div class="flex w-fulls text-left space-x-2">
				<label for="first_name" class="block mb-1 text-sm text-gray-900 my-auto w-[25%]"
					>Last name</label
				>
				<input
					type="text"
					id="first_name"
					class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg block w-[70%] p-0.5 pl-2 cursor-text"
					value={info.Surname}
					disabled
				/>
			</div>
			<div class="flex w-fulls text-left space-x-2">
				<label for="first_name" class="block mb-1 text-sm text-gray-900 my-auto w-[25%]"
					>Parent name</label
				>
				<input
					type="text"
					id="first_name"
					class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg block w-[70%] p-0.5 pl-2 cursor-text"
					value={info.ParentGivenName}
					disabled
				/>
			</div>
			<div class="flex w-fulls text-left space-x-2">
				<label for="first_name" class="block mb-1 text-sm text-gray-900 my-auto w-[25%]"
					>Date of birth</label
				>
				<input
					type="text"
					id="first_name"
					class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg block w-[70%] p-0.5 pl-2 cursor-text"
					value={info.DateOfBirth}
					disabled
				/>
			</div>
			<div>
				<label for="first_name" class="block mb-1 text-sm text-gray-900 my-auto"
					>Place, State and Country of birth</label
				>
				<input
					type="text"
					id="first_name"
					class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg block w-[95%] p-0.5 pl-2 mx-2 cursor-text"
					value={`${info.PlaceOfBirth}, ${info.CommunityOfBirth}, ${info.StateOfBirth}`}
					disabled
				/>
			</div>
			<div>
				<label for="first_name" class="block mb-1 text-sm text-gray-900 my-auto"
					>Living address</label
				>
				<input
					type="text"
					id="first_name"
					class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg block w-[95%] p-0.5 pl-2 mx-2 cursor-text"
					value={`${info.Community}, ${info.Place}, ${info.Street} ${info.HouseNumber}`}
					disabled
				/>
			</div>
			<div class="flex w-fulls text-left space-x-2">
				<label for="first_name" class="block mb-1 text-sm text-gray-900 my-auto w-[45%]"
					>Date of address update</label
				>
				<input
					type="text"
					id="first_name"
					class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg block w-[50%] p-0.5 pl-2 cursor-text"
					value={info.AddressDate}
					disabled
				/>
			</div>
			<div class="flex">
				<div class="flex">
					<label for="first_name" class="block mb-1 text-sm text-gray-900 my-auto w-[25%] ml-[-3px]"
						>JMBG</label
					>
					<input
						type="text"
						id="first_name"
						class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg block w-[70%] p-0.5 pl-2 cursor-text"
						value={info.PersonalNumber}
						disabled
					/>
				</div>
				<div class="flex">
					<label for="first_name" class="block mb-1 text-sm text-gray-900 my-auto w-[25%] mr-[-1px]"
						>Sex</label
					>
					<input
						type="text"
						id="first_name"
						class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg block w-[70%] p-0.5 pl-2 cursor-text"
						value="M"
						disabled
					/>
				</div>
			</div>
		</div>
	</div>
	<div class="flex mx-3 p-2 border border-gray-400 rounded-lg col-span-4">
		<div>
			<label for="first_name" class="block mb-1 text-sm text-gray-900 my-auto"
				>Issuing Authority</label
			>
			<input
				type="text"
				id="first_name"
				class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg block w-[90%] p-0.5 pl-2 mx-2 cursor-text"
				value="ПС У ТИТЕЛУ"
				disabled
			/>
		</div>
		<div>
			<label for="first_name" class="block mb-1 text-sm text-gray-900 my-auto">Document No.</label>
			<input
				type="text"
				id="first_name"
				class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg block w-[95%] p-0.5 pl-2 mx-2 cursor-text"
				value={info.DocRegNo}
				disabled
			/>
		</div>
		<div>
			<label for="first_name" class="block mb-1 text-sm text-gray-900 my-auto">Issuing Date</label>
			<input
				type="text"
				id="first_name"
				class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg block w-[95%] p-0.5 pl-2 mx-2 cursor-text"
				value={info.IssuingDate}
				disabled
			/>
		</div>
		<div>
			<label for="first_name" class="block mb-1 text-sm text-gray-900 my-auto"
				>Expiration Date</label
			>
			<input
				type="text"
				id="first_name"
				class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg block w-[90%] p-0.5 pl-2 mx-2 cursor-text"
				value={info.ExpiryDate}
				disabled
			/>
		</div>
	</div>
	<div class="h-[17%]">
		<p class="text-xs text-left p-1">
			1. У чипу личне карте, подаци о имену и презимену имаоца личне карте исписани су на
			националном писму онако како су исписани на самом обрасцу личне карте, док су остали подаци
			исписани латиничким писмом. <br />
			2. Ако се име лица састоји од две речи чија је укупна дужина између 20 и 30 карактера или презимена
			од две речи чија је укупна дужина између 30 и 36 карактера, у чипу личне карте издате пре 18.08.2014.
			године, друга реч у имену или презимену скраћује се на прва два карактера.
		</p>
	</div>
</div>
