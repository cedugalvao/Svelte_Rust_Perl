<script>
	export let textButton = '';
	export let classList = '';
	export let rota = '';
	import { navigate } from 'svelte-navigator';
</script>

<a href="{rota}" class=" {classList} bg-transparent border text-white font-semibold py-2 px-4 rounded-3xl">
	{textButton}
</a>
