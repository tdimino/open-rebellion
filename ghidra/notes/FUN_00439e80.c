
void FUN_00439e80(int param_1)

{
  if (param_1 != 0x119) {
    if (param_1 == 0x11b) {
      DAT_006b28b0 = DAT_006b28b0 ^ 0x1000;
    }
    else if (param_1 == 0x11e) {
      DAT_006b28b0 = DAT_006b28b0 ^ 0x8000;
      return;
    }
  }
  return;
}

