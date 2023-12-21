use strict;
use warnings;

sub intersection {
    my ($a1, $a2) = @_; my @a1 = @$a1; my @a2 = @$a2;
    my %a1; my %a2;
    $a1{$_} = 1 foreach (@a1);
    for my $e (@a2) {
        if ($a1{$e}) {
            $a2{$e} = 1;
        }
    }
    return keys %a2;
}

my $input_file = "input.sf";

open(my $input_fh, "<", $input_file) || die "could not open file $input_file";
my @lines = map {[map {[$_ =~ /(\d+)/g]} split /\||:/ , $_]} <$input_fh>;
my @copies = (1) x @lines;
my $part1 = 0; my $part2;

for my $line (@lines) {
    my ($id, $card, $numbers) = @{$line};
    $id = $id->[0];
    my $same = intersection($card, $numbers);
    if ($same > 0) {
        $part1 += 2 ** ($same - 1);
    }
    
    for my $add (0..$same - 1) {
        $copies[$id + $add] += $copies[$id - 1];
    }
}

map {$part2 += $_} @copies;

print "part1: $part1\npart2: $part2\n";

close($input_fh);
