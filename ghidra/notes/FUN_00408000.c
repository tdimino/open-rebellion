
void __fastcall FUN_00408000(void *param_1)

{
  char *pcVar1;
  undefined1 auStack_2c [4];
  undefined4 uStack_28;
  undefined4 local_18 [3];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00629a48;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005f2f50(local_18);
  local_4 = 0;
  pcVar1 = (char *)FUN_00583c40(*(int *)((int)param_1 + *(int *)((int)param_1 + 0xa4) * 4 + 0xc0) +
                                0x98);
  uStack_28 = 0x40804e;
  FUN_005f35e0(local_18,pcVar1);
  FUN_005f2f90(auStack_2c,(int)local_18);
  FUN_00401320(*(undefined4 *)((int)param_1 + 0xa4));
  FUN_005f2f90(auStack_2c,(int)local_18);
  FUN_004080c0(param_1,1,*(int *)((int)param_1 + 0xa4));
  local_4 = 0xffffffff;
  FUN_005f2ff0(local_18);
  ExceptionList = local_c;
  return;
}

