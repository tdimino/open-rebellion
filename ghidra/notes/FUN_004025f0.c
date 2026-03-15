
undefined4 __cdecl FUN_004025f0(int param_1)

{
  undefined4 uVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00629298;
  local_c = ExceptionList;
  uVar1 = 0;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00619730();
  if (param_1 != 0x32000242) {
    FUN_00619730();
    if (param_1 != 0x30000240) {
      FUN_00619730();
      if (param_1 != 0x31000241) {
        FUN_00619730();
        if (param_1 != 0x33000243) {
          FUN_00619730();
          if (param_1 != 0x34000280) {
            FUN_00619730();
            if (param_1 != 0x35000281) goto LAB_004026ed;
          }
        }
      }
    }
  }
  uVar1 = 1;
LAB_004026ed:
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return uVar1;
}

