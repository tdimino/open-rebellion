
int __cdecl FUN_005597e0(int param_1,int param_2)

{
  if (param_1 == 1) {
    if (param_2 == 0) {
      return DAT_006bb3f0;
    }
    return DAT_006bb408;
  }
  if (param_1 == 2) {
    if (param_2 == 0) {
      return DAT_006bb440;
    }
    return DAT_006bb4ac;
  }
  if (param_1 != 3) {
    return param_2;
  }
  if (param_2 == 0) {
    return (((DAT_00661a88 - DAT_006bb3f0) - DAT_006bb440) - DAT_006bb408) - DAT_006bb4ac;
  }
  return 0;
}

