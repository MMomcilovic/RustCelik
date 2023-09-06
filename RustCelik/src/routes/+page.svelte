<script lang="ts">
	import UserInfo from '../components/UserInfo.svelte';
	import { listen } from '@tauri-apps/api/event';
	import type { Event } from '@tauri-apps/api/event';
	import type { personalId } from '../types/personalId.type';
	import { onMount } from 'svelte';

	let info: personalId = {
		DocRegNo: '',
		IssuingDate: '',
		ExpiryDate: '',
		IssuingAuthority: '',
		PersonalNumber: '',
		Surname: '',
		GivenName: '',
		ParentGivenName: '',
		Sex: '',
		PlaceOfBirth: '',
		CommunityOfBirth: '',
		StateOfBirth: '',
		DateOfBirth: '',
		State: '',
		Community: '',
		Place: '',
		Street: '',
		HouseNumber: '',
		AddressDate: '',
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
