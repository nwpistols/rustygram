<script lang="ts">
	import { store } from '$lib/store';

	import { postCreate, uploadPhoto } from '$lib/store/services/posts';

	import { writable } from 'svelte/store';

	const createPostForm = writable<{
		files: FileList;
		avatar: string;
		caption: string;
	}>({
		files: null,
		avatar: '',
		caption: ''
	});

	const onFileSelected = (
		e: Event & {
			currentTarget: EventTarget & HTMLInputElement;
		}
	) => {
		let image = e.currentTarget.files[0];
		let reader = new FileReader();
		reader.readAsDataURL(image);
		reader.onload = (e) => {
			$createPostForm.avatar = e.target.result as string;
		};
	};

	let loading = false;

	async function onSubmit() {
		loading = true;
		try {
			const { data } = await store.dispatch(uploadPhoto.initiate($createPostForm.files[0]));
			console.log(data);
			await store.dispatch(
				postCreate.initiate({ caption: $createPostForm.caption, url: data.filepath })
			);
		} catch (error) {
			console.error(error);
			loading = false;
		} finally {
			createPostForm.set({ files: null, avatar: '', caption: '' });
			loading = false;
		}
	}
</script>

<h2 class="text-3xl my-3">Create Post</h2>

<form class="w-full" on:submit|preventDefault={onSubmit}>
	<div class="mb-3 form-control">
		<label for="caption" class="label label-text"> Caption </label>
		<textarea
			name="caption"
			class="textarea h-32 w-full"
			bind:value={$createPostForm.caption}
			required
		/>
	</div>

	<div>
		{#if $createPostForm.avatar}
			<img src={$createPostForm.avatar} alt="post uplaod" />
		{/if}

		<input
			class="my-3 text-center"
			type="file"
			name="img"
			id="img"
			accept=".jpg, .jpeg, .png"
			on:change={onFileSelected}
			bind:files={$createPostForm.files}
			required
		/>
	</div>

	<button type="submit" class="btn btn-primary mt-3 w-full">Submit</button>
</form>

<style>
	img {
		max-width: 400px;
	}
</style>
