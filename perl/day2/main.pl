use warnings;
use strict;

my $input_file = "input.sf";

open( my $input_fh, "<", $input_file ) || die "could not open file";

my @lines = <$input_fh>;

my %max_cubes = ( "red" => 12, "green" => 13, "blue" => 14 );
my ( $part1, $part2 ) = 0;

foreach my $line (@lines) {
    my @cubes       = reverse $line =~ /(\d+) (blue|red|green)/g;
    my $line_number = ( $line =~ /Game (\d+)/ )[0];

    my $all    = 1;
    my %record = ( "red" => 0, "green" => 0, "blue" => 0 );

    while ( my ( $i, $j ) = splice( @cubes, 0, 2 ) ) {
        if ( $max_cubes{$i} < $j ) { $all = 0; }
        if ( $record{$i} < $j ) { $record{$i} = $j; }
    }

    $part2 += $record{"red"} * $record{"green"} * $record{"blue"};

    if ($all) { $part1 += $line_number; }
}

print "part1: $part1\npart2: $part2\n";

close($input_fh);
