use strict;
use warnings;
use diagnostics;

use lib '.';
use Engine;

my $input_file = "input.sf";

open( my $input_fh, "<", $input_file );
my @lines = <$input_fh>;

my $engine = Engine->from( \@lines );
my ( $part1, $part2 ) = $engine->solve();

print "part1: $part1\npart2: $part2\n";

close($input_file);

