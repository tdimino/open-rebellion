
void __fastcall FUN_00407f60(void *param_1)

{
  undefined1 auStack_2c [4];
  undefined4 uStack_28;
  undefined4 local_18 [3];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00629a28;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005f2f50(local_18);
  local_4 = 0;
  if ((*(int *)((int)param_1 + 0x9c) != 4) && (*(int *)((int)param_1 + 0x9c) != 5)) {
    uStack_28 = 0x407fa6;
    FUN_00597340();
    FUN_00597360();
  }
  uStack_28 = 0x407fba;
  FUN_00401380(*(undefined4 *)((int)param_1 + 0xa4));
  FUN_005f2f90(auStack_2c,(int)local_18);
  FUN_004080c0(param_1,2,*(int *)((int)param_1 + 0xa4));
  local_4 = 0xffffffff;
  FUN_005f2ff0(local_18);
  ExceptionList = local_c;
  return;
}

