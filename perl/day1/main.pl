use warnings;
use strict;

my $file_name = "input.sf";
open( my $input_fh, "<", $file_name ) || die "cant open input file";

my @lines = <$input_fh>;

my %tr = (
    "one"   => 1,
    "two"   => 2,
    "three" => 3,
    "four"  => 4,
    "five"  => 5,
    "six"   => 6,
    "seven" => 7,
    "eight" => 8,
    "nine"  => 9
);

my $re = join '|', keys %tr;
my ( $part1, $part2 ) = 0;

my @test = ( map { [ $_ =~ /\d/g ] } @lines );
foreach my $line (@lines) {
    my @digits1 = ( $line =~ /\d/g );
    my @digits2 = map { $_ =~ m/\d/ ? $_ : $tr{$_} } ( $line =~ /(?=(\d|$re))/g );
    $part1 += $digits1[0] . $digits1[-1];
    $part2 += $digits2[0] . $digits2[-1];
}

print "part1: ", $part1, "\npart2: $part2\n";

close($input_fh);

