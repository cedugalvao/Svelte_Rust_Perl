<script>
	let nome = '';
	let cpf = '';
	let email = '';
	let senha = '';
	let confirmarSenha = '';

	async function submitForm() {
		const response = await fetch('http://localhost:8000', {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({ nome, cpf, email, senha, confirmarSenha })
		});

		if (!response.ok) {
			const message = `An error has occured: ${response.status}`;
			throw new Error(message);
		}

		const data = await response.json();
		console.log(data);
	}
</script>

<div class="w-full">
	<div class="w-full flex">
		<div class="bg-[#3687A2] w-full flex pr-0 justify-center items-center h-screen-80">
			<div class="max-w-[1216px] w-full">
				<h1 class="text-[#F7AD19] text-4xl font-bold mb-4 text-left">Olá, seja bem vindo!</h1>
				<p class="text-white my-5 text-justify">
					Para abrir uma conta grátis e abrir sua conta com segurança, precisamos saber mais sobre
					você. Vamos começar?
				</p>
				<form on:submit|preventDefault={submitForm} class="grid grid-cols-2 gap-4">
					<div class="campo">
						<label for="nome" class="text-white pl-3">Nome</label>
						<input
							bind:value={nome}
							type="text"
							name="nome"
							id="nome"
							class="w-full rounded-full p-3 mb-4"
							placeholder="Seu nome completo é ..."
							required
						/>
					</div>
					<div class="campo">
						<label for="cpf" class="text-white pl-3">CPF</label>
						<input
							bind:value={cpf}
							type="text"
							name="cpf"
							id="cpf"
							maxlength="11"
							class="w-full rounded-full p-3 mb-4"
							placeholder="Dígitos do CPF sem pontos"
							required
						/>
					</div>
					<div class="campo">
						<label for="email" class="text-white pl-3">Email</label>
						<input
							bind:value={email}
							type="email"
							name="email"
							id="email"
							class="w-full rounded-full p-3 mb-4"
							placeholder="Seu email pessoal"
							required
						/>
					</div>
					<div class="campo">
						<label for="senha" class="text-white pl-3">Senha</label>
						<input
							bind:value={senha}
							type="password"
							name="senha"
							id="senha"
							class="w-full rounded-full p-3 mb-4"
							placeholder="Uma senha ultrasecreta"
							required
						/>
					</div>
					<div class="campo">
						<label for="confirmarSenha" class="text-white pl-3">Confirmar Senha</label>
						<input
							bind:value={confirmarSenha}
							type="password"
							name="confirmarSenha"
							id="confirmarSenha"
							class="w-full rounded-full p-3 mb-4"
							placeholder="Confirme sua senha ultrasecreta"
							required
						/>
					</div>
					<div class="flex justify-center items-center">
						<button
							type="submit"
							class="bg-[#F2A007] rounded-full p-[13px] mt-[7px] w-full text-white"
							>Cadastrar</button
						>
					</div>
				</form>
			</div>
		</div>
	</div>
</div>
