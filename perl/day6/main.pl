use 5.38.1;
use strict;
use warnings;
use POSIX;

sub zip {
    my ( $first, $second ) = @_;
    my @res = ();

    for my $count ( 0 .. ( scalar( ( scalar @$first ) < ( scalar @$second ) ) ? @$first : @$second ) - 1 ) {
        push @res, [ $first->[$count], $second->[$count] ];
    }

    @res;
}

sub abc_formula {
    my ( $a, $b, $c ) = @_;
    my $d = sqrt( ( $b**2 ) - ( 4 * $a * $c ) );
    ( ( -$b - $d ) / 2 * $a, ( -$b + $d ) / 2 * $a );
}

sub ways_to_win {
    my $records = shift;
    my @records = @$records;
    my $e       = 0.0001;
    my $res     = 1;

    for my $elem (@records) {
        my ( $time,  $distance ) = @$elem;
        my ( $first, $second )   = abc_formula( -1, $time, -$distance );
        my $temp = ( floor( $first - $e ) - ceil( $second + $e ) ) + 1;
        $res *= $temp;
    }

    $res;
}

my $input_file = "input.sf";

open( my $input_fh, "<", $input_file ) or die "could not open file $input_file";

my @lines = <$input_fh>;

my @time     = $lines[0] =~ /(\d+)/g;
my @distance = $lines[1] =~ /(\d+)/g;
my @record   = zip( \@time, \@distance );
my $part1    = ways_to_win( \@record );
my $part2    = ways_to_win( [ [ ( join '', @time ), ( join '', @distance ) ] ] );

say "part1: $part1\npart2: $part2";

