<script lang="ts">
	import UserInfo from '../components/UserInfo.svelte';
	import { listen } from '@tauri-apps/api/event';
	import type { Event } from '@tauri-apps/api/event';
	import type { personalId } from '../types/personalId.type';
	import { onMount } from 'svelte';

	let info: personalId = {
		DocRegNo: '010831794',
		IssuingDate: '20.07.2020',
		ExpiryDate: '20.07.2030',
		IssuingAuthority: 'ПС У ТИТЕЛУ',
		PersonalNumber: '2501001774510',
		Surname: 'MOMЧИЛОВИЋ',
		GivenName: 'МИЛАДИН',
		ParentGivenName: 'BOŽO',
		Sex: 'M',
		PlaceOfBirth: 'LJUBOVIJA',
		CommunityOfBirth: 'LJUBOVIJA',
		StateOfBirth: 'REPUBLIKA SRBIJA',
		DateOfBirth: '25.01.2001',
		State: 'SRB',
		Community: 'TITEL',
		Place: 'TITEL',
		Street: 'SVETOZARA MILETIĆA',
		HouseNumber: '096',
		AddressDate: '20.07.2020.',
		Image: ''
	};

	let infoRead = false;
	let cardInserted = false;
	let readerFound = false;

	onMount(async () => {
		await listen('card_info', (e: Event<any>) => {
			if (e.payload === 'No reader found') {
				readerFound = false;
				return;
			}
			readerFound = true;
			if (e.payload === 'No card inserted') {
				cardInserted = false;
				return;
			}
			cardInserted = true;
			infoRead = true;
			info = JSON.parse(e.payload);
		});
	});
</script>

<div class="h-[100%]">
	<div class="text-center h-[100%]">
		<UserInfo {info} />
		{#if readerFound}
			{#if cardInserted}
				{#if infoRead}
					<h1>Card info read</h1>
				{:else}
					<h1>Waiting for card info</h1>
				{/if}
			{:else}
				<h1>No card detected! Insert your card.</h1>
			{/if}
		{:else}
			<h1>Reader not found. Connect valid reader.</h1>
		{/if}
	</div>
</div>
