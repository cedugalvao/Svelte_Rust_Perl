<script>
	export let textButton = '';
	export let classList = '';
    import { navigate } from 'svelte-navigator';

    function next(){
        const formulario = document.getElementsByClassName('form-boleto');
        var input = document.getElementsByClassName('input-boleto');
        const confirmaBoleto = document.getElementsByClassName('confirma-boleto');


        if(input[0].value == '12345'){
            confirmaBoleto[0].classList.remove('hidden');
            formulario[0].classList.add('hidden');
        }
        //Se o boleto não existir
        else{
            alert('Boleto não encontrado, verifique o código de barras e tente novamente.');
            input[0].value = '';
        }
        console.log(input[0].value);
    }

</script>

<button type="submit" on:click={next} class="{classList} bg-transparent border text-white font-semibold py-2 px-4 rounded-3xl">
	{textButton}
</button>
