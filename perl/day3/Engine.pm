package Engine;

use strict;
use warnings;
use diagnostics;

use Data::Dumper;

sub get_adjacent {
    my ($pos) = @_;
    my ( $i, $j ) = @$pos;
    my @res = ();

    for my $ii ( -1 .. 1 ) {
        for my $jj ( -1 .. 1 ) {
            push @res, [ $i + $ii, $j + $jj ];
        }
    }

    return @res;
}

sub get_part_numbers {
    my ( $self, $part ) = @_;
    my @adj = get_adjacent($part);
    my @res = ();

    for my $elem (@adj) {
        my ( $i, $j ) = @$elem;
        my $number = $self->{part_numbers}->{"$i,$j"};
        if ($number) {
            push @res, $number;
        }
    }

    return @res;
}

sub parse_line {
    my ( $line_number, $line ) = @_;
    $line =~ s/\s+$//;
    my %pos_part_numbers = ();
    my @numbers          = $line =~ /(\d+)/g;
    my @parts            = ();
    my $count            = 0;

    while ( $line =~ /(\d+)/g ) {
        for my $e ( $-[0] .. $+[0] - 1 ) {
            $pos_part_numbers{"$line_number,$e"} = [ $numbers[$count], 3**$line_number * 5**$count ];
        }
        $count++;
    }
    $count = 0;
    while ( $line =~ /([^0-9\.])/g ) {
        push @parts, [ $line_number, $-[0] ];
    }

    return ( \%pos_part_numbers, \@parts );
}

sub from {
    my ( $class, $lines ) = @_;
    my @lines        = @$lines;
    my %part_numbers = ();
    my @parts        = ();
    my $line_number  = 0;

    for my $line (@lines) {
        my ( $part_numbers, $parts ) = parse_line( $line_number++, $line );
        %part_numbers = ( %part_numbers, %$part_numbers );
        @parts        = ( @parts,        @$parts );
    }

    my $self = bless {
        part_numbers => \%part_numbers,
        parts        => \@parts,
    }, $class;
}

sub solve {
    my ($self) = @_;
    my ( $part1, $part2 ) = ( 0, 0 );

    for my $elem ( @{ $self->{parts} } ) {
        my @numbers = $self->get_part_numbers($elem);
        my %uniq    = ();
        for my $j (@numbers) {
            my ( $m, $n ) = @$j;
            $uniq{$n} = $m;
        }
        map { $part1 += $_ } ( values %uniq );
        if ( values %uniq == 2 ) {
            my $prod = 1;
            map { $prod *= $_ } ( values %uniq );
            $part2 += $prod;
        }
    }

    return ( $part1, $part2 );
}

1;

