<script lang="ts">
	import UserInfo from '../components/UserInfo.svelte';
	import { listen } from '@tauri-apps/api/event';
	import type { Event } from '@tauri-apps/api/event';
	import type { personalId } from '../types/personalId.type';
	import { onMount } from 'svelte';
    import { fade } from 'svelte/transition';

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
		Entrance: '',
		Floor: '',
		AppartmentNumber: '',
		Image: ''
	};

	let infoRead = false;
	let cardInserted = false;
	let readerFound = false;
	let readingError = false;
	let docSaved = false;

	onMount(async () => {
		await listen('card_info', (e: Event<any>) => {
			if (e.payload === 'Doc saved!') {
				docSaved = true;
				return;
			}
			docSaved = false;
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
			if (e.payload === 'Error while reading card!') {
				readingError = true;
				return;
			}
			readingError = false;
			info = JSON.parse(e.payload);
			info.DateOfBirth = formatDate(info.DateOfBirth);
			info.AddressDate = formatDate(info.AddressDate);
			info.IssuingDate = formatDate(info.IssuingDate);
			info.ExpiryDate = formatDate(info.ExpiryDate);
			infoRead = true;
		});
	});

	function formatDate(date: string) {
		return `${date.slice(0, 2)}.${date.slice(2, 4)}.${date.slice(4)}.`;
	}
</script>

<div class="h-[100%]">
	<div class="text-center h-[100%]">
		<UserInfo {info} />
		{#if docSaved}
		<h1 class="text-green-500" in:fade>Dokument sacuvan u <i>Documents</i> folderu!</h1>
		{:else if readerFound}
			{#if cardInserted}
				{#if infoRead}
					<h1 class="text-green-500" in:fade>Картица прочитана!</h1>
				{:else}
					<h1>Чекам податке са картице!</h1>
				{/if}
			{:else if readingError}
				<h1 class="text-yellow-400" in:fade>Грешка приликом читања картице! Поново убаците картицу.</h1>
			{:else}
				<h1 class="text-yellow-400" in:fade>Картица није детектована! Убаците картицу.</h1>
			{/if}
		{:else}
			<h1 class="text-yellow-400" in:fade>Читач није пронађен. Повежите валидан читач.</h1>
		{/if}
	</div>
</div>
