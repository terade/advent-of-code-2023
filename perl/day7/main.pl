use 5.38.1;
use strict;
use warnings;
use diagnostics;

use lib '.';
use CamelCards;

my $input_file = "input.sf";

open( my $input_fh, "<", $input_file ) or die "cant open file \"$input_file\"\n";
my @input       = <$input_fh>;
my $camel_cards = CamelCards->from( \@input );
my ( $part1, $part2 ) = $camel_cards->solve();

say "part1: $part1\npart2: $part2";

close($input_fh);

