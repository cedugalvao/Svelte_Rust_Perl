use lib './';  # Importa o módulo
use OxeBankAccountCreator;

use Data::UUID;

my $uuid_gen = Data::UUID->new;
my $account_id = $uuid_gen->create_str;

my $account_data = {
    #id => $account_id,  # Atribuir o UUID como ID da conta
    CPF => $ARGV[1],
    #account_number => '123456789',
    #balance => 1000,
    owner => $ARGV[0],
};

my $account_number = "C96C3B46-6D4E-1014-ABF7-D982D93F3D1D";
my $account_cpf = $ARGV[1];#$account_data->{CPF};

#print($account_cpf);

#create($account_data);
#credit($account_number, -1230);
#debit($account_number, -1230);
delete_account($account_cpf);

sub create {
    my($account_data) = @_;
    OxeBankAccountCreator::create_account($account_data);
}

sub delete_account {
    my ($account_cpf) = @_;
    OxeBankAccountCreator::delete_account($account_cpf);
}

sub credit {
    my($account_number, $amount) = @_;
    OxeBankAccountCreator::credit_account($account_number, $amount);
}

sub debit {
    my($account_number, $amount) = @_;
    OxeBankAccountCreator::debit_account($account_number, $amount);
}