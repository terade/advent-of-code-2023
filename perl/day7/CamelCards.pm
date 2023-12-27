package CamelCards;

use 5.38.1;
use strict;
use warnings;
use diagnostics;
use Data::Dumper;

my %tr = (
    2 => 2,
    3 => 3,
    4 => 4,
    5 => 5,
    6 => 6,
    7 => 7,
    8 => 8,
    9 => 9,
    T => 10,
    J => 11,
    Q => 12,
    K => 13,
    A => 14,
);

sub acmp {
    my ( $a, $b ) = @_;

    for my $i ( 0 .. ( @$a - 1 ) ) {
        if ( ( $a->[$i] || 0 ) <=> ( $b->[$i] || 0 ) ) {
            return ( $a->[$i] || 0 ) <=> ( $b->[$i] || 0 );
        }
    }

    if ( @$a < @$b ) {
        return -1;
    }

    0;
}

sub from {
    my ( $class, $input ) = @_;
    my @hands = (
        map {
            chomp;
            my @a = ( split / / );
            [ [ ( map { $tr{$_} } ( split //, $a[0] ) ) ], $a[1] ];
        } ( grep /\S/, @$input )
    );

    bless { hands => \@hands, }, $class;
}

sub frequencies {
    my $a = shift;
    my %o = ();
    map { $o{$_}++ } @$a;
    \%o;
}

sub cmp_rule1 {
    my ( $ar, $br ) = @_;
    my @a_occ = sort { $b <=> $a } ( values %{ frequencies($ar) } );
    my @b_occ = sort { $b <=> $a } ( values %{ frequencies($br) } );

    if ( !acmp( \@a_occ, \@b_occ ) ) {
        return acmp( $ar, $br );
    }

    acmp( \@a_occ, \@b_occ );
}

sub cmp_rule2 {
    my ( $ar, $br ) = @_;
    $ar = [
        map {
            if   ( $_ == $tr{"J"} ) {1}
            else                    {$_}
        } @$ar
    ];
    $br = [
        map {
            if   ( $_ == $tr{"J"} ) {1}
            else                    {$_}
        } @$br
    ];
    my %af = %{ frequencies($ar) };
    my %bf = %{ frequencies($br) };
    my $aj = $af{1} || 0;
    my $bj = $bf{1} || 0;
    $af{1} = 0;
    $bf{1} = 0;
    my @a_occ = sort { $b <=> $a } ( values %af );
    my @b_occ = sort { $b <=> $a } ( values %bf );
    $a_occ[0] += $aj;
    $b_occ[0] += $bj;

    if ( !acmp( \@a_occ, \@b_occ ) ) {
        return acmp( $ar, $br );
    }

    acmp( \@a_occ, \@b_occ );
}

sub solve {
    my $self   = shift;
    my @cards1 = sort { cmp_rule1( $a->[0], $b->[0] ) } @{ $self->{hands} };
    my @cards2 = sort { cmp_rule2( $a->[0], $b->[0] ) } @{ $self->{hands} };
    my $part1  = 0;
    my $part2  = 0;

    for my $i ( 0 .. ( @cards1 - 1 ) ) {
        $part1 += ( $i + 1 ) * $cards1[$i]->[1];
        $part2 += ( $i + 1 ) * $cards2[$i]->[1];
    }

    ( $part1, $part2 );
}

1;

