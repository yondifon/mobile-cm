import 'dart:convert';
import 'dart:io';

import 'package:mobile_cm/mobile_cm.dart';
import 'package:test/test.dart';

void main() {
  final cases = jsonDecode(
    File('../spec/cases.json').readAsStringSync(),
  ) as List<dynamic>;

  for (final testCase in cases) {
    final input = testCase['input'] as String;
    final operatorName = testCase['operator'] as String?;
    final expected =
        operatorName == null ? null : Operator.values.byName(operatorName);

    test('check(${jsonEncode(input)}) -> ${jsonEncode(operatorName)}', () {
      expect(check(input), expected);
      expect(isMtn(input), expected == Operator.mtn);
      expect(isOrange(input), expected == Operator.orange);
      expect(isNexttel(input), expected == Operator.nexttel);
      expect(isCamtel(input), expected == Operator.camtel);
    });
  }
}
