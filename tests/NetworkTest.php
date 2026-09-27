<?php

use Malico\MobileCM\Network;

dataset('cases', function () {
    $cases = json_decode(file_get_contents(__DIR__ . '/../spec/cases.json'), true);

    foreach ($cases as $case) {
        yield $case['input'] => [$case['input'], $case['operator']];
    }
});

test('check names the operator of the number', function (string $input, ?string $operator) {
    expect(Network::check($input))->toBe($operator);

    expect([
        'mtn' => Network::isMTN($input),
        'orange' => Network::isOrange($input),
        'nexttel' => Network::isNexttel($input),
        'camtel' => Network::isCamtel($input),
    ])->toBe([
        'mtn' => $operator === 'mtn',
        'orange' => $operator === 'orange',
        'nexttel' => $operator === 'nexttel',
        'camtel' => $operator === 'camtel',
    ]);
})->with('cases');
