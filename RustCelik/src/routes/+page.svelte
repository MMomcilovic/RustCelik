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
					<h1 class="text-green-500">Картица прочитана!</h1>
				{:else}
					<h1>Чекам податке са картице!</h1>
				{/if}
			{:else}
				<h1 class="text-yellow-400">Картица није детектована! Убаците картицу.</h1>
			{/if}
		{:else}
			<h1 class="text-yellow-400">Читач није пронађен. Повежите валидан читач.</h1>
		{/if}
	</div>
</div>
